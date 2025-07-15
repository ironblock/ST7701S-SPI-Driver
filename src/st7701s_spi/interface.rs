use frunk::LabelledGeneric;

/**
    ## Extended Address Banks

    The ST7701S exposes some extended command sets based on the setting of an
    internal register, referred to in the datasheet as **Command2 BKx**.

    > Section 12.3.1 `CND2BKxSEL`, page 260
*/
#[derive(Copy, Clone)]
pub enum Bank {
    BK0,
    BK1,
    BK3,
}

pub type Extension = Option<Bank>;
pub type Address = u8;

pub type WriteData<const S: usize> = [u8; S];
pub type ReadData = fn(&[u8]);
pub enum Operation<const S: usize> {
    Command,
    Write(WriteData<S>),
    Read(ReadData),
}

pub struct Transmission<const S: usize> {
    pub extension: Extension,
    pub address: Address,
    pub operation: Operation<S>,
}
pub type Command = Transmission<0>;

pub type Location = (Extension, Address);

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Core {
    NOP        = 0x00,
    SWRESET    = 0x01,
    RDDID      = 0x04,
    RDNUMED    = 0x05,
    RDRED      = 0x06,
    RDGREEN    = 0x07,
    RDBLUE     = 0x08,
    RDDPM      = 0x0A,
    RDDMADCTL  = 0x0B,
    RDDCOLMOD  = 0x0C,
    RDDIM      = 0x0D,
    RDDSM      = 0x0E,
    RDDSDR     = 0x0F,
    SLPIN      = 0x10,
    SLPOUT     = 0x11,
    PTLON      = 0x12,
    NORON      = 0x13,
    INVOFF     = 0x20,
    INVON      = 0x21,
    ALLPOFF    = 0x22,
    ALLPON     = 0x23,
    GAMSET     = 0x26,
    DISPOFF    = 0x28,
    DISPON     = 0x29,
    TEOFF      = 0x34,
    TEON       = 0x35,
    MADCTL     = 0x36,
    IDMOFF     = 0x38,
    IDMON      = 0x39,
    COLMOD     = 0x3A,
    GSL        = 0x45,
    WRDISBV    = 0x51,
    RDDISBV    = 0x52,
    WRCTRLD    = 0x53,
    RDCTRLD    = 0x54,
    WRCACE     = 0x55,
    RDCABC     = 0x56,
    WRCABCMB   = 0x5E,
    RDCABCMB   = 0x5F,
    RDABCSDR   = 0x68,
    RDBWLB     = 0x70,
    RDBKX      = 0x71,
    RDBKY      = 0x72,
    RDWX       = 0x73,
    RDWY       = 0x74,
    RDRGLB     = 0x75,
    RDRX       = 0x76,
    RDRY       = 0x77,
    RDGX       = 0x78,
    RDGY       = 0x79,
    RDBALB     = 0x7A,
    RDBX       = 0x7B,
    CND2BKXSEL = 0xFF,
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BK0 {
    PVGAMCTRL = 0xB0,
    NVGAMCTRL = 0xB1,
    DGMEN     = 0xB8,
    DGMLUTR   = 0xB9,
    DGMLUTB   = 0xBA,
    PWMCLK    = 0xBC,
    LNESET    = 0xC0,
    PORCTRL   = 0xC1,
    INVSET    = 0xC2,
    RGBCTRL   = 0xC3,
    PARCTRL   = 0xC5,
    SDIR      = 0xC7,
    PDOSET    = 0xC8,
    COLCTRL   = 0xCD,
    SSCTRL    = 0xCE,
    SRECTRL   = 0xE0,
    NRCTRL    = 0xE1,
    SECTRL    = 0xE2,
    CCCTRL    = 0xE3,
    SKCTRL    = 0xE4,
    NVMSETE   = 0xEA,
    CABCCTRL  = 0xEE,
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BK1 {
    VRHS     = 0xB0,
    VCOMS    = 0xB1,
    VGHSS    = 0xB2,
    TESTCMD  = 0xB3,
    VGLS     = 0xB5,
    PWCTRL1  = 0xB7,
    PWCTRL2  = 0xB8,
    PCLKS1   = 0xBA,
    PCLKS3   = 0xBC,
    SPD1     = 0xC1,
    SPD2     = 0xC2,
    MIPISET1 = 0xD0,
    MIPISET2 = 0xD1,
    MIPISET3 = 0xD2,
    MIPISET4 = 0xD3,
    NVMEN    = 0xC8,
    NVMSET   = 0xCA,
}

pub enum Instruction {
    Core(Core),
    BK0(BK0),
    BK1(BK1),
}

impl Instruction {
    pub const fn location(&self) -> Location {
        match *self {
            Self::Core(x) => (None, x as Address),
            Self::BK0(x) => (Some(Bank::BK0), x as Address),
            Self::BK1(x) => (Some(Bank::BK1), x as Address),
        }
    }

    pub const fn define<const S: usize>(&self, operation: Operation<S>) -> Transmission<S> {
        let (extension, address) = Self::location(&self);

        Transmission {
            extension,
            address,
            operation,
        }
    }

    pub const fn to_command(&self) -> Transmission<0> {
        Self::define(&self, Operation::Command)
    }

    pub const fn to_write<const S: usize>(&self, data: WriteData<S>) -> Transmission<S> {
        Self::define(&self, Operation::Write(data))
    }

    pub const fn to_read<S>(&self, handler: ReadData) -> Transmission<0> {
        Self::define(&self, Operation::Read(handler))
    }
}
