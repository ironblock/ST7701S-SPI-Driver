use std::io;

use crate::st7701s_spi::address::{Command, Read, Write};

pub type InstructionResult<T = ()> = io::Result<T>;
pub trait Connection {
    fn command<C: Command>(&self) -> Result<(), io::Error>;

    fn write<W: Write>(&self, parameters: &W::Data) -> Result<(), io::Error>;

    fn read<R: Read>(&self, buffer: &mut R::Data) -> Result<(), io::Error>;
}
