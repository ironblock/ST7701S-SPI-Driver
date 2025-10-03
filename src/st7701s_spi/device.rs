extern crate spidev;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{
    any::Any,
    io::{self, Write},
    path::Path,
};

use crate::st7701s_spi::{
    address::{CommandInstruction, DataBuffer, Location, ReadInstruction, WriteInstruction},
    state::{DeviceState, StatefulDevice},
};

pub enum DCX {
    Command = 0x00,
    Parameter = 0x01,
}

pub trait Protocol {
    fn tx_command(&mut self, location: Location) -> io::Result<usize>;
    fn tx_parameters(&mut self, parameters: impl DataBuffer) -> io::Result<usize>;
}

/// This is a 3-wire SPI implementation. Reads and writes share the SDA pin and
/// are performed half-duplex
pub struct Spi3Wire {
    device: Spidev,
}
impl Spi3Wire {
    pub const DEFAULT_OPTIONS: SpidevOptions = SpidevOptions {
        bits_per_word: Some(9),
        max_speed_hz: Some(20_0000),
        lsb_first: Some(false),
        spi_mode: Some(SpiModeFlags::SPI_MODE_0),
    };

    pub fn new(spi_device: &Path, spi_options: &SpidevOptions) -> Self {
        let mut device = Spidev::open(spi_device).expect("Failed to open SPI device");

        device
            .configure(spi_options)
            .expect("Failed to configure SPI device");

        Self { device }
    }
}

impl Protocol for Spi3Wire {
    fn tx_command(&mut self, location: Location) -> io::Result<usize> {
        self.device.write(&[DCX::Command as u8, location.as_u8()])
    }

    fn tx_parameters(&mut self, parameters: impl DataBuffer) -> io::Result<usize> {
        let mut written: usize = 0;

        for byte in parameters.into_iter() {
            written += self.device.write(&[DCX::Parameter as u8, byte])?;
        }

        io::Result::Ok(written)
    }
}

pub trait Transciever {
    fn connection(&mut self) -> &mut impl Protocol;

    fn command<C: CommandInstruction>(&mut self) -> io::Result<usize> {
        self.connection().tx_command(C::LOCATION)
    }

    fn write<W: WriteInstruction>(&mut self, parameters: W::Buffer) -> io::Result<usize> {
        let mut written: usize = 0;

        written += self.connection().tx_command(W::LOCATION)?;
        written += self.connection().tx_parameters(parameters)?;

        io::Result::Ok(written)
    }

    fn read<R: ReadInstruction, const N: usize>(
        &mut self,
        _reader: for<'a> fn(&'a [u8]) -> dyn Any,
    ) -> io::Result<usize> {
        todo!()
    }
}

pub struct ST7701S<P: Protocol>
where
    Self: Transciever,
{
    connection: P,
    state: DeviceState,
}
impl Transciever for ST7701S<Spi3Wire> {
    fn connection(&mut self) -> &mut impl Protocol {
        &mut self.connection
    }
}
impl StatefulDevice for ST7701S<Spi3Wire> {
    fn state(&self) -> &DeviceState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut DeviceState {
        &mut self.state
    }
}
impl ST7701S<Spi3Wire> {
    pub fn new(spi_device: &Path, spi_options: &SpidevOptions) -> Self {
        let mut spi = Spidev::open(spi_device).expect("Failed to open SPI device");
        spi.configure(spi_options)
            .expect("Failed to configure SPI device");

        Self {
            connection: Spi3Wire::new(spi_device, spi_options),
            state: DeviceState::new(),
        }
    }
}
