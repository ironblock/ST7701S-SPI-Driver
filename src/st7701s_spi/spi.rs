extern crate spidev;

use log::{info, warn};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::prelude::*;
use std::marker::PhantomData;
use std::path::Path;

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

        info!("{} Sent command", T::id_tag());
    }

    pub fn do_not_send_command<T: Command>(&self, reason: &str) {
        warn!("{} Did not send command: {}", T::id_tag(), reason);
    }

    pub fn write<T: WriteData>(&mut self, parameters: &T::Parameters) {
        self.command::<T>();

        for byte in T::encode(parameters) {
            self.spi.write(&[DCX::Parameter as u8, byte]);
        }

        info!("{} Sent data: {:?}", T::id_tag(), parameters);
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

pub trait Resetable {
    fn get_id(&self) -> &'static str;
    fn reset(&mut self) -> ();
}

pub struct StateItem<T> {
    pub id: &'static str,
    pub current: Option<T>,
    initial: T,
}

impl <T: Clone> StateItem<T> {
    pub fn new(id: &'static str, initial: T) -> Self {
        Self {
            current: Self::initial_value(&initial),
            initial,
            id,
        }
    }

    fn initial_value(initial: &T) -> Option<T> {
        Some(initial.clone())
    }
}

impl<T: Clone> Resetable for StateItem<T> {
    fn get_id(&self) -> &'static str {
        &self.id
    }

    fn reset(&mut self) -> () {
        self.current = Self::initial_value(&self.initial);
    }
}

// pub struct StatefulEndpoint<const ENABLED: bool, T>(Option<StateItem<T>>);

// impl<const ENABLED: bool, T> StatefulEndpoint<ENABLED, T> {
//     const fn new() -> Self {
//         Self(if ENABLED { StateItem})
//     }
// }

pub struct ChangeTracker {
    pub changed: HashMap<&'static str, &'static mut dyn Resetable>,
}

impl ChangeTracker {
    pub fn new() -> Self {
        Self {
            changed: HashMap::new(),
        }
    }

    pub fn track(&mut self, item: &'static mut dyn Resetable) {
            self.changed.insert(item.get_id(), item);
    }

    pub fn reset(&mut self) {
            for (_, item) in self.changed.drain() {
                item.reset()
        }
    }
}

pub trait Transceiver {
    fn tx_command<T: Command>(&mut self);
    fn no_command<T: Command>(&mut self, reason: &str);
    fn tx_write<T: WriteData>(&mut self, parameters: T::Parameters);
    fn no_write<T: WriteData>(&mut self, parameters: T::Parameters, reason: &str);
}

pub trait Transmission {
    fn transmit(&mut self, handler: impl Transceiver, data: impl PartialEq);
}

pub struct ToggleCommand<ON: Command, OFF: Command>(Option<StateItem<Switch>>, PhantomData<ON>, PhantomData<OFF>);

impl<ON: Command, OFF: Command> Transmission for ToggleCommand<ON, OFF> {
    fn transmit(&mut self, handler: impl Transceiver, data: impl PartialEq) {
        if let Some(state) = &mut self.0 {

        }
    }
}


    pub fn switch_command<ON: Command, OFF: Command>(
        &mut self,
        mode: Switch,
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