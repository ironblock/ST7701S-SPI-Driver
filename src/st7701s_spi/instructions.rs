use crate::st7701s_spi::protocol::connection::{
    AnyExtension, Bank0, Bank1, Bank3, CommandDefinition, ReadDefinition, WriteDefinition,
};

type ANY = AnyExtension;
type BK0 = Bank0;
type BK1 = Bank1;
type BK3 = Bank3;

#[rustfmt::skip]
pub mod special {
    use super::{ANY, WriteDefinition};

    pub type CND2BKXSEL =   WriteDefinition<ANY, 0xFF, 5>;
    pub type DSTB       =   WriteDefinition<ANY, 0xFF, 5>;
    pub type DSTBT      =   WriteDefinition<ANY, 0xFF, 5>;
}

#[rustfmt::skip]
pub mod core {
    use super::{ANY, CommandDefinition, WriteDefinition, ReadDefinition};

    pub type NOP        = CommandDefinition<ANY, 0x00>;
    pub type SWRESET    =   WriteDefinition<ANY, 0x01, 1>;
    pub type RDDID      =    ReadDefinition<ANY, 0x04, 3>;
    pub type RDNUMED    =    ReadDefinition<ANY, 0x05, 1>;
    pub type RDRED      =    ReadDefinition<ANY, 0x06, 1>;
    pub type RDGREEN    =    ReadDefinition<ANY, 0x07, 1>;
    pub type RDBLUE     =    ReadDefinition<ANY, 0x08, 1>;
    pub type RDDPM      =    ReadDefinition<ANY, 0x0A, 1>;
    pub type RDDMADCTL  =    ReadDefinition<ANY, 0x0B, 1>;
    pub type RDDCOLMOD  =    ReadDefinition<ANY, 0x0C, 1>;
    pub type RDDIM      =    ReadDefinition<ANY, 0x0D, 1>;
    pub type RDDSM      =    ReadDefinition<ANY, 0x0E, 1>;
    pub type RDDSDR     =    ReadDefinition<ANY, 0x0F, 1>;
    pub type SLPIN      = CommandDefinition<ANY, 0x10>;
    pub type SLPOUT     = CommandDefinition<ANY, 0x11>;
    pub type PTLON      = CommandDefinition<ANY, 0x12>;
    pub type NORON      = CommandDefinition<ANY, 0x13>;
    pub type INVOFF     = CommandDefinition<ANY, 0x20>;
    pub type INVON      = CommandDefinition<ANY, 0x21>;
    pub type ALLPOFF    = CommandDefinition<ANY, 0x22>;
    pub type ALLPON     = CommandDefinition<ANY, 0x23>;
    pub type GAMSET     =   WriteDefinition<ANY, 0x26, 1>;
    pub type DISPOFF    = CommandDefinition<ANY, 0x28>;
    pub type DISPON     = CommandDefinition<ANY, 0x29>;
    pub type TEOFF      = CommandDefinition<ANY, 0x34>;
    pub type TEON       =   WriteDefinition<ANY, 0x35, 1>;
    pub type MADCTL     =   WriteDefinition<ANY, 0x36, 1>;
    pub type IDMOFF     = CommandDefinition<ANY, 0x38>;
    pub type IDMON      = CommandDefinition<ANY, 0x39>;
    pub type COLMOD     =   WriteDefinition<ANY, 0x3A, 1>;
    pub type GSL        =    ReadDefinition<ANY, 0x45, 2>;
    pub type WRDISBV    =   WriteDefinition<ANY, 0x51, 1>;
    pub type RDDISBV    =    ReadDefinition<ANY, 0x52, 1>;
    pub type WRCTRLD    =   WriteDefinition<ANY, 0x53, 1>;
    pub type RDCTRLD    =    ReadDefinition<ANY, 0x54, 1>;
    pub type WRCACE     =   WriteDefinition<ANY, 0x55, 1>;
    pub type RDCABC     =    ReadDefinition<ANY, 0x56, 1>;
    pub type WRCABCMB   =   WriteDefinition<ANY, 0x5E, 1>;
    pub type RDCABCMB   =    ReadDefinition<ANY, 0x5F, 1>;
    pub type RDABCSDR   =    ReadDefinition<ANY, 0x68, 1>;
    pub type RDBWLB     =    ReadDefinition<ANY, 0x70, 1>;
    pub type RDBKX      =    ReadDefinition<ANY, 0x71, 1>;
    pub type RDBKY      =    ReadDefinition<ANY, 0x72, 1>;
    pub type RDWX       =    ReadDefinition<ANY, 0x73, 1>;
    pub type RDWY       =    ReadDefinition<ANY, 0x74, 1>;
    pub type RDRGLB     =    ReadDefinition<ANY, 0x75, 1>;
    pub type RDRX       =    ReadDefinition<ANY, 0x76, 1>;
    pub type RDRY       =    ReadDefinition<ANY, 0x77, 1>;
    pub type RDGX       =    ReadDefinition<ANY, 0x78, 1>;
    pub type RDGY       =    ReadDefinition<ANY, 0x79, 1>;
    pub type RDBALB     =    ReadDefinition<ANY, 0x7A, 1>;
    pub type RDBX       =    ReadDefinition<ANY, 0x7B, 1>;
    pub type RDBY       =    ReadDefinition<ANY, 0x7C, 1>;
    pub type RDAX       =    ReadDefinition<ANY, 0x7D, 1>;
    pub type RDAY       =    ReadDefinition<ANY, 0x7E, 1>;
    pub type RDDDBS     =    ReadDefinition<ANY, 0xA1, 1>;
    pub type RDDDBC     =    ReadDefinition<ANY, 0xA8, 1>;
    pub type RDFCS      =    ReadDefinition<ANY, 0xAA, 1>;
    pub type RDCCS      =    ReadDefinition<ANY, 0xAF, 1>;
    pub type RDID1      =    ReadDefinition<ANY, 0xDA, 1>;
    pub type RDID2      =    ReadDefinition<ANY, 0xDB, 1>;
    pub type RDID3      =    ReadDefinition<ANY, 0xDC, 1>;
}

#[rustfmt::skip]
pub mod bk0 {
    use super::{BK0,  WriteDefinition};

    pub type PVGAMCTRL  =   WriteDefinition<BK0, 0xB0, 16>;
    pub type NVGAMCTRL  =   WriteDefinition<BK0, 0xB1, 16>;
    pub type DGMEN      =   WriteDefinition<BK0, 0xB8, 1>;
    pub type DGMLUTR    =   WriteDefinition<BK0, 0xB9, 64>;
    pub type DGMLUTB    =   WriteDefinition<BK0, 0xBA, 64>;
    pub type PWMCLKSEL  =   WriteDefinition<BK0, 0xBC, 1>;
    pub type LNESET     =   WriteDefinition<BK0, 0xC0, 2>;
    pub type PORCTRL    =   WriteDefinition<BK0, 0xC1, 2>;
    pub type INVSET     =   WriteDefinition<BK0, 0xC2, 2>;
    pub type RGBCTRL    =   WriteDefinition<BK0, 0xC3, 4>;
    pub type PARCTRL    =   WriteDefinition<BK0, 0xC5, 2>;
    pub type SDIR       =   WriteDefinition<BK0, 0xC7, 1>;
    pub type PDOSET     =   WriteDefinition<BK0, 0xC8, 1>;
    pub type COLCTRL    =   WriteDefinition<BK0, 0xCD, 1>;
    pub type SRECTRL    =   WriteDefinition<BK0, 0xE0, 3>;
    pub type NRCTRL     =   WriteDefinition<BK0, 0xE1, 11>;
    pub type SECTRL     =   WriteDefinition<BK0, 0xE2, 13>;
    pub type CCCTRL     =   WriteDefinition<BK0, 0xE3, 4>;
    pub type SKCTRL     =   WriteDefinition<BK0, 0xE4, 2>;
    pub type NVMSETE    =   WriteDefinition<BK0, 0xEA, 1>;
    pub type CABCCTRL   =   WriteDefinition<BK0, 0xEE, 1>;
}

#[rustfmt::skip]
pub mod bk1 {
    use super::{BK1, WriteDefinition};

    pub type VRHS       =   WriteDefinition<BK1, 0xB0, 1>;
    pub type VCOMS      =   WriteDefinition<BK1, 0xB1, 1>;
    pub type VGHSS      =   WriteDefinition<BK1, 0xB2, 1>;
    pub type TESTCMD    =   WriteDefinition<BK1, 0xB3, 1>;
    pub type VGLS       =   WriteDefinition<BK1, 0xB5, 1>;
    pub type PWCTRL1    =   WriteDefinition<BK1, 0xB7, 1>;
    pub type PWCTRL2    =   WriteDefinition<BK1, 0xB8, 1>;
    pub type PCLKS1     =   WriteDefinition<BK1, 0xBA, 1>;
    pub type PCLKS2     =   WriteDefinition<BK1, 0xBB, 1>;
    pub type PCLKS3     =   WriteDefinition<BK1, 0xBC, 1>;
    pub type SPD1       =   WriteDefinition<BK1, 0xC1, 1>;
    pub type SPD2       =   WriteDefinition<BK1, 0xC2, 1>;
    pub type MIPISET1   =   WriteDefinition<BK1, 0xD0, 1>;
    pub type MIPISET2   =   WriteDefinition<BK1, 0xD1, 15>;
    pub type MIPISET3   =   WriteDefinition<BK1, 0xD2, 1>;
    pub type MIPISET4   =   WriteDefinition<BK1, 0xD3, 1>;
}

#[rustfmt::skip]
pub mod bk3 {
    use super::{BK3, WriteDefinition};

    pub type NVMSET     =   WriteDefinition<BK3, 0xCA, 1>;
    pub type PROMACT    =   WriteDefinition<BK3, 0xCC, 1>;
}
