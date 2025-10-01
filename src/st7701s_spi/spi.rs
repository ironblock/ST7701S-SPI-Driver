extern crate spidev;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{any::Any, fmt::Debug, io::Write};
use std::{error::Error, fmt::Display};
use std::{io, path::Path};

use crate::st7701s_spi::{
    address::{
        CommandInstruction, Location, ReadInstruction, WriteInstruction,
        core::{NORON, PTLON},
    },
    state::{DeviceState, toggle::Toggle},
};

const TRACK_STATE: bool = true;

pub enum DCX {
    Command = 0x00,
    Parameter = 0x01,
}
#[derive(Debug)]
struct StateError(String);
impl Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for StateError {}

pub const THREE_WIRE_OPTIONS: SpidevOptions = SpidevOptions {
    bits_per_word: Some(9),
    max_speed_hz: Some(20_0000),
    lsb_first: Some(false),
    spi_mode: Some(SpiModeFlags::SPI_MODE_0),
};

pub struct Transciever {
    spi: Spidev,
}

impl Transciever {
    fn tx_command(&mut self, location: Location) -> io::Result<usize> {
        self.spi.write(&[DCX::Command as u8, location.as_u8()])
    }

    fn tx_parameters(
        &mut self,
        parameters: impl AsRef<[u8]> + IntoIterator<Item = u8>,
    ) -> io::Result<usize> {
        let mut written: usize = 0;

        for byte in parameters.into_iter() {
            written += self.spi.write(&[DCX::Parameter as u8, byte])?;
        }

        io::Result::Ok(written)
    }

    pub fn command<C: CommandInstruction>(&mut self) -> io::Result<usize> {
        self.tx_command(C::LOCATION)
    }

    pub fn write<W: WriteInstruction>(
        &mut self,
        parameters: impl AsRef<[u8]> + IntoIterator<Item = u8>,
    ) -> io::Result<usize> {
        let mut written: usize = 0;

        written += self.tx_command(W::LOCATION)?;
        written += self.tx_parameters(parameters)?;

        io::Result::Ok(written)
    }

    pub fn read<R: ReadInstruction, const N: usize>(
        &mut self,
        _reader: for<'a> fn(&'a [u8]) -> dyn Any,
    ) -> io::Result<usize> {
        todo!()
    }
}

pub struct ST7701S {
    transciever: Transciever,
    state: DeviceState,
}

impl ST7701S {
    pub fn new(spi_device: &Path, spi_options: &SpidevOptions) -> Self {
        let mut spi = Spidev::open(spi_device).expect("Failed to open SPI device");
        spi.configure(spi_options)
            .expect("Failed to configure SPI device");

        Self {
            transciever: Transciever { spi },
            state: DeviceState::new(),
        }
    }

    pub const fn partial_mode<T>(&mut self) -> Toggle<PTLON, NORON, T> {
        Toggle::new(&mut self.transciever, &mut self.state.mode.partial)
    }
}

// pub type SideEffectFn = for<'a> fn(&'a mut ST7701S) -> ();
// pub type ReadHandler = for<'a> fn(&'a [u8]) -> dyn Any;

// pub enum Transmission<const PACKETS: usize = 0> {
//     Command(Address),
//     Write(Address, [u8; PACKETS]),
//     Read(Address, ReadHandler),
// }

// pub type CreateTransmissionResult = Result<Transmission, StateError>;

// pub trait Reads {
//     type Packets: AsRef<[u8]>;
//     type Handler: Fn(Self::Packets) -> dyn Any;
// }

// pub trait Writes {
//     type Packets: AsRef<[u8]>;
// }

// type ResetFn<T> = for<'a> fn(&'a mut dyn Stateful<T>);

// pub type ResetTracker<T> = HashMap<dyn Stateful<T>, ResetFn<T>>;

// pub trait Parametric<T: Default> {

//     fn initial_value() -> T where Self: Sized {
//         T::default()
//     }
// }
// pub trait Stateless<T: Default + PartialEq>: Parametric<T> {
//     fn new() -> Self where Self: Sized;
//     fn into_stateful(self, tracker: &mut ResetTracker<T>) -> impl Stateful<T> where Self: Sized ;
// }
// pub trait Stateful<T: Default + PartialEq>: Parametric<T> {
//     fn new(tracker: &mut ResetTracker<T>) -> Self where Self: Sized;
//     fn into_stateless(self, tracker: &mut ResetTracker<T>) -> impl Stateless<T> where Self: Sized;
//     fn get(&self) -> &T ;
// fn get_mut(&mut self) -> &mut T ;
// fn modify(&mut self, f: fn(&mut T)) ;
// fn reset(&mut self) ;
// }

// #[derive(Debug, PartialEq, Eq, Hash)]
// struct ParameterType<T>(PhantomData<T>);
// impl<T:  Default + PartialEq> Parametric<T> for ParameterType<T> {}
// impl <T:  Default + PartialEq> Stateless<T> for ParameterType<T> {
//     fn new() -> Self {
//         Self(PhantomData)
//     }
//     fn into_stateful(self, tracker: &mut ResetTracker<T>) -> impl Stateful<T> {
//         ParameterState::<T>::new(tracker)
//     }
// }
// impl <T:  Default + PartialEq> ParameterType<T> {
// }

// #[derive(Debug, PartialEq, Eq, Hash)]
// struct ParameterState<T>(T);
// impl<T: Default + PartialEq> Parametric<T> for ParameterState<T> {}
// impl <T: Default + PartialEq> Stateful<T> for ParameterState<T> {
//     fn new(tracker: &mut ResetTracker<T>) -> Self {
//         let instance = Self(Self::initial_value());
//         let reset_fn: ResetFn<T> = |state| state.reset();
//         tracker.insert(reset_fn);

//         instance
//     }

//     fn into_stateless(self, tracker: &mut ResetTracker<T>) -> impl Stateless<T> {
//         tracker.remove(&Self::reset);

//         ParameterType
//     }

//     fn get(&self) -> &T {
//         &self.0
//     }

//     fn get_mut(&mut self) -> &mut T {
//         &mut self.0
//     }

//     fn modify(&mut self, f: F) {
//         f(self.get_mut());
//     }

//     fn reset(&mut self) {
//         self.modify(|state| *state = T::default());
//     }
// }
