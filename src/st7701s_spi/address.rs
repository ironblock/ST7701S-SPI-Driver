use crate::st7701s_spi::protocol::connection::{Command, Read, Write};
use crate::st7701s_spi::protocol::connection::{ExtensionBk0, ExtensionBk1, ExtensionBk3};

type ALL = ();
type BK0 = ExtensionBk0;
type BK1 = ExtensionBk1;
type BK3 = ExtensionBk3;

#[rustfmt::skip]
pub mod special {
    // #![allow(clippy::upper_case_acronyms, reason = "matches datasheet naming conventions")]
    use super::*;

    pub type CND2BKXSEL =   Write<ALL, 0xFF, 5>;
    pub type DSTB       =   Write<ALL, 0xFF, 5>;
    pub type DSTBT      =   Write<ALL, 0xFF, 5>;
}

#[rustfmt::skip]
pub mod core {
    use super::*;

    pub type NOP        = Command<ALL, 0x00>;
    pub type SWRESET    =   Write<ALL, 0x01, 1>;
    pub type RDDID      =    Read<ALL, 0x04, 3>;
    pub type RDNUMED    =    Read<ALL, 0x05, 1>;
    pub type RDRED      =    Read<ALL, 0x06, 1>;
    pub type RDGREEN    =    Read<ALL, 0x07, 1>;
    pub type RDBLUE     =    Read<ALL, 0x08, 1>;
    pub type RDDPM      =    Read<ALL, 0x0A, 1>;
    pub type RDDMADCTL  =    Read<ALL, 0x0B, 1>;
    pub type RDDCOLMOD  =    Read<ALL, 0x0C, 1>;
    pub type RDDIM      =    Read<ALL, 0x0D, 1>;
    pub type RDDSM      =    Read<ALL, 0x0E, 1>;
    pub type RDDSDR     =    Read<ALL, 0x0F, 1>;
    pub type SLPIN      = Command<ALL, 0x10>;
    pub type SLPOUT     = Command<ALL, 0x11>;
    pub type PTLON      = Command<ALL, 0x12>;
    pub type NORON      = Command<ALL, 0x13>;
    pub type INVOFF     = Command<ALL, 0x20>;
    pub type INVON      = Command<ALL, 0x21>;
    pub type ALLPOFF    = Command<ALL, 0x22>;
    pub type ALLPON     = Command<ALL, 0x23>;
    pub type GAMSET     =   Write<ALL, 0x26, 1>;
    pub type DISPOFF    = Command<ALL, 0x28>;
    pub type DISPON     = Command<ALL, 0x29>;
    pub type TEOFF      = Command<ALL, 0x34>;
    pub type TEON       =   Write<ALL, 0x35, 1>;
    pub type MADCTL     =   Write<ALL, 0x36, 1>;
    pub type IDMOFF     = Command<ALL, 0x38>;
    pub type IDMON      = Command<ALL, 0x39>;
    pub type COLMOD     =   Write<ALL, 0x3A, 1>;
    pub type GSL        =    Read<ALL, 0x45, 2>;
    pub type WRDISBV    =   Write<ALL, 0x51, 1>;
    pub type RDDISBV    =    Read<ALL, 0x52, 1>;
    pub type WRCTRLD    =   Write<ALL, 0x53, 1>;
    pub type RDCTRLD    =    Read<ALL, 0x54, 1>;
    pub type WRCACE     =   Write<ALL, 0x55, 1>;
    pub type RDCABC     =    Read<ALL, 0x56, 1>;
    pub type WRCABCMB   =   Write<ALL, 0x5E, 1>;
    pub type RDCABCMB   =    Read<ALL, 0x5F, 1>;
    pub type RDABCSDR   =    Read<ALL, 0x68, 1>;
    pub type RDBWLB     =    Read<ALL, 0x70, 1>;
    pub type RDBKX      =    Read<ALL, 0x71, 1>;
    pub type RDBKY      =    Read<ALL, 0x72, 1>;
    pub type RDWX       =    Read<ALL, 0x73, 1>;
    pub type RDWY       =    Read<ALL, 0x74, 1>;
    pub type RDRGLB     =    Read<ALL, 0x75, 1>;
    pub type RDRX       =    Read<ALL, 0x76, 1>;
    pub type RDRY       =    Read<ALL, 0x77, 1>;
    pub type RDGX       =    Read<ALL, 0x78, 1>;
    pub type RDGY       =    Read<ALL, 0x79, 1>;
    pub type RDBALB     =    Read<ALL, 0x7A, 1>;
    pub type RDBX       =    Read<ALL, 0x7B, 1>;
    pub type RDBY       =    Read<ALL, 0x7C, 1>;
    pub type RDAX       =    Read<ALL, 0x7D, 1>;
    pub type RDAY       =    Read<ALL, 0x7E, 1>;
    pub type RDDDBS     =    Read<ALL, 0xA1, 1>;
    pub type RDDDBC     =    Read<ALL, 0xA8, 1>;
    pub type RDFCS      =    Read<ALL, 0xAA, 1>;
    pub type RDCCS      =    Read<ALL, 0xAF, 1>;
    pub type RDID1      =    Read<ALL, 0xDA, 1>;
    pub type RDID2      =    Read<ALL, 0xDB, 1>;
    pub type RDID3      =    Read<ALL, 0xDC, 1>;
}

#[rustfmt::skip]
pub mod bk0 {
    use super::*;

    pub type PVGAMCTRL  =   Write<BK0, 0xB0, 16>;
    pub type NVGAMCTRL  =   Write<BK0, 0xB1, 16>;
    pub type DGMEN      =   Write<BK0, 0xB8, 1>;
    pub type DGMLUTR    =   Write<BK0, 0xB9, 64>;
    pub type DGMLUTB    =   Write<BK0, 0xBA, 64>;
    pub type PWMCLKSEL  =   Write<BK0, 0xBC, 1>;
    pub type LNESET     =   Write<BK0, 0xC0, 2>;
    pub type PORCTRL    =   Write<BK0, 0xC1, 2>;
    pub type INVSET     =   Write<BK0, 0xC2, 2>;
    pub type RGBCTRL    =   Write<BK0, 0xC3, 4>;
    pub type PARCTRL    =   Write<BK0, 0xC5, 2>;
    pub type SDIR       =   Write<BK0, 0xC7, 1>;
    pub type PDOSET     =   Write<BK0, 0xC8, 1>;
    pub type COLCTRL    =   Write<BK0, 0xCD, 1>;
    pub type SRECTRL    =   Write<BK0, 0xE0, 3>;
    pub type NRCTRL     =   Write<BK0, 0xE1, 11>;
    pub type SECTRL     =   Write<BK0, 0xE2, 13>;
    pub type CCCTRL     =   Write<BK0, 0xE3, 4>;
    pub type SKCTRL     =   Write<BK0, 0xE4, 2>;
    pub type NVMSETE    =   Write<BK0, 0xEA, 1>;
    pub type CABCCTRL   =   Write<BK0, 0xEE, 1>;
}

#[rustfmt::skip]
pub mod bk1 {
    use super::*;

    pub type VRHS       =   Write<BK1, 0xB0, 1>;
    pub type VCOMS      =   Write<BK1, 0xB1, 1>;
    pub type VGHSS      =   Write<BK1, 0xB2, 1>;
    pub type TESTCMD    =   Write<BK1, 0xB3, 1>;
    pub type VGLS       =   Write<BK1, 0xB5, 1>;
    pub type PWCTRL1    =   Write<BK1, 0xB7, 1>;
    pub type PWCTRL2    =   Write<BK1, 0xB8, 1>;
    pub type PCLKS1     =   Write<BK1, 0xBA, 1>;
    pub type PCLKS2     =   Write<BK1, 0xBB, 1>;
    pub type PCLKS3     =   Write<BK1, 0xBC, 1>;
    pub type SPD1       =   Write<BK1, 0xC1, 1>;
    pub type SPD2       =   Write<BK1, 0xC2, 1>;
    pub type MIPISET1   =   Write<BK1, 0xD0, 1>;
    pub type MIPISET2   =   Write<BK1, 0xD1, 15>;
    pub type MIPISET3   =   Write<BK1, 0xD2, 1>;
    pub type MIPISET4   =   Write<BK1, 0xD3, 1>;
}

#[rustfmt::skip]
pub mod bk3 {
    use super::*;

    pub type NVMSET     =   Write<BK3, 0xCA, 1>;
    pub type PROMACT    =   Write<BK3, 0xCC, 1>;
}
