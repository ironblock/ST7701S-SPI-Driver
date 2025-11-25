use crate::{
    bit_value_enum,
    st7701s_spi::{
        parameters::general::Switch,
        transmissions::{BitMask, BitOffset, BitValue, D0, D5, D6, D7},
    },
    transmission_mapping,
};
use Switch::Off;

transmission_mapping!(
    /// ## Display Brightness Value
    /// Sets the brightness level for the display backlight
    /// > Reference: `WRDISBV` p. 221, `RDDISBV` p. 222
    pub struct Brightness<1> (
        0: (D0(value<7>),),
    );
);

transmission_mapping! {
    /// ## Display Brightness Control Modes
    /// Configures brightness control features and backlight settings
    /// > Reference: `WRCTRLD` p. 223, `RDCTRLD` p. 224
    pub struct BrightnessControl<1> (
        0: (
            D7( manual_control<1> as Switch),
            D6(display_dimming<1> as Switch),
            D5(backlight_power<1> as Switch),
        ),
    );
}

bit_value_enum! {
    /// CABC Mode Selection
    pub enum CabcMode<3> {
        #[default]
        const Off = 0b000,
        const UserInterface = 0b001,
        const StillPicture = 0b010,
        const MovingImage = 0b011,
    }
}

transmission_mapping! {
    /// ## Content Adaptive Brightness Control and Color Enhancement
    /// Configures CABC mode for automatic brightness adjustment
    /// > Reference: `WRCACE` p. 225, `RDCABC` p. 227
    pub struct AdaptiveBrightness<1> (
        0: (
            D5(color_enhancement<1> as Switch = Off),
            D0(cabc_mode<3> as CabcMode = Off),
        ),
    );
}

transmission_mapping! {
    /// ## CABC Minimum Brightness
    /// Sets the minimum brightness level for CABC operation
    /// > Reference: `WRCABCMB` p. 229, `RDCABCMB` p. 230
    pub struct MinAdaptiveBrightness<1> (
        0: (D0(min_brightness<8>),),
    );
}
