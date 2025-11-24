use crate::{st7701s_spi::parameters::general::Switch, state_struct};

use Switch::{Off, On};

state_struct! {
  pub struct PowerMode {
    pub booster: Switch = On,
    pub sleep: Switch = Off,
    pub display: Switch = Off,
  }
}

crate::bit_value_enum! {
    pub enum BiasCurrent<2> {
        #[default]
        const Off = 0b00,
        const Min = 0b01,
        const Mid = 0b10,
        const Max = 0b11,
    }
}
