extern crate spidev;

use crate::st7701s_spi::{
    address::{AnyExtension, Command, Extension, Read, Write},
    protocol::connection::{Connection, InstructionResult},
    state::domains::DeviceState, transmissions::{ParametricTransmission},
};

pub struct NotConnected;

pub trait Connected {
    type ConnectionType: Connection;
    type ExtensionType: Extension;

    fn connection(&self) -> &Self::ConnectionType;
    fn extension(&self) -> &Self::ExtensionType;
}

pub type StateModifier<T> = for<'a> fn(&'a mut <T as Stateful>::StateType);
pub type StateAccessor<T, U> = for<'a> fn(&'a <T as Stateful>::StateType) -> &'a U;
pub type StateAccessorMut<T, U> = for<'a> fn(&'a mut <T as Stateful>::StateType) -> &'a mut U;
pub trait Stateful {
    type StateType;

    fn state(&self) -> &Self::StateType;

    fn state_mut(&mut self) -> &mut Self::StateType;

    fn modify_state(&mut self, modifier: impl FnOnce(&mut Self::StateType));
}

pub trait ActiveDevice: Connected + Stateful {}
impl<T> ActiveDevice for T where T: Connected + Stateful {}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ST7701S<X, E: Extension> {
    pub connection: X,
    extension: E,
    state: DeviceState,
}

impl ST7701S<NotConnected, AnyExtension> {
    pub const fn new() -> ST7701S<NotConnected, AnyExtension> {
        ST7701S {
            connection: NotConnected,
            extension: AnyExtension,
            state: DeviceState::new(),
        }
    }

    pub fn connect<X: Connection>(self, connection: X) -> ST7701S<X, AnyExtension> {
        ST7701S {
            connection,
            extension: self.extension,
            state: self.state,
        }
    }
}

impl Default for ST7701S<NotConnected, AnyExtension> {
    fn default() -> Self {
        Self::new()
    }
}

impl<X, E: Extension> ST7701S<X, E> {
    pub fn set_extension<N: Extension>(self, extension: N) -> ST7701S<X, N> {
        ST7701S {
            connection: self.connection,
            extension,
            state: self.state,
        }
    }

    pub fn reset(self) -> ST7701S<X, AnyExtension> {
        ST7701S {
            connection: self.connection,
            extension: AnyExtension,
            state: DeviceState::new(),
        }
    }
}

impl<X: Connection, E: Extension> Stateful for ST7701S<X, E> {
    type StateType = DeviceState;

    fn state(&self) -> &Self::StateType {
        &self.state
    }

    fn state_mut(&mut self) -> &mut Self::StateType {
        &mut self.state
    }

    fn modify_state(&mut self, modifier: impl FnOnce(&mut Self::StateType)) {
        modifier(&mut self.state);
    }
}

impl<X: Connection, E: Extension> Connected for ST7701S<X, E> {
    type ConnectionType = X;
    type ExtensionType = E;

    fn connection(&self) -> &Self::ConnectionType {
        &self.connection
    }

    fn extension(&self) -> &Self::ExtensionType {
        &self.extension
    }

}

impl <X: Connection, E: Extension> ST7701S<X, E> {
    pub const fn extensions_match<T: Extension>() -> bool {
        match (E::EXTENSION, T::EXTENSION) {
            (Some(setting), Some(instruction)) => setting as u8 == instruction as u8,
            (None, None) => true,
            _ => false,
        }
    }

    pub fn command<C: Command>(&self) -> InstructionResult {
        self.connection.command::<C>()
    }

    pub fn write<W: Write>(&self, parameters: &impl ParametricTransmission<Data =  W::Data>) -> InstructionResult {
        const { assert!(Self::extensions_match::<W>(), "current selected bank does not include write command's location"); }
        self.connection.write::<W>(&parameters.as_tx_data())
    }

    pub fn read<R: Read>(&self, buffer: &mut impl ParametricTransmission<Data = R::Data>) -> InstructionResult {
        self.connection.read::<R>(&mut buffer.as_tx_data())
    }
}
