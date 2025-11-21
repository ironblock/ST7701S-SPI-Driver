use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    io,
    marker::PhantomData,
};

use crate::st7701s_spi::parameters::register::Bank;

pub trait Extension {
    const EXTENSION: Option<Bank>;
}

pub trait RequireBank<E> {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bank0;
impl Extension for Bank0 {
    const EXTENSION: Option<Bank> = Some(Bank::BK0);
}
impl RequireBank<Self> for Bank0 {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bank1;
impl Extension for Bank1 {
    const EXTENSION: Option<Bank> = Some(Bank::BK1);
}
impl RequireBank<Self> for Bank1 {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bank3;
impl Extension for Bank3 {
    const EXTENSION: Option<Bank> = Some(Bank::BK3);
}
impl RequireBank<Self> for Bank3 {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AnyExtension;
impl Extension for AnyExtension {
    const EXTENSION: Option<Bank> = None;
}
impl<E> RequireBank<E> for AnyExtension {}

pub trait Address {
    const ADDRESS: u8;
}

pub trait TxData {
    type Data: Borrow<[u8]>;
}

pub trait RxData {
    type Data: BorrowMut<[u8]>;
}

pub trait CommandInstruction<E>: Address + RequireBank<E> {}
impl<T, E> RequireBank<E> for T where T: CommandInstruction<E> {}
impl<T, E> CommandInstruction<E> for T where T: Address {}

pub trait WriteInstruction<E>: CommandInstruction<E> + TxData {}
impl<T, E> WriteInstruction<E> for T where T: Address + TxData {}

pub trait ReadInstruction<E>: CommandInstruction<E> + RxData {}
impl<T, E> ReadInstruction<E> for T where T: Address + RxData {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Command<E, const ADDRESS: u8>(PhantomData<E>)
where
    Self: CommandInstruction<E>;
impl<E, const ADDRESS: u8> Address for Command<E, ADDRESS> {
    const ADDRESS: u8 = ADDRESS;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Write<E, const ADDRESS: u8, const PACKETS: usize>(PhantomData<E>)
where
    Self: WriteInstruction<E>;
impl<E, const ADDRESS: u8, const PACKETS: usize> Address for Write<E, ADDRESS, PACKETS> {
    const ADDRESS: u8 = ADDRESS;
}
impl<E, const ADDRESS: u8, const PACKETS: usize> TxData for Write<E, ADDRESS, PACKETS> {
    type Data = [u8; PACKETS];
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Read<E, const ADDRESS: u8, const PACKETS: usize>(PhantomData<E>)
where
    Self: ReadInstruction<E>;
impl<E, const ADDRESS: u8, const PACKETS: usize> Address for Read<E, ADDRESS, PACKETS> {
    const ADDRESS: u8 = ADDRESS;
}
impl<E, const ADDRESS: u8, const PACKETS: usize> RxData for Read<E, ADDRESS, PACKETS> {
    type Data = [u8; PACKETS];
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

    fn command<C: CommandInstruction<E>>(&self) -> io::Result<()> {
        self.connection().command(C::ADDRESS)
    }

    fn write<W: WriteInstruction<E>>(&self, buffer: &W::Data) -> io::Result<()> {
        self.connection().write(W::ADDRESS, buffer.borrow())
    }

    fn read<R: ReadInstruction<E>>(&self, buffer: &mut R::Data) -> io::Result<()> {
        self.connection().read(R::ADDRESS, buffer.borrow_mut())
    }
}
