pub mod register {
    pub type Address = u8;
    pub type Extension = Option<Bank>;

    /**
        ## Extended Address Banks

        The ST7701S exposes some extended command sets based on the setting of an
        internal register, referred to in the datasheet as **Command2 BKx**.

        > Section 12.3.1 `CND2BKxSEL`, page 260
    */
#[derive(Debug)]
    pub enum Bank {
        BK0,
        BK1,
        BK3,
    }
}

pub mod gamma {
#[derive(Debug)]
    pub enum Curve {
        GC1,
        GC2,
        GC3,
        GC4,
    }
}

pub mod tearing_effect {
#[derive(Debug)]
    pub enum Blank {
        Vertical,
        VerticalHorizontal,
    }
}

pub mod data_access {
#[derive(Debug)]
    pub enum ScanDirection {
        Normal,
        Reverse,
    }

#[derive(Debug)]
    pub enum ColorOrder {
        RGB,
        BGR,
    }
}

pub mod pixel_format {
#[derive(Debug)]
    pub enum BitsPerPixel {
        /// 16 bits per pixel (RGB565)
        RGB565,
        /// 18 bits per pixel (RGB666)
        RGB666,
        /// 24 bits per pixel (RGB888)
        RGB888,
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum DataEnable {
    DE = 0x00,
    HV = 0x80,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum VsyncActive {
    Low = 0x00,
    High = 0x08,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum HsyncActive {
    Low = 0x00,
    High = 0x04,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum DataPolarity {
    Rising = 0x00,
    Falling = 0x02,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum EnablePolarity {
    Low = 0x00,
    High = 0x01,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PWMPolarity {
    Low = 0x00,
    High = 0x20,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum LEDPolarity {
    Low = 0x00,
    High = 0x10,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PixelPinout {
    Normal = 0x00,
    Condensed = 0x08,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum EndPixelFormat {
    SelfMSB = 0x00,
    GreenMSB = 0x01,
    SelfLSB = 0x02,
    Zero = 0x04,
    One = 0x05,
}

pub enum BitsPerPixel {
    /// 16 bits per pixel (RGB565)
    Rgb565 = 0x50,
    /// 18 bits per pixel (RGB666)
    Rgb666 = 0x60,
    /// 24 bits per pixel (RGB888)
    Rgb888 = 0x70,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum BrightnessControl {
    /// Ignore display brightness value and soft-set it to 0x00
    Off = 0x00,
    /// Use display brightness value normally
    On = 0x20,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum DisplayDimming {
    /// Ignore display brightness value and soft-set it to 0x00
    Off = 0x00,
    /// Use display brightness value normally
    On = 0x08,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Backlight {
    /// Disable backlight circuit. Control lines must be low.
    Off = 0x00,
    /// Enable backlight circuit. Normal behavior.
    On = 0x04,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Enhancement {
    /// Disable color enhancement
    Off = 0x00,
    /// Enable color enhancement
    On = 0x80,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum EnhancementMode {
    Low = 0x00,
    Medium = 0x10,
    High = 0x30,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum AdaptiveBrightness {
    /// Off
    Off = 0x00,
    /// User Interface Mode
    UserInterface = 0x01,
    /// Still Picture Mode
    StillPicture = 0x02,
    /// Moving Image Mode
    MovingImage = 0x03,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Inversion {
    OneDot = 0x00,
    TwoDot = 0x01,
    Column = 0x07,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum GammaOPBias {
    Off = 0x00,
    Min = 0x40,
    Middle = 0x80,
    Max = 0xC0,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum SourceOPInput {
    Off = 0x00,
    Min = 0x04,
    Middle = 0x08,
    Max = 0x0C,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum SourceOPOutput {
    Off = 0x00,
    Min = 0x01,
    Middle = 0x02,
    Max = 0x03,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum VoltageAVDD {
    Pos6_2 = 0x00,
    Pos6_4 = 0x10,
    Pos6_6 = 0x20,
    Pos6_8 = 0x30,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum VoltageAVCL {
    Neg4_4 = 0x00,
    Neg4_6 = 0x01,
    Neg4_8 = 0x02,
    Neg5_0 = 0x03,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum SunlightReadable {
    /// DEFAULT: Sunlight readable mode off
    Off = 0x00,
    /// Enable sunlight readable mode
    On = 0x10,
}
