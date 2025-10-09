use crate::{
st7701s_spi::{parameters::general::Switch, transmissions::*}, transmission_mapping
};
use pastey::paste;

pub type Brightness = BitValue<7,0>;

transmission_mapping! {
    pub struct BrightnessControl<1> (
        0: (
            D7(BCTRL[0:0] as Switch),
            D6(DD[0:0] as Switch),
            D5(BL[0:0] as Switch),
        ),
    )
}
