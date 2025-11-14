use std::{
    io::{self},
    marker::PhantomData,
};

use crate::st7701s_spi::{
    device::{ST7701S, StateAccessorMut},
    parameters::general::Switch,
    protocol::connection::{CommandInstruction, Connection, ReadInstruction, WriteInstruction},
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

    pub fn connection(&self) -> &dyn Connection {
        self.device.connection()
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
    ON: CommandInstruction<E>,
    OFF: CommandInstruction<E>,
{
    pub fn on(mut self) -> io::Result<()> {
        ON::command(self.connection()).inspect(move |_| {
            *self.field_mut() = Switch::On;
        })
    }

    pub fn off(mut self) -> io::Result<()> {
        OFF::command(self.connection()).inspect(move |_| {
            *self.field_mut() = Switch::Off;
        })
    }
}

pub type Select<'a, E, SELECT, DISABLE, P> = Abstraction<'a, E, (SELECT, DISABLE), Option<P>>;
impl<E, SELECT, DISABLE, P> Select<'_, E, SELECT, DISABLE, P>
where
    SELECT: WriteInstruction<E, Data = P::Data>,
    DISABLE: CommandInstruction<E>,
    P: Parametric,
{
    pub fn is_selected(&mut self) -> bool {
        self.field().is_some()
    }
    pub fn is_disabled(&mut self) -> bool {
        self.field().is_none()
    }

    pub fn select(mut self, parameters: P) -> io::Result<()> {
        SELECT::write_from_parameters(self.connection(), &parameters).inspect(move |_| {
            *self.field_mut() = Some(parameters);
        })
    }

    pub fn disable(mut self) -> io::Result<()> {
        DISABLE::command(self.connection()).inspect(|_| {
            *self.field_mut() = None;
        })
    }
}

pub type Configure<'a, E, READ, WRITE, P> = Abstraction<'a, E, (READ, WRITE), P>;
impl<E, READ, WRITE, P> Configure<'_, E, READ, WRITE, P>
where
    READ: ReadInstruction<E, Data = P::Data>,
    WRITE: WriteInstruction<E, Data = P::Data>,
    P: Parametric,
{
    pub fn read(mut self, parameters: &mut P) -> io::Result<()> {
        READ::read_to_parameters(self.connection(), parameters).inspect(move |_| {
            *self.field_mut() = P::from_rx_data(buffer);
        })
    }

    pub fn write(mut self, next_state: &impl Parametric<Data = WRITE::Data>) -> io::Result<()> {
        self.device
            .write_parameters::<WRITE>(&next_state)
            .inspect(move |_| {
                *self.field_mut() = next_state;
            })
    }
}
