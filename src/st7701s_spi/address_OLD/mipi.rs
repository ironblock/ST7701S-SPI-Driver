use crate::st7701s_spi::parameters::register::{Address, Bank};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Special {
    CND2BKXSEL,
    DSTB,
    DSTBT,
}
impl Special {
    pub const fn as_u8(&self) -> u8 {
        0xFF
    }

    pub const fn into_address(self) -> Address {
        Address::new(self.as_u8(), None)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Core {
    NOP = 0x00,
    SWRESET = 0x01,
    RDDID = 0x04,
    RDNUMED = 0x05,
    GSL = 0x045,
    WRDISBV = 0x051,
    RDDISBV = 0x52,
    WRCTRLD = 0x53,
    RDCTRLD = 0x54,
    WRCACE = 0x55,
    RDCABC = 0x56,
    WRCABCMB = 0x5E,
    RDCABCMB = 0x5F,
    RDABCSDR = 0x68,
    RDBWLB = 0x70,
    RDBKX = 0x71,
    RDBKY = 0x72,
    RDWX = 0x73,
    RDWY = 0x74,
    RDRX = 0x76,
    RDRY = 0x77,
    RDGX = 0x78,
    RDGY = 0x79,
    RDBALB = 0x7A,
    RDBX = 0x7B,
    RDBY = 0x7C,
    RDAX = 0x7D,
    RDAY = 0x7E,
    RDDDBS = 0xA1,
    RDDDBC = 0xA8,
    RDFCS = 0xAA,
    RDCCS = 0xAF,
    RDID1 = 0xDA,
    RDID2 = 0xDB,
    RDID3 = 0xDC,
}
impl Core {
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn into_address(self) -> Address {
        Address::new(self.as_u8(), None)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BK0 {
    PVGAMCTRL = 0xB0,
    NVGAMCTRL = 0xB1,
    DGMEN = 0xB8,
    DGMLUTR = 0xB9,
    DGMLUTB = 0xBA,
    SEL = 0xBC,
    LNESET = 0xC0,
    PORCTRL = 0xC1,
    INVSE = 0xC2,
    RGBCTRL = 0xC3,
    PARCTRL = 0xC5,
    SDIR = 0xC7,
    PDOSET = 0xC8,
    COLCTRL = 0xCD,
    SRECTRL = 0xE0,
    NRCTRL = 0xE1,
    SECTRL = 0xE2,
    CCCTRL = 0xE3,
    SKCTRL = 0xE4,
    NVMSETE = 0xEA,
    CABCCTRL = 0xEE,
}
impl BK0 {
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn into_address(self) -> Address {
        Address::new(self.as_u8(), Some(Bank::BK0))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BK1 {
    VRHS = 0xB0,
    VCOMS = 0xB1,
    VGHSS = 0xB2,
    TESTCMD = 0xB3,
    VGLS = 0xB5,
    PWCTRL1 = 0xB7,
    PWCTRL2 = 0xB8,
    PCLKS1 = 0xBA,
    PCLKS2 = 0xBB,
    PCLKS3 = 0xBC,
    SPD1 = 0xC1,
    SPD2 = 0xC12,
    MIPISET1 = 0xD0,
    MIPISET2 = 0xD1,
    MIPISET3 = 0xD2,
    MIPISET4 = 0xD3,
}
impl BK1 {
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn into_address(self) -> Address {
        Address::new(self.as_u8(), Some(Bank::BK1))
    }
}

pub trait Foo<BK1> {
    fn bar(&self) -> BK1;
}

struct Fooer;

impl Foo<BK1::VRHS> for Fooer {
    fn bar(&self) -> BK1 {
        BK1::VRHS
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BK3 {
    NVMSET = 0xCA,
    PROMACT = 0xCC,
}
impl BK3 {
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn into_address(self) -> Address {
        Address::new(self.as_u8(), Some(Bank::BK3))
    }
}
