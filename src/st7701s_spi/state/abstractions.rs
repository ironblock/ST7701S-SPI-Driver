use std::marker::PhantomData;

use crate::st7701s_spi::{
    device::{ST7701S, StateAccessorMut, TrackState},
    parameters::general::Switch,
    protocol::connection::{Command, Extension, InstructionResult, Read, Write},
    transmissions::Parametric,
};

pub struct Abstraction<'a, COMMANDS, STATE> {
    device: &'a mut TrackState + InstructionDispatcher,
    accessor: StateAccessorMut<STATE>,
    marker: PhantomData<COMMANDS>,
}
impl<'a, E: Extension, COMMANDS, STATE> Abstraction<'a, E, COMMANDS, STATE> {
    pub const fn new(device: &'a mut ST7701S<E>, accessor: StateAccessorMut<STATE>) -> Self {
        Self {
            device,
            accessor,
            marker: PhantomData,
        }
    }

    pub fn field(&mut self) -> &STATE {
        (self.accessor)(self.device.state_mut())
    }

    pub fn field_mut(&mut self) -> &mut STATE {
        (self.accessor)(self.device.state_mut())
    }
}

pub type Toggle<'a, ON, OFF> = Abstraction<'a, ON, (ON, OFF), Switch>;
impl<'a, ON, OFF> Toggle<'a, ON, OFF>
where
    ON: Command,
    OFF: Command,
{
    pub fn on(mut self) -> InstructionResult {
        self.connection().command::<ON>().inspect(move |_| {
            *self.field_mut() = Switch::On;
        })
    }

    pub fn off(mut self) -> InstructionResult {
        self.connection().command::<OFF>().inspect(move |_| {
            *self.field_mut() = Switch::Off;
        })
    }
}

pub type Select<'a, SELECT, DISABLE, P> = Abstraction<'a, (SELECT, DISABLE), Option<P>>;
impl<SELECT, DISABLE, P> Select<'_, SELECT, DISABLE, P>
where
    SELECT: Write<Data = P::Data>,
    DISABLE: Command,
    P: Parametric,
{
    pub fn is_selected(&mut self) -> bool {
        self.field().is_some()
    }
    pub fn is_disabled(&mut self) -> bool {
        self.field().is_none()
    }

    pub fn select(mut self, parameters: P) -> InstructionResult {
        self.connection()
            .write::<SELECT>(&parameters.as_tx_data())
            .inspect(move |_| {
                *self.field_mut() = Some(parameters);
            })
    }

    pub fn disable(mut self) -> InstructionResult {
        self.connection().command::<DISABLE>().inspect(|_| {
            *self.field_mut() = None;
        })
    }
}

pub type Configure<'a, READ, WRITE, P> = Abstraction<'a, (READ, WRITE), P>;
impl<READ, WRITE, P> Configure<'_, READ, WRITE, P>
where
    READ: Read<Data = P::Data>,
    WRITE: Write<Data = P::Data>,
    P: Parametric,
{
    pub fn read(mut self, buffer: &mut P::Data) -> InstructionResult {
        self.connection().read::<READ>(buffer).inspect(move |_| {
            *self.field_mut() = P::from_rx_data(buffer);
        })
    }
    pub fn write(mut self, next_state: P) -> InstructionResult {
        self.connection()
            .write::<WRITE>(&next_state.as_tx_data())
            .inspect(move |_| {
                *self.field_mut() = next_state;
            })
    }
}
