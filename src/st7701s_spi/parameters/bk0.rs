/// Line Setting Control Parameters
/// Used for BK0 LNESET (0xC0) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSettings {
    /// Line Delta Enable (LDE_EN)
    /// Controls line delta functionality
    pub line_delta_enable: u8,
    /// Line Delta Value
    /// Configures line delta parameter
    pub line_delta: u8,
}

impl Default for LineSettings {
    fn default() -> Self {
        Self {
            line_delta_enable: 0,
            line_delta: 0,
        }
    }
}

/// Porch Control Parameters
/// Used for BK0 PORCTRL (0xC1) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PorchControl {
    /// Vertical Back Porch (VBP)
    /// Number of HSYNC periods before active video
    pub vertical_back_porch: u8,
    /// Vertical Front Porch (VFP)
    /// Number of HSYNC periods after active video
    pub vertical_front_porch: u8,
}

impl Default for PorchControl {
    fn default() -> Self {
        Self {
            vertical_back_porch: 0,
            vertical_front_porch: 0,
        }
    }
}

/// Inversion Selection Parameters
/// Used for BK0 INVSET (0xC2) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InversionSettings {
    /// Inversion mode configuration
    pub inversion_mode: u8,
    /// RTNI - Real Time Noise Inversion
    pub rtni: u8,
}

impl Default for InversionSettings {
    fn default() -> Self {
        Self {
            inversion_mode: 0,
            rtni: 0,
        }
    }
}

/// RGB Control Parameters
/// Used for BK0 RGBCTRL (0xC3) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Default for RgbControl {
    fn default() -> Self {
        Self {
            param1: 0,
            param2: 0,
            param3: 0,
            param4: 0,
        }
    }
}

/// Partial Control Parameters
/// Used for BK0 PARCTRL (0xC5) command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartialControl {
    /// Partial mode start configuration
    pub start_config: u8,
    /// Partial mode end configuration
    pub end_config: u8,
}

impl Default for PartialControl {
    fn default() -> Self {
        Self {
            start_config: 0,
            end_config: 0,
        }
    }
}

/// Gamma LUT Red Table - 64 bytes for DGMLUTR (0xB9)
pub type GammaLutRed = [u8; 64];

/// Gamma LUT Blue Table - 64 bytes for DGMLUTB (0xBA)
pub type GammaLutBlue = [u8; 64];
