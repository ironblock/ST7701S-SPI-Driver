use std::{io, marker::PhantomData};

use log::info;

use crate::st7701s_spi::{
    address::{CommandInstruction, WriteInstruction},
    device::Transciever,
    parameters::general::Switch,
};

pub struct StateAbstraction<'a, DEVICE, STATE, MARKER>
where
    DEVICE: Transciever,
    MARKER: ?Sized,
{
    transciever: &'a mut DEVICE,
    current_state: &'a mut STATE,
    marker: PhantomData<MARKER>,
}
impl<'a, DEVICE, STATE, MARKER> StateAbstraction<'a, DEVICE, STATE, MARKER>
where
    DEVICE: Transciever,
    MARKER: ?Sized,
{
    pub const fn new(transciever: &'a mut DEVICE, current_state: &'a mut STATE) -> Self {
        Self {
            transciever,
            current_state,
            marker: PhantomData,
        }
    }

    pub fn command<C: CommandInstruction>(&mut self, next_state: STATE) -> io::Result<usize> {
        self.transciever.command::<C>().inspect(|_| {
            *self.current_state = next_state;
        })
    }

    pub fn write<W: WriteInstruction>(
        &mut self,
        next_state: STATE,
        parameters: W::Buffer,
    ) -> io::Result<usize> {
        self.transciever.write::<W>(parameters).inspect(|_| {
            *self.current_state = next_state;
        })
    }

    pub const fn state(&self) -> &STATE {
        self.current_state
    }
}

pub mod toggle {
    use std::io;

    use crate::st7701s_spi::{
        address::CommandInstruction, device::Transciever, parameters::general::Switch,
        state::StateAbstraction,
    };

    pub trait ToggleMode {}
    struct ToggleOn;
    impl ToggleMode for ToggleOn {}
    struct ToggleOff;
    impl ToggleMode for ToggleOff {}

    pub type Toggle<'a, DEVICE, ON, OFF, MODE> =
        StateAbstraction<'a, DEVICE, Switch, (ON, OFF, MODE)>;

    impl<DEVICE, ON, OFF> Toggle<'_, DEVICE, ON, OFF, dyn ToggleMode>
    where
        DEVICE: Transciever,
        ON: CommandInstruction,
        OFF: CommandInstruction,
    {
        pub fn is_on(&self) -> bool {
            self.state() == &Switch::On
        }

        pub fn is_off(&self) -> bool {
            self.state() == &Switch::Off
        }
    }

    impl<DEVICE, ON, OFF> Toggle<'_, DEVICE, ON, OFF, ToggleOn>
    where
        DEVICE: Transciever,
        ON: CommandInstruction,
        OFF: CommandInstruction,
    {
        pub fn off(mut self) -> io::Result<usize> {
            self.command::<OFF>(Switch::Off)
        }
    }

    impl<DEVICE, ON, OFF> Toggle<'_, DEVICE, ON, OFF, ToggleOff>
    where
        DEVICE: Transciever,
        ON: CommandInstruction,
        OFF: CommandInstruction,
    {
        pub fn on(mut self) -> io::Result<usize> {
            self.command::<OFF>(Switch::On)
        }
    }
}

pub mod configure {
    use std::io;

    use crate::st7701s_spi::{
        address::{CommandInstruction, WriteInstruction}, device::Transciever, parameters::general::EncodeData, state::StateAbstraction
    };

    trait ConfigureMode {}
    struct Configure;
    impl ConfigureMode for Configure {}
    struct Disable;
    impl ConfigureMode for Disable {}

    pub type ConfigureState<'a, DEVICE,  CONFIG, OFF, MODE, T> =
        StateAbstraction<'a, DEVICE, Option<T>, (CONFIG, OFF, MODE, T)>;

    impl<'a, DEVICE, CONFIG, OFF, T> ConfigureState<'a, DEVICE, CONFIG, OFF, Configure, T>
    where
        DEVICE: Transciever,
        CONFIG: WriteInstruction,
        OFF: CommandInstruction,
        T: EncodeData<CONFIG>,
    {
        pub fn configure(mut self, next_state: T) -> io::Result<usize> {
            let parameters = next_state.encode();
            self.write::<CONFIG>(Some(next_state), parameters)
        }
    }

    impl<'a, DEVICE, CONFIG, OFF, T> ConfigureState<'a, DEVICE, CONFIG, OFF, Disable, T>
    where
        DEVICE: Transciever,
        CONFIG: WriteInstruction,
        OFF: CommandInstruction,
    {
        pub fn disable(mut self) -> io::Result<usize> {
            self.command::<OFF>(None)
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Power {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeState {
    pub partial: Switch,
    pub idle: Switch,
    pub sleep: Switch,
    pub display: Switch,
    pub standby: Switch,
}
impl ModeState {
    pub const fn new() -> Self {
        use Switch::*;

        Self {
            partial: Off,
            idle: Off,
            sleep: On,
            display: Off,
            standby: Off,
        }
    }

    pub const fn reset(&mut self) {
        *self = Self::new();
    }

    pub const fn power_state(&self) -> Power {
        use Switch::*;

        match self {
            Self { standby: On, .. } => Power::L6,
            Self { sleep: On, .. } => Power::L5,
            Self {
                partial: On,
                idle: On,
                ..
            } => Power::L4,
            Self { idle: On, .. } => Power::L3,
            Self { partial: On, .. } => Power::L2,
            Self { .. } => Power::L1,
        }
    }
}
impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceState {
    pub mode: ModeState,
}
impl DeviceState {
    pub const fn new() -> Self {
        Self {
            mode: ModeState::new(),
        }
    }

    pub fn reset(&mut self) {
        info!("Resetting cached device state");
        self.mode.reset();
    }
}
impl Default for DeviceState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StatefulDevice {
    fn state(&self) -> &DeviceState;
    fn state_mut(&mut self) -> &mut DeviceState;
}
