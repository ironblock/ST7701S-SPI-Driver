use core::panic;
use std::fmt::Display;

use crate::{
    bit_value_enum,
    st7701s_spi::{
        parameters::general::Switch,
        transmissions::{BitMask, BitOffset, BitValue, D0, D4, D6, D7},
    },
    transmission_mapping,
};
use MipiLaneCount::OneLane;
use Switch::{Off, On};

bit_value_enum! {
    /// MIPI Lane Configuration
    pub enum MipiLaneCount<2> {
        #[default]
        const OneLane = 0b00,
        const TwoLane = 0b01,
        const ThreeLane = 0b10,
        const FourLane = 0b11,
    }
}

bit_value_enum! {
    /// MIPI Video Mode
    pub enum MipiVideoMode<2> {
        #[default]
        const NonBurstSync = 0b00,
        const BurstMode = 0b01,
        const NonBurstSyncEvents = 0b10,
    }
}

transmission_mapping! {
    /// ## MIPI Configuration Setting 1
    /// Basic MIPI interface configuration
    /// > Reference: `MIPISET1` p. 299
    pub struct MipiSetting1<1>(
        0: (
            D7(mipi_enable<1> as Switch = On),
            D4(lane_count<2> as MipiLaneCount = OneLane),
            D0(video_mode<2> as MipiVideoMode),
        ),
    );
}

transmission_mapping! {
    /// ## MIPI Configuration Setting 2
    /// Detailed MIPI timing and signal parameters
    /// > Reference: `MIPISET2` p. 300
    pub struct MipiSetting2<15>(
        0:  (D0(eotp_enable<1> as Switch = On),),
        1:  (D0(clk_lane_hs_prepare<8> = 0x06),),
        2:  (D0(clk_lane_hs_zero<8> = 0x0E),),
        3:  (D0(clk_lane_hs_trail<8> = 0x08),),
        4:  (D0(clk_lane_hs_exit<8> = 0x0A),),
        5:  (D0(data_lane_hs_prepare<8> = 0x06),),
        6:  (D0(data_lane_hs_zero<8> = 0x0C),),
        7:  (D0(data_lane_hs_trail<8> = 0x08),),
        8:  (D0(data_lane_hs_exit<8> = 0x0A),),
        9:  (D0(data_lane_lpx<8> = 0x0A),),
        10: (D0(clk_lane_lpx<8> = 0x0A),),
        11: (D0(clk_pre_time<8> = 0x03),),
        12: (D0(clk_post_time<8> = 0x0E),),
        13: (D0(data_lane_wakeup<8> = 0x00),),
        14: (D0(clk_lane_wakeup<8> = 0x00),),
    );
}

transmission_mapping! {
    /// ## MIPI Configuration Setting 3
    /// Additional MIPI protocol parameters
    /// > Reference: `MIPISET3` p. 301
    pub struct MipiSetting3<1>(
        0: (
            D7(ecc_enable<1> as Switch = On),
            D6(crc_enable<1> as Switch = On),
            D0(dsi_mode<4> = 0x0),
        ),
    );
}

transmission_mapping! {
    /// ## MIPI Configuration Setting 4
    /// Final MIPI configuration parameters
    /// > Reference: `MIPISET4` p. 302
    pub struct MipiSetting4<1>(
        0: (
            D4(bta_enable<1> as Switch = Off),
            D0(mipi_reserved<4> = 0x0),
        ),
    );
}

transmission_mapping! {
    pub struct OperatingVoltage<1>(
        0: (D0(amplitude<8> = 0x4D),),
    );
}
impl OperatingVoltage {
    #[must_use]
    pub fn values(&self) -> (u8, f32) {
        let vrha = self.amplitude().as_u8();
        let vop = f32::from(vrha).mul_add(0.0125, 3.5375);

        (vrha, vop)
    }
}
impl Display for OperatingVoltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (vrha, vop) = self.values();

        write!(f, "VOP Amplitude: {vop:.4} V (VRHA: 0x{vrha:02X})")
    }
}

transmission_mapping! {
    pub struct CommonVoltage<1>(
        0: (D0(amplitude<8> = 0x40),),
    );
}
impl CommonVoltage {
    #[must_use]
    pub fn values(&self) -> (u8, f32) {
        let vcom = self.amplitude().as_u8();
        let vop = f32::from(vcom).mul_add(0.0125, 0.1);

        (vcom, vop)
    }
}
impl Display for CommonVoltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (vcom, vop) = self.values();

        write!(f, "VCOM Amplitude: {vop:.4} V (VCOM: 0x{vcom:02X})")
    }
}

bit_value_enum! {
    /// VGH Voltage Setting
    pub enum GateHighAmplitude<4> {
        #[default]
        const Pos11_5 = 0x00,
        const Pos12_0 = 0x01,
        const Pos12_5 = 0x02,
        const Pos13_0 = 0x03,
        const Pos13_5 = 0x04,
        const Pos14_0 = 0x05,
        const Pos14_5 = 0x06,
        const Pos15_0 = 0x07,
        const Pos15_5 = 0x08,
        const Pos16_0 = 0x09,
        const Pos16_5 = 0x0A,
        const Pos17_0 = 0x0B,
    }
}
impl GateHighAmplitude {
    #[must_use]
    pub const fn from_voltage(volts: f32) -> Self {
        match volts {
            11.5 => Pos11_5,
            12.0 => Pos12_0,
            12.5 => Pos12_5,
            13.0 => Pos13_0,
            13.5 => Pos13_5,
            14.0 => Pos14_0,
            14.5 => Pos14_5,
            15.0 => Pos15_0,
            15.5 => Pos15_5,
            16.0 => Pos16_0,
            16.5 => Pos16_5,
            17.0 => Pos17_0,
            _ => panic!("Invalid VGH voltage specified"),
        }
    }

    #[must_use]
    pub const fn as_volts(&self) -> f32 {
        match self {
            Pos11_5 => 11.5,
            Pos12_0 => 12.0,
            Pos12_5 => 12.5,
            Pos13_0 => 13.0,
            Pos13_5 => 13.5,
            Pos14_0 => 14.0,
            Pos14_5 => 14.5,
            Pos15_0 => 15.0,
            Pos15_5 => 15.5,
            Pos16_0 => 16.0,
            Pos16_5 => 16.5,
            Pos17_0 => 17.0,
        }
    }
}

use GateHighAmplitude::{
    Pos11_5, Pos12_0, Pos12_5, Pos13_0, Pos13_5, Pos14_0, Pos14_5, Pos15_0, Pos15_5, Pos16_0,
    Pos16_5, Pos17_0,
};

transmission_mapping! {
    pub struct GateHighVoltage<1>(
        0: (D0(amplitude<4> as GateHighAmplitude = Pos12_5),),
    );
}
impl GateHighVoltage {
    pub fn set_amplitude_volts(self, volts: f32) -> Self {
        self.set_amplitude(GateHighAmplitude::from_voltage(volts))
    }

    #[must_use]
    pub fn values(&self) -> (u8, f32) {
        let vghss = self.amplitude();

        (vghss.as_u8(), vghss.as_volts())
    }
}
impl Display for GateHighVoltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (vghss, voltage) = self.values();

        write!(f, "VGH Voltage: {voltage:.1} V (VGHSS: 0x{vghss:02X})")
    }
}

bit_value_enum! {
    pub enum GateLowAmplitude<4> {
        #[default]
        const Neg7_06= 0x00,
        const Neg7_47= 0x01,
        const Neg7_91= 0x02,
        const Neg8_14= 0x03,
        const Neg8_65= 0x04,
        const Neg8_92= 0x05,
        const Neg9_21 = 0x06,
        const Neg9_51 = 0x07,
        const Neg9_83 = 0x08,
        const Neg10_17 = 0x09,
        const Neg10_53 = 0x0A,
        const Neg10_91 = 0x0B,
        const Neg11_31 = 0x0C,
        const Neg11_74 = 0x0D,
        const Neg12_20 = 0x0E,
        const Neg12_69 = 0x0F,
    }
}
use GateLowAmplitude::{
    Neg7_06, Neg7_47, Neg7_91, Neg8_14, Neg8_65, Neg8_92, Neg9_21, Neg9_51, Neg9_83, Neg10_17,
    Neg10_53, Neg10_91, Neg11_31, Neg11_74, Neg12_20, Neg12_69,
};
impl GateLowAmplitude {
    #[must_use]
    pub const fn from_voltage(volts: f32) -> Self {
        match volts {
            -7.06 => Neg7_06,
            -7.47 => Neg7_47,
            -7.91 => Neg7_91,
            -8.14 => Neg8_14,
            -8.65 => Neg8_65,
            -8.92 => Neg8_92,
            -9.21 => Neg9_21,
            -9.51 => Neg9_51,
            -9.83 => Neg9_83,
            -10.17 => Neg10_17,
            -10.53 => Neg10_53,
            -10.91 => Neg10_91,
            -11.31 => Neg11_31,
            -11.74 => Neg11_74,
            -12.20 => Neg12_20,
            -12.69 => Neg12_69,
            _ => panic!("Invalid VGL voltage specified"),
        }
    }

    #[must_use]
    pub const fn as_volts(&self) -> f32 {
        match self {
            Neg7_06 => -7.06,
            Neg7_47 => -7.47,
            Neg7_91 => -7.91,
            Neg8_14 => -8.14,
            Neg8_65 => -8.65,
            Neg8_92 => -8.92,
            Neg9_21 => -9.21,
            Neg9_51 => -9.51,
            Neg9_83 => -9.83,
            Neg10_17 => -10.17,
            Neg10_53 => -10.53,
            Neg10_91 => -10.91,
            Neg11_31 => -11.31,
            Neg11_74 => -11.74,
            Neg12_20 => -12.20,
            Neg12_69 => -12.69,
        }
    }
}

transmission_mapping! {
    pub struct GateLowVoltage<1>(
        0: (D0(amplitude<4> as GateLowAmplitude = Neg9_51),) = 0b0100_00000,
    );
}
impl GateLowVoltage {
    #[must_use]
    pub fn values(&self) -> (u8, f32) {
        let vgls = self.amplitude();

        (vgls.as_u8(), vgls.as_volts())
    }

    pub fn set_amplitude_volts(self, volts: f32) -> Self {
        self.set_amplitude(GateLowAmplitude::from_voltage(volts))
    }
}
impl Display for GateLowVoltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (vgls, voltage) = self.values();

        write!(f, "VGL Voltage: {voltage:.2} V (VGLS: 0x{vgls:02X})")
    }
}

bit_value_enum! {
    /// VGH Voltage Multiplier
    pub enum VghMultiplier<2> {
        #[default]
        const X2_5 = 0b00,
        const X3_0 = 0b01,
        const X3_5 = 0b10,
        const X4_0 = 0b11,
    }
}

bit_value_enum! {
    /// VGL Voltage Multiplier
    pub enum VglMultiplier<2> {
        #[default]
        const Minus2_5 = 0b00,
        const Minus3_0 = 0b01,
        const Minus3_5 = 0b10,
        const Minus4_0 = 0b11,
    }
}

transmission_mapping! {
    /// ## Power Control 2
    /// Gate driver power configuration
    /// > Reference: `PWCTRL2` p. 293
    pub struct PowerControl2<1>(
        0: (
            D4(vgh_multiplier<2> as VghMultiplier),
            D0(vgl_multiplier<2> as VglMultiplier),
        ),
    );
}

bit_value_enum! {
    /// Clock Divider Ratio
    pub enum ClockDivider<4> {
        #[default]
        const Div1 = 0b0000,
        const Div2 = 0b0001,
        const Div4 = 0b0010,
        const Div8 = 0b0011,
        const Div16 = 0b0100,
    }
}

transmission_mapping! {
    /// ## Panel Clock Setting 1
    /// Primary clock configuration
    /// > Reference: `PCLKS1` p. 294
    pub struct PanelClockSetting1<1>(
        0: (
            D4(rtni_clock_div<4> as ClockDivider),
            D0(frame_rate_control<4> = 0x0),
        ),
    );
}

transmission_mapping! {
    /// ## Panel Clock Setting 2
    /// Secondary clock timing
    /// > Reference: `PCLKS2` p. 295
    pub struct PanelClockSetting2<1>(
        0: (D0(pclk_divider<8> = 0x00),),
    );
}

transmission_mapping! {
    /// ## Panel Clock Setting 3
    /// Tertiary clock parameters
    /// > Reference: `PCLKS3` p. 296
    pub struct PanelClockSetting3<1>(
        0: (
            D4(osc_frequency<4> = 0x0),
            D0(clock_phase<4> = 0x0),
        ),
    );
}

bit_value_enum! {
    /// Source Pre-charge Period
    pub enum SourcePrecharge<4> {
        #[default]
        const Period1 = 0b0000,
        const Period2 = 0b0001,
        const Period3 = 0b0010,
        const Period4 = 0b0011,
        const Period5 = 0b0100,
        const Period6 = 0b0101,
        const Period7 = 0b0110,
        const Period8 = 0b0111,
    }
}

transmission_mapping! {
    /// ## Source Pre-Drive Timing 1
    /// Source driver timing configuration
    /// > Reference: `SPD1` p. 297
    pub struct SourcePreDriveTiming1<1>(
        0: (
            D4(precharge_period<4> as SourcePrecharge),
            D0(drive_timing<4> = 0x0),
        ),
    );
}

transmission_mapping! {
    /// ## Source Pre-Drive Timing 2
    /// Additional source timing parameters
    /// > Reference: `SPD2` p. 298
    pub struct SourcePreDriveTiming2<1>(
        0: (D0(speed_optimization<8> = 0x00),),
    );
}

/// Test Command Parameter
/// Used for factory testing and diagnostics
/// > Reference: `TESTCMD` p. 290
pub type TestCommand = u8;
