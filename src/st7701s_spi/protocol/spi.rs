use std::{fmt::{Debug, Formatter}, fs::File, io, os::fd::{AsRawFd}, sync::Arc};

use linux_embedded_hal::{
    spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer}, SpidevDevice
};

use crate::st7701s_spi::{
    protocol::connection::{Connection, DcxPacket},
};

#[derive(Clone)]
pub struct ThreeWireSPI(pub Arc<SpidevDevice>) where Self: Connection + Debug;
impl Debug for ThreeWireSPI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpidevDevice(fd={})", self.spidev().as_raw_fd())
    }
}
impl ThreeWireSPI {
    pub const DEFAULT_OPTIONS: SpidevOptions = SpidevOptions {
        bits_per_word: Some(9),
        max_speed_hz: Some(20_0000),
        lsb_first: Some(false),
        spi_mode: Some(SpiModeFlags::SPI_MODE_0),
    };

    fn spidev(&self) -> &Spidev {
        &self.0.0
    }

    fn device_file(&self) -> &File {
        self.spidev().inner()
    }
}

impl Connection for ThreeWireSPI {
    fn command(&self, address: u8) -> io::Result<()> {
        self.spidev().transfer(&mut SpidevTransfer::write(&DcxPacket::format_command(address)))
    }

    fn write(&self, address: u8, parameters: &[u8]) -> io::Result<()> {
        self.spidev().transfer_multiple(&mut [
            SpidevTransfer::write(&DcxPacket::format_command(address)),
            SpidevTransfer::write(DcxPacket::format_parameters(parameters).as_ref()),
        ])
    }

    fn read(&self, address: u8, buffer: &mut [u8]) -> io::Result<()> {
        self.spidev().transfer_multiple(&mut [
            SpidevTransfer::write(&DcxPacket::format_command(address)),
            SpidevTransfer::read((&mut *buffer).as_mut()),
        ])
    }
}
