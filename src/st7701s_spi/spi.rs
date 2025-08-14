extern crate spidev;

use log::{info, warn};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::io::prelude::*;
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
