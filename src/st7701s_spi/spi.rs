extern crate spidev;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::any::Any;
use std::{io::prelude::*};
use std::path::Path;

use crate::st7701s_spi::interface::{Buffer, Reader};

#[repr(u8)]
#[rustfmt::skip]
enum DCX {
    Command   = 0b0,
    Parameter = 0b1
}

pub trait Protocol {
    const OPTIONS: SpidevOptions;

    fn connect(path: &Path) -> Spidev {
        let mut connection = Spidev::open(path).unwrap();
        connection.configure(&Self::OPTIONS);

        connection
    }

    fn transmit_command(spi: &mut Spidev, address: u8) {
        spi.write(&[DCX::Command as u8, address]);
    }

    fn transmit_write<const N: usize>(spi: &mut Spidev, address: u8, data: Buffer<N>) {
        Self::transmit_command(spi, address);

        for byte in data {
            spi.write(&[DCX::Parameter as u8, byte]);
        }
    }

    fn transmit_read(_spi: &mut Spidev, _address: u8, _handler: Reader) {
        todo!()
    }

    fn enqueue(spi: &mut Spidev, queue: InstructionQueue) {
        for transmission in queue {
            match transmission.operation {
                Direction::Command => Self::transmit_command(spi, transmission.address),
                Direction::TX(data) => Self::transmit_write(spi, transmission.address, data),
                Direction::RX(handler) => Self::transmit_read(spi, transmission.address, handler)
            }
        }
    }
}

struct ThreeWireConnection(Spidev);

impl Protocol for ThreeWireConnection {
    const OPTIONS: SpidevOptions = SpidevOptions {
        bits_per_word: Some(9),
        max_speed_hz: Some(20_0000),
        lsb_first: Some(false),
        spi_mode: Some(SpiModeFlags::SPI_MODE_0)
    };
}

impl ThreeWireConnection {
    fn new(path: &Path) -> Self {
        Self(Self::connect(path))
    }
}

