use std::{io, marker::PhantomData};

use crate::st7701s_spi::{
    device::{ST7701S, StateAccessorMut},
    parameters::general::Switch,
    protocol::connection::{
        CommandInstruction, ConnectionOwner as _, ReadInstruction, WriteInstruction,
    },
};

pub struct Abstraction<'a, E, COMMANDS, STATE> {
    device: &'a mut ST7701S<E>,
    accessor: StateAccessorMut<STATE>,
    _commands: PhantomData<COMMANDS>,
}
impl<'a, E, COMMANDS, STATE> Abstraction<'a, E, COMMANDS, STATE> {
    pub const fn new(device: &'a mut ST7701S<E>, accessor: StateAccessorMut<STATE>) -> Self {
        Self {
            device,
            accessor,
            _commands: PhantomData,
        }
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
        self.device.command::<ON>().inspect(move |_| {
            *self.field_mut() = Switch::On;
        })
    }

    pub fn off(mut self) -> io::Result<()> {
        self.device.command::<OFF>().inspect(move |_| {
            *self.field_mut() = Switch::Off;
        })
    }
}

pub type Select<'a, E, SELECT, DISABLE, D>
where
    SELECT: WriteInstruction<E, Data = D>,
    DISABLE: CommandInstruction<E>,
= Abstraction<'a, E, (SELECT, DISABLE), Option<D>>;
impl<E, SELECT, DISABLE, D> Select<'_, E, SELECT, DISABLE, D>
where
    SELECT: WriteInstruction<E, Data = D>,
    DISABLE: CommandInstruction<E>,
{
    pub fn is_enabled(&mut self) -> bool {
        self.field_mut().is_some()
    }
    pub fn is_disabled(&mut self) -> bool {
        self.field_mut().is_none()
    }

    pub fn select(mut self, buffer: D) -> io::Result<()> {
        self.device.write::<SELECT>(&buffer).inspect(move |_| {
            *self.field_mut() = Some(buffer);
        })
    }

    pub fn disable(mut self) -> io::Result<()> {
        self.device.command::<DISABLE>().inspect(|_| {
            *self.field_mut() = None;
        })
    }
}

pub type Configure<'a, E, READ, WRITE, D>
where
    READ: ReadInstruction<E, Data = D>,
    WRITE: WriteInstruction<E, Data = D>,
= Abstraction<'a, E, (READ, WRITE), D>;
impl<E, READ, WRITE, D> Configure<'_, E, READ, WRITE, D>
where
    READ: ReadInstruction<E, Data = D>,
    WRITE: WriteInstruction<E, Data = D>,
{
    pub fn read(mut self, mut buffer: D) -> io::Result<()> {
        self.device.read::<READ>(&mut buffer).inspect(move |_| {
            *self.field_mut() = buffer;
        })
    }

    pub fn write(mut self, buffer: D) -> io::Result<()> {
        self.device.write::<WRITE>(&buffer).inspect(move |_| {
            *self.field_mut() = buffer;
        })
    }
}
