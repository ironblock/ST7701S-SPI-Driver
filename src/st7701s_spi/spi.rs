extern crate spidev;

use log::warn;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::{Any, type_name};
use std::io::prelude::*;
use std::path::Path;

use crate::st7701s_spi::interface::{Command, Data, Reader, WriteData};
use crate::st7701s_spi::state::{SelectField, State, Switch};

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
    pub state: Option<State>,
}

impl ST7701S {
    pub fn new(spi_device: &Path, spi_options: &SpidevOptions, use_state: bool) -> Self {
        let mut spi = match Spidev::open(spi_device) {
            Ok(connection) => connection,
            Err(_) => todo!(),
        };

        spi.configure(spi_options);

        if use_state {
            Self {
                spi,
                state: Some(State::default()),
            }
        } else {
            Self { spi, state: None }
        }
    }

    pub fn already_in_state<T>(&mut self, next_value: &T, select: &SelectField<T>) -> bool
    where
        T: PartialEq,
    {
        self.state
            .as_mut()
            .is_some_and(|state| *select(state) == *next_value)
    }

    pub fn mutate_state<T>(&mut self, target: T, select: &SelectField<T>) {
        if let Some(state) = &mut self.state {
            *select(state) = target;
        }
    }

    pub fn command<T: Command>(&mut self) {
        self.spi.write(&[DCX::Command as u8, T::ADDRESS]);
    }

    pub fn write<T: WriteData>(&mut self, parameters: T::Parameters) {
        self.command::<T>();

        for byte in T::encode(parameters) {
            self.spi.write(&[DCX::Parameter as u8, byte]);
        }
    }

    pub fn read<T: Command>(&self, _address: u8, _handler: Reader) {
        todo!()
    }

    pub fn switch_command<ON: Command, OFF: Command>(
        &mut self,
        mode: Switch,
        select: SelectField<Switch>,
    ) {
        if self.already_in_state(&mode, &select) {
            warn!(
                "Did not send command ({0:?}/{1:?}), mode is already {mode:?}",
                type_name::<ON>(),
                type_name::<OFF>()
            );
            return;
        }

        match mode {
            Switch::On => self.command::<ON>(),
            Switch::Off => self.command::<OFF>(),
        };

        self.mutate_state(mode, &select);
    }

    pub fn select_command<SELECT: Data, OFF: Command>(
        &mut self,
        parameters: Option<SELECT::Parameters>,
        select: SelectField<Option<SELECT::Parameters>>,
    ) {
        if self.already_in_state(&parameters, &select) {
            warn!("Did not send command")
        }

        if parameters.is_some() {
            self.write::<SELECT>(parameters);
        } else {
            self.write::<OFF>();
        }
    }
}
