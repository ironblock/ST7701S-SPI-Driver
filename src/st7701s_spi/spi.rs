extern crate spidev;

use log::{info, warn};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::{Any, type_name_of_val};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::prelude::*;
use std::path::Path;
use std::thread::current;

use crate::st7701s_spi::interface::{Command, Reader, WriteData};
use crate::st7701s_spi::state::{State, StateContainer, StateSelector, Switch};

pub type Packet = [u8; 2];
pub type Sequence<const N: usize> = [Packet; N];

#[repr(u8)]
pub enum DCX {
    Command = 0,
    Parameter = 1,
}

pub const THREE_WIRE_OPTIONS: SpidevOptions = SpidevOptions {
    bits_per_word: Some(9),
    max_speed_hz: Some(20_0000),
    lsb_first: Some(false),
    spi_mode: Some(SpiModeFlags::SPI_MODE_0),
};

pub struct ST7701S {
    spi: Spidev,
    pub state: StateContainer,
}

impl ST7701S {
    pub fn new(spi_device: &Path, spi_options: &SpidevOptions, use_state: bool) -> Self {
        let mut spi = match Spidev::open(spi_device) {
            Ok(connection) => connection,
            Err(_) => todo!(),
        };

        spi.configure(spi_options);

        let state = StateContainer(if use_state {
            Some(State::default())
        } else {
            None
        });

        Self { spi, state }
    }

    pub fn command<T: Command>(&mut self) {
        self.spi.write(&[DCX::Command as u8, T::ADDRESS.into()]);

        info!("{} Sent command", T::print_id_tag());
    }

    pub fn do_not_send_command<T: Command>(&self, reason: &str) {
        warn!("{} Did not send command: {}", T::print_id_tag(), reason);
    }

    pub fn write<T: WriteData>(&mut self, parameters: &T::Parameters) {
        self.command::<T>();

        for byte in T::encode(parameters) {
            self.spi.write(&[DCX::Parameter as u8, byte]);
        }

        info!("{} Sent data: {:?}", T::print_id_tag(), parameters);
    }

    pub fn read<T: Command>(&self, _address: u8, _handler: Reader) {
        todo!()
    }

    pub fn switch_command<ON: Command, OFF: Command>(
        &mut self,
        mode: Switch,
        select: StateSelector<Switch>,
    ) {
        if self.state.is(&mode, &select) {
            match mode {
                Switch::On => self.do_not_send_command::<ON>("Already ON"),
                Switch::Off => self.do_not_send_command::<OFF>("Already OFF"),
            }
        } else {
            match mode {
                Switch::On => self.command::<ON>(),
                Switch::Off => self.command::<OFF>(),
            }

            self.state.set(mode, &select);
        }
    }

    pub fn select_command<SELECT: WriteData, OFF: Command>(
        &mut self,
        parameters_option: Option<SELECT::Parameters>,
        select: StateSelector<Option<SELECT::Parameters>>,
    ) {
        if self.state.is(&parameters_option, &select) {
            match parameters_option {
                Some(_) => {
                    self.do_not_send_command::<SELECT>("Provided parameters match current state")
                }
                None => self.do_not_send_command::<OFF>("Already OFF"),
            }
        } else {
            if let Some(parameters) = &parameters_option {
                self.write::<SELECT>(parameters);
            } else {
                self.command::<OFF>();
            }

            self.state.set(parameters_option, &select);
        }
    }
}

pub type TrackKey = &'static str;
pub trait TrackValue: Any + Debug + PartialEq + Clone {}

pub struct StateTracker<const TRACK: bool> {
    pub state: HashMap<TrackKey, Box<dyn Any>>,
}

impl StateTracker<false> {
    fn new() -> Self {
        panic!("Should not instantiate a StateTracker if state tracking is disabled")
    }
}

impl StateTracker<true> {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn reset(&mut self) {
        self.state.clear();
    }

    fn change_is_effective<T>(&self, key: TrackKey, initial: &T, value: &T) -> bool
    where
        T: 'static + Debug + PartialEq<T>,
    {
        const NO_EFFECT: &'static str = "Command will have no effect";
        const MALFORMED: &'static str = "Command is malformed";

        if let Some(prev_any) = self.state.get(key) {
            // Key is found, need to downcast value
            if let Some(prev) = prev_any.downcast_ref::<T>() {
                // Stored value can be compared to input
                if value.eq(&prev) {
                    warn!("{NO_EFFECT}: The state of '{}' is already '{value:?}'", key);
                    return false;
                } else {
                    return true;
                }
            } else {
                warn!(
                    "{MALFORMED}: Could not downcast '{}' to '{:?}', found '{:?}'",
                    key,
                    type_name_of_val(&value),
                    type_name_of_val(&prev_any)
                );
                panic!()
            }
        } else if value == initial {
            warn!(
                "{NO_EFFECT}: Key '{}' is unset, but '{value:?}' is the same as the initial state",
                key
            );
            return false;
        } else {
            return true;
        }
    }

    fn track<T: 'static>(&mut self, key: TrackKey, value: T) {
        self.state.insert(key, Box::new(value));
    }
}

pub trait Transceiver {
    fn tx_command<T: Command>(&mut self);
    fn no_command<T: Command>(&mut self, reason: &str);
    fn tx_write<T: WriteData>(&mut self, parameters: T::Parameters);
    fn no_write<T: WriteData>(&mut self, parameters: T::Parameters, reason: &str);
}

pub trait Stateful<P> {
    const INITIAL: P;
    const ID: TrackKey;
}
pub trait CommandSequence<P, const TRACK: bool>: Stateful<P> {
    fn transmit(
        &self,
        transceiver: &mut impl Transceiver,
        state: &StateTracker<TRACK>,
        parameters: P,
    ) -> ();
}

pub trait Toggleable<ON: Command, OFF: Command> {}

impl<ON: Command, OFF: Command> CommandSequence<Switch, false> for dyn Toggleable<ON, OFF>
where
    Self: Stateful<Switch>,
{
    fn transmit(
        &self,
        transceiver: &mut impl Transceiver,
        _: &StateTracker<false>,
        mode: Switch,
    ) -> () {
        match mode {
            Switch::On => transceiver.tx_command::<ON>(),
            Switch::Off => transceiver.tx_command::<OFF>(),
        }
    }
}

impl<ON: Command, OFF: Command> CommandSequence<Switch, true> for dyn Toggleable<ON, OFF>
where
    Self: Stateful<Switch>,
{
    fn transmit(
        &self,
        transceiver: &mut impl Transceiver,
        state: &StateTracker<true>,
        mode: Switch,
    ) -> () {
        if state.change_is_effective(Self::ID, &Self::INITIAL, &mode) {
            match mode {
                Switch::On => transceiver.tx_command::<ON>(),
                Switch::Off => transceiver.tx_command::<OFF>(),
            }
        } else {
            match mode {
                Switch::On => transceiver.tx_command::<ON>(),
                Switch::Off => transceiver.tx_command::<OFF>(),
            }
        }
    }
}
