extern crate spidev;

use crate::st7701s_spi::{parameters::register::Bank, protocol::connection::Connection, state::domains::DeviceState};

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

    fn reset(&mut self);
}

pub trait ActiveDevice: Connected + Stateful {}
impl<T> ActiveDevice for T where T: Connected + Stateful {}

pub struct ST7701S<C> {
    pub connection: C,
    extension: Option<Bank>,
    state: DeviceState,
}

impl<C> ST7701S<C> {
    pub const fn new(connection: C) -> ST7701S<C> {
        ST7701S {
            connection,
            extension: None,
            state: DeviceState::new(),
        }
    }
}

impl <C: Connection> ST7701S<C> {
    pub fn extension(&self) -> Option<&Bank> {
        self.extension.as_ref()
    }

    pub fn extension_mut(&mut self) -> Option<&mut Bank> {
        self.extension.as_mut()
    }

    pub fn set_extension(&mut self, extension: Bank) {
        self.extension = Some(extension);
    }

    pub fn clear_extension(&mut self) {
        self.extension = None;
    }
}

impl<C: Connection> Stateful for ST7701S<C> {
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

impl<C: Connection> Connected for ST7701S<C> {
    type ConnectionType = C;

    fn connection(&self) -> &Self::ConnectionType {
        &self.connection
    }
}
