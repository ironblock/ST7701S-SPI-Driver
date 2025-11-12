use std::{fmt::Debug, io};

use crate::st7701s_spi::transmissions::Transmission;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CommandInstruction;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WriteInstruction;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ReadInstruction;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk0;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk1;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk3;

pub trait Instruction<T, E> {
    const ADDRESS: u8;
}

pub type Command<E> = dyn Instruction<CommandInstruction, E>;
pub type Write<E> = dyn Instruction<WriteInstruction, E> + Transmission;
pub type Read<E> = dyn Instruction<ReadInstruction, E> + Transmission;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DcxPacket {
    Command = 0,
    Parameter = 1,
}
impl DcxPacket {
    pub const fn format_command(address: u8) -> [u8; 2] {
        [DcxPacket::Command as u8, address]
    }

    pub fn format_parameters(data: &[u8]) -> impl AsRef<[u8]> {
        data.as_ref()
            .into_iter()
            .flat_map(|b| [DcxPacket::Parameter as u8, *b])
            .collect::<Vec<u8>>()
    }
}

pub type InstructionResult<T = ()> = io::Result<T>;
pub trait Connection: Send + Sync + Debug {
    fn command(&self, address: u8) -> io::Result<()>;

    fn write(&self, address: u8, write_buffer: &[u8]) -> io::Result<()>;

    fn read(&self, address: u8, read_buffer: &mut [u8]) -> io::Result<()>;
}
