use crate::{enum_argument, st7701s_spi::{transmissions::*}, transmission_mapping};
use pastey::paste;

// FIXME: This isn't necessarily correct, and the datasheet indicates
// that the initial value is "RESERVED". Maybe better to set this to
// something that can never be matched, or change the trait to not
// require an initial value?
enum_argument! {
    pub enum Curve[1:0] {
        #[default]
        GC1 = 0,
        GC2 = 1,
        GC3 = 2,
        GC4 = 3,
    }
}

transmission_mapping! {
    pub struct GammaCurve<1> (
        1: (D0(GC[1:0] as Curve),),
    )
}

enum_argument! {
    pub enum VoltageBias[1:0] {
        #[default]
        L1 = 0,
        L0 = 1,
        R0 = 2,
        R1 = 3,
    }
}



transmission_mapping! {
    /// ## Gamma Voltage Control
    /// > Reference:
    /// > - Circuit Diagram     p. 170, 171
    /// > - `PVGAMCTRL`         p. 261, 262
    /// > - `NVGAMCTRL`         p. 263, 264
    ///
    ///
    /// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
    /// |:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
    /// |     AJ0[1:0]    |   --   |   --   |              VC0[3:0]             |
    /// |     AJ1[1:0]    |                       VC4[5:0]                      |
    /// |     AJ2[1:0]    |                       VC8[5:0]                      |
    /// |   --   |   --   |   --   |                  VC16[4:0]                 |
    /// |     AJ3[1:0]    |   --   |                  VC24[4:0]                 |
    /// |   --   |   --   |   --   |   --   |              VC52[3:0]            |
    /// |   --   |   --   |                       VC80[5:0]                     |
    /// |   --   |   --   |   --   |   --   |             VC108[3:0]            |
    /// |   --   |   --   |   --   |   --   |             VC147[3:0]            |
    /// |   --   |   --   |                      VC175[5:0]                     |
    /// |   --   |   --   |   --   |   --   |             VC203[3:0]            |
    /// |     AJ4[1:0]    |   --   |                 VC231[4:0]                 |
    /// |   --   |   --   |   --   |                 VC239[4:0]                 |
    /// |     AJ5[1:0]    |                      VC247[5:0]                     |
    /// |     AJ6[1:0]    |                      VC251[5:0]                     |
    /// |     AJ7[1:0]    |   --   |                  VC255[4:0]                |
    struct VoltageControl<16>(
        1:  (D6(AJ0[1:0] as VoltageBias), D0(  VC0[3:0] as BitValue::<3,0>),),
        2:  (D6(AJ1[1:0] as VoltageBias), D0(  VC4[5:0] as BitValue::<5,0>),),
        3:  (D6(AJ2[1:0] as VoltageBias), D0(  VC8[5:0] as BitValue::<5,0>),),
        4:  (                             D0( VC16[4:0] as BitValue::<4,0>),),
        5:  (D6(AJ3[1:0] as VoltageBias), D0( VC24[4:0] as BitValue::<4,0>),),
        6:  (                             D0( VC52[5:0] as BitValue::<5,0>),),
        7:  (                             D0( VC80[5:0] as BitValue::<5,0>),),
        8:  (                             D0(VC108[3:0] as BitValue::<3,0>),),
        9:  (                             D0(VC147[3:0] as BitValue::<3,0>),),
        10: (                             D0(VC175[5:0] as BitValue::<5,0>),),
        11: (                             D0(VC203[3:0] as BitValue::<3,0>),),
        12: (D6(AJ4[1:0] as VoltageBias), D0(VC231[4:0] as BitValue::<4,0>),),
        13: (                             D0(VC239[4:0] as BitValue::<4,0>),),
        14: (D6(AJ5[1:0] as VoltageBias), D0(VC247[5:0] as BitValue::<5,0>),),
        15: (D6(AJ6[1:0] as VoltageBias), D0(VC251[5:0] as BitValue::<5,0>),),
        16: (D6(AJ7[1:0] as VoltageBias), D0(VC255[4:0] as BitValue::<4,0>),),
    )
}
