use std::io;

use linux_embedded_hal::{
    SpidevDevice,
    spidev::{SpiModeFlags, SpidevOptions, SpidevTransfer},
};

use crate::st7701s_spi::{
    address::{Command, Read, Write},
    protocol::connection::Connection,
};

pub trait SpiProtocol: Connection {
    const DEFAULT_OPTIONS: SpidevOptions;

    fn device(&self) -> &SpidevDevice;
    fn device_mut(&mut self) -> &mut SpidevDevice;
}

pub struct ThreeWireSPI
where
    Self: SpiProtocol,
{
    pub device: SpidevDevice,
}
impl ThreeWireSPI {
    const DCX_COMMAND: u8 = 0;
    const DCX_PARAMETER: u8 = 1;

    fn format_command(address: u8) -> [u8; 2] {
        [Self::DCX_COMMAND, address]
    }

    fn format_parameters(data: &[u8]) -> Vec<u8> {
        data.iter()
            .flat_map(|parameter| [Self::DCX_PARAMETER, *parameter])
            .collect::<Vec<u8>>()
    }
}
impl SpiProtocol for ThreeWireSPI {
    const DEFAULT_OPTIONS: SpidevOptions = SpidevOptions {
        bits_per_word: Some(9),
        max_speed_hz: Some(20_0000),
        lsb_first: Some(false),
        spi_mode: Some(SpiModeFlags::SPI_MODE_0),
    };

    fn device(&self) -> &SpidevDevice {
        &self.device
    }
    fn device_mut(&mut self) -> &mut SpidevDevice {
        &mut self.device
    }
}

impl Connection for ThreeWireSPI {
    fn command<C: Command>(&self) -> io::Result<()> {
        let command = Self::format_command(C::LOCATION.as_u8());

        self.device.transfer(&mut SpidevTransfer::write(&command))
    }

    fn write<W: Write>(&self, parameters: &W::Data) -> io::Result<()> {
        let command = Self::format_command(W::LOCATION.as_u8());
        let parameters = Self::format_parameters(parameters.as_ref());

        self.device.transfer_multiple(&mut [
            SpidevTransfer::write(&command),
            SpidevTransfer::write(&parameters),
        ])
    }

    fn read<R: Read>(&self, buffer: &mut R::Data) -> io::Result<()> {
        let command = Self::format_command(R::LOCATION.as_u8());

        self.device.transfer_multiple(&mut [
            SpidevTransfer::write(&command),
            SpidevTransfer::read(buffer.as_mut()),
        ])
    }
}
