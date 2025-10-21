use crate::{
    bit_value_enum,
    st7701s_spi::{parameters::general::Switch, transmissions::*},
    transmission_mapping,
};

bit_value_enum! {
    pub enum TearingEffectMode<1> {
        #[default]
        const Vertical = 0,
        const VerticalHorizontal = 1,
    }
}

transmission_mapping! {
    pub struct TearingEffectSignal<1> (
        0: (D0(tearing_effect<1> as TearingEffectMode),),
    );
}



// FIXME: This isn't necessarily correct, and the datasheet indicates
// that the initial value is "RESERVED". Maybe better to set this to
// something that can never be matched, or change the trait to not
// require an initial value?
bit_value_enum! {
    pub enum Curve<2> {
        #[default]
        const GC1 = 0,
        const GC2 = 1,
        const GC3 = 2,
        const GC4 = 3,
    }
}

transmission_mapping! {
    pub struct GammaCurve<1> (
        0: (D0(GC<2> as Curve),),
    );
}

bit_value_enum! {
    pub enum VoltageBias<2> {
        #[default]
        const L1 = 0,
        const L0 = 1,
        const R0 = 2,
        const R1 = 3,
    }
}

transmission_mapping! {
    /// ## Gamma Voltage Control
    /// > Reference:
    /// > - Circuit Diagram     p. 170, 171
    /// > - `PVGAMCTRL`         p. 261, 262
    /// > - `NVGAMCTRL`         p. 263, 264
    pub struct VoltageControl<16>(
        0:  (D6(AJ0<2> as VoltageBias), D0(  VC0<4>),),
        1:  (D6(AJ1<2> as VoltageBias), D0(  VC4<6>),),
        2:  (D6(AJ2<2> as VoltageBias), D0(  VC8<6>),),
        3:  (                           D0( VC16<4>),),
        4:  (D6(AJ3<2> as VoltageBias), D0( VC24<4>),),
        5:  (                           D0( VC52<6>),),
        6:  (                           D0( VC80<6>),),
        7:  (                           D0(VC108<4>),),
        8:  (                           D0(VC147<4>),),
        9:  (                           D0(VC175<6>),),
        10: (                           D0(VC203<4>),),
        11: (D6(AJ4<2> as VoltageBias), D0(VC231<4>),),
        12: (                           D0(VC239<4>),),
        13: (D6(AJ5<2> as VoltageBias), D0(VC247<6>),),
        14: (D6(AJ6<2> as VoltageBias), D0(VC251<6>),),
        15: (D6(AJ7<2> as VoltageBias), D0(VC255<4>),),
    );
}

transmission_mapping! {
    pub struct DisplayImageMode<1> (
        0: (
            D5(   invert_colors<1> as Switch),
            D4(all_pixels_white<1> as Switch),
            D3(all_pixels_black<1> as Switch),
            D0(     gamma_curve<2> as Curve),
        ),
    );
}

transmission_mapping! {
    pub struct DisplaySignalMode<1> (
        0: (
            D7(tearing_effect_line<1> as Switch),
            D6(tearing_effect_mode<1> as TearingEffectMode),
        ),
    );
}
