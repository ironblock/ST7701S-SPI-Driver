extern crate spidev;

use log::{error, info, warn};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::{Any, type_name, type_name_of_val};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::prelude::*;
use std::path::Path;

use crate::st7701s_spi::interface::{Command, ReadData, Reader, WriteData};
use crate::st7701s_spi::state::{State, StateContainer, Switch};

pub type Packet = [u8; 2];
pub type Sequence<const N: usize> = [Packet; N];
pub type Reset<T> = fn(&mut T) -> ();

pub const TRACK_STATE: bool = option_env!("TRACK_STATE").is_some();

const NO_EFFECT: &'static str = "Command will have no effect";
const MALFORMED: &'static str = "Command is malformed";
const NOT_TRACKING: &'static str = "STATE TRACKING IS DISABLED";

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

pub struct StateTracker {
    state: HashMap<&'static str, Box<dyn Any>>,
}
impl StateTracker {
    pub fn new() -> Self {
        let state = if TRACK_STATE {
            HashMap::new()
        } else {
            HashMap::with_capacity(0)
        };

        Self { state }
    }

    fn downcast_error<P>(&self, key: &'static str, value: &dyn Any) -> Result<(), String> {
        let message = format!(
            "{MALFORMED}: State value for \"{key}\" is not of expected type. Expected: {}, Found: {}",
            type_name::<P>(),
            type_name_of_val(value)
        );

        error!("{message}");

        return Err(message);
    }

    fn changes_current<P>(&self, key: &'static str, value: &P) -> bool
    where
        P: Debug + PartialEq + 'static,
    {
        if let Some(current_box) = self.state.get(key) {
            if let Some(current) = current_box.downcast_ref::<P>() {
                if current == value {
                    warn!("{NO_EFFECT}: The current state for \"{key}\" is already '{value:?}'");
                    return false;
                }
            } else {
                self.downcast_error::<P>(key, current_box);
                return false;
            }
        }

        true
    }

    fn changes_initial<P>(&self, key: &'static str, value: &P, initial: &P) -> bool
    where
        P: Debug + PartialEq,
    {
        if value == initial {
            warn!(
                "{NO_EFFECT}: State for \"{key}\" is unset, but '{value:?}' is the same as the initial state"
            );
            return false;
        }

        true
    }

    pub fn change_is_valid<P>(&self, key: &'static str, initial: &P, value: &P) -> bool
    where
        P: Debug + PartialEq + 'static,
    {
        if TRACK_STATE {
            return self.changes_current(key, value) || self.changes_initial(key, value, initial);
        } else {
            error!("{NOT_TRACKING}: CANNOT CHECK VALUE");
            return true;
        }
    }

    fn reset(&mut self) {
        if TRACK_STATE {
            info!("Resetting state tracker");
            self.state.clear();
        } else {
            error!("{NOT_TRACKING}: RESETTING IS NOT POSSIBLE");
        }
    }

    fn track<P>(&mut self, key: &'static str, value: P)
    where
        P: Debug + PartialEq + 'static,
    {
        if TRACK_STATE {
            self.state.insert(key, Box::new(value));
        } else {
            error!("{NOT_TRACKING}: CANNOT TRACK VALUE");
        }
    }
}

pub trait StateConstants<P: Debug + PartialEq> {
    const ID: &'static str;
    const INITIAL: P;
}

pub trait Stateful<P: Debug + PartialEq>{
    fn submit(channel: &mut impl Transceiver, parameters: &P) -> Result<(), &'static str>;
    fn reject(channel: &mut impl Transceiver, parameters: &P) -> Result<(), &'static str>;
}

pub trait Transmission<P: Debug + PartialEq> {
    fn transmit(
        &mut self,
        channel: &mut impl Transceiver,
        parameters: P,
        state: &mut StateTracker,
    ) -> Result<(), &'static str>;
}

impl<T, P> Transmission<P> for T
where
    T: StateConstants<P> + Stateful<P>,
    P: Debug + PartialEq + 'static,
{
    fn transmit(
        &mut self,
        channel: &mut impl Transceiver,
        parameters: P,
        tracker: &mut StateTracker,
    ) -> Result<(), &'static str> {
        if TRACK_STATE {
            if tracker.change_is_valid(Self::ID, &Self::INITIAL, &parameters) {
                let submit = T::submit(channel, &parameters);
                tracker.track(Self::ID, parameters);

                submit
            } else {
                T::reject(channel, &parameters)
            }
        } else {
            T::submit(channel, &parameters)
        }
    }
}


pub type Toggle<ON: Command, OFF: Command> = (ON, OFF);
impl<ON: Command, OFF: Command> Stateful<Switch> for Toggle<ON, OFF> {
    fn submit(channel: &mut impl Transceiver, mode: &Switch) -> Result<(), &'static str> {
        match mode {
            Switch::On => channel.tx_command::<ON>(),
            Switch::Off => channel.tx_command::<OFF>(),
        }
    }

    fn reject(channel: &mut impl Transceiver, mode: &Switch) -> Result<(), &'static str> {
        match mode {
            Switch::On => channel.no_command::<ON>("Already ON"),
            Switch::Off => channel.no_command::<OFF>("Already OFF"),
        }
    }
}

pub type Select<SELECT: WriteData, OFF: Command> = (SELECT, OFF);
impl<SELECT: WriteData, OFF: Command> Stateful<Option<SELECT::Parameters>>
    for Select<SELECT, OFF>
{
    fn submit(
        channel: &mut impl Transceiver,
        parameters: &Option<SELECT::Parameters>,
    ) -> Result<(), &'static str> {
        if let Some(parameters) = &parameters {
            channel.tx_write::<SELECT>(parameters)
        } else {
            channel.tx_command::<OFF>()
        }
    }

    fn reject(
        channel: &mut impl Transceiver,
        parameters: &Option<SELECT::Parameters>,
    ) -> Result<(), &'static str> {
        match parameters {
            Some(_) => channel.no_command::<SELECT>("Provided parameters match current state"),
            None => channel.no_command::<OFF>("Already OFF"),
        }
    }
}

pub type Configure<CONFIGURE: WriteData> = (CONFIGURE,);
impl<CONFIGURE: WriteData> Stateful<CONFIGURE::Parameters>
    for Configure<CONFIGURE>
{
    fn submit(
        channel: &mut impl Transceiver,
        parameters: &CONFIGURE::Parameters,
    ) -> Result<(), &'static str> {
        channel.tx_write::<CONFIGURE>(parameters)
    }

    fn reject(
        channel: &mut impl Transceiver,
        parameters: &CONFIGURE::Parameters,
    ) -> Result<(), &'static str> {
        channel.no_write::<CONFIGURE>(parameters, "Provided parameters match current state")
    }
}
