extern crate spidev;

use crate::instructions::Command;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::io;
use std::io::prelude::*;

pub struct ST7701S {
  spi: Spidev,
}

impl ST7701S {
  pub fn create_spi(device: String, options: &SpidevOptions) -> io::Result<Spidev> {
    let mut spi = Spidev::open(device)?;
    spi.configure(options)?;
    Ok(spi)
  }

  pub fn new(device: String) -> ST7701S {
    let options = SpidevOptions::new()
      .bits_per_word(9)
      .lsb_first(false)
      .max_speed_hz(20_000)
      .mode(SpiModeFlags::SPI_MODE_0)
      .build();
    let spi = ST7701S::create_spi(device, &options).unwrap();

    ST7701S { spi }
  }

  pub fn write_command(&mut self, command: Result<Command, &'static str>) {
    match command {
      Ok(c) => {
        if let Err(e) = self.spi.write(&c.serialize_address()) { println!("{}", e) };

        #[cfg(debug_assertions)]
        println!("Address:   {:#04X}", c.address);

        for parameter in c.parameters {
          #[cfg(debug_assertions)]
          println!("Parameter: {:08b}", parameter);

          if let Err(e) = self.spi.write(&Command::serialize_parameter(parameter)) { println!("{}", e) };
        }

        #[cfg(debug_assertions)]
        println!("--------------------");
      }
      Err(e) => println!("{}", e),
    }
  }

  pub fn read_command(&mut self, command: Result<Command, &'static str>) {
    match command {
      Ok(c) => {
        let mut rx_buf = [0_u8; 10];
        if let Err(e) = self.spi.write(&c.serialize_address()) { println!("{}", e) };

        if let Err(e) = self.spi.read(&mut rx_buf) { println!("{}", e) };

        println!("{:?}", rx_buf);
      }
      Err(e) => println!("{}", e),
    }
  }
}
