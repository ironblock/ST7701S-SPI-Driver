use std::{io, marker::PhantomData};

use crate::st7701s_spi::{
    address::{CommandInstruction, ReadInstruction, WriteInstruction},
    device::{Device, Stateful},
    parameters::general::{DecodeData, EncodeData, Switch},
    state::domains::DeviceState,
};

pub type StateGetter<T> = for<'a> fn(&'a DeviceState) -> &'a T;
pub type StateSetter<T> = for<'a> fn(&'a DeviceState, T);

pub struct StateAbstraction<'a, DEVICE, STATE, MARKER>
where
    DEVICE: Device + Stateful<DeviceState>,
    MARKER: ?Sized,
{
    device: &'a mut DEVICE,
    state_getter: StateGetter<STATE>,
    state_setter: StateSetter<STATE>,
    marker: PhantomData<MARKER>,
}
impl<'a, DEVICE, STATE, MARKER> StateAbstraction<'a, DEVICE, STATE, MARKER>
where
    DEVICE: Device + Stateful<DeviceState>,
    MARKER: ?Sized,
{
    pub const fn new(device: &'a mut DEVICE, state_getter: StateGetter<STATE>, state_setter: StateSetter<STATE>) -> Self {
        Self {
            device,
            state_getter,
            state_setter,
            marker: PhantomData,
        }
    }

    pub fn state(&self) -> &STATE {
        (self.state_getter)(&self.device.state())
    }

    pub fn set_state(&mut self, next_state: STATE) {
        (self.state_setter)(&self.device.state_mut(), next_state);
    }

    pub fn command<C: CommandInstruction>(mut self, next_state: STATE) -> io::Result<usize> {
        self.device.command::<C>().inspect(|_| {
            self.set_state(next_state);
        })
    }

    pub fn write<W: WriteInstruction>(
        mut self,
        next_state: STATE,
    ) -> io::Result<usize> where STATE: EncodeData<W>  {
        self.device.write::<W>(next_state.encode()).inspect(|_| {
            self.set_state(next_state);
        })
    }

    pub fn read<R: ReadInstruction>(
        mut self,
    ) -> Result<R::Buffer, std::io::Error> where STATE: DecodeData<R>  {
        self.device.read::<R>().inspect(|buffer| {
            self.set_state(STATE::decode(buffer));
        })
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
        (self.state_getter)(&self.device.state()).is_on()
    }

    pub fn is_off(&mut self) -> bool {
        (self.state_getter)(&self.device.state()).is_off()
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
    pub fn is_on(&self) -> bool {
        self.state().is_some()
    }

    pub fn is_off(&self) -> bool {
        self.state().is_none()
    }

    pub fn configure(self, next_state: STATE) -> io::Result<usize> {
        let parameters = next_state.encode();
        self.write::<SELECT>(Some(next_state))
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
    pub fn get(&mut self) -> io::Result<STATE> {
        self.device
            .read::<READ>()
            .map(|buffer| STATE::decode(&buffer))
    }

    pub fn set(self, next_state: STATE) -> io::Result<usize> {
        let parameters = next_state.encode();
        self.write::<WRITE>(next_state, parameters)
    }
}
