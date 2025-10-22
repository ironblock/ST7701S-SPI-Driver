use crate::{
    st7701s_spi::{
        parameters::{
            brightness::*,
            display::{DisplayImageMode, TearingEffectSignal},
            general::Switch,
            register::CommandExtension,
        },
        state::derived::Power,
    },
    state_struct,
};

use Switch::*;

// RDDPM
//   - backlight state
//   - sleep on/off
state_struct! {
  pub struct ModeState {
      pub standby: Switch = Off,
      pub sleep:   Switch = On,
      pub partial: Switch = Off,
      pub idle:    Switch = Off,
      pub display: Switch = Off,
  }
}
impl ModeState {
    pub const fn power_state(&self) -> Power {
        match self {
            Self { standby: On, .. } => Power::L6,
            Self { sleep: On, .. }  => Power::L5,
            Self { partial: On, idle: On, .. }  => Power::L4,
            Self { idle: On, .. } => Power::L3,
            Self { partial: On, .. } => Power::L2,
            _ => Power::L1,
        }
    }
}

state_struct! {
  pub struct ConfigurationState {
    pub brightness:             Brightness            = Brightness::new(),
    pub brightness_control:     BrightnessControl     = BrightnessControl::new(),
    pub adaptive_brightness:    AdaptiveBrightness    = AdaptiveBrightness::new(),
    pub min_adaptive_brightness: MinAdaptiveBrightness = MinAdaptiveBrightness::new(),
  }
}

state_struct! {
    pub struct DeviceState {
        pub command_extension: CommandExtension = CommandExtension::new(),
        pub mode: ModeState = ModeState::new(),
        pub image: DisplayImageMode = DisplayImageMode::new(),
        pub tearing_effect: Option<TearingEffectSignal> = Some(TearingEffectSignal::new()),
        pub config: ConfigurationState = ConfigurationState::new(),
    }
}

