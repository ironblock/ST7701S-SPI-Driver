extern crate spidev;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{io::prelude::*};
use std::path::Path;

use crate::st7701s_spi::interface::{Buffer, Reader};

#[repr(u8)]
#[rustfmt::skip]
enum DCX {
    Command   = 0,
    Parameter = 1
}


pub const THREE_WIRE_OPTIONS: SpidevOptions = SpidevOptions {
    bits_per_word: Some(9),
    max_speed_hz: Some(20_0000),
    lsb_first: Some(false),
    spi_mode: Some(SpiModeFlags::SPI_MODE_0)
};

pub struct ST7701S {
    pub spi: Spidev,
}

impl ST7701S {
    fn new(path: &Path, options: &SpidevOptions) -> Self {
        Self {
            spi: Self::connect(path, options) }
    }

    fn connect(path: &Path, options: &SpidevOptions) -> Spidev {
        match Spidev::open(path) {
            Ok(connection) => connection,
            Err(_) => todo!()
        }
    }

    fn command(&self, address: u8) {
        self.spi.write(&[DCX::Command as u8, address]);
    }

    fn write_data<const N: usize>(&self, address: u8, data: Buffer<N>) {
        self.command(address);

        for byte in data {
            self.spi.write(&[DCX::Parameter as u8, byte]);
        }
    }

    fn read_data(&self, _address: u8, _handler: Reader) {
        todo!()
    }
}
