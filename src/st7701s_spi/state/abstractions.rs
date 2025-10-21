use std::{marker::PhantomData};

use crate::st7701s_spi::{
    address::{Command,  Read, Write},
    device::{ActiveDevice, StateAccessorMut},
    parameters::general::Switch,
    protocol::connection::{Connection, InstructionResult},
    transmissions::Parametric,
};

pub struct Abstraction<'a, DEVICE: ActiveDevice, COMMANDS, STATE> {
    device: &'a mut DEVICE,
    accessor: StateAccessorMut<DEVICE, STATE>,
    marker: PhantomData<COMMANDS>,
}
impl<'a, DEVICE: ActiveDevice, COMMANDS, STATE> Abstraction<'a, DEVICE, COMMANDS, STATE> {
    pub const fn new(device: &'a mut DEVICE, accessor: StateAccessorMut<DEVICE, STATE>) -> Self {
        Self {
            device,
            accessor,
            marker: PhantomData,
        }
    }

    pub fn connection(&self) -> &DEVICE::ConnectionType {
        self.device.connection()
    }

    pub fn field(&mut self) -> &STATE {
        (self.accessor)(self.device.state_mut())
    }

    pub fn field_mut(&mut self) -> &mut STATE {
        (self.accessor)(self.device.state_mut())
    }
}

pub type Toggle<'a, DEVICE, ON, OFF> = Abstraction<'a, DEVICE, (ON, OFF), Switch>;
impl<'a, DEVICE, ON, OFF> Toggle<'a, DEVICE, ON, OFF>
where
    DEVICE: ActiveDevice,
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

pub type Select<'a, DEVICE, SELECT, DISABLE, P> =
    Abstraction<'a, DEVICE, (SELECT, DISABLE), Option<P>>;
impl<DEVICE, SELECT, DISABLE, P> Select<'_, DEVICE, SELECT, DISABLE, P>
where
    DEVICE: ActiveDevice,
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

    fn select(mut self, parameters: P) -> InstructionResult {
        self.connection()
            .write::<SELECT>(&parameters.as_tx_data())
            .inspect(move |_| {
                *self.field_mut() = Some(parameters);
            })
    }

    fn disable(mut self) -> InstructionResult {
        self.connection().command::<DISABLE>().inspect(|_| {
            *self.field_mut() = None;
        })
    }
}

pub type Configure<'a,
    DEVICE,
    READ,
    WRITE,
    P> = Abstraction<'a, DEVICE, (READ, WRITE), P>;
impl<DEVICE, READ, WRITE, P> Configure<'_, DEVICE, READ, WRITE, P>
where
    DEVICE: ActiveDevice,
    READ: Read<Data = P::Data>,
    WRITE: Write<Data = P::Data>,
    P: Parametric,
{
    fn read(mut self, buffer: &mut P::Data) -> InstructionResult {
        self.connection().read::<READ>(buffer).inspect(move |_| {
            *self.field_mut() = P::from_rx_data(buffer);
        })
    }
    fn write(mut self, next_state: P) -> InstructionResult {
        self.connection()
            .write::<WRITE>(&next_state.as_tx_data())
            .inspect(move |_| {
                *self.field_mut() = next_state;
            })
    }
}
