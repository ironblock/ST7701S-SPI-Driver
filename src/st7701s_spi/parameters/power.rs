use crate::{st7701s_spi::parameters::general::{Switch}, state_struct};

use Switch::{On, Off};

state_struct! {
  pub struct PowerMode {
    pub booster: Switch = On,
    pub sleep: Switch = Off,
    pub display: Switch = Off,
  }
}

