use std::{
    fmt::{Debug, Formatter},
    io,
    os::fd::AsRawFd,
    path::Path,
};

use linux_embedded_hal::{
    SPIError, SpidevDevice,
    spidev::{SpiModeFlags, SpidevOptions, SpidevTransfer},
};

use crate::st7701s_spi::protocol::connection::{Connection, DcxPacket};

pub struct ThreeWireSPI
where
    Self: Connection,
{
    spidev: SpidevDevice,
}

#[allow(clippy::missing_errors_doc, reason = "IO errors are self-explanatory")]
impl ThreeWireSPI {
    pub const DEFAULT_OPTIONS: SpidevOptions = SpidevOptions {
        bits_per_word: Some(9),
        max_speed_hz: Some(20_0000),
        lsb_first: Some(false),
        spi_mode: Some(SpiModeFlags::SPI_MODE_0),
    };

    /// Opens a new "three wire" SPI connection to the specified display.
    ///
    /// Alias for [`SpidevDevice::open`]
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SPIError> {
        SpidevDevice::open(path).map(|spidev| Self { spidev })
    }

    #[must_use]
    pub const fn spidev(&self) -> &SpidevDevice {
        &self.spidev
    }

    #[must_use]
    pub const fn spidev_mut(&mut self) -> &mut SpidevDevice {
        &mut self.spidev
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

    fn write(&self, address: u8, write_buffer: &[u8]) -> io::Result<()> {
        self.spidev.transfer_multiple(&mut [
            SpidevTransfer::write(&DcxPacket::format_command(address)),
            SpidevTransfer::write(DcxPacket::format_parameters(write_buffer).as_ref()),
        ])
    }

    fn read(&self, address: u8, read_buffer: &mut [u8]) -> io::Result<()> {
        self.spidev.transfer_multiple(&mut [
            SpidevTransfer::write(&DcxPacket::format_command(address)),
            SpidevTransfer::read((*read_buffer).as_mut()),
        ])
    }
}
