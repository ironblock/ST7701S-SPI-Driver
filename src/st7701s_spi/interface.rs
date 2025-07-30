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
    const INITIAL: Self::Parameters;

    type Parameters;
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

/**
 * # System Commands
 *
 * Unless otherwise noted, any given page reference refers to the confidential
 * Sitronix ST7701S Datatsheet v1.2 (Oct. 2017).
 */
pub mod core {
    use crate::st7701s_spi::{
        parameters::{data_access, gamma, pixel_format, tearing_effect},
        state::Toggle,
    };

    use super::*;

    /**
      ### `0x00` `NOP`  No Operation
      > Reference: p. 187
    */
    pub struct NOP;
    impl Location for NOP {
        const ADDRESS: u8 = 0x00;
    }

    /**
      ### `0x01` `SWRESET`  Software Reset

      > Reference: p. 188
    */
    pub struct SWRESET;
    impl Location for SWRESET {
        const ADDRESS: u8 = 0x01;
    }
    impl WriteData for SWRESET {
        const BYTES: Bytes = 1;
        const INITIAL: Self::Parameters = ();

        type Parameters = ();
    }
    impl SWRESET {
        /**
            #### `SWRESET` Write Parameters

            It's never stated anywhere why D0 is 1, but it's indicated in both the
            primary reference table on p. 184 and again on SWRESET's detail page.

            As an additional contradiction, p. 184 refers to SWRESET as a **command**
            (with no arguments), and p. 188 refers to it as a **write**. As only a
            write can have arguments and 0x01 is the constant argument in both
            references, SWRESET's canonical representation here is as a **write**.

            |    |   D7  |   D6  |   D5  |   D4  |   D3  |   D2  |   D1  |   D0  |
            |:--:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
            | P1 |   0   |   0   |   0   |   0   |   0   |   0   |   0   |   1   |
        */
        pub const fn encode_data() -> Buffer<{ Self::BYTES }> {
            const P1: u8 = 0b0000_0001;
            [P1]
        }
    }

    /**
      ### `0x04` `RDDID`  Read Display ID
      > Reference: p. 189
    */
    pub struct RDDID;
    impl Location for RDDID {
        const ADDRESS: Address = 0x04;
    }
    impl ReadData for RDDID {
        const BYTES: Bytes = 4;
    }
    impl RDDID {
        /**
            #### `RDDID` Read Parameters

            |    |   D7  |   D6  |   D5  |   D4  |   D3  |   D2  |   D1  |   D0  |
            |:--:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
            | P1 |   -   |   -   |   -   |   -   |   -   |   -   |   -   |   -   |
            | P2 |   *   |   *   |   *   |   *   |   *   |   *   |   *   |   *   |
            | P3 |   *   |   *   |   *   |   *   |   *   |   *   |   *   |   *   |
            | P4 |   *   |   *   |   *   |   *   |   *   |   *   |   *   |   *   |
        */
        const fn decode(_response: &[u8]) -> Buffer<{ Self::BYTES }> {
            // P1 - IGNORE
            // P2 - Manufacturer ID
            // P2 - Version ID
            // P3 - Module ID
            todo!()
        }
    }

    /**
      ### `0x05` `RDNUMED`  Read Number of Errors on DSI

      Only relevant for MIPI interfaces, not implemented here.

      > Reference: p. 190
    */
    pub struct RDNUMED;
    impl Location for RDNUMED {
        const ADDRESS: Address = 0x05;
    }

    /**
      ### `0x06` `RDRED`  Read the first pixel of Red Color
      > Reference: p. 191
    */
    pub struct RDRED;
    impl Location for RDRED {
        const ADDRESS: Address = 0x06;
    }
    impl ReadData for RDRED {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x07` `RDGREEN`  Read the first pixel of Green Color
      > Reference: p. 192
    */
    pub struct RDGREEN;
    impl Location for RDGREEN {
        const ADDRESS: Address = 0x07;
    }
    impl ReadData for RDGREEN {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x08` `RDBLUE`  Read the first pixel of Blue Color
      > Reference: p. 193
    */
    pub struct RDBLUE;
    impl Location for RDBLUE {
        const ADDRESS: Address = 0x08;
    }
    impl ReadData for RDBLUE {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x0A` `RDDPM`  Read Display Power Mode
      > Reference: p. 194
    */
    pub struct RDDPM;
    impl Location for RDDPM {
        const ADDRESS: Address = 0x0A;
    }
    impl ReadData for RDDPM {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x0B` `RDDMADCTL`  Read Display MADCTL
      > Reference: p. 195
    */
    pub struct RDDMADCTL;
    impl Location for RDDMADCTL {
        const ADDRESS: Address = 0x0B;
    }
    impl ReadData for RDDMADCTL {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x0C` `RDDCOLMOD`  Read Display Pixel Format
      > Reference: p. 196
    */
    pub struct RDDCOLMOD;
    impl Location for RDDCOLMOD {
        const ADDRESS: Address = 0x0C;
    }
    impl ReadData for RDDCOLMOD {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x0D` `RDDIM`  Read Display Image Mode
      > Reference: p. 197
    */
    pub struct RDDIM;
    impl Location for RDDIM {
        const ADDRESS: Address = 0x0D;
    }
    impl ReadData for RDDIM {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x0E` `RDDSM`  Read Display Signal Mode
      > Reference: p. 198
    */
    pub struct RDDSM;
    impl Location for RDDSM {
        const ADDRESS: Address = 0x0E;
    }
    impl ReadData for RDDSM {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x0F` `RDDSDR`  Read Display Self-Diagnostic Result
      > Reference: p. 199
    */
    pub struct RDDSDR;
    impl Location for RDDSDR {
        const ADDRESS: Address = 0x0F;
    }
    impl ReadData for RDDSDR {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x10` `SLPIN`  Sleep in
      > Reference: p. 200
    */
    pub struct SLPIN;
    impl Location for SLPIN {
        const ADDRESS: Address = 0x10;
    }

    /**
      ### `0x11` `SLPOUT`  Sleep Out
      > Reference: p. 201
    */
    pub struct SLPOUT;
    impl Location for SLPOUT {
        const ADDRESS: Address = 0x11;
    }

    /**
      ### `0x12` `PTLON`  Partial Display Mode On
      > Reference: p. 202
    */
    pub struct PTLON;
    impl Location for PTLON {
        const ADDRESS: Address = 0x12;
    }

    /**
      ### `0x13` `NORON`  Normal Display Mode On
      > Reference: p. 203
    */
    pub struct NORON;
    impl Location for NORON {
        const ADDRESS: Address = 0x13;
    }

    /**
      ### `0x20` `INVOFF`  Display Inversion Off
      > Reference: p. 204
    */
    pub struct INVOFF;
    impl Location for INVOFF {
        const ADDRESS: Address = 0x20;
    }

    /**
      ### `0x21` `INVON`  Display Inversion On
      > Reference: p. 205
    */
    pub struct INVON;
    impl Location for INVON {
        const ADDRESS: Address = 0x21;
    }

    /**
      ### `0x22` `ALLPOFF`  All Pixel Off
      > Reference: p. 206
    */
    pub struct ALLPOFF;
    impl Location for ALLPOFF {
        const ADDRESS: Address = 0x22;
    }

    /**
      ### `0x23` `ALLPON`  All Pixel ON
      > Reference: p. 207
    */
    pub struct ALLPON;
    impl Location for ALLPON {
        const ADDRESS: Address = 0x23;
    }

    /**
      ### `0x26` `GAMSET`  Gamma Set
      > Reference: p. 208
    */
    pub struct GAMSET;
    impl Location for GAMSET {
        const ADDRESS: Address = 0x26;
    }
    impl WriteData for GAMSET {
        const BYTES: Bytes = 1;
        const INITIAL: Self::Parameters = (gamma::Curve::GC1,);
        type Parameters = (gamma::Curve,);
    }
    impl GAMSET {
        /**
            #### Write Parameters

            Curve 1: G=2.2
            Curve 2: Reserved
            Curve 3: Reserved
            Curve 4: Reserved

            |    |   D7  |   D6  |   D5  |   D4  |   D3  |   D2  |   D1  |   D0  |
            |:--:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
            | P1 |   -   |   -   |   -   |   -   | GC[3] | GC[2] | GC[1] | GC[0] |
        */
        pub const fn encode_data(
            (gc,): <Self as WriteData>::Parameters,
        ) -> Buffer<{ Self::BYTES }> {
            let gc_data = match gc {
                gamma::Curve::GC1 => 0x01,
                gamma::Curve::GC2 => 0x02,
                gamma::Curve::GC3 => 0x04,
                gamma::Curve::GC4 => 0x08,
            };

            [gc_data]
        }
    }

    /**
      ### `0x28` `DISPOFF`  Display Off
      > Reference: p. 209
    */
    pub struct DISPOFF;
    impl Location for DISPOFF {
        const ADDRESS: Address = 0x28;
    }

    /**
      ### `0x29` `DISPON`  Display On
      > Reference: p. 210
    */
    pub struct DISPON;
    impl Location for DISPON {
        const ADDRESS: Address = 0x29;
    }

    /**
      ### `0x34` `TEOFF`  Tearing Effect Line OFF
      > Reference: p. 211
    */
    pub struct TEOFF;
    impl Location for TEOFF {
        const ADDRESS: Address = 0x34;
    }

    /**
      ### `0x35` `TEON`  Tearing Effect Line ON
      > Reference: p. 212
    */
    pub struct TEON;
    impl Location for TEON {
        const ADDRESS: Address = 0x35;
    }
    impl WriteData for TEON {
        const BYTES: Bytes = 1;
        const INITIAL: Self::Parameters = (tearing_effect::Signal::VBlank,);
        type Parameters = (tearing_effect::Signal,);
    }
    impl TEON {
        /**
            #### Write Parameters

            0: V-Blanking
            1: VH-Blanking

            NOTE: This mode can also be disabled with TEOFF

            |    |   D7  |   D6  |   D5  |   D4  |   D3  |   D2  |   D1  |   D0  |
            |:--:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
            | P1 |   -   |   -   |   -   |   -   |   -   |   -   |   -   |   TE  |
        */
        pub const fn encode_data(
            (te,): <Self as WriteData>::Parameters,
        ) -> Buffer<{ Self::BYTES }> {
            let te_data = match te {
                tearing_effect::Signal::VBlank => 0,
                tearing_effect::Signal::VHBlank => 1,
            };

            [te_data]
        }
    }

    /**
      ### `0x36` `MADCTL`  Display data access control
      > Reference: p. 214
    */
    pub struct MADCTL;
    impl Location for MADCTL {
        const ADDRESS: Address = 0x36;
    }
    /**
        #### Write Parameters

        ML:
          0: Normal
          1: Reverse
        CO:
          0: RGB
          1: BGR

        NOTE: This mode can also be disabled with TEOFF

        |    |   D7  |   D6  |   D5  |   D4  |   D3  |   D2  |   D1  |   D0  |
        |:--:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
        | P1 |   -   |   -   |   -   |   ML  |   CO  |   -   |   -   |   -   |
    */
    impl WriteData for MADCTL {
        const BYTES: Bytes = 1;
        const INITIAL: Self::Parameters = (
            data_access::ScanDirection::Normal,
            data_access::ColorOrder::Rgb,
        );
        type Parameters = (data_access::ScanDirection, data_access::ColorOrder);
    }
    impl MADCTL {
        pub const fn encode_data(
            (ml, co): <Self as WriteData>::Parameters,
        ) -> Buffer<{ Self::BYTES }> {
            let p1_scan = match ml {
                data_access::ScanDirection::Normal => 0,
                data_access::ScanDirection::Reverse => 1 << 4,
            };
            let p1_color = match co {
                data_access::ColorOrder::Rgb => 0,
                data_access::ColorOrder::Bgr => 1 << 3,
            };

            [p1_scan | p1_color]
        }
    }

    /**
      ### `0x38` `IDMOFF`  Idle Mode Off
      > Reference: p. 215
    */
    pub struct IDMOFF;
    impl Location for IDMOFF {
        const ADDRESS: Address = 0x38;
    }

    /**
      ### `0x39` `IDMON`  Idle Mode On
      > Reference: p. 216
    */
    pub struct IDMON;
    impl Location for IDMON {
        const ADDRESS: Address = 0x39;
    }

    /**
      ### `0x3A` `COLMOD`  Interface Pixel Format
      > Reference: p. 218
    */
    pub struct COLMOD;
    impl Location for COLMOD {
        const ADDRESS: Address = 0x3A;
    }
    impl WriteData for COLMOD {
        const BYTES: Bytes = 1;
        const INITIAL: Self::Parameters = (pixel_format::BitsPerPixel::RGB888,);
        type Parameters = (pixel_format::BitsPerPixel,);
    }
    impl COLMOD {
        pub const fn encode_data(
            (bpp,): <Self as WriteData>::Parameters,
        ) -> Buffer<{ Self::BYTES }> {
            let bpp_data = match bpp {
                pixel_format::BitsPerPixel::RGB565 => 101 << 4,
                pixel_format::BitsPerPixel::RGB666 => 110 << 4,
                pixel_format::BitsPerPixel::RGB888 => 111 << 4,
            };

            [bpp_data]
        }
    }

    /**
      ### `0x045` `GSL` Get Scan Line
      > See p. 219
    */
    pub struct GSL;
    impl Location for GSL {
        const ADDRESS: Address = 0x45;
    }
    impl ReadData for GSL {
        const BYTES: Bytes = 2;
    }

    /**
      ### `0x051` `WRDISBV` Write Display Brightness
      > See p. 220
    */
    pub struct WRDISBV;
    impl Location for WRDISBV {
        const ADDRESS: Address = 0x51;
    }
    impl WriteData for WRDISBV {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x52` `RDDISBV` Read Display Brightness Value
      > See p. 221
    */
    pub struct RDDISBV;
    impl Location for RDDISBV {
        const ADDRESS: Address = 0x52;
    }
    impl ReadData for RDDISBV {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x53` `WRCTRLD` Write CTRL Display
      > See p. 222
    */
    pub struct WRCTRLD;
    impl Location for WRCTRLD {
        const ADDRESS: Address = 0x53;
    }
    impl WriteData for WRCTRLD {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x54` `RDCTRLD` Read CTRL Display
      > See p. 224
    */
    pub struct RDCTRLD;
    impl Location for RDCTRLD {
        const ADDRESS: Address = 0x54;
    }
    impl ReadData for RDCTRLD {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x55` `WRCACE` Write Content Adaptive Brightness Control and Color Enhancement
      > See p. 225
    */
    pub struct WRCACE;
    impl Location for WRCACE {
        const ADDRESS: Address = 0x55;
    }
    impl WriteData for WRCACE {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x56` `RDCABC` Read Content Adaptive Brightness Control
      > See p. 227
    */
    pub struct RDCABC;
    impl Location for RDCABC {
        const ADDRESS: Address = 0x56;
    }
    impl ReadData for RDCABC {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x5E` `WRCABCMB` Write CABC Minimum Brightness
      > See p. 229
    */
    pub struct WRCABCMB;
    impl Location for WRCABCMB {
        const ADDRESS: Address = 0x5E;
    }
    impl WriteData for WRCABCMB {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x5F` `RDCABCMB` Read CABC Minimum Brightness
      > See p. 230
    */
    pub struct RDCABCMB;
    impl Location for RDCABCMB {
        const ADDRESS: Address = 0x5F;
    }
    impl ReadData for RDCABCMB {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x68` `RDABCSDR` Read Automatic Brightness Control Self-Diagnostic Result
      > See p. 231
    */
    pub struct RDABCSDR;
    impl Location for RDABCSDR {
        const ADDRESS: Address = 0x68;
    }
    impl ReadData for RDABCSDR {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x70` `RDBWLB` Read Black/White Low Bits
      > See p. 232
    */
    pub struct RDBWLB;
    impl Location for RDBWLB {
        const ADDRESS: Address = 0x70;
    }
    impl ReadData for RDBWLB {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x71` `RDBkx` Read Bkx
      > See p. 233
    */
    pub struct RDBKX;
    impl Location for RDBKX {
        const ADDRESS: Address = 0x71;
    }
    impl ReadData for RDBKX {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x72` `RDBky` Read Bky
      > See p. 234
    */
    pub struct RDBKY;
    impl Location for RDBKY {
        const ADDRESS: Address = 0x72;
    }
    impl ReadData for RDBKY {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x73` `RDWx` Read Wx
      > See p. 235
    */
    pub struct RDWX;
    impl Location for RDWX {
        const ADDRESS: Address = 0x73;
    }
    impl ReadData for RDWX {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x74` `RDWy` Read Wy
      > See p. 236
    */
    pub struct RDWY;
    impl Location for RDWY {
        const ADDRESS: Address = 0x74;
    }
    impl ReadData for RDWY {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x75` `RDRGLB` Read Red/Green Low Bits
      > See p. 237
    */
    pub struct RDRGLB;
    impl Location for RDRGLB {
        const ADDRESS: Address = 0x75;
    }
    impl ReadData for RDRGLB {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x76` `RDRx` Read Rx
      > See p. 238
    */
    pub struct RDRX;
    impl Location for RDRX {
        const ADDRESS: Address = 0x76;
    }
    impl ReadData for RDRX {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x77` `RDRy` Read Ry
      > See p. 239
    */
    pub struct RDRY;
    impl Location for RDRY {
        const ADDRESS: Address = 0x77;
    }
    impl ReadData for RDRY {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x78` `RDGx` Read Gx
      > See p. 240
    */
    pub struct RDGX;
    impl Location for RDGX {
        const ADDRESS: Address = 0x78;
    }
    impl ReadData for RDGX {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x79` `RDGy` Read Gy
      > See p. 241
    */
    pub struct RDGY;
    impl Location for RDGY {
        const ADDRESS: Address = 0x79;
    }
    impl ReadData for RDGY {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x7A` `RDBALB` Read Blue/A Color Low Bits
      > See p. 242
    */
    pub struct RDBALB;
    impl Location for RDBALB {
        const ADDRESS: Address = 0x7A;
    }
    impl ReadData for RDBALB {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x7B` `RDBx` Read Bx
      > See p. 243
    */
    pub struct RDBX;
    impl Location for RDBX {
        const ADDRESS: Address = 0x7B;
    }
    impl ReadData for RDBX {
        const BYTES: Bytes = 1;
    }

    /**
      ### `0x7C` `RDBy` Read By
      > See p. 244
    */
    pub struct RDBy;
    impl Location for RDBy {
        const ADDRESS: u8 = 0x7C;
    }

    /**
      ### `0x7D` `RDAx` Read Ax
      > See p. 245
    */
    pub struct RDAx;
    impl Location for RDAx {
        const ADDRESS: u8 = 0x7D;
    }

    /**
      ### `0x7E` `RDAy` Read Ay
      > See p. 246
    */
    pub struct RDAy;
    impl Location for RDAy {
        const ADDRESS: u8 = 0x7E;
    }

    /**
      ### `0xA1` `RDDDBS` Read DDB Start
      > See p. 247
    */
    pub struct RDDDBS;
    impl Location for RDDDBS {
        const ADDRESS: u8 = 0xA1;
    }

    /**
      ### `0xA8` `RDDDBC` Read DDB Continue
      > See p. 249
    */
    pub struct RDDDBC;
    impl Location for RDDDBC {
        const ADDRESS: u8 = 0xA8;
    }

    /**
      ### `0xAA` `RDFCS` Read First Checksum
      > See p. 250
    */
    pub struct RDFCS;
    impl Location for RDFCS {
        const ADDRESS: u8 = 0xAA;
    }

    /**
      ### `0xAF` `RDCCS` Read Continue Checksum
      > See p. 251
    */
    pub struct RDCCS;
    impl Location for RDCCS {
        const ADDRESS: u8 = 0xAF;
    }

    /**
      ### `0xDA` `RDID1` Read ID1
      > See p. 252
    */
    pub struct RDID1;
    impl Location for RDID1 {
        const ADDRESS: u8 = 0xDA;
    }

    /**
      ### `0xDB` `RDID2` Read ID2
      > See p. 253
    */
    pub struct RDID2;
    impl Location for RDID2 {
        const ADDRESS: u8 = 0xDB;
    }

    /**
      ### `0xDC` `RDID3` Read ID3
      > See p. 254
    */
    pub struct RDID3;
    impl Location for RDID3 {
        const ADDRESS: u8 = 0xDC;
    }

    /**
      ### `0xFF` `CND2BKxSEL Command2 BKx Selection
      > See p. 260

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
    impl Location for CND2BKXSEL {
        const ADDRESS: Address = 0xFF;
    }
    impl WriteData for CND2BKXSEL {
        const BYTES: Bytes = 5;
        const INITIAL: Self::Parameters = (Toggle::Off, Bank::BK0);

        type Parameters = (Toggle, Bank);
    }
    impl CND2BKXSEL {
        pub const fn encode_data(
            (cn2, bkxsel): <Self as WriteData>::Parameters,
        ) -> Buffer<{ Self::BYTES }> {
            const P1: u8 = 0b0111_0111;
            const P2: u8 = 0b0000_0001;
            const P3: u8 = 0b0000_0000;
            const P4: u8 = 0b0000_0000;

            let p5_toggle: u8 = match cn2 {
                Toggle::Off => 0b0000_0000,
                Toggle::On => 0b0001_0000,
            };

            let p5_bank: u8 = match bkxsel {
                Bank::BK0 => 0b0000_0000,
                Bank::BK1 => 0b0000_0001,
                Bank::BK3 => 0b0000_0011,
            };

            [P1, P2, P3, P4, p5_toggle | p5_bank]
        }
    }
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
      > See p. 261
    */
    pub struct PVGAMCTRL;
    impl Location for PVGAMCTRL  { const ADDRESS:   Address   = 0xB0;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PVGAMCTRL { const BYTES:     Bytes     = 16;  }

    /**
      ### `0xB1` `NVGAMCTRL` Negative Voltage Gamma Control
      > See p. 263
    */
    pub struct NVGAMCTRL;
    impl Location for NVGAMCTRL  { const ADDRESS:   Address   = 0xB1;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for NVGAMCTRL { const BYTES:     Bytes     = 16;  }

    /**
      ### `0xB8` `DGMEN` Digital Gamma Enable
      > See p. 265
    */
    pub struct DGMEN;
    impl Location for DGMEN      { const ADDRESS:   Address   = 0xB8;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for DGMEN     { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xB9` `DGMLUTR` Digital Gamma Look-up Table for Red
      > See p. 266
    */
    pub struct DGMLUTR;
    impl Location for DGMLUTR    { const ADDRESS:   Address   = 0xB9;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for DGMLUTR   { const BYTES:     Bytes     = 130; }

    /**
      ### `0xBA` `DGMLUTB` Digital Gamma Look-up Table for Blue
      > See p. 267
    */
    pub struct DGMLUTB;
    impl Location for DGMLUTB    { const ADDRESS:   Address   = 0xBA;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for DGMLUTB   { const BYTES:     Bytes     = 130; }

    /**
      ### `0xBC` `SEL` PWM CLK select
      > See p. 268
    */
    pub struct PWMCLK;
    impl Location for PWMCLK     { const ADDRESS:   Address   = 0xBC;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PWMCLK    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xC0` `LNESET` Display Line Setting
      > See p. 269
    */
    pub struct LNESET;
    impl Location for LNESET     { const ADDRESS:   Address   = 0xC0;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for LNESET    { const BYTES:     Bytes     = 2;   }

    /**
      ### `0xC1` `PORCTRL` Porch Control
      > See p. 270
    */
    pub struct PORCTRL;
    impl Location for PORCTRL    { const ADDRESS:   Address   = 0xC1;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PORCTRL   { const BYTES:     Bytes     = 2;   }

    /**
      ### `0xC2` `INVSE` Inversion selection & Frame Rate Control
      > See p. 271
    */
    pub struct INVSET;
    impl Location for INVSET     { const ADDRESS:   Address   = 0xC2;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for INVSET    { const BYTES:     Bytes     = 2;   }

    /**
      ### `0xC3` `RGBCTRL` RGB control
      > See p. 272
    */
    pub struct RGBCTRL;
    impl Location for RGBCTRL    { const ADDRESS:   Address   = 0xC3;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for RGBCTRL   { const BYTES:     Bytes     = 3;   }

    /**
      ### `0xC5` `PARCTRL` Partial Mode Control
      > See p. 273
    */
    pub struct PARCTRL;
    impl Location for PARCTRL    { const ADDRESS:   Address   = 0xC5;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PARCTRL   { const BYTES:     Bytes     = 4;   }

    /**
      ### `0xC7` `SDIR` X-direction Control
      > See p. 274
    */
    pub struct SDIR;
    impl Location for SDIR       { const ADDRESS:   Address   = 0xC7;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SDIR      { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xC8` `PDOSET` Pseudo-Dot inversion diving setting
      > See p. 275
    */
    pub struct PDOSET;
    impl Location for PDOSET     { const ADDRESS:   Address   = 0xC8;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for PDOSET    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xCD` `COLCTRL` Color Control
      > See p. 276
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
      > See p. 278
    */
    pub struct SRECTRL;
    impl Location for SRECTRL    { const ADDRESS:   Address   = 0xE0;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SRECTRL   { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE1` `NRCTRL` Noise Reduce Control
      > See p. 279
    */
    pub struct NRCTRL;
    impl Location for NRCTRL     { const ADDRESS:   Address   = 0xE1;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for NRCTRL    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE2` `SECTRL` Sharpness Control
      > See p. 280
    */
    pub struct SECTRL;
    impl Location for SECTRL     { const ADDRESS:   Address   = 0xE2;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for SECTRL    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE3` `CCCTRL` Color Calibration Control
      > See p. 281
    */
    pub struct CCCTRL;
    impl Location for CCCTRL     { const ADDRESS:   Address   = 0xE3;
                                   const EXTENSION: Extension = BK0; }
    impl WriteData for CCCTRL    { const BYTES:     Bytes     = 1;   }

    /**
      ### `0xE4` `SKCTRL` Skin Tone Preservation Control
      > See p. 282
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
