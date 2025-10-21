use std::io;

use crate::st7701s_spi::address::{Command, Read, Write};

pub type InstructionResult = io::Result<()>;
pub trait Connection {
    fn command<C: Command>(&self) -> InstructionResult;

    fn write<W: Write>(&self, parameters: &W::Data) -> InstructionResult;

    fn read<R: Read>(&self, buffer: &mut R::Data) -> InstructionResult;
}
