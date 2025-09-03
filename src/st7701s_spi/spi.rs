extern crate spidev;

use core::panic;
use log::{info, warn};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::{Any, type_name, type_name_of_val};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::prelude::*;
use std::path::Path;

use crate::st7701s_spi::interface::{BufferDecoder, Command, Reader, Writer};
use crate::st7701s_spi::state::Switch;

pub type Packet = [u8; 2];
pub type Sequence<const N: usize> = [Packet; N];
pub type Reset<T> = fn(&mut T) -> ();

// TODO: This should use cfg
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
    pub state: StateTracker,
}

impl ST7701S {
    pub fn new(spi_device: &Path, spi_options: &SpidevOptions) -> Self {
        let mut spi = match Spidev::open(spi_device) {
            Ok(connection) => connection,
            Err(_) => todo!(),
        };

        spi.configure(spi_options);

        Self {
            spi,
            state: StateTracker::new(),
        }
    }
}

pub trait Transceiver {
    fn tx_command<T: Command>(&mut self) -> Result<(), &'static str>;
    fn no_command<T: Command>(&mut self, reason: &'static str) -> Result<(), &'static str>;
    fn tx_write<T: Writer>(&mut self, parameters: &T::Parameters) -> Result<(), &'static str>;
    fn no_write<T: Writer>(
        &mut self,
        parameters: &T::Parameters,
        reason: &'static str,
    ) -> Result<(), &'static str>;
    fn tx_read<T: Reader>(&mut self, handler: &BufferDecoder) -> Result<(), &'static str>;
    fn no_read<T: Reader>(
        &mut self,
        handler: &BufferDecoder,
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

    fn tx_write<T: Writer>(&mut self, parameters: &T::Parameters) -> Result<(), &'static str> {
        self.tx_command::<T>();

        for byte in T::encode(parameters) {
            self.spi.write(&[DCX::Parameter as u8, byte]);
        }

        info!("{} Sent data: {:?}", T::print_id_tag(), parameters);

        Ok(())
    }

    fn no_write<T: Writer>(
        &mut self,
        _parameters: &T::Parameters,
        _reason: &str,
    ) -> Result<(), &'static str> {
        todo!()
    }

    fn tx_read<T: Reader>(&mut self, _handler: &BufferDecoder) -> Result<(), &'static str> {
        todo!()
    }

    fn no_read<T: Reader>(
        &mut self,
        _handler: &BufferDecoder,
        _reason: &str,
    ) -> Result<(), &'static str> {
        todo!()
    }
}

pub struct StateTracker {
    pub tracker: HashMap<String, Box<dyn Any>>,
}
impl StateTracker {
    pub fn new() -> Self {
        Self {
            tracker: if TRACK_STATE {
                info!("State tracking is ENABLED");
                HashMap::new()
            } else {
                HashMap::with_capacity(0)
            },
        }
    }

    pub fn reset(&mut self) -> () {
        info!("Resetting state tracker");

        self.tracker.clear();
    }

    pub fn set<ITEM: StateItem + ?Sized>(&mut self, value: ITEM::Parameters) {
        self.tracker.insert(ITEM::key(), Box::new(value));
    }

    pub fn get<ITEM: StateItem + ?Sized>(&self) -> Option<&ITEM::Parameters> {
        self.tracker.get(&ITEM::key()).and_then(|current_box| {
            current_box.downcast_ref::<ITEM::Parameters>().or_else(|| {
                panic!(
                    "{MALFORMED}: Expected \"{}\" to be type \"{}\" instead of \"{}\"",
                    ITEM::key(),
                    type_name::<ITEM::Parameters>(),
                    type_name_of_val(current_box)
                );
            })
        })
    }

    pub fn is<ITEM: StateItem + ?Sized>(&self, value: &ITEM::Parameters) -> bool {
        self.get::<ITEM>().is_some_and(|current| current == value)
    }

    pub fn is_unset<ITEM: StateItem + ?Sized>(&self) -> bool {
        self.get::<ITEM>().is_none()
    }

    fn change_is_valid<ITEM: StateItem + ?Sized>(&self, value: &ITEM::Parameters) -> bool {
        if self.is_unset::<ITEM>() && value == &ITEM::initial() {
            warn!(
                "{NO_EFFECT}: State for \"{}\" already has an initial value of '{:?}'",
                ITEM::key(),
                value
            );

            return false;
        }

        if self.is::<ITEM>(&value) {
            format!(
                "{NO_EFFECT}: State for \"{}\" is already '{:?}'",
                ITEM::key(),
                value
            );

            return false;
        }

        return true;
    }
}

pub trait StateItem {
    type Parameters: Debug + PartialEq + 'static;

    fn key() -> String;
    fn initial() -> Self::Parameters;
    fn submit(
        channel: &mut impl Transceiver,
        parameters: &Self::Parameters,
    ) -> Result<(), &'static str>;
    fn reject(
        channel: &mut impl Transceiver,
        parameters: &Self::Parameters,
    ) -> Result<(), &'static str>;
}

pub trait Transmission<P, const STATEFUL: bool> {
    fn transmit(
        channel: &mut impl Transceiver,
        parameters: P,
        state: &mut StateTracker,
    ) -> Result<(), &'static str>;
}

pub trait SideEffect<P: Debug + PartialEq + 'static, const STATEFUL: bool>:
    Transmission<P, STATEFUL>
{
    fn on_transmit_success(
        _channel: &mut impl Transceiver,
        _parameters: P,
        _state: &mut StateTracker,
    ) {
    }

    fn on_transmit_failure(
        _channel: &mut impl Transceiver,
        _parameters: P,
        _state: &mut StateTracker,
    ) {
    }
}

pub trait Momentary<COMMAND: Command> {}

impl<COMMAND: Command> Transmission<(), false> for dyn Momentary<COMMAND> {
    fn transmit(
        channel: &mut impl Transceiver,
        _: (),
        _: &mut StateTracker,
    ) -> Result<(), &'static str> {
        channel.tx_command::<COMMAND>()
    }
}

pub trait Toggle<ON: Command, OFF: Command> {}
impl<ON: Command, OFF: Command> StateItem for dyn Toggle<ON, OFF> {
    type Parameters = Switch;

    fn key() -> String {
        format!("Toggle<{}, {}>", ON::NAME, OFF::NAME)
    }

    fn initial() -> Self::Parameters {
        Switch::Off
    }

    fn submit(channel: &mut impl Transceiver, mode: &Self::Parameters) -> Result<(), &'static str> {
        match mode {
            Switch::On => channel.tx_command::<ON>(),
            Switch::Off => channel.tx_command::<OFF>(),
        }
    }

    fn reject(channel: &mut impl Transceiver, mode: &Self::Parameters) -> Result<(), &'static str> {
        match mode {
            Switch::On => channel.no_command::<ON>("Already ON"),
            Switch::Off => channel.no_command::<OFF>("Already OFF"),
        }
    }
}

pub trait Select<SELECT: Writer, OFF: Command> {}
impl<SELECT: Writer, OFF: Command> StateItem for dyn Select<SELECT, OFF> {
    type Parameters = Option<SELECT::Parameters>;

    fn key() -> String {
        format!("Select<{}, {}>", SELECT::NAME, OFF::NAME)
    }

    fn initial() -> Self::Parameters {
        Some(SELECT::INITIAL)
    }

    fn submit(
        channel: &mut impl Transceiver,
        parameters: &Self::Parameters,
    ) -> Result<(), &'static str> {
        if let Some(parameters) = &parameters {
            channel.tx_write::<SELECT>(parameters)
        } else {
            channel.tx_command::<OFF>()
        }
    }

    fn reject(
        channel: &mut impl Transceiver,
        parameters: &Self::Parameters,
    ) -> Result<(), &'static str> {
        match parameters {
            Some(_) => channel.no_command::<SELECT>("Provided parameters match current state"),
            None => channel.no_command::<OFF>("Already OFF"),
        }
    }
}

pub trait Configure<CONFIGURE> where CONFIGURE: Writer {}
impl<CONFIGURE: Writer> StateItem for dyn Configure<CONFIGURE> {
    type Parameters = CONFIGURE::Parameters;

    fn key() -> String {
        format!("Configure<{}>", CONFIGURE::NAME)
    }

    fn initial() -> Self::Parameters {
        CONFIGURE::INITIAL
    }

    fn submit(
        channel: &mut impl Transceiver,
        parameters: &Self::Parameters,
    ) -> Result<(), &'static str> {
        channel.tx_write::<CONFIGURE>(parameters)
    }

    fn reject(
        channel: &mut impl Transceiver,
        parameters: &Self::Parameters,
    ) -> Result<(), &'static str> {
        channel.no_write::<CONFIGURE>(parameters, "Provided parameters match current state")
    }
}

impl<ITEM: StateItem> Transmission<ITEM::Parameters, true> for ITEM {
    fn transmit(
        channel: &mut impl Transceiver,
        parameters: ITEM::Parameters,
        state: &mut StateTracker,
    ) -> Result<(), &'static str> {
        if TRACK_STATE {
            if state.change_is_valid::<ITEM>(&parameters) {
                let submit = ITEM::submit(channel, &parameters);
                state.set::<ITEM>(parameters);

                submit
            } else {
                ITEM::reject(channel, &parameters)
            }
        } else {
            ITEM::submit(channel, &parameters)
        }
    }
}
