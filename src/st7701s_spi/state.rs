use crate::st7701s_spi::{
    interface::{Location, Operation},
    parameters::{data_access, gamma, pixel_format, register, tearing_effect},
};

#[derive(PartialEq)]
pub enum Switch {
    On,
    Off,
}


pub const fn toggle<ON: Location, OFF: Location>(mode: Switch) -> Operation<0> {
    match mode {
        Switch::Off => Operation::command::<OFF>(),
        Switch::On => Operation::command::<ON>(),
    }
}

pub enum Direction {
    Normal,
    Reverse
}

pub enum Logic {
    Low,
    High,
}

pub enum Edge {
    Rising,
    Falling,
}

pub enum Power {
    L1,
    L2,
    L3,
    L4,
    L5,
}

impl Power {
    const fn determine(state: State) -> Self {
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
    pub tearing_effect: Option<tearing_effect::Blank>,
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
    brightness_control: Switch,
    brightness_dimming: Switch,
    brightness_backlight: Switch,
    color_enhancement: Switch,
    extended_commands: None,
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

