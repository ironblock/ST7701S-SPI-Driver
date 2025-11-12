use std::marker::PhantomData;

use crate::st7701s_spi::{
    device::{ST7701S, StateAccessorMut},
    parameters::general::Switch,
    protocol::connection::{Command,  InstructionResult, Read, Write},
    transmissions::Parametric,
};

pub struct Abstraction<'a, E, COMMANDS, STATE> {
    device: &'a mut ST7701S<E>,
    accessor: StateAccessorMut<STATE>,
    marker: PhantomData<COMMANDS>,
}
impl<'a, E, COMMANDS, STATE> Abstraction<'a, E, COMMANDS, STATE> {
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

pub type Toggle<'a, E, ON, OFF> = Abstraction<'a, E, (ON, OFF), Switch>;
impl<'a, E, ON, OFF> Toggle<'a, E, ON, OFF>
where
    ON: Command<E>,
    OFF: Command<E>,
{
    pub fn on(mut self) -> InstructionResult {
        self.device.command::<ON>().inspect(move |_| {
            *self.field_mut() = Switch::On;
        })
    }

    pub fn off(mut self) -> InstructionResult {
        self.device.command::<OFF>().inspect(move |_| {
            *self.field_mut() = Switch::Off;
        })
    }
}

pub type Select<'a, E, SELECT, DISABLE, P> =
    Abstraction<'a, E, (SELECT, DISABLE), Option<P>>;
impl<E, SELECT, DISABLE, P> Select<'_, E, SELECT, DISABLE, P>
where
    SELECT: Write<E, Data = P::Data>,
    DISABLE: Command<E>,
    P: Parametric,
{
    pub fn is_selected(&mut self) -> bool {
        self.field().is_some()
    }
    pub fn is_disabled(&mut self) -> bool {
        self.field().is_none()
    }

    pub fn select(mut self, parameters: P) -> InstructionResult {
        self.device
            .write_parameters::<SELECT>(&parameters)
            .inspect(move |_| {
                *self.field_mut() = Some(parameters);
            })
    }

    pub fn disable(mut self) -> InstructionResult {
        self.device.command::<DISABLE>().inspect(|_| {
            *self.field_mut() = None;
        })
    }
}

pub type Configure<'a, E, READ, WRITE, P> = Abstraction<'a, E, (READ, WRITE), P>;
impl<E, READ, WRITE, P> Configure<'_, E, READ, WRITE, P>
where
    READ: Read<E, Data = P::Data>,
    WRITE: Write<E, Data = P::Data>,
    P: Parametric,
{
    pub fn read(mut self, buffer: &mut impl Parametric<Data = READ::Data>) -> InstructionResult {
        self.device.read_to_buffer::<READ>(buffer).inspect(move |_| {
            *self.field_mut() = P::from_rx_data(buffer);
        })
    }
    
    pub fn write(mut self, next_state: & impl Parametric<Data = WRITE::Data>) -> InstructionResult {
        self.device
            .write_parameters::<WRITE>(&next_state)
            .inspect(move |_| {
                *self.field_mut() = next_state;
            })
    }
}
