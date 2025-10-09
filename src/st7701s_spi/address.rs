use std::array;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Location {
    All(u8),
    BK0(u8),
    BK1(u8),
    BK3(u8),
}
impl Location {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::All(addr) => *addr,
            Self::BK0(addr) => *addr,
            Self::BK1(addr) => *addr,
            Self::BK3(addr) => *addr,
        }
    }
}

pub trait DataBuffer: AsRef<[u8]> + AsMut<[u8]> + IntoIterator<Item = u8> {}
impl<const N: usize> DataBuffer for [u8; N] where Self: IntoIterator<Item = u8, IntoIter = array::IntoIter<u8, N>> {}

pub trait CommandInstruction {
    const LOCATION: Location;
}
pub trait WriteInstruction {
    const LOCATION: Location;
    type Buffer: DataBuffer;
}
pub trait ReadInstruction {
    const LOCATION: Location;
    type Buffer: DataBuffer;
}

macro_rules! instructions {
    (@as_location $EVIS:vis fn(&self) => ($LOC:path, $ADDR:literal)) => {
        pub const fn as_location(&self) -> Location {
            $LOC($ADDR)
        }
    };
    (@as_location $EVIS:vis fn(&self) => ($LOC:path)) => {
        pub const fn as_location(&self) -> Location where Self: Sized + Copy {
            $LOC(*self as u8)
        }
    };
    ($EVIS:vis enum $GROUP:ident<$LOC:path $(,$SHARED:literal)?> {
       $($DVIS:vis const $NAME:ident = ($($ADDR:literal,)? $TYPE:ident$(<$PACKET:literal>)*),)+
    }) => {
        #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
        #[repr(u8)]
        $EVIS enum $GROUP {
            $($NAME$( = $ADDR)?),+
        }
        impl $GROUP {
            instructions!(@as_location $EVIS fn(&self) => ($LOC$(, $SHARED)?));
        }

        $(
            $DVIS struct $NAME;
            impl $TYPE for $NAME {
                const LOCATION: Location = $GROUP::as_location(&$GROUP::$NAME);
                $(type Buffer = [u8; $PACKET];)?
            }
        )+
    };
}

pub mod special {
    use crate::st7701s_spi::address::{ Location, WriteInstruction};
    use WriteInstruction as Write;

    instructions! {
            pub enum Special<Location::All, 0xFF> {
                pub const CND2BKXSEL = (Write<5>),
                pub const DSTB       = (Write<5>),
                pub const DSTBT      = (Write<5>),
            }
    }
}

pub mod core {
    use crate::st7701s_spi::address::{
         CommandInstruction as Command, Location, ReadInstruction as Read, WriteInstruction as Write,
    };

    instructions! {
        enum Core<Location::All> {
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
    use crate::st7701s_spi::address::{ Location, WriteInstruction as Write};

    instructions! {
        pub enum BK0<Location::BK0> {
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
    use crate::st7701s_spi::address::{ Location, WriteInstruction as Write};

    instructions! {
        pub enum BK1<Location::BK1> {
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
    use crate::st7701s_spi::address::{ Location, WriteInstruction as Write};

    instructions! {
        pub enum BK3<Location::BK3> {
            pub const NVMSET         = (0xCA, Write<1>),
            pub const PROMACT        = (0xCC, Write<1>),
        }
    }
}
