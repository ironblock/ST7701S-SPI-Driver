use crate::st7701s_spi::protocol::connection::{
    AnyExtension, Bank0, Bank1, Bank3, Command, Read, Write,
};

type ANY = AnyExtension;
type BK0 = Bank0;
type BK1 = Bank1;
type BK3 = Bank3;

#[rustfmt::skip]
pub mod special {
    use super::{ANY, Write};

    pub type CND2BKXSEL =   Write<ANY, 0xFF, 5>;
    pub type DSTB       =   Write<ANY, 0xFF, 5>;
    pub type DSTBT      =   Write<ANY, 0xFF, 5>;
}

#[rustfmt::skip]
pub mod core {
    use super::{ANY, Command, Write, Read};

    pub type NOP        = Command<ANY, 0x00>;
    pub type SWRESET    =   Write<ANY, 0x01, 1>;
    pub type RDDID      =    Read<ANY, 0x04, 3>;
    pub type RDNUMED    =    Read<ANY, 0x05, 1>;
    pub type RDRED      =    Read<ANY, 0x06, 1>;
    pub type RDGREEN    =    Read<ANY, 0x07, 1>;
    pub type RDBLUE     =    Read<ANY, 0x08, 1>;
    pub type RDDPM      =    Read<ANY, 0x0A, 1>;
    pub type RDDMADCTL  =    Read<ANY, 0x0B, 1>;
    pub type RDDCOLMOD  =    Read<ANY, 0x0C, 1>;
    pub type RDDIM      =    Read<ANY, 0x0D, 1>;
    pub type RDDSM      =    Read<ANY, 0x0E, 1>;
    pub type RDDSDR     =    Read<ANY, 0x0F, 1>;
    pub type SLPIN      = Command<ANY, 0x10>;
    pub type SLPOUT     = Command<ANY, 0x11>;
    pub type PTLON      = Command<ANY, 0x12>;
    pub type NORON      = Command<ANY, 0x13>;
    pub type INVOFF     = Command<ANY, 0x20>;
    pub type INVON      = Command<ANY, 0x21>;
    pub type ALLPOFF    = Command<ANY, 0x22>;
    pub type ALLPON     = Command<ANY, 0x23>;
    pub type GAMSET     =   Write<ANY, 0x26, 1>;
    pub type DISPOFF    = Command<ANY, 0x28>;
    pub type DISPON     = Command<ANY, 0x29>;
    pub type TEOFF      = Command<ANY, 0x34>;
    pub type TEON       =   Write<ANY, 0x35, 1>;
    pub type MADCTL     =   Write<ANY, 0x36, 1>;
    pub type IDMOFF     = Command<ANY, 0x38>;
    pub type IDMON      = Command<ANY, 0x39>;
    pub type COLMOD     =   Write<ANY, 0x3A, 1>;
    pub type GSL        =    Read<ANY, 0x45, 2>;
    pub type WRDISBV    =   Write<ANY, 0x51, 1>;
    pub type RDDISBV    =    Read<ANY, 0x52, 1>;
    pub type WRCTRLD    =   Write<ANY, 0x53, 1>;
    pub type RDCTRLD    =    Read<ANY, 0x54, 1>;
    pub type WRCACE     =   Write<ANY, 0x55, 1>;
    pub type RDCABC     =    Read<ANY, 0x56, 1>;
    pub type WRCABCMB   =   Write<ANY, 0x5E, 1>;
    pub type RDCABCMB   =    Read<ANY, 0x5F, 1>;
    pub type RDABCSDR   =    Read<ANY, 0x68, 1>;
    pub type RDBWLB     =    Read<ANY, 0x70, 1>;
    pub type RDBKX      =    Read<ANY, 0x71, 1>;
    pub type RDBKY      =    Read<ANY, 0x72, 1>;
    pub type RDWX       =    Read<ANY, 0x73, 1>;
    pub type RDWY       =    Read<ANY, 0x74, 1>;
    pub type RDRGLB     =    Read<ANY, 0x75, 1>;
    pub type RDRX       =    Read<ANY, 0x76, 1>;
    pub type RDRY       =    Read<ANY, 0x77, 1>;
    pub type RDGX       =    Read<ANY, 0x78, 1>;
    pub type RDGY       =    Read<ANY, 0x79, 1>;
    pub type RDBALB     =    Read<ANY, 0x7A, 1>;
    pub type RDBX       =    Read<ANY, 0x7B, 1>;
    pub type RDBY       =    Read<ANY, 0x7C, 1>;
    pub type RDAX       =    Read<ANY, 0x7D, 1>;
    pub type RDAY       =    Read<ANY, 0x7E, 1>;
    pub type RDDDBS     =    Read<ANY, 0xA1, 1>;
    pub type RDDDBC     =    Read<ANY, 0xA8, 1>;
    pub type RDFCS      =    Read<ANY, 0xAA, 1>;
    pub type RDCCS      =    Read<ANY, 0xAF, 1>;
    pub type RDID1      =    Read<ANY, 0xDA, 1>;
    pub type RDID2      =    Read<ANY, 0xDB, 1>;
    pub type RDID3      =    Read<ANY, 0xDC, 1>;
}

#[rustfmt::skip]
pub mod bk0 {
    use super::{BK0,  Write};

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
    use super::{BK1, Write};

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
    use super::{BK3, Write};

    pub type NVMSET     =   Write<BK3, 0xCA, 1>;
    pub type PROMACT    =   Write<BK3, 0xCC, 1>;
}
