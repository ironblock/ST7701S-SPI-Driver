extern crate spidev;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::Any;
use std::{io::prelude::*};
use std::path::Path;

use crate::st7701s_spi::interface::{Buffer, Location, Reader, WriteData};
use crate::st7701s_spi::state::State;

pub type Packet = [u8; 2];
pub type Sequence<const N: usize> = [Packet; N];

#[repr(u8)]
pub enum DCX {
    Command = 0,
    Parameter = 1
}

pub const THREE_WIRE_OPTIONS: SpidevOptions = SpidevOptions {
    bits_per_word: Some(9),
    max_speed_hz: Some(20_0000),
    lsb_first: Some(false),
    spi_mode: Some(SpiModeFlags::SPI_MODE_0)
};

pub struct ST7701S {
    spi: Spidev,
    pub state: Option<State>,
}

impl ST7701S {
    pub fn new(spi_device: &Path, spi_options: &SpidevOptions, use_state: bool) -> Self {
        let mut spi = match Spidev::open(spi_device) {
            Ok(connection) => connection,
            Err(_) => todo!()
        };

        spi.configure(spi_options);

        let mut state = if use_state {
            Some(State::default())
        } else {
            None
        };

        Self { spi, state }
    }

    pub fn state_is(&mut self, condition: fn(&State) -> bool) -> bool {
        if let Some(current) = &self.state {
            condition(current)
        } else {
            false
        }
    }

    pub fn command<T: Location>(&mut self) {
        self.spi.write(&[DCX::Command as u8, T::ADDRESS]);
    }

    pub fn write<T: WriteData>(&mut self, parameters: T::Parameters) {
        self.command::<T>();

        let data = T::encode(parameters);

        for byte in data {
            self.spi.write( &[DCX::Parameter as u8, byte]);
        }
    }

    pub fn read<T: Location>(&self, _address: u8, _handler: Reader) {
        todo!()
    }
}
