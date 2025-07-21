pub enum GammaCurve {
    One,
    Two,
    Three,
    Four,
}

pub enum TearingEffect {
    VBlank,
    VHBlank,
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

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ScanDirection {
    Normal = 0x00,
    Reverse = 0x10,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ColorOrder {
    /// RGB mode
    Rgb = 0x00,
    /// BGR mode
    Bgr = 0x08,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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
