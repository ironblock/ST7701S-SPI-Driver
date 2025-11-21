use std::{
    error::Error,
    fmt::{Debug, Formatter},
    fs::File,
    io,
    mem::MaybeUninit,
    os::fd::AsRawFd,
    path::Path,
    sync::{Arc, Mutex},
};

use linux_embedded_hal::{
    SPIError, SpidevDevice,
    spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer},
};

use crate::st7701s_spi::protocol::connection::{Connection, DcxPacket};

pub struct ThreeWireSPI
where
    Self: Connection,
{
    pub spidev: SpidevDevice,
}

impl ThreeWireSPI {
    pub const DEFAULT_OPTIONS: SpidevOptions = SpidevOptions {
        bits_per_word: Some(9),
        max_speed_hz: Some(20_0000),
        lsb_first: Some(false),
        spi_mode: Some(SpiModeFlags::SPI_MODE_0),
    };

    pub fn open<P>(path: P) -> Result<Self, SPIError>
    where
        P: AsRef<Path>,
    {
        let spidev = SpidevDevice::open(path)?;

        Ok(Self { spidev })
    }

    pub fn configure(&mut self, options: &SpidevOptions) -> io::Result<()> {
        self.spidev.configure(options)
    }

    pub fn default_configuration(&mut self) -> io::Result<()> {
        self.configure(&Self::DEFAULT_OPTIONS)
    }
}
impl Debug for ThreeWireSPI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpidevDevice(fd={})", self.spidev.as_raw_fd())
    }
}

impl Connection for ThreeWireSPI {
    fn command(&self, address: u8) -> io::Result<()> {
        self.spidev
            .transfer(&mut SpidevTransfer::write(&DcxPacket::format_command(
                address,
            )))
    }

    fn write(&self, address: u8, parameters: &[u8]) -> io::Result<()> {
        self.spidev.transfer_multiple(&mut [
            SpidevTransfer::write(&DcxPacket::format_command(address)),
            SpidevTransfer::write(DcxPacket::format_parameters(parameters).as_ref()),
        ])
    }

    fn read(&self, address: u8, buffer: &mut [u8]) -> io::Result<()> {
        self.spidev.transfer_multiple(&mut [
            SpidevTransfer::write(&DcxPacket::format_command(address)),
            SpidevTransfer::read((*buffer).as_mut()),
        ])
    }
}
