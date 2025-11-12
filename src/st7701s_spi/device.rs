extern crate spidev;

use std::any::Any;

use crate::st7701s_spi::{
    protocol::connection::{Command, Connection, InstructionResult, Read, Write},
    state::domains::DeviceState,
    transmissions::Parametric,
};

pub type StateModifier = for<'a> fn(&'a mut DeviceState);
pub type StateAccessor<T> = for<'a> fn(&'a DeviceState) -> &'a T;
pub type StateAccessorMut<T> = for<'a> fn(&'a mut DeviceState) -> &'a mut T;

#[derive(Debug)]
pub struct ST7701S<E: ?Sized> {
    state: DeviceState,
    connection: &'static dyn Connection,
    extension: E,
}

impl<E: ?Sized> ST7701S<E> {
    pub const fn new(connection: &'static impl Connection) -> ST7701S<impl Any> {
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

    pub fn set_extension<N>(self, extension: N) -> ST7701S<N> {
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

    pub fn modify_state(&mut self, modifier: StateModifier) {
        modifier(&mut self.state);
    }

    pub fn reset(self) -> ST7701S<impl Any> {
        ST7701S {
            connection: self.connection,
            extension: (),
            state: DeviceState::new(),
        }
    }

    pub fn command<C: Command<E>>(&self) -> InstructionResult {
        self.connection.command(C::ADDRESS)
    }

    pub fn write_buffer<W: Write<E>>(&self, data: &W::Data) -> InstructionResult {
        self.connection.write(W::ADDRESS, data.as_ref())
    }

    pub fn write_parameters<W: Write<E>>(
        &self,
        parameters: &impl Parametric<Data = W::Data>,
    ) -> InstructionResult {
        self.write_buffer::<W>(&parameters.as_tx_data())
    }

    pub fn read_to_buffer<R: Read<E>>(&self, buffer: &mut R::Data) -> InstructionResult {
        self.connection.read(R::ADDRESS, buffer.as_mut())
    }

    pub fn read_to_parameters<R: Read<E>>(
        &self,
        parameters: &mut impl Parametric<Data = R::Data>,
    ) -> InstructionResult {
        parameters.from_rx_data(self.read_to_buffer::<R>(&mut parameters.as_tx_data()))
    }
}
