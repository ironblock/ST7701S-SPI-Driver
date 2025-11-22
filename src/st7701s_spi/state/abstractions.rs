use std::{io, marker::PhantomData};

use crate::st7701s_spi::{
    device::{ST7701S, StateAccessorMut},
    parameters::general::Switch,
    protocol::connection::{
        Address, Connection, ConnectionOwner as _, RxData, TxData,
        extended_instructions::RequiredExtension,
    },
};

#[derive(Debug)]
pub struct Abstraction<'a, X, E, COMMANDS, STATE> {
    device: &'a mut ST7701S<X, E>,
    accessor: StateAccessorMut<STATE>,
    _commands: PhantomData<COMMANDS>,
}
impl<'a, X, E, COMMANDS, STATE> Abstraction<'a, X, E, COMMANDS, STATE> {
    pub const fn new(device: &'a mut ST7701S<X, E>, accessor: StateAccessorMut<STATE>) -> Self {
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

pub type Toggle<'a, X, E, ON, OFF> = Abstraction<'a, X, E, (ON, OFF), Switch>;
impl<X, E, ON, OFF> Toggle<'_, X, E, ON, OFF>
where
    X: Connection,
    ON: Address + RequiredExtension<E>,
    OFF: Address + RequiredExtension<E>,
{
    pub fn on(mut self) -> io::Result<()> {
        self.device.command::<ON>().inspect(move |()| {
            *self.field_mut() = Switch::On;
        })
    }

    pub fn off(mut self) -> io::Result<()> {
        self.device.command::<OFF>().inspect(move |()| {
            *self.field_mut() = Switch::Off;
        })
    }
}

pub type Select<'a, X, E, SELECT, DISABLE, D> = Abstraction<'a, X, E, (SELECT, DISABLE), Option<D>>;
impl<X, E, SELECT, DISABLE, D> Select<'_, X, E, SELECT, DISABLE, D>
where
    X: Connection,
    SELECT: Address + RequiredExtension<E> + TxData<Data = D>,
    DISABLE: Address + RequiredExtension<E>,
{
    pub fn is_enabled(&mut self) -> bool {
        self.field_mut().is_some()
    }
    pub fn is_disabled(&mut self) -> bool {
        self.field_mut().is_none()
    }

    pub fn select(mut self, buffer: D) -> io::Result<()> {
        self.device.write::<SELECT>(&buffer).inspect(move |()| {
            *self.field_mut() = Some(buffer);
        })
    }

    pub fn disable(mut self) -> io::Result<()> {
        self.device.command::<DISABLE>().inspect(|()| {
            *self.field_mut() = None;
        })
    }
}

pub type Configure<'a, X, E, READ, WRITE, D> = Abstraction<'a, X, E, (READ, WRITE), D>;
impl<X, E, READ, WRITE, D> Configure<'_, X, E, READ, WRITE, D>
where
    X: Connection,
    READ: Address + RequiredExtension<E> + RxData<Data = D>,
    WRITE: Address + RequiredExtension<E> + TxData<Data = D>,
{
    pub fn read(mut self, mut buffer: D) -> io::Result<()> {
        self.device.read::<READ>(&mut buffer).inspect(move |()| {
            *self.field_mut() = buffer;
        })
    }

    pub fn write(mut self, buffer: D) -> io::Result<()> {
        self.device.write::<WRITE>(&buffer).inspect(move |()| {
            *self.field_mut() = buffer;
        })
    }
}
