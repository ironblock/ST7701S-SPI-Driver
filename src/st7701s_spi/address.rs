pub enum Location {
    All(u8),
    BK0(u8),
    BK1(u8),
    BK3(u8),
}
impl Location {
    pub fn as_u8(&self) -> u8 {
        match self {
            Location::All(addr) => *addr,
            Location::BK0(addr) => *addr,
            Location::BK1(addr) => *addr,
            Location::BK3(addr) => *addr,
        }
    }
}

pub trait CommandInstruction {
    const LOCATION: Location;
}
pub trait WriteInstruction {
    const LOCATION: Location;
    type Buffer: AsRef<[u8]> + IntoIterator<Item = u8>;
}
pub trait ReadInstruction {
    const LOCATION: Location;
    type Buffer: AsRef<[u8]>;
}

pub mod special {
    use crate::st7701s_spi::address::{Location, WriteInstruction};

    const LOCATION: Location = Location::All(0xFF);
    const PARAMETERS: usize = 5;

    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct CND2BKXSEL;
    impl WriteInstruction for CND2BKXSEL {
        const LOCATION: Location = LOCATION;
        type Buffer = [u8; PARAMETERS];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct DSTB;
    impl WriteInstruction for DSTB {
        const LOCATION: Location = LOCATION;
        type Buffer = [u8; PARAMETERS];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct DSTBT;
    impl WriteInstruction for DSTBT {
        const LOCATION: Location = LOCATION;
        type Buffer = [u8; PARAMETERS];
    }
}

pub mod core {
    use crate::st7701s_spi::address::{
        CommandInstruction, Location, ReadInstruction, WriteInstruction,
    };

    #[repr(u8)]
    pub enum Core {
        NOP = 0x00,
        SWRESET = 0x01,
        RDDID = 0x04,
        RDNUMED = 0x05,
        RDRED = 0x06,
        RDGREEN = 0x07,
        RDBLUE = 0x08,
        RDDPM = 0x0A,
        RDDMADCTL = 0x0B,
        RDDCOLMOD = 0x0C,
        RDDIM = 0x0D,
        RDDSM = 0x0E,
        RDDSDR = 0x0F,
        SLPIN = 0x10,
        SLPOUT = 0x11,
        PTLON = 0x12,
        NORON = 0x13,
        INVOFF = 0x20,
        INVON = 0x21,
        ALLPOFF = 0x22,
        ALLPON = 0x23,
        GAMSET = 0x26,
        DISPOFF = 0x28,
        DISPON = 0x29,
        TEOFF = 0x34,
        TEON = 0x35,
        MADCTL = 0x36,
        IDMOFF = 0x38,
        IDMON = 0x39,
        COLMOD = 0x3A,
        GSL = 0x45,
        WRDISBV = 0x51,
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
        RDRGLB = 0x75,
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
        pub const fn as_location(self) -> Location {
            Location::All(self as u8)
        }
    }

    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct NOP;
    impl CommandInstruction for NOP {
        const LOCATION: Location = Core::NOP.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SWRESET;
    impl WriteInstruction for SWRESET {
        const LOCATION: Location = Core::SWRESET.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDID;
    impl ReadInstruction for RDDID {
        const LOCATION: Location = Core::RDDID.as_location();
        type Buffer = [u8; 3];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDNUMED;
    impl ReadInstruction for RDNUMED {
        const LOCATION: Location = Core::RDNUMED.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDRED;
    impl ReadInstruction for RDRED {
        const LOCATION: Location = Core::RDRED.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDGREEN;
    impl ReadInstruction for RDGREEN {
        const LOCATION: Location = Core::RDGREEN.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDBLUE;
    impl ReadInstruction for RDBLUE {
        const LOCATION: Location = Core::RDBLUE.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDPM;
    impl ReadInstruction for RDDPM {
        const LOCATION: Location = Core::RDDPM.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDMADCTL;
    impl ReadInstruction for RDDMADCTL {
        const LOCATION: Location = Core::RDDMADCTL.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDCOLMOD;
    impl ReadInstruction for RDDCOLMOD {
        const LOCATION: Location = Core::RDDCOLMOD.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDIM;
    impl ReadInstruction for RDDIM {
        const LOCATION: Location = Core::RDDIM.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDSM;
    impl ReadInstruction for RDDSM {
        const LOCATION: Location = Core::RDDSM.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDSDR;
    impl ReadInstruction for RDDSDR {
        const LOCATION: Location = Core::RDDSDR.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SLPIN;
    impl CommandInstruction for SLPIN {
        const LOCATION: Location = Core::SLPIN.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SLPOUT;
    impl CommandInstruction for SLPOUT {
        const LOCATION: Location = Core::SLPOUT.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PTLON;
    impl CommandInstruction for PTLON {
        const LOCATION: Location = Core::PTLON.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct NORON;
    impl CommandInstruction for NORON {
        const LOCATION: Location = Core::NORON.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct INVOFF;
    impl CommandInstruction for INVOFF {
        const LOCATION: Location = Core::INVOFF.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct INVON;
    impl CommandInstruction for INVON {
        const LOCATION: Location = Core::INVON.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct ALLPOFF;
    impl CommandInstruction for ALLPOFF {
        const LOCATION: Location = Core::ALLPOFF.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct ALLPON;
    impl CommandInstruction for ALLPON {
        const LOCATION: Location = Core::ALLPON.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct GAMSET;
    impl WriteInstruction for GAMSET {
        const LOCATION: Location = Core::GAMSET.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct DISPOFF;
    impl CommandInstruction for DISPOFF {
        const LOCATION: Location = Core::DISPOFF.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct DISPON;
    impl CommandInstruction for DISPON {
        const LOCATION: Location = Core::DISPON.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct TEOFF;
    impl CommandInstruction for TEOFF {
        const LOCATION: Location = Core::TEOFF.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct TEON;
    impl WriteInstruction for TEON {
        const LOCATION: Location = Core::TEON.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct MADCTL;
    impl WriteInstruction for MADCTL {
        const LOCATION: Location = Core::MADCTL.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct IDMOFF;
    impl CommandInstruction for IDMOFF {
        const LOCATION: Location = Core::IDMOFF.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct IDMON;
    impl CommandInstruction for IDMON {
        const LOCATION: Location = Core::IDMON.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct COLMOD;
    impl WriteInstruction for COLMOD {
        const LOCATION: Location = Core::COLMOD.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct GSL;
    impl ReadInstruction for GSL {
        const LOCATION: Location = Core::GSL.as_location();
        type Buffer = [u8; 2];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct WRDISBV;
    impl WriteInstruction for WRDISBV {
        const LOCATION: Location = Core::WRDISBV.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDISBV;
    impl ReadInstruction for RDDISBV {
        const LOCATION: Location = Core::RDDISBV.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct WRCTRLD;
    impl WriteInstruction for WRCTRLD {
        const LOCATION: Location = Core::WRCTRLD.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDCTRLD;
    impl ReadInstruction for RDCTRLD {
        const LOCATION: Location = Core::RDCTRLD.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct WRCACE;
    impl WriteInstruction for WRCACE {
        const LOCATION: Location = Core::WRCACE.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDCABC;
    impl ReadInstruction for RDCABC {
        const LOCATION: Location = Core::RDCABC.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct WRCABCMB;
    impl WriteInstruction for WRCABCMB {
        const LOCATION: Location = Core::WRCABCMB.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDCABCMB;
    impl ReadInstruction for RDCABCMB {
        const LOCATION: Location = Core::RDCABCMB.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDABCSDR;
    impl ReadInstruction for RDABCSDR {
        const LOCATION: Location = Core::RDABCSDR.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDBWLB;
    impl ReadInstruction for RDBWLB {
        const LOCATION: Location = Core::RDBWLB.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDBKX;
    impl ReadInstruction for RDBKX {
        const LOCATION: Location = Core::RDBKX.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDBKY;
    impl ReadInstruction for RDBKY {
        const LOCATION: Location = Core::RDBKY.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDWX;
    impl ReadInstruction for RDWX {
        const LOCATION: Location = Core::RDWX.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDWY;
    impl ReadInstruction for RDWY {
        const LOCATION: Location = Core::RDWY.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDRGLB;
    impl ReadInstruction for RDRGLB {
        const LOCATION: Location = Core::RDRGLB.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDRX;
    impl ReadInstruction for RDRX {
        const LOCATION: Location = Core::RDRX.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDRY;
    impl ReadInstruction for RDRY {
        const LOCATION: Location = Core::RDRY.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDGX;
    impl ReadInstruction for RDGX {
        const LOCATION: Location = Core::RDGX.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDGY;
    impl ReadInstruction for RDGY {
        const LOCATION: Location = Core::RDGY.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDBALB;
    impl ReadInstruction for RDBALB {
        const LOCATION: Location = Core::RDBALB.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDBX;
    impl ReadInstruction for RDBX {
        const LOCATION: Location = Core::RDBX.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDBY;
    impl ReadInstruction for RDBY {
        const LOCATION: Location = Core::RDBY.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDAX;
    impl ReadInstruction for RDAX {
        const LOCATION: Location = Core::RDAX.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDAY;
    impl ReadInstruction for RDAY {
        const LOCATION: Location = Core::RDAY.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDDBS;
    impl ReadInstruction for RDDDBS {
        const LOCATION: Location = Core::RDDDBS.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDDDBC;
    impl ReadInstruction for RDDDBC {
        const LOCATION: Location = Core::RDDDBC.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDFCS;
    impl ReadInstruction for RDFCS {
        const LOCATION: Location = Core::RDFCS.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDCCS;
    impl ReadInstruction for RDCCS {
        const LOCATION: Location = Core::RDCCS.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDID1;
    impl ReadInstruction for RDID1 {
        const LOCATION: Location = Core::RDID1.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDID2;
    impl ReadInstruction for RDID2 {
        const LOCATION: Location = Core::RDID2.as_location();
        type Buffer = [u8; 1];
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RDID3;
    impl ReadInstruction for RDID3 {
        const LOCATION: Location = Core::RDID3.as_location();
        type Buffer = [u8; 1];
    }
}

pub mod bk0 {
    use crate::st7701s_spi::address::{CommandInstruction, Location};

    #[repr(u8)]
    pub enum BK0 {
        PVGAMCTRL = 0xB0,
        NVGAMCTRL = 0xB1,
        DGMEN = 0xB8,
        DGMLUTR = 0xB9,
        DGMLUTB = 0xBA,
        PWMCLKSEL = 0xBC,
        LNESET = 0xC0,
        PORCTRL = 0xC1,
        INVSET = 0xC2,
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
        pub const fn as_location(self) -> Location {
            Location::BK0(self as u8)
        }
    }

    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PVGAMCTRL;
    impl CommandInstruction for PVGAMCTRL {
        const LOCATION: Location = BK0::PVGAMCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct NVGAMCTRL;
    impl CommandInstruction for NVGAMCTRL {
        const LOCATION: Location = BK0::NVGAMCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct DGMEN;
    impl CommandInstruction for DGMEN {
        const LOCATION: Location = BK0::DGMEN.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct DGMLUTR;
    impl CommandInstruction for DGMLUTR {
        const LOCATION: Location = BK0::DGMLUTR.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct DGMLUTB;
    impl CommandInstruction for DGMLUTB {
        const LOCATION: Location = BK0::DGMLUTB.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PWMCLKSEL;
    impl CommandInstruction for PWMCLKSEL {
        const LOCATION: Location = BK0::PWMCLKSEL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct LNESET;
    impl CommandInstruction for LNESET {
        const LOCATION: Location = BK0::LNESET.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PORCTRL;
    impl CommandInstruction for PORCTRL {
        const LOCATION: Location = BK0::PORCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct INVSET;
    impl CommandInstruction for INVSET {
        const LOCATION: Location = BK0::INVSET.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct RGBCTRL;
    impl CommandInstruction for RGBCTRL {
        const LOCATION: Location = BK0::RGBCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PARCTRL;
    impl CommandInstruction for PARCTRL {
        const LOCATION: Location = BK0::PARCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SDIR;
    impl CommandInstruction for SDIR {
        const LOCATION: Location = BK0::SDIR.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PDOSET;
    impl CommandInstruction for PDOSET {
        const LOCATION: Location = BK0::PDOSET.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct COLCTRL;
    impl CommandInstruction for COLCTRL {
        const LOCATION: Location = BK0::COLCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SRECTRL;
    impl CommandInstruction for SRECTRL {
        const LOCATION: Location = BK0::SRECTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct NRCTRL;
    impl CommandInstruction for NRCTRL {
        const LOCATION: Location = BK0::NRCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SECTRL;
    impl CommandInstruction for SECTRL {
        const LOCATION: Location = BK0::SECTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct CCCTRL;
    impl CommandInstruction for CCCTRL {
        const LOCATION: Location = BK0::CCCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SKCTRL;
    impl CommandInstruction for SKCTRL {
        const LOCATION: Location = BK0::SKCTRL.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct NVMSETE;
    impl CommandInstruction for NVMSETE {
        const LOCATION: Location = BK0::NVMSETE.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct CABCCTRL;
    impl CommandInstruction for CABCCTRL {
        const LOCATION: Location = BK0::CABCCTRL.as_location();
    }
}

pub mod bk1 {
    use crate::st7701s_spi::address::{CommandInstruction, Location};

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
        pub const fn as_location(self) -> Location {
            Location::BK1(self as u8)
        }
    }

    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct VRHS;
    impl CommandInstruction for VRHS {
        const LOCATION: Location = BK1::VRHS.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct VCOMS;
    impl CommandInstruction for VCOMS {
        const LOCATION: Location = BK1::VCOMS.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct VGHSS;
    impl CommandInstruction for VGHSS {
        const LOCATION: Location = BK1::VGHSS.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct TESTCMD;
    impl CommandInstruction for TESTCMD {
        const LOCATION: Location = BK1::TESTCMD.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct VGLS;
    impl CommandInstruction for VGLS {
        const LOCATION: Location = BK1::VGLS.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PWCTRL1;
    impl CommandInstruction for PWCTRL1 {
        const LOCATION: Location = BK1::PWCTRL1.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PWCTRL2;
    impl CommandInstruction for PWCTRL2 {
        const LOCATION: Location = BK1::PWCTRL2.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PCLKS1;
    impl CommandInstruction for PCLKS1 {
        const LOCATION: Location = BK1::PCLKS1.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PCLKS2;
    impl CommandInstruction for PCLKS2 {
        const LOCATION: Location = BK1::PCLKS2.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PCLKS3;
    impl CommandInstruction for PCLKS3 {
        const LOCATION: Location = BK1::PCLKS3.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SPD1;
    impl CommandInstruction for SPD1 {
        const LOCATION: Location = BK1::SPD1.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct SPD2;
    impl CommandInstruction for SPD2 {
        const LOCATION: Location = BK1::SPD2.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct MIPISET1;
    impl CommandInstruction for MIPISET1 {
        const LOCATION: Location = BK1::MIPISET1.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct MIPISET2;
    impl CommandInstruction for MIPISET2 {
        const LOCATION: Location = BK1::MIPISET2.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct MIPISET3;
    impl CommandInstruction for MIPISET3 {
        const LOCATION: Location = BK1::MIPISET3.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct MIPISET4;
    impl CommandInstruction for MIPISET4 {
        const LOCATION: Location = BK1::MIPISET4.as_location();
    }
}

pub mod bk3 {
    use crate::st7701s_spi::address::{CommandInstruction, Location};

    #[repr(u8)]
    pub enum BK3 {
        NVMSET = 0xCA,
        PROMACT = 0xCC,
    }
    impl BK3 {
        pub const fn as_location(self) -> Location {
            Location::BK3(self as u8)
        }
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct NVMSET;
    impl CommandInstruction for NVMSET {
        const LOCATION: Location = BK3::NVMSET.as_location();
    }
    
    #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
    pub struct PROMACT;
    impl CommandInstruction for PROMACT {
        const LOCATION: Location = BK3::PROMACT.as_location();
    }
}
