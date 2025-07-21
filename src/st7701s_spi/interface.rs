use std::any::Any;

pub type Address = u8;
pub type Extension = Option<Bank>;
pub type Bytes = usize;

pub type Buffer<const N: usize> = [u8; N];
pub type Reader = for<'a> fn(&'a [u8]);

/**
    ## Extended Address Banks

    The ST7701S exposes some extended command sets based on the setting of an
    internal register, referred to in the datasheet as **Command2 BKx**.

    > Section 12.3.1 `CND2BKxSEL`, page 260
*/
#[repr(u8)]
pub enum Bank {
    BK0,
    BK1,
    BK3,
}

pub trait Location: Sized {
    const ADDRESS: Address;
    const EXTENSION: Extension = None;
}

pub trait WriteData: Location {
    const BYTES: Bytes;
}

pub trait ReadData: Location {
    const BYTES: Bytes;
}

pub enum Operation<const N: Bytes> {
    Command(Address, Extension),
    Write(Address, Extension, Buffer<N>),
    Read(Address, Extension, Reader),
}

impl<const N: Bytes> Operation<N> {
    pub const fn command<L: Location>() -> Self {
        Self::Command(L::ADDRESS, L::EXTENSION)
    }

    pub const fn write<L: Location>(buffer: Buffer<N>) -> Self {
        Self::Write(L::ADDRESS, L::EXTENSION, buffer)
    }

    pub const fn read<L: Location>(handler: Reader) -> Self {
        Self::Read(L::ADDRESS, L::EXTENSION, handler)
    }
}

pub const fn allocate_buffer<const BYTES: usize>() -> [u8; BYTES] {
    [0; BYTES]
}

#[rustfmt::skip]
pub mod core {
    use super::*;

    /**
      ### `0x00` `NOP`  No Operation
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 187
    */
    pub struct NOP;
    impl Location for NOP         { const ADDRESS: u8      = 0x00; }

    /**
      ### `0x01` `SWRESET`  Software Reset

      #### Write Parameters

      It's never stated anywhere why D0 is 1, but it's indicated in both the
      primary reference table on p. 184 and again on SWRESET's detail page. As
      an additional contradiction, p. 184 refers to SWRESET as a **command**
      (with no arguments), and p. 188 refers to it as a **write**. As only a
      write can have arguments and 0x01 is the constant argument in both
      references, SWRESET's canonical representation here is as a **write**.

      |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
      |:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
      |   --   |   --   |   --   |   --   |   --   |   --   |   --   |    1   |

      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 188
    */
    pub struct SWRESET;
    impl Location for SWRESET     { const ADDRESS: u8      = 0x01; }
    impl WriteData for SWRESET    { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x04` `RDDID`  Read Display ID
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 189
    */
    pub struct RDDID;
    impl Location for RDDID       { const ADDRESS: Address = 0x04; }
    impl ReadData for RDDID       { const BYTES:   Bytes   = 4;    }

    /**
      ### `0x05` `RDNUMED`  Read Number of Errors on DSI
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 190
    */
    pub struct RDNUMED;
    impl Location for RDNUMED     { const ADDRESS: Address = 0x05; }
    impl ReadData for RDNUMED     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x06` `RDRED`  Read the first pixel of Red Color
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 191
    */
    pub struct RDRED;
    impl Location for RDRED       { const ADDRESS: Address = 0x06; }
    impl ReadData for RDRED       { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x07` `RDGREEN`  Read the first pixel of Green Color
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 192
    */
    pub struct RDGREEN;
    impl Location for RDGREEN     { const ADDRESS: Address = 0x07; }
    impl ReadData for RDGREEN     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x08` `RDBLUE`  Read the first pixel of Blue Color
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 193
    */
    pub struct RDBLUE;
    impl Location for RDBLUE      { const ADDRESS: Address = 0x08; }
    impl ReadData for RDBLUE      { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x0A` `RDDPM`  Read Display Power Mode
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 194
    */
    pub struct RDDPM;
    impl Location for RDDPM       { const ADDRESS: Address = 0x0A; }
    impl ReadData for RDDPM       { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x0B` `RDDMADCTL`  Read Display MADCTL
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 195
    */
    pub struct RDDMADCTL;
    impl Location for RDDMADCTL   { const ADDRESS: Address = 0x0B; }
    impl ReadData for RDDMADCTL   { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x0C` `RDDCOLMOD`  Read Display Pixel Format
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 196
    */
    pub struct RDDCOLMOD;
    impl Location for RDDCOLMOD   { const ADDRESS: Address = 0x0C; }
    impl ReadData for RDDCOLMOD   { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x0D` `RDDIM`  Read Display Image Mode
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 197
    */
    pub struct RDDIM;
    impl Location for RDDIM       { const ADDRESS: Address = 0x0D; }
    impl ReadData for RDDIM       { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x0E` `RDDSM`  Read Display Signal Mode
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 198
    */
    pub struct RDDSM;
    impl Location for RDDSM       { const ADDRESS: Address = 0x0E; }
    impl ReadData for RDDSM       { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x0F` `RDDSDR`  Read Display Self-Diagnostic Result
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 199
    */
    pub struct RDDSDR;
    impl Location for RDDSDR      { const ADDRESS: Address = 0x0F; }
    impl ReadData for RDDSDR      { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x10` `SLPIN`  Sleep in
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 200
    */
    pub struct SLPIN;
    impl Location for SLPIN       { const ADDRESS: Address = 0x10; }

    /**
      ### `0x11` `SLPOUT`  Sleep Out
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 201
    */
    pub struct SLPOUT;
    impl Location for SLPOUT      { const ADDRESS: Address = 0x11; }

    /**
      ### `0x12` `PTLON`  Partial Display Mode On
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 202
    */
    pub struct PTLON;
    impl Location for PTLON       { const ADDRESS: Address = 0x12; }

    /**
      ### `0x13` `NORON`  Normal Display Mode On
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 203
    */
    pub struct NORON;
    impl Location for NORON       { const ADDRESS: Address = 0x13; }

    /**
      ### `0x20` `INVOFF`  Display Inversion Off
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 204
    */
    pub struct INVOFF;
    impl Location for INVOFF      { const ADDRESS: Address = 0x20; }

    /**
      ### `0x21` `INVON`  Display Inversion On
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 205
    */
    pub struct INVON;
    impl Location for INVON       { const ADDRESS: Address = 0x21; }

    /**
      ### `0x22` `ALLPOFF`  All Pixel Off
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 206
    */
    pub struct ALLPOFF;
    impl Location for ALLPOFF     { const ADDRESS: Address = 0x22; }

    /**
      ### `0x23` `ALLPON`  All Pixel ON
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 207
    */
    pub struct ALLPON;
    impl Location for ALLPON      { const ADDRESS: Address = 0x23; }

    /**
      ### `0x26` `GAMSET`  Gamma Set
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 208
    */
    pub struct GAMSET;
    impl Location for GAMSET      { const ADDRESS: Address = 0x26; }
    impl WriteData for GAMSET     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x28` `DISPOFF`  Display Off
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 209
    */
    pub struct DISPOFF;
    impl Location for DISPOFF     { const ADDRESS: Address = 0x28; }

    /**
      ### `0x29` `DISPON`  Display On
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 210
    */
    pub struct DISPON;
    impl Location for DISPON      { const ADDRESS: Address = 0x29; }

    /**
      ### `0x34` `TEOFF`  Tearing Effect Line OFF
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 211
    */
    pub struct TEOFF;
    impl Location for TEOFF       { const ADDRESS: Address = 0x34; }

    /**
      ### `0x35` `TEON`  Tearing Effect Line ON
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 212
    */
    pub struct TEON;
    impl Location for TEON        { const ADDRESS: Address = 0x35; }
    impl WriteData for TEON       { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x36` `MADCTL`  Display data access control
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 214
    */
    pub struct MADCTL;
    impl Location for MADCTL      { const ADDRESS: Address = 0x36; }
    impl WriteData for MADCTL     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x38` `IDMOFF`  Idle Mode Off
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 215
    */
    pub struct IDMOFF;
    impl Location for IDMOFF      { const ADDRESS: Address = 0x38; }

    /**
      ### `0x39` `IDMON`  Idle Mode On
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 216
    */
    pub struct IDMON;
    impl Location for IDMON       { const ADDRESS: Address = 0x39; }

    /**
      ### `0x3A` `COLMOD`  Interface Pixel Format
      > Sitronix ST7701S Datatsheet v1.2 (Oct. 2017), p. 218
    */
    pub struct COLMOD;
    impl Location for COLMOD      { const ADDRESS: Address = 0x3A; }
    impl WriteData for COLMOD     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x045` `GSL` Get Scan Line
      > v1.2, Page 219
    */
    pub struct GSL;
    impl Location for GSL         { const ADDRESS: Address = 0x45; }
    impl ReadData for GSL         { const BYTES:   Bytes   = 2;    }

    /**
      ### `0x051` `WRDISBV` Write Display Brightness
      > v1.2, Page 220
    */
    pub struct WRDISBV;
    impl Location for WRDISBV     { const ADDRESS: Address = 0x51; }
    impl WriteData for WRDISBV    { const BYTES:   Bytes   = 1;    }


/**
  ### `0x52` `RDDISBV` Read Display Brightness Value
  > v1.2, Page 221
*/
    pub struct RDDISBV;
    impl Location for RDDISBV     { const ADDRESS: Address = 0x52; }
    impl ReadData for RDDISBV     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x53` `WRCTRLD` Write CTRL Display
      > v1.2, Page 222
    */
    pub struct WRCTRLD;
    impl Location for WRCTRLD     { const ADDRESS: Address = 0x53; }
    impl WriteData for WRCTRLD    { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x54` `RDCTRLD` Read CTRL Display
      > v1.2, Page 224
    */
    pub struct RDCTRLD;
    impl Location for RDCTRLD     { const ADDRESS: Address = 0x54; }
    impl ReadData for RDCTRLD     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x55` `WRCACE` Write Content Adaptive Brightness Control and Color Enhancement
      > v1.2, Page 225
    */
    pub struct WRCACE;
    impl Location for WRCACE      { const ADDRESS: Address = 0x55; }
    impl WriteData for WRCACE     { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x56` `RDCABC` Read Content Adaptive Brightness Control
      > v1.2, Page 227
    */
    pub struct RDCABC;
    impl Location for RDCABC      { const ADDRESS: Address = 0x56; }
    impl ReadData for RDCABC      { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x5E` `WRCABCMB` Write CABC Minimum Brightness
      > v1.2, Page 229
    */
    pub struct WRCABCMB;
    impl Location for WRCABCMB    { const ADDRESS: Address = 0x5E; }
    impl WriteData for WRCABCMB   { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x5F` `RDCABCMB` Read CABC Minimum Brightness
      > v1.2, Page 230
    */
    pub struct RDCABCMB;
    impl Location for RDCABCMB    { const ADDRESS: Address = 0x5F; }
    impl ReadData for RDCABCMB    { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x68` `RDABCSDR` Read Automatic Brightness Control Self-Diagnostic Result
      > v1.2, Page 231
    */
    pub struct RDABCSDR;
    impl Location for RDABCSDR    { const ADDRESS: Address = 0x68; }
    impl ReadData for RDABCSDR    { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x70` `RDBWLB` Read Black/White Low Bits
      > v1.2, Page 232
    */
    pub struct RDBWLB;
    impl Location for RDBWLB      { const ADDRESS: Address = 0x70; }
    impl ReadData for RDBWLB      { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x71` `RDBkx` Read Bkx
      > v1.2, Page 233
    */
    pub struct RDBKX;
    impl Location for RDBKX       { const ADDRESS: Address = 0x71; }
    impl ReadData for RDBKX       { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x72` `RDBky` Read Bky
      > v1.2, Page 234
    */
    pub struct RDBKY;
    impl Location for RDBKY       { const ADDRESS: Address = 0x72; }
    impl ReadData for RDBKY       { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x73` `RDWx` Read Wx
      > v1.2, Page 235
    */
    pub struct RDWX;
    impl Location for RDWX        { const ADDRESS: Address = 0x73; }
    impl ReadData for RDWX        { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x74` `RDWy` Read Wy
      > v1.2, Page 236
    */
    pub struct RDWY;
    impl Location for RDWY        { const ADDRESS: Address = 0x74; }
    impl ReadData for RDWY        { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x75` `RDRGLB` Read Red/Green Low Bits
      > v1.2, Page 237
    */
    pub struct RDRGLB;
    impl Location for RDRGLB      { const ADDRESS: Address = 0x75; }
    impl ReadData for RDRGLB      { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x76` `RDRx` Read Rx
      > v1.2, Page 238
    */
    pub struct RDRX;
    impl Location for RDRX        { const ADDRESS: Address = 0x76; }
    impl ReadData for RDRX        { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x77` `RDRy` Read Ry
      > v1.2, Page 239
    */
    pub struct RDRY;
    impl Location for RDRY        { const ADDRESS: Address = 0x77; }
    impl ReadData for RDRY        { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x78` `RDGx` Read Gx
      > v1.2, Page 240
    */
    pub struct RDGX;
    impl Location for RDGX        { const ADDRESS: Address = 0x78; }
    impl ReadData for RDGX        { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x79` `RDGy` Read Gy
      > v1.2, Page 241
    */
    pub struct RDGY;
    impl Location for RDGY        { const ADDRESS: Address = 0x79; }
    impl ReadData for RDGY        { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x7A` `RDBALB` Read Blue/A Color Low Bits
      > v1.2, Page 242
    */
    pub struct RDBALB;
    impl Location for RDBALB      { const ADDRESS: Address = 0x7A; }
    impl ReadData for RDBALB      { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x7B` `RDBx` Read Bx
      > v1.2, Page 243
    */
    pub struct RDBX;
    impl Location for RDBX        { const ADDRESS: Address = 0x7B; }
    impl ReadData for RDBX        { const BYTES:   Bytes   = 1;    }

    /**
      ### `0x7C` `RDBy` Read By
      > v1.2, Page 244
    */

    /**
      ### `0x7D` `RDAx` Read Ax
      > v1.2, Page 245
    */

    /**
      ### `0x7E` `RDAy` Read Ay
      > v1.2, Page 246
    */

    /**
      ### `0xA1` `RDDDBS` Read DDB Start
      > v1.2, Page 247
    */

    /**
      ### `0xA8` `RDDDBC` Read DDB Continue
      > v1.2, Page 249
    */

    /**
      ### `0xAA` `RDFCS` Read First Checksum
      > v1.2, Page 250
    */

    /**
      ### `0xAF` `RDCCS` Read Continue Checksum
      > v1.2, Page 251
    */

    /**
      ### `0xDA` `RDID1` Read ID1
      > v1.2, Page 252
    */

    /**
      ### `0xDB` `RDID2` Read ID2
      > v1.2, Page 253
    */

    /**
      ### `0xDC` `RDID3` Read ID3
      > v1.2, Page 254
    */

    /**
      ### `0xFF` `CND2BKxSEL Command2 BKx Selection
      > v1.2, Page 260

      #### Write Parameters

      |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
      |:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
      |    0   |    1   |    1   |    1   |    0   |    1   |    1   |    1   |
      |    0   |    0   |    0   |    0   |    0   |    0   |    0   |    1   |
      |    0   |    0   |    0   |    0   |    0   |    0   |    0   |    0   |
      |    0   |    0   |    0   |    0   |    0   |    0   |    0   |    0   |
      |    0   |    0   |    0   |   CN2  |    0   |    0   |    0   | BKxSEL |
    */
    pub struct CND2BKXSEL;
    impl Location for CND2BKXSEL  { const ADDRESS: Address = 0xFF; }
    impl WriteData for CND2BKXSEL { const BYTES:   Bytes   = 5;    }
}

/**
 ## BK0 COMMANDS
*/
#[rustfmt::skip]
pub mod bk0 {
    use super::*;

    const BK0: Extension = Some(Bank::BK0);

    /**
      ### `0xB0` `PVGAMCTRL` Positive Voltage Gamma Control
      > v1.2, Page 261
    */
    pub struct PVGAMCTRL;
    impl Location for PVGAMCTRL  { const ADDRESS:   Address   = 0xB0;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PVGAMCTRL { const BYTES:     Bytes     = 16;  }

    /**
      ### `0xB1` `NVGAMCTRL` Negative Voltage Gamma Control
      > v1.2, Page 263
    */
    pub struct NVGAMCTRL;
    impl Location for NVGAMCTRL  { const ADDRESS:   Address   = 0xB1;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for NVGAMCTRL { const BYTES:     Bytes     = 16;  }

    /**
      ### `0xB8` `DGMEN` Digital Gamma Enable
      > v1.2, Page 265
    */
    pub struct DGMEN;
    impl Location for DGMEN      { const ADDRESS:   Address   = 0xB8;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for DGMEN     { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xB9` `DGMLUTR` Digital Gamma Look-up Table for Red
      > v1.2, Page 266
    */
    pub struct DGMLUTR;
    impl Location for DGMLUTR    { const ADDRESS:   Address   = 0xB9;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for DGMLUTR   { const BYTES:     Bytes     = 130; }

    /**
      ### `0xBA` `DGMLUTB` Digital Gamma Look-up Table for Blue
      > v1.2, Page 267
    */
    pub struct DGMLUTB;
    impl Location for DGMLUTB    { const ADDRESS:   Address   = 0xBA;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for DGMLUTB   { const BYTES:     Bytes     = 130; }

    /**
      ### `0xBC` `SEL` PWM CLK select
      > v1.2, Page 268
    */
    pub struct PWMCLK;
    impl Location for PWMCLK     { const ADDRESS:   Address   = 0xBC;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PWMCLK    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xC0` `LNESET` Display Line Setting
      > v1.2, Page 269
    */
    pub struct LNESET;
    impl Location for LNESET     { const ADDRESS:   Address   = 0xC0;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for LNESET    { const BYTES:     Bytes     = 2;   }

    /**
      ### `0xC1` `PORCTRL` Porch Control
      > v1.2, Page 270
    */
    pub struct PORCTRL;
    impl Location for PORCTRL    { const ADDRESS:   Address   = 0xC1;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PORCTRL   { const BYTES:     Bytes     = 2;   }

    /**
      ### `0xC2` `INVSE` Inversion selection & Frame Rate Control
      > v1.2, Page 271
    */
    pub struct INVSET;
    impl Location for INVSET     { const ADDRESS:   Address   = 0xC2;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for INVSET    { const BYTES:     Bytes     = 2;   }

    /**
      ### `0xC3` `RGBCTRL` RGB control
      > v1.2, Page 272
    */
    pub struct RGBCTRL;
    impl Location for RGBCTRL    { const ADDRESS:   Address   = 0xC3;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for RGBCTRL   { const BYTES:     Bytes     = 3;   }

    /**
      ### `0xC5` `PARCTRL` Partial Mode Control
      > v1.2, Page 273
    */
    pub struct PARCTRL;
    impl Location for PARCTRL    { const ADDRESS:   Address   = 0xC5;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PARCTRL   { const BYTES:     Bytes     = 4;   }

    /**
      ### `0xC7` `SDIR` X-direction Control
      > v1.2, Page 274
    */
    pub struct SDIR;
    impl Location for SDIR       { const ADDRESS:   Address   = 0xC7;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SDIR      { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xC8` `PDOSET` Pseudo-Dot inversion diving setting
      > v1.2, Page 275
    */
    pub struct PDOSET;
    impl Location for PDOSET     { const ADDRESS:   Address   = 0xC8;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PDOSET    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xCD` `COLCTRL` Color Control
      > v1.2, Page 276
    */
    pub struct COLCTRL;
    impl Location for COLCTRL    { const ADDRESS:   Address   = 0xCD;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for COLCTRL   { const BYTES:     Bytes     = 1;   }

    pub struct SSCTRL;
    impl Location for SSCTRL     { const ADDRESS:   Address   = 0xCE;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SSCTRL    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE0` `SECTRL` Sunlight Readable Enhancement
      > v1.2, Page 278
    */
    pub struct SRECTRL;
    impl Location for SRECTRL    { const ADDRESS:   Address   = 0xE0;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SRECTRL   { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE1` `NRCTRL` Noise Reduce Control
      > v1.2, Page 279
    */
    pub struct NRCTRL;
    impl Location for NRCTRL     { const ADDRESS:   Address   = 0xE1;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for NRCTRL    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE2` `SECTRL` Sharpness Control
      > v1.2, Page 280
    */
    pub struct SECTRL;
    impl Location for SECTRL     { const ADDRESS:   Address   = 0xE2;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SECTRL    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE3` `CCCTRL` Color Calibration Control
      > v1.2, Page 281
    */
    pub struct CCCTRL;
    impl Location for CCCTRL     { const ADDRESS:   Address   = 0xE3;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for CCCTRL    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE4` `SKCTRL` Skin Tone Preservation Control
      > v1.2, Page 282
    */
    pub struct SKCTRL;
    impl Location for SKCTRL     { const ADDRESS:   Address   = 0xE4;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SKCTRL    { const BYTES:     Bytes     = 1;   }

    pub struct NVMSETE;
    impl Location for NVMSETE    { const ADDRESS:   Address   = 0xEA;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for NVMSETE   { const BYTES:     Bytes     = 1;   }

    pub struct CABCCTRL;
    impl Location for CABCCTRL   { const ADDRESS:   Address   = 0xEE;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for CABCCTRL  { const BYTES:     Bytes     = 1;   }
}

/**
 ## BK1 COMMANDS
*/
#[rustfmt::skip]
pub mod bk1 {
    use super::*;

    const BK1: Extension = Some(Bank::BK1);

    pub struct VCOMS;
    impl Location for VCOMS      { const ADDRESS:   Address   = 0xB1;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for VCOMS     { const BYTES:     Bytes     = 1;   }

    pub struct VGHSS;
    impl Location for VGHSS      { const ADDRESS:   Address   = 0xB2;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for VGHSS     { const BYTES:     Bytes     = 1;   }

    pub struct TESTCMD;
    impl Location for TESTCMD    { const ADDRESS:   Address   = 0xB3;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for TESTCMD   { const BYTES:     Bytes     = 1;   }

    pub struct VGLS;
    impl Location for VGLS       { const ADDRESS:   Address   = 0xB5;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for VGLS      { const BYTES:     Bytes     = 1;   }

    pub struct PWCTRL1;
    impl Location for PWCTRL1    { const ADDRESS:   Address   = 0xB7;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for PWCTRL1   { const BYTES:     Bytes     = 1;   }

    pub struct PWCTRL2;
    impl Location for PWCTRL2    { const ADDRESS:   Address   = 0xB8;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for PWCTRL2   { const BYTES:     Bytes     = 1;   }

    pub struct PCLKS1;
    impl Location for PCLKS1     { const ADDRESS:   Address   = 0xBA;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for PCLKS1    { const BYTES:     Bytes     = 1;   }

    pub struct PCLKS3;
    impl Location for PCLKS3     { const ADDRESS:   Address   = 0xBC;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for PCLKS3    { const BYTES:     Bytes     = 1;   }

    pub struct SPD1;
    impl Location for SPD1       { const ADDRESS:   Address   = 0xC1;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for SPD1      { const BYTES:     Bytes     = 1;   }

    pub struct SPD2;
    impl Location for SPD2       { const ADDRESS:   Address   = 0xC2;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for SPD2      { const BYTES:     Bytes     = 1;   }

    pub struct MIPISET1;
    impl Location for MIPISET1   { const ADDRESS:   Address   = 0xD0;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for MIPISET1  { const BYTES:     Bytes     = 1;   }

    pub struct MIPISET2;
    impl Location for MIPISET2   { const ADDRESS:   Address   = 0xD1;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for MIPISET2  { const BYTES:     Bytes     = 4;   }

    pub struct MIPISET3;
    impl Location for MIPISET3   { const ADDRESS:   Address   = 0xD2;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for MIPISET3  { const BYTES:     Bytes     = 1;   }

    pub struct MIPISET4;
    impl Location for MIPISET4   { const ADDRESS:   Address   = 0xD3;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for MIPISET4  { const BYTES:     Bytes     = 2;   }

    pub struct NVMEN;
    impl Location for NVMEN      { const ADDRESS:   Address   = 0xC8;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for NVMEN     { const BYTES:     Bytes     = 4;   }

    pub struct NVMSET;
    impl Location for NVMSET     { const ADDRESS:   Address   = 0xCA;
                                   const EXTENSION: Extension = BK1; }
    impl WriteData for NVMSET    { const BYTES:     Bytes     = 3;   }

}

/**
 ## BK3 COMMANDS
*/
pub mod bk3 {}
