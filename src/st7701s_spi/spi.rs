extern crate spidev;

use log::error;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::io;
use std::io::prelude::*;

use crate::st7701s_spi::commands::OldCommand;

pub struct HalfDuplexSPI {
    spi: Spidev,
    options: SpidevOptions,
}

impl HalfDuplexSPI {
    pub fn create_spi(device: String, options: &SpidevOptions) -> io::Result<Spidev> {
        let mut spi = Spidev::open(device)?;
        spi.configure(options)?;
        Ok(spi)
    }

    pub fn new(device: String) -> HalfDuplexSPI {
        let options = SpidevOptions::new()
            .bits_per_word(9)
            .lsb_first(false)
            .max_speed_hz(20_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        let spi = HalfDuplexSPI::create_spi(device, &options).unwrap();

        HalfDuplexSPI { options, spi }
    }

    pub fn write_command(&mut self, command: Result<OldCommand, &'static str>) {
        match command {
            Ok(c) => {
                self.spi
                    .write(&c.serialize_address())
                    .unwrap_or_else(|_| panic!("Failed to write to address {:#04X}", c.address));

                for parameter in c.parameters {
                    self.spi
                        .write(&OldCommand::serialize_parameter(parameter))
                        .unwrap_or_else(|_| panic!("Failed to write parameter {parameter:#04X}"));
                }
            }
            Err(e) => error!("{e}"),
        }
    }

    pub fn read_command(&mut self, command: Result<OldCommand, &'static str>) {
        match command {
            Ok(c) => {
                let mut rx_buf = [0_u8; 10];
                self.spi
                    .write(&c.serialize_address())
                    .unwrap_or_else(|_| panic!("Failed to write to address {:#04X}", c.address));
                self.spi
                    .read(&mut rx_buf)
                    .unwrap_or_else(|_| panic!("Failed to read from address {:#04X}", c.address));
                println!("{rx_buf:?}");
            }
            Err(e) => println!("{e}"),
        }
    }
}
