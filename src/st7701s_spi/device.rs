extern crate spidev;

use crate::st7701s_spi::{
    protocol::connection::{
        Command, Connection, Extension, ExtensionAll, InstructionResult, Read, Write,
        extensions_match,
    },
    state::domains::DeviceState,
    transmissions::ParametricTransmission,
};

pub type StateModifier = for<'a> fn(&'a mut DeviceState);
pub type StateAccessor<T> = for<'a> fn(&'a DeviceState) -> &'a T;
pub type StateAccessorMut<T> = for<'a> fn(&'a mut DeviceState) -> &'a mut T;

pub trait TrackState {
    fn state(&self) -> &DeviceState;
    fn state_mut(&mut self) -> &mut DeviceState;
    fn modify_state(&mut self, modifier: StateModifier);
    fn reset(self) -> Self;
}

pub trait ExtendedCommands<E: Extension> {
    fn extension(&self) -> &E;
    fn extension_mut(&mut self) -> &mut E;
    fn set_extension<N: Extension>(self, extension: N) -> ST7701S<N>;
}

pub trait InstructionDispatcher {
    fn command<C: Command>(&self) -> InstructionResult;
    fn write<W: Write>(&self, data: &W::Data) -> InstructionResult;
    fn write_parameters<W: Write>(
        &self,
        parameters: impl ParametricTransmission<Data = W::Data>,
    ) -> InstructionResult;
    fn read<R: Read>(
        &self,
        buffer: &mut impl ParametricTransmission<Data = R::Data>,
    ) -> InstructionResult;
}

#[derive(Debug)]
pub struct ST7701S<E: Extension>
where
    Self: TrackState + ExtendedCommands<E> + InstructionDispatcher,
{
    connection: &'static dyn Connection,
    extension: E,
    state: DeviceState,
}

impl<E: Extension> ST7701S<E> {
    pub const fn new(connection: &'static impl Connection) -> ST7701S<ExtensionAll> {
        ST7701S {
            connection,
            extension: ExtensionAll,
            state: DeviceState::new(),
        }
    }
}

impl<E: Extension> TrackState for ST7701S<E> {
    fn state(&self) -> &DeviceState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut DeviceState {
        &mut self.state
    }

    fn modify_state(&mut self, modifier: StateModifier) {
        modifier(&mut self.state);
    }

    fn reset(self) -> Self {
        ST7701S {
            extension: ExtensionAll,
            state: DeviceState::new(),
            connection: self.connection,
        }
    }
}

impl<E: Extension> ExtendedCommands<E> for ST7701S<E> {
    fn extension(&self) -> &E {
        &self.extension
    }

    fn extension_mut(&mut self) -> &mut E {
        &mut self.extension
    }

    fn set_extension<N: Extension>(self, extension: N) -> ST7701S<N> {
        ST7701S {
            extension,
            state: self.state,
            connection: self.connection,
        }
    }
}

impl<E: Extension> InstructionDispatcher for ST7701S<E> {
    fn command<C: Command>(&self) -> InstructionResult {
        self.connection.command(C::ADDRESS)
    }

    fn write<W: Write>(&self, data: &W::Data) -> InstructionResult {
        const {
            assert!(
                extensions_match::<E, W>(),
                "current selected bank does not include write command's location"
            );
        }

        self.connection.write(W::ADDRESS, data.as_ref())
    }

    fn write_parameters<W: Write>(
        &self,
        parameters: impl ParametricTransmission<Data = W::Data>,
    ) -> InstructionResult {
        self.write::<W>(&parameters.as_tx_data())
    }

    fn read<R: Read>(
        &self,
        buffer: &mut impl ParametricTransmission<Data = R::Data>,
    ) -> InstructionResult {
        self.connection
            .read(R::ADDRESS, &mut buffer.as_tx_data().as_mut())
    }
}
