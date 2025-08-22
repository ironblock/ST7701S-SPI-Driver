use std::fmt::Debug;

use crate::st7701s_spi::parameters::register::{Address, Extension};

pub type Bytes = usize;

pub type Buffer<const N: usize> = [u8; N];
pub type Reader = for<'a> fn(&'a [u8]);

pub trait Command: Sized + Debug {
    const NAME: &str;
    const ADDRESS: Address;
    const EXTENSION: Extension = Extension(None);

    fn print_id_tag() -> String {
        format!("[{} ({}{})]", Self::NAME, Self::EXTENSION, Self::ADDRESS)
    }
}

pub trait Data: Command {
    type Parameters: Debug + PartialEq;
    type Packets: Ord + IntoIterator<Item = u8>;
}

pub trait WriteData: Data {
    fn encode(parameters: &Self::Parameters) -> Self::Packets;
}

pub trait ReadData: Data {
    fn decode(packets: &Self::Packets) -> Self::Parameters;
}

/**
 * # System Commands
 *
 * Unless otherwise noted, any given page reference refers to the confidential
 * Sitronix ST7701S Datatsheet v1.2 (Oct. 2017).
 */
pub mod core {
    use crate::st7701s_spi::{
        parameters::{data_access, gamma, pixel_format, register::Bank, tearing_effect},
        state::Switch,
    };

    use super::*;

    /**
      ### `0x00` `NOP`  No Operation
      > Reference: p. 187
    */
    #[derive(Debug)]
    pub struct NOP;
    impl Command for NOP {
        const NAME: &str = "NOP";
        const ADDRESS: Address = Address(0x00);
    }

    /**
      ### `0x01` `SWRESET`  Software Reset

      > Reference: p. 188
    */
    #[derive(Debug)]
    pub struct SWRESET;
    impl Command for SWRESET {
        const NAME: &str = "SWRESET";
        const ADDRESS: Address = Address(0x01);
    }
    impl Data for SWRESET {
        type Parameters = ();
        type Packets = Buffer<1>;
    }
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
    impl WriteData for SWRESET {
        fn encode(_: &Self::Parameters) -> Self::Packets {
            const P1: u8 = 0b0000_0001;

            [P1]
        }
    }

    /**
      ### `0x04` `RDDID`  Read Display ID
      > Reference: p. 189
    */
    #[derive(Debug)]
    pub struct RDDID;
    impl Command for RDDID {
        const NAME: &str = "RDDID";
        const ADDRESS: Address = Address(0x04);
    }
    impl Data for RDDID {
        type Parameters = ();
        type Packets = Buffer<4>;
    }
    impl ReadData for RDDID {
        /**
            #### `RDDID` Read Parameters

            |    |   D7  |   D6  |   D5  |   D4  |   D3  |   D2  |   D1  |   D0  |
            |:--:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
            | P1 |   -   |   -   |   -   |   -   |   -   |   -   |   -   |   -   |
            | P2 |   *   |   *   |   *   |   *   |   *   |   *   |   *   |   *   |
            | P3 |   *   |   *   |   *   |   *   |   *   |   *   |   *   |   *   |
            | P4 |   *   |   *   |   *   |   *   |   *   |   *   |   *   |   *   |
        */
        fn decode(_: &Self::Packets) -> Self::Parameters {
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
    #[derive(Debug)]
    pub struct RDNUMED;
    impl Command for RDNUMED {
        const NAME: &str = "RDNUMED";
        const ADDRESS: Address = Address(0x05);
    }

    /**
      ### `0x06` `RDRED`  Read the first pixel of Red Color
      > Reference: p. 191
    */
    #[derive(Debug)]
    pub struct RDRED;
    impl Command for RDRED {
        const NAME: &str = "RDRED";
        const ADDRESS: Address = Address(0x06);
    }
    impl Data for RDRED {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x07` `RDGREEN`  Read the first pixel of Green Color
      > Reference: p. 192
    */
    #[derive(Debug)]
    pub struct RDGREEN;
    impl Command for RDGREEN {
        const NAME: &str = "RDGREEN";
        const ADDRESS: Address = Address(0x07);
    }
    impl Data for RDGREEN {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x08` `RDBLUE`  Read the first pixel of Blue Color
      > Reference: p. 193
    */
    #[derive(Debug)]
    pub struct RDBLUE;
    impl Command for RDBLUE {
        const NAME: &str = "RDBLUE";
        const ADDRESS: Address = Address(0x08);
    }
    impl Data for RDBLUE {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x0A` `RDDPM`  Read Display Power Mode
      > Reference: p. 194
    */
    #[derive(Debug)]
    pub struct RDDPM;
    impl Command for RDDPM {
        const NAME: &str = "RDDPM";
        const ADDRESS: Address = Address(0x0A);
    }
    impl Data for RDDPM {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x0B` `RDDMADCTL`  Read Display MADCTL
      > Reference: p. 195
    */
    #[derive(Debug)]
    pub struct RDDMADCTL;
    impl Command for RDDMADCTL {
        const NAME: &str = "RDDMADCTL";
        const ADDRESS: Address = Address(0x0B);
    }
    impl Data for RDDMADCTL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x0C` `RDDCOLMOD`  Read Display Pixel Format
      > Reference: p. 196
    */
    #[derive(Debug)]
    pub struct RDDCOLMOD;
    impl Command for RDDCOLMOD {
        const NAME: &str = "RDDCOLMOD";
        const ADDRESS: Address = Address(0x0C);
    }
    impl Data for RDDCOLMOD {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x0D` `RDDIM`  Read Display Image Mode
      > Reference: p. 197
    */
    #[derive(Debug)]
    pub struct RDDIM;
    impl Command for RDDIM {
        const NAME: &str = "RDDIM";
        const ADDRESS: Address = Address(0x0D);
    }
    impl Data for RDDIM {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x0E` `RDDSM`  Read Display Signal Mode
      > Reference: p. 198
    */
    #[derive(Debug)]
    pub struct RDDSM;
    impl Command for RDDSM {
        const NAME: &str = "RDDSM";
        const ADDRESS: Address = Address(0x0E);
    }
    impl Data for RDDSM {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x0F` `RDDSDR`  Read Display Self-Diagnostic Result
      > Reference: p. 199
    */
    #[derive(Debug)]
    pub struct RDDSDR;
    impl Command for RDDSDR {
        const NAME: &str = "RDDSDR";
        const ADDRESS: Address = Address(0x0F);
    }
    impl Data for RDDSDR {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x10` `SLPIN`  Sleep in
      > Reference: p. 200
    */
    #[derive(Debug)]
    pub struct SLPIN;
    impl Command for SLPIN {
        const NAME: &str = "SLPIN";
        const ADDRESS: Address = Address(0x10);
    }

    /**
      ### `0x11` `SLPOUT`  Sleep Out
      > Reference: p. 201
    */
    #[derive(Debug)]
    pub struct SLPOUT;
    impl Command for SLPOUT {
        const NAME: &str = "SLPOUT";
        const ADDRESS: Address = Address(0x11);
    }

    /**
      ### `0x12` `PTLON`  Partial Display Mode On
      > Reference: p. 202
    */
    #[derive(Debug)]
    pub struct PTLON;
    impl Command for PTLON {
        const NAME: &str = "PTLON";
        const ADDRESS: Address = Address(0x12);
    }

    /**
      ### `0x13` `NORON`  Normal Display Mode On
      > Reference: p. 203
    */
    #[derive(Debug)]
    pub struct NORON;
    impl Command for NORON {
        const NAME: &str = "NORON";
        const ADDRESS: Address = Address(0x13);
    }

    /**
      ### `0x20` `INVOFF`  Display Inversion Off
      > Reference: p. 204
    */
    #[derive(Debug)]
    pub struct INVOFF;
    impl Command for INVOFF {
        const NAME: &str = "INVOFF";
        const ADDRESS: Address = Address(0x20);
    }

    /**
      ### `0x21` `INVON`  Display Inversion On
      > Reference: p. 205
    */
    #[derive(Debug)]
    pub struct INVON;
    impl Command for INVON {
        const NAME: &str = "INVON";
        const ADDRESS: Address = Address(0x21);
    }

    /**
      ### `0x22` `ALLPOFF`  All Pixel Off
      > Reference: p. 206
    */
    #[derive(Debug)]
    pub struct ALLPOFF;
    impl Command for ALLPOFF {
        const NAME: &str = "ALLPOFF";
        const ADDRESS: Address = Address(0x22);
    }

    /**
      ### `0x23` `ALLPON`  All Pixel ON
      > Reference: p. 207
    */
    #[derive(Debug)]
    pub struct ALLPON;
    impl Command for ALLPON {
        const NAME: &str = "ALLPON";
        const ADDRESS: Address = Address(0x23);
    }

    /**
      ### `0x26` `GAMSET`  Gamma Set
      > Reference: p. 208
    */
    #[derive(Debug)]
    pub struct GAMSET;
    impl Command for GAMSET {
        const NAME: &str = "GAMSET";
        const ADDRESS: Address = Address(0x26);
    }
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
    impl Data for GAMSET {
        type Parameters = (gamma::Curve,);
        type Packets = Buffer<1>;
    }
    impl WriteData for GAMSET {
        fn encode((gc,): &Self::Parameters) -> Self::Packets {
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
    #[derive(Debug)]
    pub struct DISPOFF;
    impl Command for DISPOFF {
        const NAME: &str = "DISPOFF";
        const ADDRESS: Address = Address(0x28);
    }

    /**
      ### `0x29` `DISPON`  Display On
      > Reference: p. 210
    */
    #[derive(Debug)]
    pub struct DISPON;
    impl Command for DISPON {
        const NAME: &str = "DISPON";
        const ADDRESS: Address = Address(0x29);
    }

    /**
      ### `0x34` `TEOFF`  Tearing Effect Line OFF
      > Reference: p. 211
    */
    #[derive(Debug)]
    pub struct TEOFF;
    impl Command for TEOFF {
        const NAME: &str = "TEOFF";
        const ADDRESS: Address = Address(0x34);
    }

    /**
      ### `0x35` `TEON`  Tearing Effect Line ON
      > Reference: p. 212
    */
    #[derive(Debug)]
    pub struct TEON;
    impl Command for TEON {
        const NAME: &str = "TEON";
        const ADDRESS: Address = Address(0x35);
    }
    impl Data for TEON {
        type Packets = Buffer<1>;

        type Parameters = (tearing_effect::Blank,);
    }
    impl WriteData for TEON {
        /**
            #### Write Parameters

            0: V-Blanking
            1: VH-Blanking

            NOTE: This mode can also be disabled with TEOFF

            |    |   D7  |   D6  |   D5  |   D4  |   D3  |   D2  |   D1  |   D0  |
            |:--:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
            | P1 |   -   |   -   |   -   |   -   |   -   |   -   |   -   |   TE  |
        */
        fn encode((te,): &Self::Parameters) -> Self::Packets {
            let te_data = match te {
                tearing_effect::Blank::Vertical => 0,
                tearing_effect::Blank::VerticalHorizontal => 1,
            };

            [te_data]
        }
    }

    /**
      ### `0x36` `MADCTL`  Display data access control
      > Reference: p. 214
    */
    #[derive(Debug)]
    pub struct MADCTL;
    impl Command for MADCTL {
        const NAME: &str = "MADCTL";
        const ADDRESS: Address = Address(0x36);
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
    impl Data for MADCTL {
        type Packets = Buffer<1>;

        type Parameters = (data_access::ScanDirection, data_access::ColorOrder);
    }
    impl WriteData for MADCTL {
        fn encode((ml, co): &Self::Parameters) -> Self::Packets {
            let p1_scan = match ml {
                data_access::ScanDirection::Normal => 0,
                data_access::ScanDirection::Reverse => 1 << 4,
            };
            let p1_color = match co {
                data_access::ColorOrder::RGB => 0,
                data_access::ColorOrder::BGR => 1 << 3,
            };

            [p1_scan | p1_color]
        }
    }

    /**
      ### `0x38` `IDMOFF`  Idle Mode Off
      > Reference: p. 215
    */
    #[derive(Debug)]
    pub struct IDMOFF;
    impl Command for IDMOFF {
        const NAME: &str = "IDMOFF";
        const ADDRESS: Address = Address(0x38);
    }

    /**
      ### `0x39` `IDMON`  Idle Mode On
      > Reference: p. 216
    */
    #[derive(Debug)]
    pub struct IDMON;
    impl Command for IDMON {
        const NAME: &str = "IDMON";
        const ADDRESS: Address = Address(0x39);
    }

    /**
      ### `0x3A` `COLMOD`  Interface Pixel Format
      > Reference: p. 218
    */
    #[derive(Debug)]
    pub struct COLMOD;
    impl Command for COLMOD {
        const NAME: &str = "COLMOD";
        const ADDRESS: Address = Address(0x3A);
    }
    impl Data for COLMOD {
        type Packets = Buffer<1>;
        type Parameters = (pixel_format::BitsPerPixel,);
    }
    impl WriteData for COLMOD {
        fn encode((bpp,): &Self::Parameters) -> Self::Packets {
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
    #[derive(Debug)]
    pub struct GSL;
    impl Command for GSL {
        const NAME: &str = "GSL";
        const ADDRESS: Address = Address(0x45);
    }
    impl Data for GSL {
        type Parameters = ();
        type Packets = Buffer<2>;
    }

    /**
      ### `0x051` `WRDISBV` Write Display Brightness
      > See p. 220
    */
    #[derive(Debug)]
    pub struct WRDISBV;
    impl Command for WRDISBV {
        const NAME: &str = "WRDISBV";
        const ADDRESS: Address = Address(0x51);
    }
    impl Data for WRDISBV {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x52` `RDDISBV` Read Display Brightness Value
      > See p. 221
    */
    #[derive(Debug)]
    pub struct RDDISBV;
    impl Command for RDDISBV {
        const NAME: &str = "RDDISBV";
        const ADDRESS: Address = Address(0x52);
    }
    impl Data for RDDISBV {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x53` `WRCTRLD` Write CTRL Display
      > See p. 222
    */
    #[derive(Debug)]
    pub struct WRCTRLD;
    impl Command for WRCTRLD {
        const NAME: &str = "WRCTRLD";
        const ADDRESS: Address = Address(0x53);
    }
    impl Data for WRCTRLD {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x54` `RDCTRLD` Read CTRL Display
      > See p. 224
    */
    #[derive(Debug)]
    pub struct RDCTRLD;
    impl Command for RDCTRLD {
        const NAME: &str = "RDCTRLD";
        const ADDRESS: Address = Address(0x54);
    }
    impl Data for RDCTRLD {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x55` `WRCACE` Write Content Adaptive Brightness Control and Color Enhancement
      > See p. 225
    */
    #[derive(Debug)]
    pub struct WRCACE;
    impl Command for WRCACE {
        const NAME: &str = "WRCACE";
        const ADDRESS: Address = Address(0x55);
    }
    impl Data for WRCACE {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x56` `RDCABC` Read Content Adaptive Brightness Control
      > See p. 227
    */
    #[derive(Debug)]
    pub struct RDCABC;
    impl Command for RDCABC {
        const NAME: &str = "RDCABC";
        const ADDRESS: Address = Address(0x56);
    }
    impl Data for RDCABC {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x5E` `WRCABCMB` Write CABC Minimum Brightness
      > See p. 229
    */
    #[derive(Debug)]
    pub struct WRCABCMB;
    impl Command for WRCABCMB {
        const NAME: &str = "WRCABCMB";
        const ADDRESS: Address = Address(0x5E);
    }
    impl Data for WRCABCMB {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x5F` `RDCABCMB` Read CABC Minimum Brightness
      > See p. 230
    */
    #[derive(Debug)]
    pub struct RDCABCMB;
    impl Command for RDCABCMB {
        const NAME: &str = "RDCABCMB";
        const ADDRESS: Address = Address(0x5F);
    }
    impl Data for RDCABCMB {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x68` `RDABCSDR` Read Automatic Brightness Control Self-Diagnostic Result
      > See p. 231
    */
    #[derive(Debug)]
    pub struct RDABCSDR;
    impl Command for RDABCSDR {
        const NAME: &str = "RDABCSDR";
        const ADDRESS: Address = Address(0x68);
    }
    impl Data for RDABCSDR {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x70` `RDBWLB` Read Black/White Low Bits
      > See p. 232
    */
    #[derive(Debug)]
    pub struct RDBWLB;
    impl Command for RDBWLB {
        const NAME: &str = "RDBWLB";
        const ADDRESS: Address = Address(0x70);
    }
    impl Data for RDBWLB {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x71` `RDBkx` Read Bkx
      > See p. 233
    */
    #[derive(Debug)]
    pub struct RDBKX;
    impl Command for RDBKX {
        const NAME: &str = "RDBKX";
        const ADDRESS: Address = Address(0x71);
    }
    impl Data for RDBKX {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x72` `RDBky` Read Bky
      > See p. 234
    */
    #[derive(Debug)]
    pub struct RDBKY;
    impl Command for RDBKY {
        const NAME: &str = "RDBKY";
        const ADDRESS: Address = Address(0x72);
    }
    impl Data for RDBKY {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x73` `RDWx` Read Wx
      > See p. 235
    */
    #[derive(Debug)]
    pub struct RDWX;
    impl Command for RDWX {
        const NAME: &str = "RDWX";
        const ADDRESS: Address = Address(0x73);
    }
    impl Data for RDWX {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x74` `RDWy` Read Wy
      > See p. 236
    */
    #[derive(Debug)]
    pub struct RDWY;
    impl Command for RDWY {
        const NAME: &str = "RDWY";
        const ADDRESS: Address = Address(0x74);
    }
    impl Data for RDWY {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x75` `RDRGLB` Read Red/Green Low Bits
      > See p. 237
    */
    #[derive(Debug)]
    pub struct RDRGLB;
    impl Command for RDRGLB {
        const NAME: &str = "RDRGLB";
        const ADDRESS: Address = Address(0x75);
    }
    impl Data for RDRGLB {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x76` `RDRx` Read Rx
      > See p. 238
    */
    #[derive(Debug)]
    pub struct RDRX;
    impl Command for RDRX {
        const NAME: &str = "RDRX";
        const ADDRESS: Address = Address(0x76);
    }
    impl Data for RDRX {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x77` `RDRy` Read Ry
      > See p. 239
    */
    #[derive(Debug)]
    pub struct RDRY;
    impl Command for RDRY {
        const NAME: &str = "RDRY";
        const ADDRESS: Address = Address(0x77);
    }
    impl Data for RDRY {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x78` `RDGx` Read Gx
      > See p. 240
    */
    #[derive(Debug)]
    pub struct RDGX;
    impl Command for RDGX {
        const NAME: &str = "RDGX";
        const ADDRESS: Address = Address(0x78);
    }
    impl Data for RDGX {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x79` `RDGy` Read Gy
      > See p. 241
    */
    #[derive(Debug)]
    pub struct RDGY;
    impl Command for RDGY {
        const NAME: &str = "RDGY";
        const ADDRESS: Address = Address(0x79);
    }
    impl Data for RDGY {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x7A` `RDBALB` Read Blue/A Color Low Bits
      > See p. 242
    */
    #[derive(Debug)]
    pub struct RDBALB;
    impl Command for RDBALB {
        const NAME: &str = "RDBALB";
        const ADDRESS: Address = Address(0x7A);
    }
    impl Data for RDBALB {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x7B` `RDBx` Read Bx
      > See p. 243
    */
    #[derive(Debug)]
    pub struct RDBX;
    impl Command for RDBX {
        const NAME: &str = "RDBX";
        const ADDRESS: Address = Address(0x7B);
    }
    impl Data for RDBX {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0x7C` `RDBy` Read By
      > See p. 244
    */
    #[derive(Debug)]
    pub struct RDBy;
    impl Command for RDBy {
        const NAME: &str = "RDBy";
        const ADDRESS: Address = Address(0x7C);
    }

    /**
      ### `0x7D` `RDAx` Read Ax
      > See p. 245
    */
    #[derive(Debug)]
    pub struct RDAx;
    impl Command for RDAx {
        const NAME: &str = "RDAx";
        const ADDRESS: Address = Address(0x7D);
    }

    /**
      ### `0x7E` `RDAy` Read Ay
      > See p. 246
    */
    #[derive(Debug)]
    pub struct RDAy;
    impl Command for RDAy {
        const NAME: &str = "RDAy";
        const ADDRESS: Address = Address(0x7E);
    }

    /**
      ### `0xA1` `RDDDBS` Read DDB Start
      > See p. 247
    */
    #[derive(Debug)]
    pub struct RDDDBS;
    impl Command for RDDDBS {
        const NAME: &str = "RDDDBS";
        const ADDRESS: Address = Address(0xA1);
    }

    /**
      ### `0xA8` `RDDDBC` Read DDB Continue
      > See p. 249
    */
    #[derive(Debug)]
    pub struct RDDDBC;
    impl Command for RDDDBC {
        const NAME: &str = "RDDDBC";
        const ADDRESS: Address = Address(0xA8);
    }

    /**
      ### `0xAA` `RDFCS` Read First Checksum
      > See p. 250
    */
    #[derive(Debug)]
    pub struct RDFCS;
    impl Command for RDFCS {
        const NAME: &str = "RDFCS";
        const ADDRESS: Address = Address(0xAA);
    }

    /**
      ### `0xAF` `RDCCS` Read Continue Checksum
      > See p. 251
    */
    #[derive(Debug)]
    pub struct RDCCS;
    impl Command for RDCCS {
        const NAME: &str = "RDCCS";
        const ADDRESS: Address = Address(0xAF);
    }

    /**
      ### `0xDA` `RDID1` Read ID1
      > See p. 252
    */
    #[derive(Debug)]
    pub struct RDID1;
    impl Command for RDID1 {
        const NAME: &str = "RDID1";
        const ADDRESS: Address = Address(0xDA);
    }

    /**
      ### `0xDB` `RDID2` Read ID2
      > See p. 253
    */
    #[derive(Debug)]
    pub struct RDID2;
    impl Command for RDID2 {
        const NAME: &str = "RDID2";
        const ADDRESS: Address = Address(0xDB);
    }

    /**
      ### `0xDC` `RDID3` Read ID3
      > See p. 254
    */
    #[derive(Debug)]
    pub struct RDID3;
    impl Command for RDID3 {
        const NAME: &str = "RDID3";
        const ADDRESS: Address = Address(0xDC);
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
    #[derive(Debug)]
    pub struct CND2BKXSEL;
    impl Command for CND2BKXSEL {
        const NAME: &str = "CND2BKXSEL";
        const ADDRESS: Address = Address(0xFF);
    }
    impl Data for CND2BKXSEL {
        type Parameters = (Switch, Bank);
        type Packets = Buffer<5>;
    }
    impl WriteData for CND2BKXSEL {
        fn encode((cn2, bkxsel): &Self::Parameters) -> Self::Packets {
            const P1: u8 = 0b0111_0111;
            const P2: u8 = 0b0000_0001;
            const P3: u8 = 0b0000_0000;
            const P4: u8 = 0b0000_0000;

            let p5_toggle: u8 = match cn2 {
                Switch::Off => 0b0000_0000,
                Switch::On => 0b0001_0000,
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
pub mod bk0 {
    use crate::st7701s_spi::parameters::register::Bank;

    use super::*;

    const BK0: Extension = Extension(Some(Bank::BK0));

    /**
      ### `0xB0` `PVGAMCTRL` Positive Voltage Gamma Control
      > See p. 261
    */
    #[derive(Debug)]
    pub struct PVGAMCTRL;
    impl Command for PVGAMCTRL {
        const NAME: &str = "PVGAMCTRL";
        const ADDRESS: Address = Address(0xB0);
        const EXTENSION: Extension = BK0;
    }
    impl Data for PVGAMCTRL {
        type Parameters = ();
        type Packets = Buffer<16>;
    }

    /**
      ### `0xB1` `NVGAMCTRL` Negative Voltage Gamma Control
      > See p. 263
    */
    #[derive(Debug)]
    pub struct NVGAMCTRL;
    impl Command for NVGAMCTRL {
        const NAME: &str = "NVGAMCTRL";
        const ADDRESS: Address = Address(0xB1);
        const EXTENSION: Extension = BK0;
    }
    impl Data for NVGAMCTRL {
        type Parameters = ();
        type Packets = Buffer<16>;
    }

    /**
      ### `0xB8` `DGMEN` Digital Gamma Enable
      > See p. 265
    */
    #[derive(Debug)]
    pub struct DGMEN;
    impl Command for DGMEN {
        const NAME: &str = "DGMEN";
        const ADDRESS: Address = Address(0xB8);
        const EXTENSION: Extension = BK0;
    }
    impl Data for DGMEN {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xB9` `DGMLUTR` Digital Gamma Look-up Table for Red
      > See p. 266
    */
    #[derive(Debug)]
    pub struct DGMLUTR;
    impl Command for DGMLUTR {
        const NAME: &str = "DGMLUTR";
        const ADDRESS: Address = Address(0xB9);
        const EXTENSION: Extension = BK0;
    }
    impl Data for DGMLUTR {
        type Parameters = ();
        type Packets = Buffer<130>;
    }

    /**
      ### `0xBA` `DGMLUTB` Digital Gamma Look-up Table for Blue
      > See p. 267
    */
    #[derive(Debug)]
    pub struct DGMLUTB;
    impl Command for DGMLUTB {
        const NAME: &str = "DGMLUTB";
        const ADDRESS: Address = Address(0xBA);
        const EXTENSION: Extension = BK0;
    }
    impl Data for DGMLUTB {
        type Parameters = ();
        type Packets = Buffer<130>;
    }

    /**
      ### `0xBC` `SEL` PWM CLK select
      > See p. 268
    */
    #[derive(Debug)]
    pub struct PWMCLK;
    impl Command for PWMCLK {
        const NAME: &str = "PWMCLK";
        const ADDRESS: Address = Address(0xBC);
        const EXTENSION: Extension = BK0;
    }
    impl Data for PWMCLK {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xC0` `LNESET` Display Line Setting
      > See p. 269
    */
    #[derive(Debug)]
    pub struct LNESET;
    impl Command for LNESET {
        const NAME: &str = "LNESET";
        const ADDRESS: Address = Address(0xC0);
        const EXTENSION: Extension = BK0;
    }
    impl Data for LNESET {
        type Parameters = ();
        type Packets = Buffer<2>;
    }

    /**
      ### `0xC1` `PORCTRL` Porch Control
      > See p. 270
    */
    #[derive(Debug)]
    pub struct PORCTRL;
    impl Command for PORCTRL {
        const NAME: &str = "PORCTRL";
        const ADDRESS: Address = Address(0xC1);
        const EXTENSION: Extension = BK0;
    }
    impl Data for PORCTRL {
        type Parameters = ();
        type Packets = Buffer<2>;
    }

    /**
      ### `0xC2` `INVSE` Inversion selection & Frame Rate Control
      > See p. 271
    */
    #[derive(Debug)]
    pub struct INVSET;
    impl Command for INVSET {
        const NAME: &str = "INVSET";
        const ADDRESS: Address = Address(0xC2);
        const EXTENSION: Extension = BK0;
    }
    impl Data for INVSET {
        type Parameters = ();
        type Packets = Buffer<2>;
    }

    /**
      ### `0xC3` `RGBCTRL` RGB control
      > See p. 272
    */
    #[derive(Debug)]
    pub struct RGBCTRL;
    impl Command for RGBCTRL {
        const NAME: &str = "RGBCTRL";
        const ADDRESS: Address = Address(0xC3);
        const EXTENSION: Extension = BK0;
    }
    impl Data for RGBCTRL {
        type Parameters = ();
        type Packets = Buffer<3>;
    }

    /**
      ### `0xC5` `PARCTRL` Partial Mode Control
      > See p. 273
    */
    #[derive(Debug)]
    pub struct PARCTRL;
    impl Command for PARCTRL {
        const NAME: &str = "PARCTRL";
        const ADDRESS: Address = Address(0xC5);
        const EXTENSION: Extension = BK0;
    }
    impl Data for PARCTRL {
        type Parameters = ();
        type Packets = Buffer<4>;
    }

    /**
      ### `0xC7` `SDIR` X-direction Control
      > See p. 274
    */
    #[derive(Debug)]
    pub struct SDIR;
    impl Command for SDIR {
        const NAME: &str = "SDIR";
        const ADDRESS: Address = Address(0xC7);
        const EXTENSION: Extension = BK0;
    }
    impl Data for SDIR {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xC8` `PDOSET` Pseudo-Dot inversion diving setting
      > See p. 275
    */
    #[derive(Debug)]
    pub struct PDOSET;
    impl Command for PDOSET {
        const NAME: &str = "PDOSET";
        const ADDRESS: Address = Address(0xC8);
        const EXTENSION: Extension = BK0;
    }
    impl Data for PDOSET {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xCD` `COLCTRL` Color Control
      > See p. 276
    */
    #[derive(Debug)]
    pub struct COLCTRL;
    impl Command for COLCTRL {
        const NAME: &str = "COLCTRL";
        const ADDRESS: Address = Address(0xCD);
        const EXTENSION: Extension = BK0;
    }
    impl Data for COLCTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct SSCTRL;
    impl Command for SSCTRL {
        const NAME: &str = "SSCTRL";
        const ADDRESS: Address = Address(0xCE);
        const EXTENSION: Extension = BK0;
    }
    impl Data for SSCTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xE0` `SECTRL` Sunlight Readable Enhancement
      > See p. 278
    */
    #[derive(Debug)]
    pub struct SRECTRL;
    impl Command for SRECTRL {
        const NAME: &str = "SRECTRL";
        const ADDRESS: Address = Address(0xE0);
        const EXTENSION: Extension = BK0;
    }
    impl Data for SRECTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xE1` `NRCTRL` Noise Reduce Control
      > See p. 279
    */
    #[derive(Debug)]
    pub struct NRCTRL;
    impl Command for NRCTRL {
        const NAME: &str = "NRCTRL";
        const ADDRESS: Address = Address(0xE1);
        const EXTENSION: Extension = BK0;
    }
    impl Data for NRCTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xE2` `SECTRL` Sharpness Control
      > See p. 280
    */
    #[derive(Debug)]
    pub struct SECTRL;
    impl Command for SECTRL {
        const NAME: &str = "SECTRL";
        const ADDRESS: Address = Address(0xE2);
        const EXTENSION: Extension = BK0;
    }
    impl Data for SECTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xE3` `CCCTRL` Color Calibration Control
      > See p. 281
    */
    #[derive(Debug)]
    pub struct CCCTRL;
    impl Command for CCCTRL {
        const NAME: &str = "CCCTRL";
        const ADDRESS: Address = Address(0xE3);
        const EXTENSION: Extension = BK0;
    }
    impl Data for CCCTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    /**
      ### `0xE4` `SKCTRL` Skin Tone Preservation Control
      > See p. 282
    */
    #[derive(Debug)]
    pub struct SKCTRL;
    impl Command for SKCTRL {
        const NAME: &str = "SKCTRL";
        const ADDRESS: Address = Address(0xE4);
        const EXTENSION: Extension = BK0;
    }
    impl Data for SKCTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct NVMSETE;
    impl Command for NVMSETE {
        const NAME: &str = "NVMSETE";
        const ADDRESS: Address = Address(0xEA);
        const EXTENSION: Extension = BK0;
    }
    impl Data for NVMSETE {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct CABCCTRL;
    impl Command for CABCCTRL {
        const NAME: &str = "CABCCTRL";
        const ADDRESS: Address = Address(0xEE);
        const EXTENSION: Extension = BK0;
    }
    impl Data for CABCCTRL {
        type Parameters = ();
        type Packets = Buffer<1>;
    }
}

/**
 ## BK1 COMMANDS
*/
pub mod bk1 {
    use crate::st7701s_spi::parameters::register::Bank;

    use super::*;

    const BK1: Extension = Extension(Some(Bank::BK1));

    #[derive(Debug)]
    pub struct VCOMS;
    impl Command for VCOMS {
        const NAME: &str = "VCOMS";
        const ADDRESS: Address = Address(0xB1);
        const EXTENSION: Extension = BK1;
    }
    impl Data for VCOMS {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct VGHSS;
    impl Command for VGHSS {
        const NAME: &str = "VGHSS";
        const ADDRESS: Address = Address(0xB2);
        const EXTENSION: Extension = BK1;
    }
    impl Data for VGHSS {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct TESTCMD;
    impl Command for TESTCMD {
        const NAME: &str = "TESTCMD";
        const ADDRESS: Address = Address(0xB3);
        const EXTENSION: Extension = BK1;
    }
    impl Data for TESTCMD {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct VGLS;
    impl Command for VGLS {
        const NAME: &str = "VGLS";
        const ADDRESS: Address = Address(0xB5);
        const EXTENSION: Extension = BK1;
    }
    impl Data for VGLS {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct PWCTRL1;
    impl Command for PWCTRL1 {
        const NAME: &str = "PWCTRL1";
        const ADDRESS: Address = Address(0xB7);
        const EXTENSION: Extension = BK1;
    }
    impl Data for PWCTRL1 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct PWCTRL2;
    impl Command for PWCTRL2 {
        const NAME: &str = "PWCTRL2";
        const ADDRESS: Address = Address(0xB8);
        const EXTENSION: Extension = BK1;
    }
    impl Data for PWCTRL2 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct PCLKS1;
    impl Command for PCLKS1 {
        const NAME: &str = "PCLKS1";
        const ADDRESS: Address = Address(0xBA);
        const EXTENSION: Extension = BK1;
    }
    impl Data for PCLKS1 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct PCLKS3;
    impl Command for PCLKS3 {
        const NAME: &str = "PCLKS3";
        const ADDRESS: Address = Address(0xBC);
        const EXTENSION: Extension = BK1;
    }
    impl Data for PCLKS3 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct SPD1;
    impl Command for SPD1 {
        const NAME: &str = "SPD1";
        const ADDRESS: Address = Address(0xC1);
        const EXTENSION: Extension = BK1;
    }
    impl Data for SPD1 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct SPD2;
    impl Command for SPD2 {
        const NAME: &str = "SPD2";
        const ADDRESS: Address = Address(0xC2);
        const EXTENSION: Extension = BK1;
    }
    impl Data for SPD2 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct MIPISET1;
    impl Command for MIPISET1 {
        const NAME: &str = "MIPISET1";
        const ADDRESS: Address = Address(0xD0);
        const EXTENSION: Extension = BK1;
    }
    impl Data for MIPISET1 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct MIPISET2;
    impl Command for MIPISET2 {
        const NAME: &str = "MIPISET2";
        const ADDRESS: Address = Address(0xD1);
        const EXTENSION: Extension = BK1;
    }
    impl Data for MIPISET2 {
        type Parameters = ();
        type Packets = Buffer<4>;
    }

    #[derive(Debug)]
    pub struct MIPISET3;
    impl Command for MIPISET3 {
        const NAME: &str = "MIPISET3";
        const ADDRESS: Address = Address(0xD2);
        const EXTENSION: Extension = BK1;
    }
    impl Data for MIPISET3 {
        type Parameters = ();
        type Packets = Buffer<1>;
    }

    #[derive(Debug)]
    pub struct MIPISET4;
    impl Command for MIPISET4 {
        const NAME: &str = "MIPISET4";
        const ADDRESS: Address = Address(0xD3);
        const EXTENSION: Extension = BK1;
    }
    impl Data for MIPISET4 {
        type Parameters = ();
        type Packets = Buffer<2>;
    }

    #[derive(Debug)]
    pub struct NVMEN;
    impl Command for NVMEN {
        const NAME: &str = "NVMEN";
        const ADDRESS: Address = Address(0xC8);
        const EXTENSION: Extension = BK1;
    }
    impl Data for NVMEN {
        type Parameters = ();
        type Packets = Buffer<4>;
    }

    #[derive(Debug)]
    pub struct NVMSET;
    impl Command for NVMSET {
        const NAME: &str = "NVMSET";
        const ADDRESS: Address = Address(0xCA);
        const EXTENSION: Extension = BK1;
    }
    impl Data for NVMSET {
        type Parameters = ();
        type Packets = Buffer<3>;
    }
}

/**
 ## BK3 COMMANDS
*/
pub mod bk3 {}
