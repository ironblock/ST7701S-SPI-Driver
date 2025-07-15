use crate::st7701s_spi::interface::{Instruction, Transmission};

pub enum Toggle {
    On,
    Off,
}

pub struct ToggleCommands {
    pub on: Instruction,
    pub off: Instruction,
}

impl ToggleCommands {
    pub const fn new(on: Instruction, off: Instruction) -> Self {
        Self { on, off }
    }

    pub const fn toggle(&self, mode: Toggle) -> Transmission<0> {
        match mode {
            Toggle::Off => self.on.to_command(),
            Toggle::On => self.off.to_command(),
        }
    }
}

pub struct SelectCommands {
    pub select: Instruction,
    pub disable: Instruction,
}

// pub const fn toggle_state(commands: ToggleCommands, mode: Toggle) -> Transmission {
//     match mode {
//         Toggle::Off => commands.on.to_command(),
//         Toggle::On => commands.off.to_command(),
//     }
// }

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

// const NO_OPERATION: Instruction =
//     Instruction::from_location(Core::NOP.location(), Operation::Command);
// const SLEEP_MODE: ToggleState = ToggleState::new(Core::SLPIN.location(), Core::SLPOUT.location());
