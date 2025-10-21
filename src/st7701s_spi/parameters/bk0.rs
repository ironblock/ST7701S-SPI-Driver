use crate::{st7701s_spi::{parameters::general::Switch, transmissions::*}, transmission_mapping};

use Switch::*;

transmission_mapping!(
    pub struct LineSettings<2>(
        0: (D7(line_delta_enable<1> as Switch = On), D0(lines<7> = 0x2B),),
        1: (D1(line_delta<7>),),
    );
);

/// Porch Control Parameters
/// Used for BK0 PORCTRL (0xC1) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub struct PorchControl {
    /// Vertical Back Porch (VBP)
    /// Number of HSYNC periods before active video
    pub vertical_back_porch: u8,
    /// Vertical Front Porch (VFP)
    /// Number of HSYNC periods after active video
    pub vertical_front_porch: u8,
}


/// Inversion Selection Parameters
/// Used for BK0 INVSET (0xC2) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub struct InversionSettings {
    /// Inversion mode configuration
    pub inversion_mode: u8,
    /// RTNI - Real Time Noise Inversion
    pub rtni: u8,
}


/// RGB Control Parameters
/// Used for BK0 RGBCTRL (0xC3) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub struct RgbControl {
    /// RGB interface control parameter 1
    pub param1: u8,
    /// RGB interface control parameter 2
    pub param2: u8,
    /// RGB interface control parameter 3
    pub param3: u8,
    /// RGB interface control parameter 4
    pub param4: u8,
}


/// Partial Control Parameters
/// Used for BK0 PARCTRL (0xC5) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub struct PartialControl {
    /// Partial mode start configuration
    pub start_config: u8,
    /// Partial mode end configuration
    pub end_config: u8,
}


/// Gamma LUT Red Table - 64 bytes for DGMLUTR (0xB9)
pub type GammaLutRed = [u8; 64];

/// Gamma LUT Blue Table - 64 bytes for DGMLUTB (0xBA)
pub type GammaLutBlue = [u8; 64];
