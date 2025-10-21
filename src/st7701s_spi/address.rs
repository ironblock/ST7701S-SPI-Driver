use crate::st7701s_spi::{parameters::register::Bank, transmissions::Transmission};

pub trait Extension {
    const EXTENSION: Option<Bank>;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AnyExtension;
impl Extension for AnyExtension {
    const EXTENSION: Option<Bank> = None;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk0;
impl Extension for ExtensionBk0 {
    const EXTENSION: Option<Bank> = Some(Bank::BK0);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk1;
impl Extension for ExtensionBk1 {
    const EXTENSION: Option<Bank> = Some(Bank::BK1);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExtensionBk3;
impl Extension for ExtensionBk3 {
    const EXTENSION: Option<Bank> = Some(Bank::BK3);
}

pub trait Instruction: Extension {
    const ADDRESS: u8;
}

pub trait Command: Instruction {}
impl <T> Command for T where T: Instruction {}

pub trait Write: Instruction + Transmission {}
impl <T> Write for T where T: Instruction + Transmission {}

pub trait Read: Instruction + Transmission {
}
impl <T> Read for T where T: Instruction + Transmission {}

macro_rules! instructions {
    (@as_u8 $EVIS:vis fn(&self) => ($ADDR:literal)) => {
        pub const fn as_u8(&self) -> u8 {
            $ADDR
        }
    };
    (@as_u8 $EVIS:vis fn(&self) => ()) => {
        pub const fn as_u8(&self) -> u8 {
            *self as u8
        }
    };
    ($EVIS:vis enum $GROUP:ident<$LOC:ident $(,$SHARED:literal)?> {
       $($DVIS:vis const $NAME:ident = ($($ADDR:literal,)? $TYPE:ident$(<$PACKET:literal>)*),)+
    }) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[repr(u8)]
        $EVIS enum $GROUP where
                      Self: $crate::st7701s_spi::address::Extension {
            $($NAME$( = $ADDR)?),+
        }
        impl $crate::st7701s_spi::address::Extension for $GROUP {
            const EXTENSION: Option<$crate::st7701s_spi::parameters::register::Bank> = $LOC::EXTENSION;
        }
        impl $GROUP {
            instructions!(@as_u8 $EVIS fn(&self) => ($($SHARED)?));
        }

        $(
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            $DVIS struct $NAME
                where Self: $TYPE,
                      Self: $crate::st7701s_spi::address::Extension
                $(
                    , Self: $crate::st7701s_spi::transmissions::Transmission<Data = [u8; $PACKET]>
                )?;

            impl $crate::st7701s_spi::address::Extension for $NAME {
                const EXTENSION: Option<$crate::st7701s_spi::parameters::register::Bank> = $LOC::EXTENSION;
            }
            impl $crate::st7701s_spi::address::Instruction for $NAME {
                const ADDRESS: u8 = $GROUP::as_u8(&$GROUP::$NAME);
            }
            $(
                impl $crate::st7701s_spi::transmissions::Transmission for $NAME {
                    type Data = [u8; $PACKET];
                    type MapToData<U> = [U; $PACKET];
                }
            )?
        )+
    };
}

pub mod special {
    #![allow(clippy::upper_case_acronyms)]
    use crate::st7701s_spi::address::{AnyExtension, Write};

    instructions! {
        pub enum Special<AnyExtension, 0xFF> {
            pub const CND2BKXSEL = (Write<5>),
            pub const DSTB       = (Write<5>),
            pub const DSTBT      = (Write<5>),
        }
    }
}

pub mod core {
    #![allow(clippy::upper_case_acronyms)]
    use crate::st7701s_spi::address::{AnyExtension, Command, Read, Write};

    instructions! {
        enum Core<AnyExtension> {
            pub const NOP            = (0x00, Command),
            pub const SWRESET        = (0x01, Write<1>),
            pub const RDDID          = (0x04, Read<3>),
            pub const RDNUMED        = (0x05, Read<1>),
            pub const RDRED          = (0x06, Read<1>),
            pub const RDGREEN        = (0x07, Read<1>),
            pub const RDBLUE         = (0x08, Read<1>),
            pub const RDDPM          = (0x0A, Read<1>),
            pub const RDDMADCTL      = (0x0B, Read<1>),
            pub const RDDCOLMOD      = (0x0C, Read<1>),
            pub const RDDIM          = (0x0D, Read<1>),
            pub const RDDSM          = (0x0E, Read<1>),
            pub const RDDSDR         = (0x0F, Read<1>),
            pub const SLPIN          = (0x10, Command),
            pub const SLPOUT         = (0x11, Command),
            pub const PTLON          = (0x12, Command),
            pub const NORON          = (0x13, Command),
            pub const INVOFF         = (0x20, Command),
            pub const INVON          = (0x21, Command),
            pub const ALLPOFF        = (0x22, Command),
            pub const ALLPON         = (0x23, Command),
            pub const GAMSET         = (0x26, Write<1>),
            pub const DISPOFF        = (0x28, Command),
            pub const DISPON         = (0x29, Command),
            pub const TEOFF          = (0x34, Command),
            pub const TEON           = (0x35, Write<1>),
            pub const MADCTL         = (0x36, Write<1>),
            pub const IDMOFF         = (0x38, Command),
            pub const IDMON          = (0x39, Command),
            pub const COLMOD         = (0x3A, Write<1>),
            pub const GSL            = (0x45, Read<2>),
            pub const WRDISBV        = (0x51, Write<1>),
            pub const RDDISBV        = (0x52, Read<1>),
            pub const WRCTRLD        = (0x53, Write<1>),
            pub const RDCTRLD        = (0x54, Read<1>),
            pub const WRCACE         = (0x55, Write<1>),
            pub const RDCABC         = (0x56, Read<1>),
            pub const WRCABCMB       = (0x5E, Write<1>),
            pub const RDCABCMB       = (0x5F, Read<1>),
            pub const RDABCSDR       = (0x68, Read<1>),
            pub const RDBWLB         = (0x70, Read<1>),
            pub const RDBKX          = (0x71, Read<1>),
            pub const RDBKY          = (0x72, Read<1>),
            pub const RDWX           = (0x73, Read<1>),
            pub const RDWY           = (0x74, Read<1>),
            pub const RDRGLB         = (0x75, Read<1>),
            pub const RDRX           = (0x76, Read<1>),
            pub const RDRY           = (0x77, Read<1>),
            pub const RDGX           = (0x78, Read<1>),
            pub const RDGY           = (0x79, Read<1>),
            pub const RDBALB         = (0x7A, Read<1>),
            pub const RDBX           = (0x7B, Read<1>),
            pub const RDBY           = (0x7C, Read<1>),
            pub const RDAX           = (0x7D, Read<1>),
            pub const RDAY           = (0x7E, Read<1>),
            pub const RDDDBS         = (0xA1, Read<1>),
            pub const RDDDBC         = (0xA8, Read<1>),
            pub const RDFCS          = (0xAA, Read<1>),
            pub const RDCCS          = (0xAF, Read<1>),
            pub const RDID1          = (0xDA, Read<1>),
            pub const RDID2          = (0xDB, Read<1>),
            pub const RDID3          = (0xDC, Read<1>),
        }
    }
}

pub mod bk0 {
    #![allow(clippy::upper_case_acronyms)]
    use crate::st7701s_spi::{address::{ExtensionBk0, Write}};

    instructions! {
        pub enum BK0<ExtensionBk0> {
            pub const PVGAMCTRL          = (0xB0, Write<16>),
            pub const NVGAMCTRL          = (0xB1, Write<16>),
            pub const DGMEN              = (0xB8, Write<1>),
            pub const DGMLUTR            = (0xB9, Write<64>),
            pub const DGMLUTB            = (0xBA, Write<64>),
            pub const PWMCLKSEL          = (0xBC, Write<1>),
            pub const LNESET             = (0xC0, Write<2>),
            pub const PORCTRL            = (0xC1, Write<2>),
            pub const INVSET             = (0xC2, Write<2>),
            pub const RGBCTRL            = (0xC3, Write<4>),
            pub const PARCTRL            = (0xC5, Write<2>),
            pub const SDIR               = (0xC7, Write<1>),
            pub const PDOSET             = (0xC8, Write<1>),
            pub const COLCTRL            = (0xCD, Write<1>),
            pub const SRECTRL            = (0xE0, Write<3>),
            pub const NRCTRL             = (0xE1, Write<11>),
            pub const SECTRL             = (0xE2, Write<13>),
            pub const CCCTRL             = (0xE3, Write<4>),
            pub const SKCTRL             = (0xE4, Write<2>),
            pub const NVMSETE            = (0xEA, Write<1>),
            pub const CABCCTRL           = (0xEE, Write<1>),
        }
    }
}

pub mod bk1 {
    #![allow(clippy::upper_case_acronyms)]
    use crate::st7701s_spi::address::{ExtensionBk1, Write};

    instructions! {
        pub enum BK1<ExtensionBk1> {
            pub const VRHS           = (0xB0, Write<1>),
            pub const VCOMS          = (0xB1, Write<1>),
            pub const VGHSS          = (0xB2, Write<1>),
            pub const TESTCMD        = (0xB3, Write<1>),
            pub const VGLS           = (0xB5, Write<1>),
            pub const PWCTRL1        = (0xB7, Write<1>),
            pub const PWCTRL2        = (0xB8, Write<1>),
            pub const PCLKS1         = (0xBA, Write<1>),
            pub const PCLKS2         = (0xBB, Write<1>),
            pub const PCLKS3         = (0xBC, Write<1>),
            pub const SPD1           = (0xC1, Write<1>),
            pub const SPD2           = (0xC2, Write<1>),
            pub const MIPISET1       = (0xD0, Write<1>),
            pub const MIPISET2       = (0xD1, Write<15>),
            pub const MIPISET3       = (0xD2, Write<1>),
            pub const MIPISET4       = (0xD3, Write<1>),
        }
    }
}

pub mod bk3 {
    #![allow(clippy::upper_case_acronyms)]
    use crate::st7701s_spi::address::{ExtensionBk3, Write};

    instructions! {
        pub enum BK3<ExtensionBk3> {
            pub const NVMSET         = (0xCA, Write<1>),
            pub const PROMACT        = (0xCC, Write<1>),
        }
    }
}
