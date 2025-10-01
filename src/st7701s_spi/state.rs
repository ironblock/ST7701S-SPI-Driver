use std::{io, marker::PhantomData};

use crate::st7701s_spi::{
    address::{CommandInstruction, WriteInstruction},
    parameters::{self, general::{EncodeData, Switch}},
    spi::Transciever,
};

static NO_EFFECT: &str = "Command will have no effect";
static MALFORMED: &str = "Command is malformed";
static NOT_TRACKING: &str = "STATE TRACKING IS DISABLED";

struct StateAbstraction<'a, STATE, MARKER: ?Sized> {
    transciever: &'a mut Transciever,
    current_state: &'a mut STATE,
    marker: PhantomData<MARKER>,
}
impl<'a, STATE, MARKER: ?Sized> StateAbstraction<'a, STATE, MARKER> {
    pub const fn new(transciever: &'a mut Transciever, current_state: &'a mut STATE) -> Self {
        Self {
            transciever,
            current_state,
            marker: PhantomData,
        }
    }

    pub fn command<C: CommandInstruction>(&mut self, next_state: STATE) -> io::Result<usize> {
        self.transciever.command::<C>().map(|written| {
            *self.current_state = next_state;

            written
        })
    }

    pub fn write<W: WriteInstruction>(&mut self, next_state: STATE, parameters: impl AsRef<[u8]> + IntoIterator<Item = u8>) -> io::Result<usize>
    where
        STATE: EncodeData,
    {
        self.transciever
            .write::<W>( parameters)
            .map(|written| {
                *self.current_state = next_state;

                written
            })
    }

    pub const fn state(&self) -> &STATE {
        self.current_state
    }
}

pub mod toggle {
    use std::io;

    use crate::st7701s_spi::{
        address::CommandInstruction, parameters::general::Switch, state::StateAbstraction,
    };

    trait ToggleMode {}
    struct ToggleOn;
    impl ToggleMode for ToggleOn {}
    struct ToggleOff;
    impl ToggleMode for ToggleOff {}

    pub type Toggle<'a, ON, OFF, MODE>
    where
        ON: CommandInstruction,
        OFF: CommandInstruction,
        MODE: ToggleMode,
    = StateAbstraction<'a, Switch, (ON, OFF, MODE)>;

    impl<ON, OFF> Toggle<'_, ON, OFF, dyn ToggleMode>
    where
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

    impl<ON, OFF> Toggle<'_, ON, OFF, ToggleOn>
    where
        ON: CommandInstruction,
        OFF: CommandInstruction,
    {
        pub fn off(mut self) -> io::Result<usize> {
            self.command::<OFF>(Switch::Off)
        }
    }

    impl<ON, OFF> Toggle<'_, ON, OFF, ToggleOff>
    where
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
        address::{CommandInstruction, WriteInstruction},
        parameters::general::EncodeData,
        state::StateAbstraction,
    };

    trait ConfigureMode {}
    struct Configure;
    impl ConfigureMode for Configure {}
    struct Disable;
    impl ConfigureMode for Disable {}

    pub type ConfigureState<'a, CONFIG, OFF, MODE, T>
    where
        CONFIG: WriteInstruction,
        OFF: CommandInstruction,
        MODE: ConfigureMode,
        T: EncodeData,
    = StateAbstraction<'a, T, (CONFIG, OFF, MODE, T)>;

    impl<'a, CONFIG, OFF, T> ConfigureState<'a, CONFIG, OFF, Configure, T>
    where
        CONFIG: WriteInstruction,
        OFF: CommandInstruction,
        T: EncodeData,
    {
        pub fn configure(mut self, next_state: T) -> io::Result<usize> {
            self.write::<CONFIG>(next_state, &next_state.encode())
        }
    }

    impl<'a, CONFIG, OFF, T> ConfigureState<'a, CONFIG, OFF, Disable, T>
    where
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
pub struct DeviceState {
    pub mode: ModeState,
}
impl DeviceState {
    pub const fn new() -> Self {
        Self {
            mode: ModeState::new(),
        }
    }
}
impl Default for DeviceState {
    fn default() -> Self {
        Self::new()
    }
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
