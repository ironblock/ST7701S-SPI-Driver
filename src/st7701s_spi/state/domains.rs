use crate::{
    st7701s_spi::{
        parameters::{brightness::*, general::Switch},
        state::derived::Power,
    },
    state_struct,
};

use Switch::*;

state_struct! {
  pub struct ModeState {
      pub partial: Switch = Off,
      pub idle: Switch = Off,
      pub sleep: Switch = On,
      pub display: Switch = Off,
      pub standby: Switch = Off,
  }
}
impl ModeState {
    pub const fn power_state(&self) -> Power {
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

state_struct! {
  pub struct ConfigurationState {
    pub invert_colors: Switch = Off,
    pub all_pixels_white: Switch = Off,
    pub all_pixels_black: Switch = Off,
    pub brightness_value: Brightness = Brightness::new(0),
    pub brightness_control: BrightnessControl = BrightnessControl::new(),
  }
}

state_struct! {
    pub struct DeviceState {
        pub mode: ModeState = ModeState::new(),
        pub config: ConfigurationState = ConfigurationState::new(),
    }
}
