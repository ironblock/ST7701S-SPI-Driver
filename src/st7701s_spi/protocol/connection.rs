use std::{any::Any, fmt::Debug, io};

use crate::st7701s_spi::transmissions::{Parametric, Transmission};

pub trait ExtensionVariant {}
impl<T> ExtensionVariant for T {}

// pub type AnyExtension = dyn Any + 'static;
// impl ExtensionVariant for AnyExtension {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk0
where
    Self: ExtensionVariant;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk1
where
    Self: ExtensionVariant;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk3
where
    Self: ExtensionVariant;

pub trait InstructionVariant {}
impl<T> InstructionVariant for T {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CommandVariant
where
    Self: InstructionVariant;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WriteVariant
where
    Self: InstructionVariant;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ReadVariant
where
    Self: InstructionVariant;

pub trait Location<E> {
    const ADDRESS: u8;
}

// pub trait RequiresExtension<E: ExtensionVariant> {}
// impl <E: ExtensionVariant, T: Location<E>> RequiresExtension<E> for T {}


pub trait CommandInstruction<E>: Location<E> {
    fn command(device: &dyn Connection) -> io::Result<()> {
        device.command(Self::ADDRESS)
    }
}

pub trait WriteInstruction<E>: Location<E> + Transmission{
    fn write_from_buffer(device: &dyn Connection, data: &Self::Data) -> io::Result<()> {
        device.write(Self::ADDRESS, data.as_ref())
    }

    fn write_from_parameters(
        device: &dyn Connection,
        parameters: &impl Parametric<Data = Self::Data>,
    ) -> io::Result<()> {
        Self::write_from_buffer(device, &parameters.as_tx_data())
    }
}

pub trait ReadInstruction<E>: Location<E> + Transmission{
    fn read_to_buffer(device: &dyn Connection, buffer: &mut Self::Data) -> io::Result<()> {
        device.read(Self::ADDRESS, buffer.as_mut())
    }

    fn read_to_parameters<P: Parametric<Data = Self::Data>>(
        device: &dyn Connection,
    ) -> io::Result<()> {
        // TODO: This needs to be rewritten to avoid needless initialization
        device.read(Self::ADDRESS, buffer.as_mut())

        Self::read_to_buffer(device, &mut parameters.as_rx_data())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Instruction<V, E, const ADDRESS: u8, const PACKETS: usize>
where
    Self: Location<E> + Sized,
{
    _extension: std::marker::PhantomData<E>,
    _variant: std::marker::PhantomData<V>,
}
impl<V, E, const ADDRESS: u8, const PACKETS: usize> Location<E>
    for Instruction<V, E, ADDRESS, PACKETS>
{
    const ADDRESS: u8 = ADDRESS;
}

pub type Command<E, const ADDRESS: u8> = Instruction<CommandVariant, E, ADDRESS, 0>;
impl<E, const ADDRESS: u8> CommandInstruction<E> for Command<E, ADDRESS> {}

pub type Write<E, const ADDRESS: u8, const PACKETS: usize> =
    Instruction<WriteVariant, E, ADDRESS, PACKETS>;
impl<E, const ADDRESS: u8, const PACKETS: usize> Transmission
    for Write<E, ADDRESS, PACKETS>
{
    type Data = [u8; PACKETS];
    type MapToData<U> = [U; PACKETS];
}
impl<E, const ADDRESS: u8, const PACKETS: usize> WriteInstruction<E>
    for Write<E, ADDRESS, PACKETS>
{
}

pub type Read<E, const ADDRESS: u8, const PACKETS: usize> =
    Instruction<ReadVariant, E, ADDRESS, PACKETS>;
impl<E, const ADDRESS: u8, const PACKETS: usize> Transmission
    for Read<E, ADDRESS, PACKETS>
{
    type Data = [u8; PACKETS];
    type MapToData<U> = [U; PACKETS];
}
impl<E, const ADDRESS: u8, const PACKETS: usize> ReadInstruction<E>
    for Read<E, ADDRESS, PACKETS>
{
}

/// # D/CX Packet Types
///
/// Implementations of the ST7701S interface circuit may omit the use of a
/// dedicated D/CX pin to disambiguate command and data packets. For these
/// implementations, the transmission protocol requires a marker bit to identify
/// the type of packet being sent. This enum defines the two packet types used
/// in such protocols.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DcxPacket {
    Command = 0,
    Parameter = 1,
}
impl DcxPacket {
    /// Formats a command as a pair of packets
    pub const fn format_command(address: u8) -> [u8; 2] {
        [DcxPacket::Command as u8, address]
    }

    /// Formats an array of parameters as an array of pairs of packets
    pub fn format_parameters(data: &[u8]) -> impl AsRef<[u8]> {
        data.as_ref()
            .into_iter()
            .flat_map(|b| [DcxPacket::Parameter as u8, *b])
            .collect::<Vec<u8>>()
    }
}

pub trait Connection: Send + Sync + Debug {
    /// Send a conceptual "Command" to the the specified address.
    /// Depending on the protocol used, this may be implemented as multiple
    /// packets within a single transmission.
    fn command(&self, address: u8) -> io::Result<()>;

    /// Send a conceptual "Write" to the specified address, impemented as a
    /// "Command" transmission followed by the data contained in
    /// `write_buffer`. Depending on the protocol, `write_buffer` may need to be
    /// transformed into a sequence of multiple packets.
    fn write(&self, address: u8, write_buffer: &[u8]) -> io::Result<()>;

    /// Send a conceptual "Read" to the specified address, implemented as a
    /// "Command" transmission followed by reading data packets into
    /// mutable buffer `read_buffer`.
    fn read(&self, address: u8, read_buffer: &mut [u8]) -> io::Result<()>;
}
