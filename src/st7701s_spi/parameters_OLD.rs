pub enum DataEnable {
    DE = 0x00,
    HV = 0x80,
}

pub enum VsyncActive {
    Low = 0x00,
    High = 0x08,
}
pub enum HsyncActive {
    Low = 0x00,
    High = 0x04,
}
pub enum DataPolarity {
    Rising = 0x00,
    Falling = 0x02,
}
pub enum EnablePolarity {
    Low = 0x00,
    High = 0x01,
}

pub enum PWMPolarity {
    Low = 0x00,
    High = 0x20,
}

pub enum LEDPolarity {
    Low = 0x00,
    High = 0x10,
}

pub enum PixelPinout {
    Normal = 0x00,
    Condensed = 0x08,
}
pub enum EndPixelFormat {
    SelfMSB = 0x00,
    GreenMSB = 0x01,
    SelfLSB = 0x02,
    Zero = 0x04,
    One = 0x05,
}

pub enum Inversion {
    OneDot = 0x00,
    TwoDot = 0x01,
    Column = 0x07,
}

pub enum GammaOPBias {
    Off = 0x00,
    Min = 0x40,
    Middle = 0x80,
    Max = 0xC0,
}

pub enum SourceOPInput {
    Off = 0x00,
    Min = 0x04,
    Middle = 0x08,
    Max = 0x0C,
}

pub enum SourceOPOutput {
    Off = 0x00,
    Min = 0x01,
    Middle = 0x02,
    Max = 0x03,
}

pub enum VoltageAVDD {
    Pos6_2 = 0x00,
    Pos6_4 = 0x10,
    Pos6_6 = 0x20,
    Pos6_8 = 0x30,
}

pub enum VoltageAVCL {
    Neg4_4 = 0x00,
    Neg4_6 = 0x01,
    Neg4_8 = 0x02,
    Neg5_0 = 0x03,
}

pub enum SunlightReadable {
    /// DEFAULT: Sunlight readable mode off
    Off = 0x00,
    /// Enable sunlight readable mode
    On = 0x10,
}
