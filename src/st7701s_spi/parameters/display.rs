use crate::{
    bit_value_enum,
    st7701s_spi::{parameters::general::Switch, transmissions::*},
    transmission_mapping,
};
use Switch::*;

// ============================================================================
// Tearing Effect - p. 212
// ============================================================================

bit_value_enum! {
    /// Tearing Effect Signal Mode
    pub enum TearingEffectMode<1> {
        #[default]
        const Vertical = 0,
        const VerticalHorizontal = 1,
    }
}

transmission_mapping! {
    /// ## Tearing Effect Line Configuration
    /// Configures the tearing effect signal output mode
    /// > Reference: `TELON` p. 212
    pub struct TearingEffectSignal<1> (
        0: (D0(tearing_effect<1> as TearingEffectMode),),
    );
}

// ============================================================================
// Gamma Curve Selection - p. 208
// ============================================================================

// FIXME: This isn't necessarily correct, and the datasheet indicates
// that the initial value is "RESERVED". Maybe better to set this to
// something that can never be matched, or change the trait to not
// require an initial value?
bit_value_enum! {
    /// Gamma Curve Selection
    pub enum Curve<2> {
        #[default]
        const GC1 = 0,
        const GC2 = 1,
        const GC3 = 2,
        const GC4 = 3,
    }
}

transmission_mapping! {
    /// ## Gamma Curve Selection
    /// Selects one of four predefined gamma curves
    /// > Reference: `GAMSET` p. 208
    pub struct GammaCurve<1> (
        0: (D0(GC<2> as Curve),),
    );
}

// ============================================================================
// Gamma Voltage Control - p. 261-264
// ============================================================================

bit_value_enum! {
    /// Voltage Bias Adjustment
    pub enum VoltageBias<2> {
        #[default]
        const A = 0x00,
        const B = 0x01,
        const C = 0x02,
        const D = 0x03,
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

// ============================================================================
// Display Image Mode - p. 204-208
// ============================================================================

transmission_mapping! {
    /// ## Display Image Mode Settings
    /// Controls color inversion, pixel fill modes, and gamma curve selection
    /// > Reference: `INVOFF`/`INVON` p. 204-205, `ALLPOFF`/`ALLPON` p. 206-207, `GAMSET` p. 208
    pub struct DisplayImageMode<1> (
        0: (
            D5(   invert_colors<1> as Switch),
            D4(all_pixels_white<1> as Switch),
            D3(all_pixels_black<1> as Switch),
            D0(     gamma_curve<2> as Curve),
        ),
    );
}

// ============================================================================
// Display Signal Mode - p. 211-212
// ============================================================================

transmission_mapping! {
    /// ## Tearing Effect Signal Configuration
    /// Configures tearing effect signal output
    /// > Reference: `TEOFF`/`TELON` p. 211-212
    pub struct DisplaySignalMode<1> (
        0: (
            D7(tearing_effect_line<1> as Switch),
            D6(tearing_effect_mode<1> as TearingEffectMode),
        ),
    );
}

// ============================================================================
// Line Settings - p. 213
// ============================================================================

transmission_mapping!(
    /// ## Tearing Effect Scan Line
    /// Sets the scan line at which the tearing effect signal is output
    /// > Reference: `TESCAN` p. 213
    pub struct LineSettings<2>(
        0: (D7(extra_line<1> as Switch = On), D0(line<7> = 0x2B),),
        1: (D1(line_delta<2>),),
    );
);

// ============================================================================
// Porch Control - p. 269
// ============================================================================

transmission_mapping!(
    /// ## Vertical Porch Control
    /// Configures vertical back and front porch timing
    /// > Reference: `BK0: PORCTRL` p. 269
    pub struct PorchControl<2>(
        0: (D0(vertical_back_porch<8>  = 0x04),),
        1: (D0(vertical_front_porch<8> = 0x02),),
    );
);

// ============================================================================
// Inversion Selection - p. 270
// ============================================================================

bit_value_enum!(
    /// Polarity Inversion Pattern
    pub enum PolarityInversion<3> {
        #[default]
        const OneDot = 0b000,
        const TwoDot = 0b001,
        const Column = 0b111,
    }
);

transmission_mapping!(
    /// ## Display Inversion Selection
    /// Configures the display inversion pattern and timing
    /// > Reference: `BK0: INVSEL` p. 270
    pub struct InversionSelection<2>(
        0: (D0(polarity_inversion<3> as PolarityInversion),),
        1: (D0(RTNI<5>),),
    );
);