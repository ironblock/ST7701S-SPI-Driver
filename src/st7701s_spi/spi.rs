extern crate spidev;

use log::{error, info, warn};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::prelude::*;
use std::marker::PhantomData;
use std::path::Path;

use crate::st7701s_spi::interface::{Command, ReadData, Reader, WriteData};
use crate::st7701s_spi::state::{State, StateContainer, StateSelector, Switch};

pub type Packet = [u8; 2];
pub type Sequence<const N: usize> = [Packet; N];
pub type TrackKey = &'static str;

pub const TRACK_STATE: bool = option_env!("TRACK_STATE").is_some();

const NO_EFFECT: &'static str = "Command will have no effect";
const MALFORMED: &'static str = "Command is malformed";

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
    pub fn new(spi_device: &Path, spi_options: &SpidevOptions) -> Self {
        let mut spi = match Spidev::open(spi_device) {
            Ok(connection) => connection,
            Err(_) => todo!(),
        };

        spi.configure(spi_options);

        let state = StateContainer(if TRACK_STATE {
            Some(State::default())
        } else {
            None
        });

        Self { spi, state }
    }
}

pub trait Transceiver {
    fn tx_command<T: Command>(&mut self) -> Result<(), &'static str>;
    fn no_command<T: Command>(&mut self, reason: &'static str) -> Result<(), &'static str>;
    fn tx_write<T: WriteData>(&mut self, parameters: &T::Parameters) -> Result<(), &'static str>;
    fn no_write<T: WriteData>(
        &mut self,
        parameters: &T::Parameters,
        reason: &'static str,
    ) -> Result<(), &'static str>;
    fn tx_read<T: ReadData>(&mut self, handler: &Reader) -> Result<(), &'static str>;
    fn no_read<T: ReadData>(
        &mut self,
        handler: &Reader,
        reason: &'static str,
    ) -> Result<(), &'static str>;
}

impl Transceiver for ST7701S {
    fn tx_command<T: Command>(&mut self) -> Result<(), &'static str> {
        self.spi.write(&[DCX::Command as u8, T::ADDRESS.into()]);

        info!("{} Sent command", T::print_id_tag());
        Ok(())
    }

    fn no_command<T: Command>(&mut self, reason: &'static str) -> Result<(), &'static str> {
        warn!("{} Did not send command: {}", T::print_id_tag(), reason);
        Err(reason)
    }

    fn tx_write<T: WriteData>(&mut self, parameters: &T::Parameters) -> Result<(), &'static str> {
        self.tx_command::<T>();

        for byte in T::encode(parameters) {
            self.spi.write(&[DCX::Parameter as u8, byte]);
        }

        info!("{} Sent data: {:?}", T::print_id_tag(), parameters);

        Ok(())
    }

    fn no_write<T: WriteData>(
        &mut self,
        _parameters: &T::Parameters,
        _reason: &str,
    ) -> Result<(), &'static str> {
        todo!()
    }

    fn tx_read<T: Command>(&mut self, _handler: &Reader) -> Result<(), &'static str> {
        todo!()
    }

    fn no_read<T: Command>(
        &mut self,
        _handler: &Reader,
        _reason: &str,
    ) -> Result<(), &'static str> {
        todo!()
    }
}


pub struct StateTracker<const TRACK: bool = TRACK_STATE> {
    pub state: HashSet<fn()>,
}

impl StateTracker<false> {
    fn new() -> Self {
        Self {
            state: HashSet::with_capacity(0),
        }
    }

    fn reset(&mut self) {}
    fn track<T>(&mut self, _value: T) {}
}

impl StateTracker<true> {
    fn new() -> Self {
        Self {
            state: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.state.drain().for_each(|reset| reset());
    }

    fn track(&mut self, value: fn()) {
        self.state.insert(value);
    }
}

struct StateItem<P, const TRACK: bool = TRACK_STATE>
where
    P: Debug + PartialEq,
{
    name: String,
    pub initial: P,
    pub current: Option<P>,
}

impl<P, const TRACK: bool> StateItem<P, TRACK>
where
    P: Debug + PartialEq,
{
    pub const fn new(initial: P, name: String) -> Self {
        Self {
            name,
            initial,
            current: None,
        }
    }
}

impl<P> StateItem<P, false>
where
    P: Debug + PartialEq,
{
    pub fn change_is_valid(&self, _value: &P) -> bool {
        error!("STATE TRACKING IS DISABLED, CHECKING FOR CHANGES IS NOT POSSIBLE");
        true
    }

    pub fn set(&mut self, _: P) {}
    pub fn reset(&mut self) {}
}

impl<T> StateItem<T, true>
where
    T: Debug + PartialEq,
{
    fn current_value_already_is(&self, value: &T) -> bool {
        self.current.as_ref().is_some_and(|x| x == value)
    }

    fn unset_value_but_initial_is(&self, value: &T) -> bool {
        self.current.is_none() && &self.initial != value
    }

    fn change_is_valid(&self, value: &T) -> bool {
        if self.current_value_already_is(&value) {
            warn!(
                "{NO_EFFECT}: The current state for '{}' is already '{value:?}'",
                self.name
            );
            return false;
        }

        if self.current.is_none() && &self.initial != value {
            warn!(
                "{}: State for '{}' is unset, but '{value:?}' is the same as the initial state",
                NO_EFFECT, self.name
            );
            return false;
        }

        return true;
    }

    pub fn set(&mut self, value: T) {
        self.current = Some(value);
    }

    pub fn reset(&mut self) {
        self.current = None;
    }
}

pub trait StatefulTransmission<P>
where
    P: PartialEq + Debug,
{
    fn new(initial: P) -> Self;
    fn submit(&self, channel: &mut impl Transceiver, parameters: &P) -> Result<(), &'static str>;
    fn reject(&self, channel: &mut impl Transceiver, parameters: &P) -> Result<(), &'static str>;

    fn state(&mut self) -> &mut StateItem<P>;
    fn reset(&mut self) {
        self.state().reset();
    }
}

pub trait Transmission<P> {
    fn transmit(
        &mut self,
        channel: &mut impl Transceiver,
        parameters: P,
        tracker: &mut StateTracker,
    ) -> Result<(), &'static str>;
}

impl<T, P> Transmission<P> for T
where
    T: StatefulTransmission<P>,
    P: PartialEq + Debug,
{
    fn transmit(
        &mut self,
        channel: &mut impl Transceiver,
        parameters: P,
        tracker: &mut StateTracker,
    ) -> Result<(), &'static str> {
        if TRACK_STATE {
            if self.state().change_is_valid(&parameters) {
                let submit = self.submit(channel, &parameters);
                self.state().set(parameters);
                tracker.track(<Self as StatefulTransmission<P>>::reset);

                submit
            } else {
                self.reject(channel, &parameters)
            }
        } else {
            self.submit(channel, &parameters)
        }
    }
}

pub struct Toggle<ON: Command, OFF: Command> {
    pub state: StateItem<Switch>,
    _on: PhantomData<ON>,
    _off: PhantomData<OFF>,
}

impl<ON: Command, OFF: Command> StatefulTransmission<Switch> for Toggle<ON, OFF> {
    fn new(initial: Switch) -> Self {
        let name = format!("Toggle<{}, {}>", ON::NAME, OFF::NAME);

        Self {
            state: StateItem::new(initial, name),
            _on: PhantomData,
            _off: PhantomData,
        }
    }

    fn submit(&self, channel: &mut impl Transceiver, mode: &Switch) -> Result<(), &'static str> {
        match mode {
            Switch::On => channel.tx_command::<ON>(),
            Switch::Off => channel.tx_command::<OFF>(),
        }
    }

    fn reject(&self, channel: &mut impl Transceiver, mode: &Switch) -> Result<(), &'static str> {
        match mode {
            Switch::On => channel.no_command::<ON>("Already ON"),
            Switch::Off => channel.no_command::<OFF>("Already OFF"),
        }
    }

    fn state(&mut self) -> &mut StateItem<Switch> {
        &mut self.state
    }
}

pub struct Select<SELECT: WriteData, OFF: Command> {
    pub state: StateItem<Option<SELECT::Parameters>>,
    _select: PhantomData<SELECT>,
    _off: PhantomData<OFF>,
}

impl<SELECT: WriteData, OFF: Command> StatefulTransmission<Option<SELECT::Parameters>> for Select<SELECT, OFF> {
    fn new(initial: Option<SELECT::Parameters>) -> Self {
        let name = format!("Toggle<{}, {}>", SELECT::NAME, OFF::NAME);

        Self {
            state: StateItem::new(initial, name),
            _select: PhantomData,
            _off: PhantomData,
        }
    }

    fn submit(&self, channel: &mut impl Transceiver, parameters: &Option<SELECT::Parameters>) -> Result<(), &'static str> {

            if let Some(parameters) = &parameters {
                channel.tx_write::<SELECT>(parameters)
            } else {
                channel.tx_command::<OFF>()
            }
    }

    fn reject(&self, channel: &mut impl Transceiver, parameters: &Option<SELECT::Parameters>) -> Result<(), &'static str> {

            match parameters {
                Some(_) => channel.no_command::<SELECT>("Provided parameters match current state"),
                None => channel.no_command::<OFF>("Already OFF"),
            }
    }

    fn state(&mut self) -> &mut StateItem<Option<SELECT::Parameters>> {
        &mut self.state
    }
}

pub struct Configure<CONFIGURE: WriteData> {
    pub state: StateItem<CONFIGURE::Parameters>,
    _configure: PhantomData<CONFIGURE>,
}

impl<CONFIGURE: WriteData> StatefulTransmission<CONFIGURE::Parameters> for Configure<CONFIGURE> {
    fn new(initial: CONFIGURE::Parameters) -> Self {
        let name = format!("Configure<{}>", CONFIGURE::NAME);

        Self {
            state: StateItem::new(initial, name),
            _configure: PhantomData,
        }
    }

    fn submit(&self, channel: &mut impl Transceiver, parameters: &CONFIGURE::Parameters) -> Result<(), &'static str> {
        channel.tx_write::<CONFIGURE>(parameters)
    }

    fn reject(&self, channel: &mut impl Transceiver, parameters: &CONFIGURE::Parameters) -> Result<(), &'static str> {

        channel.no_write::<CONFIGURE>(
            parameters,
            "Provided parameters match current state",
        )
    }

    fn state(&mut self) -> &mut StateItem<CONFIGURE::Parameters> {
        &mut self.state
    }
}
