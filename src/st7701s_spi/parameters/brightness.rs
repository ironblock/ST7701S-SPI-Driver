use crate::{
st7701s_spi::{parameters::general::Switch, transmissions::*}, transmission_mapping
};

pub type Brightness = BitValue<7>;

transmission_mapping! {
    pub struct BrightnessControl<1> (
        0: (
            D7( manual_control<1> as Switch),
            D6(display_dimming<1> as Switch),
            D5(backlight_power<1> as Switch),
        ),
    )
}
