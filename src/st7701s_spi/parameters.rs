pub mod register {
    use crate::st7701s_spi::state::Switch;
    use std::fmt::{self, Display, Formatter};

    pub type ExtendedCommands = Switch;

    #[derive(Debug)]
    pub struct Address(pub u8);
    impl Display for Address {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "0x{:02X}", self.0)
        }
    }
    impl From<Address> for u8 {
        fn from(value: Address) -> Self {
            value.0
        }
    }

    /**
        ## Extended Address Banks

        The ST7701S exposes some extended command sets based on the setting of an
        internal register, referred to in the datasheet as **Command2 BKx**.

        > Section 12.3.1 `CND2BKxSEL`, page 260
    */
    #[derive(Debug, PartialEq)]
    pub enum Bank {
        BK0,
        BK1,
        BK3,
    }

    #[derive(Debug)]
    pub struct Extension(pub Option<Bank>);

    impl Display for Extension {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self.0 {
                Some(Bank::BK0) => write!(f, "BK0/"),
                Some(Bank::BK1) => write!(f, "BK1/"),
                Some(Bank::BK3) => write!(f, "BK3/"),
                None => write!(f, ""),
            }
        }
    }
}

pub mod gamma {
    #[derive(Debug, PartialEq)]
    pub enum Curve {
        GC1,
        GC2,
        GC3,
        GC4,
    }
}

pub mod tearing_effect {
    #[derive(Debug, PartialEq)]
    pub enum Blank {
        Vertical,
        VerticalHorizontal,
    }
}

pub mod data_access {
    use crate::st7701s_spi::state::Direction;

    pub type ScanDirection = Direction;

    #[derive(Debug, PartialEq)]
    pub enum ColorOrder {
        RGB,
        BGR,
    }
}

pub mod pixel_format {
    #[derive(Debug, PartialEq)]
    pub enum BitsPerPixel {
        RGB565,
        RGB666,
        RGB888,
    }
}

pub mod brightness {
    use crate::st7701s_spi::state::Switch;

    pub type Control = Switch;
    pub type Dimming = Switch;
    pub type Backlight = Switch;

    pub type Minimum = u8;
}

pub mod color {
    use crate::st7701s_spi::state::Switch;

    pub type Enhancement = Switch;

    pub enum EnhanceLevel {
        Low,
        Medium,
        High,
    }

    pub enum AdaptiveBrightness {
        Off,
        UserInterface,
        StillPicture,
        MovingImage,
    }
}

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
