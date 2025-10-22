extern crate spidev;

use crate::st7701s_spi::{
    address::{AnyExtension, Extension},
    protocol::connection::Connection,
    state::domains::DeviceState,
};

pub struct NotConnected;

pub trait Connected {
    type ConnectionType: Connection;

    fn connection(&self) -> &Self::ConnectionType;
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
pub struct ST7701S<C, E> {
    pub connection: C,
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

    pub fn connect<C: Connection>(self, connection: C) -> ST7701S<C, AnyExtension> {
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

impl<C, E> ST7701S<C, E> {
    pub fn set_extension<N: Extension>(self, extension: N) -> ST7701S<C, N> {
        ST7701S {
            connection: self.connection,
            extension,
            state: self.state,
        }
    }

    pub fn reset(self) -> ST7701S<C, AnyExtension> {
        ST7701S {
            connection: self.connection,
            extension: AnyExtension,
            state: DeviceState::new(),
        }
    }
}

impl<C: Connection, E> Stateful for ST7701S<C, E> {
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

impl<C: Connection, E> Connected for ST7701S<C, E> {
    type ConnectionType = C;

    fn connection(&self) -> &Self::ConnectionType {
        &self.connection
    }
}
