use std::{any::Any, ops::{Deref, Shl}};

use frunk::labelled::chars::M;


pub trait Bit<const D: u8> {}
impl Bit<0> for () {}
impl Bit<1> for () {}
impl Bit<2> for () {}
impl Bit<3> for () {}
impl Bit<4> for () {}
impl Bit<5> for () {}
impl Bit<6> for () {}
impl Bit<7> for () {}

pub struct Parameter(u8);

pub trait BitRange<const MSB: u8, const LSB: u8> where (): Bit<MSB>, (): Bit<LSB> {
    fn new(value: u8) -> Self;
}

impl<const MSB: u8, const LSB: u8> BitRange<MSB, LSB> for Parameter where (): Bit<MSB>, (): Bit<LSB> {
    fn new(input: u8) -> Self {
        const {
            assert!(MSB > LSB, "Expected MSB to be greater than LSB");
        }

        let mask = const { 2_u8.pow(MSB as u32 - LSB as u32 + 1) - 1 };
        let value = input & mask;
        assert!(value == input, "Input value is out of range");

        Self(value << LSB)
    }
}

// impl<const MSB: u8, const LSB: u8> Deref for BitRange<MSB, LSB> where (): Bit<MSB>, (): Bit<LSB> {
//     type Target = u8;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

pub trait Packet {
    fn merge(self) -> u8;
}
impl <A> Packet for (A,) where A: Into<u8> {
    fn merge(self) -> u8 {
        self.0.into()
    }
}

// #[derive(PartialEq, Debug)]
pub trait BitRange<const BITS: u32 = 8, const LSB: u8 = 0> {
    fn mask(self) -> u8;
}
impl <const BITS: u32, const LSB: u8> BitRange<BITS, LSB> for u8 {
    fn mask(self) -> u8 {
        const {
            assert!(BITS > 0, "Expected BITS to be greater than 0");
            assert!(BITS <= 8, "Expected BITS to be less than or equal to 8");
            assert!(LSB <= 8, "Expected LSB to be less than or equal to 8");
            assert!(
                BITS > (8 - LSB as u32),
                "Expected BITS to be greater than (8 - LSB)"
            );
        }

        let mask = 2_u8.pow(BITS) - 1;

        (mask & self) << LSB
    }
}

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
        BK0 = 0,
        BK1 = 1,
        BK3 = 3,
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

    pub struct Location {
        pub extension: Extension,
        pub address: Address,
    }
    impl Location {
        pub const fn new(extension: Option<Bank>, address: u8) -> Self {
            Self {
                extension: Extension(extension),
                address: Address(address),
            }
        }

        pub const fn in_core(address: u8) -> Self {
            Self::new(None, address)
        }

        pub const fn in_bank(bank: Bank, address: u8) -> Self {
            Self::new(Some(bank), address)
        }
    }
}

pub mod gamma {
    use crate::st7701s_spi::parameters::{BitRange, Packet, BitRange};

    #[derive(Debug, PartialEq)]
    pub enum Curve {
        GC1,
        GC2,
        GC3,
        GC4,
    }


    pub struct VoltageControl {
        pub aj0: BitRange<2, 6>,
        pub vc0: BitRange<4, 0>,
        pub aj1: BitRange<2, 6>,
        pub vc4: BitRange<6, 0>,
        pub aj2: BitRange<2, 6>,
        pub vc8: BitRange<6, 0>,
        pub vc16: BitRange<5, 0>,
        pub aj3: BitRange<2, 6>,
        pub vc24: BitRange<5, 0>,
        pub vc52: BitRange<4, 0>,
        pub vc80: BitRange<6, 0>,
        pub vc108: BitRange<4, 0>,
        pub vc147: BitRange<4, 0>,
        pub vc175: BitRange<6, 0>,
        pub vc203: BitRange<4, 0>,
        pub aj4: BitRange<2, 6>,
        pub vc231: BitRange<5, 0>,
        pub vc239: BitRange<5, 0>,
        pub aj5: BitRange<2, 6>,
        pub vc247: BitRange<6, 0>,
        pub aj6: BitRange<2, 6>,
        pub vc251: BitRange<6, 0>,
        pub aj7: BitRange<2, 6>,
        pub vc255: BitRange<5, 0>,
    }

    impl VoltageControl {
        const fn encode(self) -> [u8; 16] {
            let transmission: [
            (*self.aj0 | *self.vc0),
            (self.aj1, self.vc4).merge(),
            (self.aj2, self.vc8).merge(),
            (self.vc16).merge(),
            (self.aj3, self.vc24).merge(),
            (self.vc52).merge(),
            (self.vc80).merge(),
            (self.vc108).merge(),
            (self.vc147).merge(),
            (self.vc175).merge(),
            (self.vc203).merge(),
            (self.aj4, self.vc231).merge(),
            (self.vc239).merge(),
            (self.aj5, self.vc247).merge(),
            (self.aj6, self.vc251).merge(),
            (self.aj7, self.vc255).merge(),
            ]
        }
    }

    pub type AJ0P = BitRange<2, 6>;
    pub type AJ0N = AJ0P;
    pub type VC0P = BitRange<4, 0>;
    pub type VC0N = VC0P;

    pub type AJ1P = BitRange<2, 6>;
    pub type AJ1N = AJ1P;
    pub type VC4P = BitRange<6, 0>;
    pub type VC4N = VC4P;

    pub type AJ2P = BitRange<2, 6>;
    pub type AJ2N = AJ2P;
    pub type VC8P = BitRange<6, 0>;
    pub type VC8N = VC8P;

    pub type VC16P = BitRange<5, 0>;
    pub type VC16N = VC16P;

    pub type AJ3P = BitRange<2, 6>;
    pub type AJ3N = AJ3P;
    pub type VC24P = BitRange<5, 0>;
    pub type VC24N = VC24P;

    pub type VC52P = BitRange<4, 0>;
    pub type VC52N = VC52P;

    pub type VC80P = BitRange<6, 0>;
    pub type VC80N = VC80P;

    pub type VC108P = BitRange<4, 0>;
    pub type VC108N = VC108P;

    pub type VC147P = BitRange<4, 0>;
    pub type VC147N = VC147P;

    pub type VC175P = BitRange<6, 0>;
    pub type VC175N = VC175P;

    pub type VC203P = BitRange<4, 0>;
    pub type VC203N = VC203P;

    pub type AJ4P = BitRange<2, 6>;
    pub type AJ4N = AJ4P;
    pub type VC231P = BitRange<5, 0>;
    pub type VC231N = VC231P;

    pub type VC239P = BitRange<5, 0>;
    pub type VC239N = VC239P;

    pub type AJ5P = BitRange<2, 6>;
    pub type AJ5N = AJ5P;
    pub type VC247P = BitRange<6, 0>;
    pub type VC247N = VC247P;

    pub type AJ6P = BitRange<2, 6>;
    pub type AJ6N = AJ6P;
    pub type VC251P = BitRange<6, 0>;
    pub type VC251N = VC251P;

    pub type AJ7P = BitRange<2, 6>;
    pub type AJ7N = AJ7P;
    pub type VC255P = BitRange<5, 0>;
    pub type VC255N = VC255P;
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
