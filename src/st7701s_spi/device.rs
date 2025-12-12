use crate::st7701s_spi::{
    protocol::connection::{Connection, ConnectionOwner},
    state::domains::DeviceState,
};

pub type StateModifier = for<'a> fn(&'a mut DeviceState);
pub type StateAccessor<T> = for<'a> fn(&'a DeviceState) -> &'a T;
pub type StateAccessorMut<T> = for<'a> fn(&'a mut DeviceState) -> &'a mut T;

#[derive(Debug)]
pub struct ST7701S<X, E = ()> {
    state: DeviceState,
    connection: X,
    extension: E,
}

impl<X, E> ST7701S<X, E> {
    pub const fn new(connection: X) -> ST7701S<X> {
        ST7701S {
            state: DeviceState::new(),
            connection,
            extension: (),
        }
    }

    pub const fn extension(&self) -> &E {
        &self.extension
    }

    pub const fn extension_mut(&mut self) -> &mut E {
        &mut self.extension
    }

    pub fn set_extension<N>(self, extension: N) -> ST7701S<X, N> {
        ST7701S {
            state: self.state,
            connection: self.connection,
            extension,
        }
    }

    pub const fn state(&self) -> &DeviceState {
        &self.state
    }

    pub const fn state_mut(&mut self) -> &mut DeviceState {
        &mut self.state
    }

    pub fn modify_state(&mut self, modifier: impl FnOnce(&mut DeviceState)) {
        modifier(&mut self.state);
    }

    pub fn reset(self) -> ST7701S<X, ()> {
        ST7701S {
            connection: self.connection,
            extension: (),
            state: DeviceState::new(),
        }
    }
}

impl<X, E> ConnectionOwner<X, E> for ST7701S<X, E>
where
    X: Connection,
{
    fn connection(&self) -> &X {
        &self.connection
    }
}
