use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    io,
    marker::PhantomData,
};

use crate::st7701s_spi::parameters::register::Bank;

pub(crate) mod extended_instructions {
    pub trait RequiredExtension<E> {}
}

use extended_instructions::RequiredExtension;

pub trait Extension {
    const EXTENSION: Option<Bank>;
}

pub trait Address {
    const ADDRESS: u8;
}

pub trait TxData {
    type Data: Borrow<[u8]>;
}

pub trait RxData {
    type Data: BorrowMut<[u8]>;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bank0;
impl Extension for Bank0 {
    const EXTENSION: Option<Bank> = Some(Bank::BK0);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bank1;
impl Extension for Bank1 {
    const EXTENSION: Option<Bank> = Some(Bank::BK1);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bank3;
impl Extension for Bank3 {
    const EXTENSION: Option<Bank> = Some(Bank::BK3);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AnyExtension;
impl Extension for AnyExtension {
    const EXTENSION: Option<Bank> = None;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CommandVariant;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WriteVariant;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ReadVariant;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Instruction<V, E, const A: u8, const N: usize = 0> {
    _variant: PhantomData<V>,
    _extension: PhantomData<E>,
}
impl<V, E, const A: u8, const N: usize> Address for Instruction<V, E, A, N> {
    const ADDRESS: u8 = A;
}
impl<V, const A: u8, const N: usize> RequiredExtension<Bank0> for Instruction<V, Bank0, A, N> {}
impl<V, const A: u8, const N: usize> RequiredExtension<Bank1> for Instruction<V, Bank1, A, N> {}
impl<V, const A: u8, const N: usize> RequiredExtension<Bank3> for Instruction<V, Bank3, A, N> {}
impl<V, E, const A: u8, const N: usize> RequiredExtension<E>
    for Instruction<V, AnyExtension, A, N>
{
}

pub type CommandDefinition<E, const A: u8> = Instruction<CommandVariant, E, A, 0>;
pub type WriteDefinition<E, const A: u8, const N: usize> = Instruction<WriteVariant, E, A, N>;
impl<E, const A: u8, const N: usize> TxData for WriteDefinition<E, A, N> {
    type Data = [u8; N];
}
pub type ReadDefinition<E, const A: u8, const N: usize> = Instruction<ReadVariant, E, A, N>;
impl<E, const A: u8, const N: usize> RxData for ReadDefinition<E, A, N> {
    type Data = [u8; N];
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
    #[must_use]
    pub const fn format_command(address: u8) -> [u8; 2] {
        [Self::Command as u8, address]
    }

    /// Formats an array of parameters as an array of pairs of packets
    #[must_use]
    pub fn format_parameters(data: &[u8]) -> impl AsRef<[u8]> {
        data.as_ref()
            .iter()
            .flat_map(|b| [Self::Parameter as u8, *b])
            .collect::<Vec<u8>>()
    }
}

pub trait Connection: Send + Sync + Debug {
    /// Send a "Command" instruction to the the specified address.
    /// Depending on the protocol used, this may be implemented as multiple
    /// packets within a single transmission.
    fn command(&self, address: u8) -> io::Result<()>;

    /// Send a "Write" instruction to the specified address, impemented as a
    /// "Command" transmission followed by the data contained in
    /// `write_buffer`. Depending on the protocol, `write_buffer` may need to be
    /// transformed into a sequence of multiple packets.
    fn write(&self, address: u8, write_buffer: &[u8]) -> io::Result<()>;

    /// Send a "Read" instruction to the specified address, implemented as a
    /// "Command" transmission followed by reading data packets into
    /// mutable buffer `read_buffer`.
    fn read(&self, address: u8, read_buffer: &mut [u8]) -> io::Result<()>;
}

pub trait ConnectionOwner<X, E>
where
    X: Connection,
{
    fn connection(&self) -> &X;

    fn command<C: Address + RequiredExtension<E>>(&self) -> io::Result<()> {
        self.connection().command(C::ADDRESS)
    }

    fn write<W: Address + TxData + RequiredExtension<E>>(
        &self,
        buffer: &W::Data,
    ) -> io::Result<()> {
        self.connection().write(W::ADDRESS, buffer.borrow())
    }

    fn read<R: Address + RxData + RequiredExtension<E>>(
        &self,
        buffer: &mut R::Data,
    ) -> io::Result<()> {
        self.connection().read(R::ADDRESS, buffer.borrow_mut())
    }
}
