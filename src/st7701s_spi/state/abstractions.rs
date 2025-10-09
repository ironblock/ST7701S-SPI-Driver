use std::{io, marker::PhantomData};

use crate::st7701s_spi::{
    address::{CommandInstruction, ReadInstruction, WriteInstruction},
    device::{Device, Stateful},
    parameters::general::{DecodeData, EncodeData, Switch},
    state::domains::DeviceState,
};

pub type StateSelector<STATE> = for<'a> fn(&'a mut DeviceState) -> &'a mut STATE;

pub struct StateAbstraction<'a, DEVICE, STATE, MARKER>
where
    DEVICE: Device + Stateful<DeviceState>,
    MARKER: ?Sized,
{
    device: &'a mut DEVICE,
    selector: StateSelector<STATE>,
    marker: PhantomData<MARKER>,
}
impl<'a, DEVICE, STATE, MARKER> StateAbstraction<'a, DEVICE, STATE, MARKER>
where
    DEVICE: Device + Stateful<DeviceState>,
    MARKER: ?Sized,
{
    pub const fn new(device: &'a mut DEVICE, selector: StateSelector<STATE>) -> Self {
        Self {
            device,
            selector,
            marker: PhantomData,
        }
    }

    pub fn command<C: CommandInstruction>(mut self, next_state: STATE) -> io::Result<usize> {
        self.device.command::<C>().inspect(|_| {
            *(self.state_mut()) = next_state;
        })
    }

    pub fn write<W: WriteInstruction>(
        mut self,
        next_state: STATE,
        parameters: W::Buffer,
    ) -> io::Result<usize> {
        self.device.write::<W>(parameters).inspect(|_| {
            *(self.state_mut()) = next_state;
        })
    }

    pub fn state_mut(&mut self) -> &mut STATE {
        (self.selector)(self.device.state_mut())
    }
}

pub type Toggle<'a, DEVICE, ON, OFF> = StateAbstraction<'a, DEVICE, Switch, (ON, OFF)>;
impl<DEVICE, ON, OFF> Toggle<'_, DEVICE, ON, OFF>
where
    DEVICE: Device + Stateful<DeviceState>,
    ON: CommandInstruction,
    OFF: CommandInstruction,
{
    pub fn is_on(&mut self) -> bool {
        self.state_mut() == &Switch::On
    }

    pub fn is_off(&mut self) -> bool {
        self.state_mut() == &Switch::Off
    }

    pub fn on(self) -> io::Result<usize> {
        self.command::<ON>(Switch::On)
    }

    pub fn off(self) -> io::Result<usize> {
        self.command::<OFF>(Switch::Off)
    }
}

pub type Select<'a, DEVICE, STATE, SELECT, DISABLE> =
    StateAbstraction<'a, DEVICE, Option<STATE>, (SELECT, DISABLE)>;
impl<DEVICE, STATE, SELECT, DISABLE> Select<'_, DEVICE, STATE, SELECT, DISABLE>
where
    DEVICE: Device + Stateful<DeviceState>,
    STATE: EncodeData<SELECT>,
    SELECT: WriteInstruction,
    DISABLE: CommandInstruction,
{
    pub fn is_on(&mut self) -> bool {
        self.state_mut().is_some()
    }

    pub fn is_off(&mut self) -> bool {
        self.state_mut().is_none()
    }

    pub fn configure(self, next_state: STATE) -> io::Result<usize> {
        let parameters = next_state.encode();
        self.write::<SELECT>(Some(next_state), parameters)
    }

    pub fn disable(self) -> io::Result<usize> {
        self.command::<DISABLE>(None)
    }
}

pub type Configure<'a, DEVICE, STATE, READ, WRITE> =
    StateAbstraction<'a, DEVICE, STATE, (READ, WRITE)>;

impl<DEVICE, STATE, READ, WRITE> Configure<'_, DEVICE, STATE, READ, WRITE>
where
    DEVICE: Device + Stateful<DeviceState>,
    STATE: EncodeData<WRITE> + DecodeData<READ>,
    READ: ReadInstruction,
    WRITE: WriteInstruction,
{
    pub fn read_configuration(&mut self) -> io::Result<STATE> {
        self.device
            .read::<READ>()
            .map(|buffer| STATE::decode(&buffer))
    }

    pub fn write_configuration(self, next_state: STATE) -> io::Result<usize> {
        let parameters = next_state.encode();
        self.write::<WRITE>(next_state, parameters)
    }
}
