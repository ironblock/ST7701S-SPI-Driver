use crate::{
    bit_value_enum,
    st7701s_spi::{parameters::general::Switch, transmissions::*},
    transmission_mapping,
};
use MipiLaneCount::*;
use Switch::*;

// ============================================================================
// MIPI Settings (MIPISET1-4) - p. 299-302
// ============================================================================

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

// ============================================================================
// Voltage Control Settings - p. 287-291
// ============================================================================

/// VRHS - Vreg1 Output Positive Voltage (0xB0)
/// Range: Typically 4.0V to 6.5V in steps
/// > Reference: `VRHS` p. 287
pub type VregPositiveVoltage = u8;

/// VCOMS - VCOM Voltage Setting (0xB1)
/// Range: Typically -2.5V to 0V in steps
/// > Reference: `VCOMS` p. 288
pub type VcomVoltage = u8;

/// VGHSS - VGH Voltage Setting (0xB2)
/// Range: Gate high voltage, typically 10V to 17V
/// > Reference: `VGHSS` p. 289
pub type VghVoltage = u8;

/// VGLS - VGL Voltage Setting (0xB5)
/// Range: Gate low voltage, typically -10V to -7V
/// > Reference: `VGLS` p. 291
pub type VglVoltage = u8;

// ============================================================================
// Power Control Settings - p. 292-293
// ============================================================================

bit_value_enum! {
    /// AVDD Voltage Level
    pub enum AvddLevel<3> {
        #[default]
        const V6_4 = 0b000,
        const V6_6 = 0b001,
        const V6_8 = 0b010,
        const V7_0 = 0b011,
        const V7_2 = 0b100,
        const V7_4 = 0b101,
        const V7_6 = 0b110,
        const V7_8 = 0b111,
    }
}

bit_value_enum! {
    /// AVCL Voltage Level
    pub enum AvclLevel<2> {
        #[default]
        const Minus4_5 = 0b00,
        const Minus4_7 = 0b01,
        const Minus4_9 = 0b10,
        const Minus5_1 = 0b11,
    }
}

transmission_mapping! {
    /// ## Power Control 1
    /// Primary power supply configuration
    /// > Reference: `PWCTRL1` p. 292
    pub struct PowerControl1<1>(
        0: (
            D4(avdd_level<3> as AvddLevel),
            D0(avcl_level<2> as AvclLevel),
        ),
    );
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

// ============================================================================
// Panel Clock Settings - p. 294-296
// ============================================================================

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

// ============================================================================
// Source Drive Timing - p. 297-298
// ============================================================================

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

// ============================================================================
// Test Command - p. 290
// ============================================================================

/// Test Command Parameter
/// Used for factory testing and diagnostics
/// > Reference: `TESTCMD` p. 290
pub type TestCommand = u8;
