use std::fmt::{self};

use crate::st7701s_spi::parameters::{data_access, gamma, pixel_format, register, tearing_effect};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Switch {
    On,
    Off,
}

#[derive(Debug)]
pub enum Direction {
    Normal,
    Reverse,
}

#[derive(Debug)]
pub enum Logic {
    Low,
    High,
}

#[derive(Debug)]
pub enum Edge {
    Rising,
    Falling,
}

#[derive(Debug)]
pub enum Power {
    L1,
    L2,
    L3,
    L4,
    L5,
}

impl Power {
    const fn level(state: State) -> Self {
        use Switch::*;

        match state {
            State {
                partial_mode: Off,
                idle_mode: Off,
                sleep_mode: Off,
                ..
            } => Self::L1,
            State {
                partial_mode: On,
                idle_mode: Off,
                sleep_mode: Off,
                ..
            } => Self::L2,
            State {
                partial_mode: Off,
                idle_mode: On,
                sleep_mode: Off,
                ..
            } => Self::L3,
            State {
                partial_mode: On,
                idle_mode: On,
                sleep_mode: Off,
                ..
            } => Self::L4,
            State { sleep_mode: On, .. } => Self::L5,
        }
    }
}

pub type StateSelector<V> = for<'a> fn(&'a mut State) -> &'a mut V;
pub struct StateContainer(pub Option<State>);

impl StateContainer {
    pub fn new(use_state: bool) -> Self {
        Self(if use_state {
            Some(State::default())
        } else {
            None
        })
    }

    pub fn reset(&mut self) {
        if let Some(state) = &mut self.0 {
            *state = State::default();
        }
    }

    pub fn is<V: PartialEq>(&mut self, value: &V, select: &StateSelector<V>) -> bool {
        if let Some(state) = &mut self.0 {
            select(state) == value
        } else {
            false
        }
    }

    pub fn set<V: PartialEq>(&mut self, value: V, select: &StateSelector<V>) {
        if let Some(state) = &mut self.0 {
            *select(state) = value
        }
    }
}

pub struct State {
    pub partial_mode: Switch,
    pub idle_mode: Switch,
    pub sleep_mode: Switch,
    pub display_output: Switch,
    pub invert_picture: Switch,
    pub brightness_control: Switch,
    pub brightness_dimming: Switch,
    pub brightness_backlight: Switch,
    pub color_enhancement: Switch,
    pub extended_commands: register::Extension,
    pub gamma_curve: gamma::Curve,
    pub tearing_effect: Option<(tearing_effect::Blank,)>,
    pub color_order: data_access::ColorOrder,
    pub scan_direction: data_access::ScanDirection,
    pub color_mode: (),
    pub enhancement_mode: (),
    pub enhancement_adaptive: (),
    pub bits_per_pixel: pixel_format::BitsPerPixel,
}

impl Default for State {
    fn default() -> Self {
        Self {
            partial_mode: Switch::Off,
            idle_mode: Switch::Off,
            sleep_mode: Switch::Off,
            display_output: Switch::Off,
            invert_picture: Switch::Off,
            brightness_control: Switch::Off,
            brightness_dimming: Switch::Off,
            brightness_backlight: Switch::Off,
            color_enhancement: Switch::Off,
            extended_commands: register::Extension(None),
            gamma_curve: gamma::Curve::GC1,
            tearing_effect: None,
            color_order: data_access::ColorOrder::RGB,
            scan_direction: data_access::ScanDirection::Normal,
            color_mode: (),
            enhancement_mode: (),
            enhancement_adaptive: (),
            bits_per_pixel: pixel_format::BitsPerPixel::RGB888,
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("Idle Mode", &self.idle_mode)
            .field("Sleep Mode", &self.sleep_mode)
            .field("Display Output", &self.display_output)
            .field("Invert Picture", &self.invert_picture)
            .field("Brightness Control", &self.brightness_control)
            .field("Brightness Dimming", &self.brightness_dimming)
            .field("Brightness Backlight", &self.brightness_backlight)
            .field("Color Enhancement", &self.color_enhancement)
            .field("Extended Commands", &self.extended_commands)
            .field("Gamma Curve", &self.gamma_curve)
            .field("Tearing Effect", &self.tearing_effect)
            .field("Color Order", &self.color_order)
            .field("Scan Direction", &self.scan_direction)
            .field("Color Mode", &self.color_mode)
            .field("Enhancement Mode", &self.enhancement_mode)
            .field("Enhancement Adaptive", &self.enhancement_adaptive)
            .field("Bits Per Pixel", &self.bits_per_pixel)
            .finish()
    }
}

pub type SelectField<T> = fn(&mut State) -> &mut T;
pub type MutateField<T> = fn(&mut State, T);
