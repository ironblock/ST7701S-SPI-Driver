extern crate spidev;

use crate::st7701s_spi::{protocol::connection::Connection, state::domains::DeviceState};

struct NotConnected;

pub trait Connected {
    type ConnectionType: Connection;

    fn connection(&self) -> &Self::ConnectionType;
}

pub type StateModifier<T: Stateful> = for<'a> fn(&'a mut T::StateType);
pub type StateAccessor<T: Stateful, U> = for<'a> fn(&'a T::StateType) -> &'a U;
pub type StateAccessorMut<T: Stateful, U> = for<'a> fn(&'a mut T::StateType) -> &'a mut U;
pub trait Stateful {
    type StateType;

    fn state(&self) -> &Self::StateType;

    fn state_mut(&mut self) -> &mut Self::StateType;

    fn modify_state(&mut self, modifier: impl FnOnce(&mut Self::StateType));

    fn reset(&mut self);
}

pub trait ActiveDevice: Connected + Stateful {}
impl<T> ActiveDevice for T where T: Connected + Stateful {}

pub struct ST7701S<C, E> {
    pub connection: C,
    extension: Option<E>,
    state: DeviceState,
}

impl<C, E> ST7701S<C, E> {
    pub const fn new() -> ST7701S<NotConnected, E> {
        ST7701S {
            connection: NotConnected,
            extension: None,
            state: DeviceState::new(),
        }
    }

    pub fn set_extension(&mut self, extension: E) {
        self.extension = Some(extension);
    }
}

impl<E> ST7701S<NotConnected, E> {
    pub fn connect<C: Connection>(self, connection: C) -> ST7701S<C, E> {
        ST7701S {
            connection,
            extension: self.extension,
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

    fn reset(&mut self) {
        self.extension = None;
        self.state = DeviceState::default();
    }
}

impl<C: Connection, E> Connected for ST7701S<C, E> {
    type ConnectionType = C;

    fn connection(&self) -> &Self::ConnectionType {
        &self.connection
    }
}
