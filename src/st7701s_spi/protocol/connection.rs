use std::{fmt::Debug, io};

use crate::st7701s_spi::{parameters::register::Bank, transmissions::Transmission};

pub trait Extension {
    const EXTENSION: Option<Bank>;
}

pub const fn extensions_match<E1: Extension, E2: Extension>() -> bool {
    match (E1::EXTENSION, E2::EXTENSION) {
        (Some(setting), Some(instruction)) => setting as u8 == instruction as u8,
        (None, None) => true,
        _ => false,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionAll;
impl Extension for ExtensionAll {
    const EXTENSION: Option<Bank> = None;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk0;
impl Extension for ExtensionBk0 {
    const EXTENSION: Option<Bank> = Some(Bank::BK0);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk1;
impl Extension for ExtensionBk1 {
    const EXTENSION: Option<Bank> = Some(Bank::BK1);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk3;
impl Extension for ExtensionBk3 {
    const EXTENSION: Option<Bank> = Some(Bank::BK3);
}

pub trait Instruction: Extension {
    const ADDRESS: u8;
}

pub trait Command: Instruction + Extension {}
impl<T> Command for T where T: Instruction {}

pub trait Write: Instruction + Extension + Transmission {}
impl<T> Write for T where T: Instruction + Transmission {}

pub trait Read: Instruction + Extension + Transmission {}
impl<T> Read for T where T: Instruction + Transmission {}

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
