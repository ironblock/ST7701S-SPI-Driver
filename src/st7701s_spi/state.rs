use crate::st7701s_spi::interface::{Location, Operation};

pub enum Toggle {
    On,
    Off,
}

pub const fn toggle<ON: Location, OFF: Location>(mode: Toggle) -> Operation<0> {
    match mode {
        Toggle::Off => Operation::command::<OFF>(),
        Toggle::On => Operation::command::<ON>(),
    }
}

pub enum Logic {
    Low,
    High,
}

pub enum Edge {
    Rising,
    Falling,
}

// pub enum Sleep {
//     In,
//     Out,
// }

// pub enum Mode {
//     Normal,
//     Partial,
// }

// pub enum Idle {
//     Disabled,
//     Enabled,
// }

// pub enum PowerLevel {
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
// }

// impl PowerLevel {
//     const fn determine(mode: Mode, idle: Idle, sleep: Sleep) -> Self {
//         match (mode, idle, sleep) {
//             (Mode::Normal, Idle::Disabled, Sleep::Out) => Self::One,
//             (Mode::Partial, Idle::Disabled, Sleep::Out) => Self::Two,
//             (Mode::Normal, Idle::Enabled, Sleep::Out) => Self::Three,
//             (Mode::Partial, Idle::Enabled, Sleep::Out) => Self::Four,
//             (_, _, Sleep::In) => Self::Five,
//         }
//     }
// }

pub struct DriverState {
    sleep: Toggle,
    display: Toggle,
    invert: Toggle,
    idle: Toggle,
    brightness_control: Toggle,
    brightness_dimming: Toggle,
    brightness_backlight: Toggle,
    color_enhancement: Toggle,
    extended_commands: (),
    gamma_curve: (),
    tearing_effect: (),
    color_mode: (),
    enhancement_mode: (),
    enhancement_adaptive: (),
}
