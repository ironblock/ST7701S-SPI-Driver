use crate::st7701s_spi::{
    address_OLD::mipi::{BK0, BK1, BK3, Core, Special},
    parameters::register::Address,
};

type ParameterCount = usize;
type PacketCount = usize;

pub trait Command {
    const ADDRESS: Address;
}
pub trait Write: Command {
    const PACKETS: u8;
}

pub trait Read: Command {
    const PACKETS: u8;
}

/// ## `0x00` `NOP`  No Operation
/// > Reference: p. 187
///
/// This command performs no action and is used as a placeholder or to terminate
/// parameter write sequences. It does not affect the display state.

pub struct NOP;
impl Command for NOP {
    const ADDRESS: Address = Core::NOP.into_address();
}

/// ## `0x01` `SWRESET`  Software Reset
/// > Reference: p. 188
///
/// Resets all internal registers to their default values. The framebuffer is
/// not affected. After issuing this command, wait at least 5ms before sending
/// another command. If the display is in sleep mode, wait at least 120ms before
/// exiting sleep.
///
/// ### Parameters
/// It's never stated anywhere why D0 is 1, but it's indicated in both the
/// primary reference table on p. 184 and again on SWRESET's detail page.
///
/// As an additional contradiction, p. 184 refers to SWRESET as a **command**
/// (with no arguments), and p. 188 refers to it as a **write**. As only a
/// write can have arguments and 0x01 is the constant argument in both
/// references, SWRESET's canonical representation here is as a **write**.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// | P1 |   --   |   --   |   --   |   --   |   --   |   --   |   --   |    1   |
pub struct SWRESET;
impl Command for SWRESET {
    const ADDRESS: Address = Core::SWRESET.into_address();
}
impl Write for SWRESET {
    const PACKETS: u8 = 1;
}

/// ## `0x04` `RDDID`  Read Display ID
/// > Reference: p. 189
///
/// Reads the display identification information from the device. This is used
/// to verify the display model and manufacturer.
pub struct RDDID;
impl Command for RDDID {
    const ADDRESS: Address = Core::RDDID.into_address();
}
impl Read for RDDID {
    const PACKETS: u8 = 3;
}

/// ## `0x05` `RDNUMED`  Read Number of Errors on DSI
/// > Reference: p. 190
///
/// Returns the number of transmission errors detected on the DSI interface.
/// This is mainly relevant for MIPI DSI configurations.
pub struct RDNUMED;
impl Command for RDNUMED {
    const ADDRESS: Address = Core::RDNUMED.into_address();
}
impl Read for RDNUMED {
    const PACKETS: u8 = 1;
}

/// ## `0x045` `GSL` Get Scan Line
/// > Reference: p. 219
///
/// Reads the current scan line being refreshed on the display. Useful for
/// synchronization and diagnostics.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           SCAN_LINE[15:0]                         |
pub struct GSL;
impl Command for GSL {
    const ADDRESS: Address = Core::GSL.into_address();
}
impl Read for GSL {
    const PACKETS: u8 = 3;
}

/// ## `0x051` `WRDISBV` Write Display Brightness
/// > Reference: p. 220
///
/// Sets the display brightness to a specified 8-bit value. 0x00 is the lowest
/// brightness, 0xFF is the highest.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Display Brightness Value [7:0]                    |
pub struct WRDISBV;
impl Command for WRDISBV {
    const ADDRESS: Address = Core::WRDISBV.into_address();
}
impl Write for WRDISBV {
    const PACKETS: u8 = 1;
}

/// ## `0x52` `RDDISBV` Read Display Brightness Value
/// > Reference: p. 221
///
/// Returns the current display brightness setting as an 8-bit value.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Display Brightness Value [7:0]                    |
pub struct RDDISBV;
impl Command for RDDISBV {
    const ADDRESS: Address = Core::RDDISBV.into_address();
}
impl Read for RDDISBV {
    const PACKETS: u8 = 1;
}

/// ## `0x53` `WRCTRLD` Write CTRL Display
/// > Reference: p. 222
///
/// Configures display control features such as brightness control, dimming, and
/// backlight. Each bit enables or disables a specific function.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   --   |   --   |  BCTRL |   --   |   DD   |   BL   |   --   |   --   |
pub struct WRCTRLD;
impl Command for WRCTRLD {
    const ADDRESS: Address = Core::WRCTRLD.into_address();
}
impl Write for WRCTRLD {
    const PACKETS: u8 = 1;
}

/// ## `0x54` `RDCTRLD` Read CTRL Display
/// > Reference: p. 224
///
/// Returns the current state of display control features, including brightness
/// control, dimming, and backlight.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   --   |   --   |  BCTRL |   --   |   DD   |   BL   |   --   |   --   |
pub struct RDCTRLD;
impl Command for RDCTRLD {
    const ADDRESS: Address = Core::RDCTRLD.into_address();
}
impl Read for RDCTRLD {
    const PACKETS: u8 = 1;
}

/// ## `0x55` `WRCACE` Write Content Adaptive Brightness Control and Color Enhancement
/// > Reference: p. 225
///
/// Sets parameters for adaptive brightness and color enhancement. Enables or
/// disables color enhancement and selects the enhancement mode.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   CE   |   --   |    CEMD[1:0]    |   --   |   --   |    CABC[1:0]    |
pub struct WRCACE;
impl Command for WRCACE {
    const ADDRESS: Address = Core::WRCACE.into_address();
}
impl Write for WRCACE {
    const PACKETS: u8 = 1;
}

/// ## `0x56` `RDCABC` Read Content Adaptive Brightness Control
/// > Reference: p. 227
///
/// Returns the current settings for adaptive brightness and color enhancement.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   CE   |   --   |    CEMD[1:0]    |   --   |   --   |    CABC[1:0]    |
pub struct RDCABC;
impl Command for RDCABC {
    const ADDRESS: Address = Core::RDCABC.into_address();
}
impl Read for RDCABC {
    const PACKETS: u8 = 1;
}

/// ## `0x5E` `WRCABCMB` Write CABC Minimum Brightness
/// > Reference: p. 229
///
/// Sets the minimum brightness value for Content Adaptive Brightness Control
/// (CABC).
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Minimum Brightness Value [7:0]                    |
pub struct WRCABCMB;
impl Command for WRCABCMB {
    const ADDRESS: Address = Core::WRCABCMB.into_address();
}
impl Write for WRCABCMB {
    const PACKETS: u8 = 1;
}

/// ## `0x5F` `RDCABCMB` Read CABC Minimum Brightness
/// > Reference: p. 230
///
/// Returns the minimum brightness value used by CABC.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Minimum Brightness Value [7:0]                    |
pub struct RDCABCMB;
impl Command for RDCABCMB {
    const ADDRESS: Address = Core::RDCABCMB.into_address();
}
impl Read for RDCABCMB {
    const PACKETS: u8 = 1;
}

/// ## `0x68` `RDABCSDR` Read Automatic Brightness Control Self-Diagnostic Result
/// > Reference: p. 231
///
/// Reads the result of the automatic brightness control self-diagnostic test.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Diagnostic Result [7:0]                           |
pub struct RDABCSDR;
impl Command for RDABCSDR {
    const ADDRESS: Address = Core::RDABCSDR.into_address();
}
impl Read for RDABCSDR {
    const PACKETS: u8 = 1;
}

/// ## `0x70` `RDBWLB` Read Black/White Low Bits
/// > Reference: p. 232
///
/// Returns the low bits of the black and white color settings for calibration
/// and diagnostics.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Black/White Low Bits [7:0]                        |
pub struct RDBWLB;
impl Command for RDBWLB {
    const ADDRESS: Address = Core::RDBWLB.into_address();
}
impl Read for RDBWLB {
    const PACKETS: u8 = 1;
}

/// ## `0x71` `RDBkx` Read Bkx
/// > Reference: p. 233
///
/// Reads the Bkx calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Bkx Value [7:0]                                   |
pub struct RDBKX;
impl Command for RDBKX {
    const ADDRESS: Address = Core::RDBKX.into_address();
}
impl Read for RDBKX {
    const PACKETS: u8 = 1;
}

/// ## `0x72` `RDBky` Read Bky
/// > Reference: p. 234
///
/// Reads the Bky calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Bky Value [7:0]                                   |
pub struct RDBKY;
impl Command for RDBKY {
    const ADDRESS: Address = Core::RDBKY.into_address();
}
impl Read for RDBKY {
    const PACKETS: u8 = 1;
}

/// ## `0x73` `RDWx` Read Wx
/// > Reference: p. 235
///
/// Reads the Wx calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Wx Value [7:0]                                    |
pub struct RDWX;
impl Command for RDWX {
    const ADDRESS: Address = Core::RDWX.into_address();
}
impl Read for RDWX {
    const PACKETS: u8 = 1;
}

/// ## `0x74` `RDWy` Read Wy
/// > Reference: p. 236
///
/// Reads the Wy calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Wy Value [7:0]                                    |
pub struct RDWY;
impl Command for RDWY {
    const ADDRESS: Address = Core::RDWY.into_address();
}
impl Read for RDWY {
    const PACKETS: u8 = 1;
}

/// ## `0x76` `RDRx` Read Rx
/// > Reference: p. 239
///
/// Reads the Rx calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Rx Value [7:0]                                    |
pub struct RDRX;
impl Command for RDRX {
    const ADDRESS: Address = Core::RDRX.into_address();
}
impl Read for RDRX {
    const PACKETS: u8 = 1;
}

/// ## `0x77` `RDRy` Read Ry
/// > Reference: p. 239
///
/// Reads the Ry calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Ry Value [7:0]                                    |
pub struct RDRY;
impl Command for RDRY {
    const ADDRESS: Address = Core::RDRY.into_address();
}
impl Read for RDRY {
    const PACKETS: u8 = 1;
}

/// ## `0x78` `RDGx` Read Gx
/// > Reference: p. 240
///
/// Reads the Gx calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Gx Value [7:0]                                    |
pub struct RDGX;
impl Command for RDGX {
    const ADDRESS: Address = Core::RDGX.into_address();
}
impl Read for RDGX {
    const PACKETS: u8 = 1;
}

/// ## `0x79` `RDGy` Read Gy
/// > Reference: p. 241
///
/// Reads the Gy calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Gy Value [7:0]                                    |
pub struct RDGY;
impl Command for RDGY {
    const ADDRESS: Address = Core::RDGY.into_address();
}
impl Read for RDGY {
    const PACKETS: u8 = 1;
}

/// ## `0x7A` `RDBALB` Read Blue/A Color Low Bits
/// > Reference: p. 242
///
/// Returns the low bits of the blue and A color settings for calibration and
/// diagnostics.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Blue/A Color Low Bits [7:0]                       |
pub struct RDBALB;
impl Command for RDBALB {
    const ADDRESS: Address = Core::RDBALB.into_address();
}
impl Read for RDBALB {
    const PACKETS: u8 = 1;
}

/// ## `0x7B` `RDBx` Read Bx
/// > Reference: p. 243
///
/// Reads the Bx calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Bx Value [7:0]                                    |
pub struct RDBX;
impl Command for RDBX {
    const ADDRESS: Address = Core::RDBX.into_address();
}
impl Read for RDBX {
    const PACKETS: u8 = 1;
}

/// ## `0x7C` `RDBy` Read By
/// > Reference: p. 244
///
/// Reads the By calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     By Value [7:0]                                    |
pub struct RDBY;
impl Command for RDBY {
    const ADDRESS: Address = Core::RDBY.into_address();
}
impl Read for RDBY {
    const PACKETS: u8 = 1;
}

/// ## `0x7D` `RDAx` Read Ax
/// > Reference: p. 245
///
/// Reads the Ax calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Ax Value [7:0]                                    |
pub struct RDAX;
impl Command for RDAX {
    const ADDRESS: Address = Core::RDAX.into_address();
}
impl Read for RDAX {
    const PACKETS: u8 = 1;
}

/// ## `0x7E` `RDAy` Read Ay
/// > Reference: p. 246
///
/// Reads the Ay calibration value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Ay Value [7:0]                                    |
pub struct RDAY;
impl Command for RDAY {
    const ADDRESS: Address = Core::RDAY.into_address();
}
impl Read for RDAY {
    const PACKETS: u8 = 1;
}

/// ## `0xA1` `RDDDBS` Read DDB Start
/// > Reference: p. 247
///
/// Reads the initial value of the Display Data Bus (DDB) for diagnostics.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     DDB Start Value [7:0]                             |
pub struct RDDDBS;
impl Command for RDDDBS {
    const ADDRESS: Address = Core::RDDDBS.into_address();
}
impl Read for RDDDBS {
    const PACKETS: u8 = 1;
}

/// ## `0xA8` `RDDDBC` Read DDB Continue
/// > Reference: p. 249
///
/// Reads the next value of the Display Data Bus (DDB) for diagnostics.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     DDB Continue Value [7:0]                          |
pub struct RDDDBC;
impl Command for RDDDBC {
    const ADDRESS: Address = Core::RDDDBC.into_address();
}
impl Read for RDDDBC {
    const PACKETS: u8 = 1;
}

/// ## `0xAA` `RDFCS` Read First Checksum
/// > Reference: p. 250
///
/// Reads the first checksum value for verifying data integrity.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     First Checksum Value [7:0]                        |
pub struct RDFCS;
impl Command for RDFCS {
    const ADDRESS: Address = Core::RDFCS.into_address();
}
impl Read for RDFCS {
    const PACKETS: u8 = 1;
}

/// ## `0xAF` `RDCCS` Read Continue Checksum
/// > Reference: p. 251
///
/// Reads the next checksum value for continued data integrity verification.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     Continue Checksum Value [7:0]                      |
pub struct RDCCS;
impl Command for RDCCS {
    const ADDRESS: Address = Core::RDCCS.into_address();
}
impl Read for RDCCS {
    const PACKETS: u8 = 1;
}

/// ## `0xDA` `RDID1` Read ID1
/// > Reference: p. 252
///
/// Reads the first identification value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     ID1 Value [7:0]                                   |
pub struct RDID1;
impl Command for RDID1 {
    const ADDRESS: Address = Core::RDID1.into_address();
}
impl Read for RDID1 {
    const PACKETS: u8 = 1;
}

/// ## `0xDB` `RDID2` Read ID2
/// > Reference: p. 253
///
/// Reads the second identification value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     ID2 Value [7:0]                                   |
pub struct RDID2;
impl Command for RDID2 {
    const ADDRESS: Address = Core::RDID2.into_address();
}
impl Read for RDID2 {
    const PACKETS: u8 = 1;
}

/// ## `0xDC` `RDID3` Read ID3
/// > Reference: p. 254
///
/// Reads the third identification value from the device.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |                     ID3 Value [7:0]                                   |
pub struct RDID3;
impl Command for RDID3 {
    const ADDRESS: Address = Core::RDID3.into_address();
}
impl Read for RDID3 {
    const PACKETS: u8 = 1;
}

/// ## `0xFF` `CND2BKxSEL Command2 BKx Selection
/// > Reference: p. 260
///
/// Selects the extended command bank (BK0, BK1, BK3) for subsequent operations.
/// This command is required before sending any extended command and ensures the
/// correct register bank is active.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |    0   |    1   |    1   |    1   |    0   |    1   |    1   |    1   |
/// |    0   |    0   |    0   |    0   |    0   |    0   |    0   |    1   |
/// |    0   |    0   |    0   |    0   |    0   |    0   |    0   |    0   |
/// |    0   |    0   |    0   |    0   |    0   |    0   |    0   |    0   |
/// |    0   |    0   |    0   |   CN2  |    0   |    0   |    0   | BKxSEL |
pub struct CND2BKXSEL;
impl Command for CND2BKXSEL {
    const ADDRESS: Address = Special::CND2BKXSEL.into_address();
}
impl Write for CND2BKXSEL {
    const PACKETS: u8 = 5;
}

/// ## `BK0: 0xB0` `PVGAMCTRL` Positive Voltage Gamma Control
/// > See p. 261
///
/// Configures the positive voltage gamma curve for the display. This command
/// allows fine-tuning of the display's color response and image quality by
/// setting multiple voltage control points.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |    AJ0P[1:0]    |   --   |   --   |             VC0P[3:0]             |
/// |    AJ1P[1:0]    |                      VC4P[5:0]                      |
/// |    AJ2P[1:0]    |                      VC8P[5:0]                      |
/// |   --   |   --   |   --   |                 VC16P[4:0]                 |
/// |    AJ3P[1:0]    |   --   |                 VC24P[4:0]                 |
/// |   --   |   --   |   --   |   --   |             VC52P[3:0]            |
/// |   --   |   --   |                      VC80P[5:0]                     |
/// |   --   |   --   |   --   |   --   |            VC108P[3:0]            |
/// |   --   |   --   |   --   |   --   |            VC147P[3:0]            |
/// |   --   |   --   |                     VC175P[5:0]                     |
/// |   --   |   --   |   --   |   --   |            VC203P[3:0]            |
/// |    AJ4P[1:0]    |   --   |                VC231P[4:0]                 |
/// |   --   |   --   |   --   |                VC239P[4:0]                 |
/// |    AJ5P[1:0]    |                     VC247P[5:0]                     |
/// |    AJ6P[1:0]    |                     VC251P[5:0]                     |
/// |    AJ7P[1:0]    |   --   |                 VC255P[4:0]                |
pub struct PVGAMCTRL;
impl Command for PVGAMCTRL {
    const ADDRESS: Address = BK0::PVGAMCTRL.into_address();
}
impl Write for PVGAMCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xB1` `NVGAMCTRL` Negative Voltage Gamma Control
/// > Reference: p. 263
///
/// Configures the negative voltage gamma curve for the display. This command
/// complements PVGAMCTRL and is used to adjust the display's color response for
/// negative voltages.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |    AJ0N[1:0]    |   --   |   --   |             VC0N[3:0]             |
/// |    AJ1N[1:0]    |                      VC4N[5:0]                      |
/// |    AJ2N[1:0]    |                      VC8N[5:0]                      |
/// |   --   |   --   |   --   |                 VC16N[4:0]                 |
/// |    AJ3N[1:0]    |   --   |                 VC24N[4:0]                 |
/// |   --   |   --   |   --   |   --   |             VC52N[3:0]            |
/// |   --   |   --   |                      VC80N[5:0]                     |
/// |   --   |   --   |   --   |   --   |            VC108N[3:0]            |
/// |   --   |   --   |   --   |   --   |            VC147N[3:0]            |
/// |   --   |   --   |                     VC175N[5:0]                     |
/// |   --   |   --   |   --   |   --   |            VC203N[3:0]            |
/// |    AJ4N[1:0]    |   --   |                VC231N[4:0]                 |
/// |   --   |   --   |   --   |                VC239N[4:0]                 |
/// |    AJ5N[1:0]    |                     VC247N[5:0]                     |
/// |    AJ6N[1:0]    |                     VC251N[5:0]                     |
/// |    AJ7N[1:0]    |   --   |                 VC255N[4:0]                |
pub struct NVGAMCTRL;
impl Command for NVGAMCTRL {
    const ADDRESS: Address = BK0::NVGAMCTRL.into_address();
}
impl Write for NVGAMCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xB8` `DGMEN` Digital Gamma Enable
/// > Reference: p. 265
///
/// Enables or disables digital gamma correction. When enabled, the display uses
/// digital gamma look-up tables for color adjustment.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   --   |   --   |   --   |   --   |   --   |   --   |   --   | DGMEN  |
pub struct DGMEN;
impl Command for DGMEN {
    const ADDRESS: Address = BK0::DGMEN.into_address();
}
impl Write for DGMEN {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xB9` `DGMLUTR` Digital Gamma Look-up Table for Red
/// > Reference: p. 266
///
/// Sets the digital gamma look-up table for the red color channel. Each entry
/// defines the gamma correction for a specific input value.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |         LUTR[7:0] (16 entries, one per write)                        |
pub struct DGMLUTR;
impl Command for DGMLUTR {
    const ADDRESS: Address = BK0::DGMLUTR.into_address();
}
impl Write for DGMLUTR {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xBA` `DGMLUTB` Digital Gamma Look-up Table for Blue
/// > Reference: p. 267
///
/// Sets the digital gamma look-up table for the blue color channel. Each entry
/// defines the gamma correction for a specific input value.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |         LUTB[7:0] (16 entries, one per write)              |
pub struct DGMLUTB;
impl Command for DGMLUTB {
    const ADDRESS: Address = BK0::DGMLUTB.into_address();
}
impl Write for DGMLUTB {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xBC` `SEL` PWM CLK select
/// > Reference: p. 268
///
/// Selects the clock source for the PWM signal used in backlight control.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   --   |   --   |   --   |   --   |   --   |   --   |   --   | PWMSEL |
pub struct SEL;
impl Command for SEL {
    const ADDRESS: Address = BK0::SEL.into_address();
}
impl Write for SEL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xC0` `LNESET` Display Line Setting
/// > Reference: p. 269
///
/// Configures the number of display lines and line delta for the panel. This
/// affects the vertical resolution and timing.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   LDE_EN   |           LINE[6:0]           |
/// |           LINE_DELTA[7:0]                  |
pub struct LNESET;
impl Command for LNESET {
    const ADDRESS: Address = BK0::LNESET.into_address();
}
impl Write for LNESET {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xC1` `PORCTRL` Porch Control
/// > Reference: p. 270
///
/// Sets the front and back porch timing for the display. Proper porch settings
/// are important for stable image rendering and synchronization.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           FRONT_PORCH[7:0]                 |
/// |           BACK_PORCH[7:0]                  |
pub struct PORCTRL;
impl Command for PORCTRL {
    const ADDRESS: Address = BK0::PORCTRL.into_address();
}
impl Write for PORCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xC2` `INVSE` Inversion selection & Frame Rate Control
/// > Reference: p. 271
///
/// Selects the inversion mode and frame rate settings for the display. These
/// parameters help reduce flicker and improve image stability.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           NLINV[2:0]   |   --   |   --   |   --   |   --   |   --   |
/// |           RTNI[7:0]                              |
pub struct INVSE;
impl Command for INVSE {
    const ADDRESS: Address = BK0::INVSE.into_address();
}
impl Write for INVSE {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xC3` `RGBCTRL` RGB control
/// > Reference: p. 272
///
/// Configures the RGB interface mode and signal polarities. This command is
/// essential for matching the display's timing and signal requirements to the
/// host system.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |  DEHV  |   --   |   --   |   --   |   VSP  |   HSP  |   DP   |   EP   |
/// |           HBP[7:0]                              |
/// |           VBP[7:0]                              |
pub struct RGBCTRL;
impl Command for RGBCTRL {
    const ADDRESS: Address = BK0::RGBCTRL.into_address();
}
impl Write for RGBCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xC5` `PARCTRL` Partial Mode Control
/// > Reference: p. 273
///
/// Enables and configures partial display mode, allowing only a portion of the
/// screen to be updated or refreshed.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           PARTIAL_MODE[7:0]                    |
pub struct PARCTRL;
impl Command for PARCTRL {
    const ADDRESS: Address = BK0::PARCTRL.into_address();
}
impl Write for PARCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xC7` `SDIR` X-direction Control
/// > Reference: p. 274
///
/// Sets the direction of pixel scanning along the X-axis. This is used for
/// display orientation and mirroring.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           SDIR[7:0]                             |
pub struct SDIR;
impl Command for SDIR {
    const ADDRESS: Address = BK0::SDIR.into_address();
}
impl Write for SDIR {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xC8` `PDOSET` Pseudo-Dot inversion diving setting
/// > Reference: p. 275
///
/// Configures pseudo-dot inversion settings to improve display uniformity and
/// reduce artifacts.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           PDOSET[7:0]                           |
pub struct PDOSET;
impl Command for PDOSET {
    const ADDRESS: Address = BK0::PDOSET.into_address();
}
impl Write for PDOSET {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xCD` `COLCTRL` Color Control
/// > Reference: p. 276
///
/// Adjusts color control parameters such as PWM polarity, LED polarity, pixel
/// format, and end pixel format. These settings affect color rendering and
/// backlight behavior.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   --   |   --   |   PWM  |   LED  |   MDT  |            EPF           |
pub struct COLCTRL;
impl Command for COLCTRL {
    const ADDRESS: Address = BK0::COLCTRL.into_address();
}
impl Write for COLCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xE0` `SECTRL` Sunlight Readable Enhancement
/// > Reference: p. 278
///
/// Enables and configures sunlight readability enhancement features, improving
/// display visibility in bright environments.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |   --   |   --   |   --   |   --   |   SRE  |      SRE_alpha[3:0]      |
pub struct SRECTRL;
impl Command for SRECTRL {
    const ADDRESS: Address = BK0::SRECTRL.into_address();
}
impl Write for SRECTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xE1` `NRCTRL` Noise Reduce Control
/// > Reference: p. 279
///
/// Sets noise reduction parameters to improve image quality and reduce visual
/// artifacts.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           NRCTRL[7:0]                            |
pub struct NRCTRL;
impl Command for NRCTRL {
    const ADDRESS: Address = BK0::NRCTRL.into_address();
}
impl Write for NRCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xE2` `SECTRL` Sharpness Control
/// > Reference: p. 280
///
/// Adjusts display sharpness settings to enhance image clarity.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           SECTRL[7:0]                            |
pub struct SECTRL;
impl Command for SECTRL {
    const ADDRESS: Address = BK0::SECTRL.into_address();
}
impl Write for SECTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xE3` `CCCTRL` Color Calibration Control
/// > Reference: p. 281
///
/// Configures color calibration parameters for fine-tuning display color
/// accuracy.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           CCCTRL[7:0]                            |
pub struct CCCTRL;
impl Command for CCCTRL {
    const ADDRESS: Address = BK0::CCCTRL.into_address();
}
impl Write for CCCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xE4` `SKCTRL` Skin Tone Preservation Control
/// > Reference: p. 282
///
/// Enables and adjusts skin tone preservation features to improve the rendering
/// of human skin colors.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |           SKCTRL[7:0]                            |
pub struct SKCTRL;
impl Command for SKCTRL {
    const ADDRESS: Address = BK0::SKCTRL.into_address();
}
impl Write for SKCTRL {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xEA` `NVMSETE` NVM Address Setting Enable
/// > Reference: p. 305
///
/// Enables the setting of NVM (Non-Volatile Memory) address, allowing access to
/// memory configuration registers.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      NVM Address Enable Value [7:0]                                 |
pub struct NVMSETE;
impl Command for NVMSETE {
    const ADDRESS: Address = BK0::NVMSETE.into_address();
}
impl Write for NVMSETE {
    const PACKETS: u8 = 1;
}

/// ## `BK0: 0xEE` `CABCCTRL` CABC Control
/// > Reference: p. 283
///
/// Controls the Content Adaptive Brightness Control (CABC) feature, which
/// automatically adjusts display brightness based on image content to improve
/// power efficiency and viewing comfort.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      CABC Mode/Level/Enable (see datasheet)                      |
pub struct CABCCTRL;
impl Command for CABCCTRL {
    const ADDRESS: Address = BK0::CABCCTRL.into_address();
}
impl Write for CABCCTRL {
    const PACKETS: u8 = 1;
}

/// ## `Special: 0xFF` `DSTB` Deep Standby Mode Enable
/// > Reference: p. 285
///
/// Enables deep standby mode, reducing power consumption to a minimum. The
/// display will not respond to most commands until reactivated.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      DSTB Enable/Disable (see datasheet)                         |
pub struct DSTB;
impl Command for DSTB {
    const ADDRESS: Address = Special::DSTB.into_address();
}
impl Write for DSTB {
    const PACKETS: u8 = 1;
}

/// ## `Special: 0xFF` `DSTBT` Deep Standby Mode Active
/// > Reference: p. 286
///
/// Indicates whether deep standby mode is currently active. Used for
/// diagnostics and power management.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      DSTBT Status (see datasheet)                                 |
pub struct DSTBT;
impl Command for DSTBT {
    const ADDRESS: Address = Special::DSTBT.into_address();
}
impl Read for DSTBT {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xB0` `VRHS` Vop Amplitude Setting
/// > Reference: p. 287
///
/// Sets the Vop amplitude, which controls the driving voltage for the display
/// panel. Adjusting this value can affect display brightness and stability.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Vop Amplitude Value [7:0]                                    |
pub struct VRHS;
impl Command for VRHS {
    const ADDRESS: Address = BK1::VRHS.into_address();
}
impl Write for VRHS {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xB1` `VCOMS` VCOM Amplitude Setting
/// > Reference: p. 288
///
/// Sets the VCOM amplitude, which determines the common voltage level for the
/// display. Proper VCOM settings help reduce flicker and improve image quality.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      VCOM Amplitude Value [7:0]                                   |
pub struct VCOMS;
impl Command for VCOMS {
    const ADDRESS: Address = BK1::VCOMS.into_address();
}
impl Write for VCOMS {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xB2` `VGHSS` VGH Voltage Setting
/// > Reference: p. 289
///
/// Sets the VGH voltage, which is used for gate driver circuits in the display.
/// Adjusting this value can affect panel performance and reliability.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      VGH Voltage Value [7:0]                                      |
pub struct VGHSS;
impl Command for VGHSS {
    const ADDRESS: Address = BK1::VGHSS.into_address();
}
impl Write for VGHSS {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xB3` `TESTCMD` Test Command Setting
/// > Reference: p. 290
///
/// Used for factory or engineering test purposes. Allows access to special test
/// modes or registers not used in normal operation.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Test Command Value [7:0]                                     |
pub struct TESTCMD;
impl Command for TESTCMD {
    const ADDRESS: Address = BK1::TESTCMD.into_address();
}
impl Write for TESTCMD {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xB5` `VGLS` VGL Voltage Setting
/// > Reference: p. 291
///
/// Sets the VGL voltage, which is used for gate driver circuits in the display.
/// Proper VGL settings help ensure stable operation.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      VGL Voltage Value [7:0]                                      |
pub struct VGLS;
impl Command for VGLS {
    const ADDRESS: Address = BK1::VGLS.into_address();
}
impl Write for VGLS {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xB7` `PWCTRL1` Power Control 1
/// > Reference: p. 292
///
/// Configures primary power control settings for the display, including voltage
/// regulators and power sequencing.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Power Control 1 Value [7:0]                                   |
pub struct PWCTRL1;
impl Command for PWCTRL1 {
    const ADDRESS: Address = BK1::PWCTRL1.into_address();
}
impl Write for PWCTRL1 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xB8` `PWCTRL2` Power Control 2
/// > Reference: p. 293
///
/// Configures secondary power control settings for the display, such as
/// additional voltage rails or timing parameters.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Power Control 2 Value [7:0]                                   |
pub struct PWCTRL2;
impl Command for PWCTRL2 {
    const ADDRESS: Address = BK1::PWCTRL2.into_address();
}
impl Write for PWCTRL2 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xBA` `PCLKS1` Power Pumping Clock Selection 1
/// > Reference: p. 295
///
/// Selects the clock source for power pumping circuits, which are used to
/// generate high voltages for the display.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Clock Selection Value [7:0]                                   |
pub struct PCLKS1;
impl Command for PCLKS1 {
    const ADDRESS: Address = BK1::PCLKS1.into_address();
}
impl Write for PCLKS1 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xBB` `PCLKS2` Power Pumping Clock Selection 2
/// > Reference: p. 296
///
/// Selects an alternate clock source for power pumping circuits, providing
/// additional flexibility for voltage generation.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Clock Selection Value [7:0]                                   |
pub struct PCLKS2;
impl Command for PCLKS2 {
    const ADDRESS: Address = BK1::PCLKS2.into_address();
}
impl Write for PCLKS2 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xBC` `PCLKS3` Power Pumping Clock Selection 3
/// > Reference: p. 297
///
/// Selects a third clock source for power pumping circuits, used for advanced
/// power management and optimization.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Clock Selection Value [7:0]                                   |
pub struct PCLKS3;
impl Command for PCLKS3 {
    const ADDRESS: Address = BK1::PCLKS3.into_address();
}
impl Write for PCLKS3 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xC1` `SPD1` Source Pre-Drive Timing Set 1
/// > Reference: p. 298
///
/// Configures timing parameters for the source driver pre-drive stage, which
/// affects signal integrity and display performance.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Pre-Drive Timing Value [7:0]                                  |
pub struct SPD1;
impl Command for SPD1 {
    const ADDRESS: Address = BK1::SPD1.into_address();
}
impl Write for SPD1 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xC12` `SPD2` Source Pre-Drive Timing Set 2
/// > Reference: p. 299
///
/// Configures additional timing parameters for the source driver pre-drive
/// stage, allowing further optimization of signal quality.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      Pre-Drive Timing Value [7:0]                                  |
pub struct SPD2;
impl Command for SPD2 {
    const ADDRESS: Address = BK1::SPD2.into_address();
}
impl Write for SPD2 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xD0` `MIPISET1` MIPI Setting 1
/// > Reference: p. 300
///
/// Configures MIPI interface settings, such as lane configuration, timing, and
/// signal levels for proper communication with the display.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      MIPI Setting Value [7:0]                                      |
pub struct MIPISET1;
impl Command for MIPISET1 {
    const ADDRESS: Address = BK1::MIPISET1.into_address();
}
impl Write for MIPISET1 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xD1` `MIPISET2` MIPI Setting 2
/// > Reference: p. 301
///
/// Sets additional MIPI interface parameters, such as advanced timing or signal
/// options. Used for fine-tuning communication reliability.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      MIPI Setting Value [7:0]                                      |
pub struct MIPISET2;
impl Command for MIPISET2 {
    const ADDRESS: Address = BK1::MIPISET2.into_address();
}
impl Write for MIPISET2 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xD2` `MIPISET3` MIPI Setting 3
/// > Reference: p. 303
///
/// Sets further MIPI interface options, such as error correction or signal
/// calibration.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      MIPI Setting Value [7:0]                                      |
pub struct MIPISET3;
impl Command for MIPISET3 {
    const ADDRESS: Address = BK1::MIPISET3.into_address();
}
impl Write for MIPISET3 {
    const PACKETS: u8 = 1;
}

/// ## `BK1: 0xD3` `MIPISET4` MIPI Setting 4
/// > Reference: p. 304
///
/// Sets final MIPI interface parameters, completing the configuration for
/// optimal display communication.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      MIPI Setting Value [7:0]                                      |
pub struct MIPISET4;
impl Command for MIPISET4 {
    const ADDRESS: Address = BK1::MIPISET4.into_address();
}
impl Write for MIPISET4 {
    const PACKETS: u8 = 1;
}

/// ## `BK3: 0xCA` `NVMSET` NVM Manual Control Setting
/// > Reference: p. 306
///
/// Allows manual control of NVM settings, such as programming or erasing memory
/// blocks for calibration or configuration.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      NVM Manual Control Value [7:0]                                 |
pub struct NVMSET;
impl Command for NVMSET {
    const ADDRESS: Address = BK3::NVMSET.into_address();
}
impl Write for NVMSET {
    const PACKETS: u8 = 1;
}

/// ## `BK3: 0xCC` `PROMACT` NVM Program Active
/// > Reference: p. 307
///
/// Activates NVM programming mode, allowing data to be written to non-volatile
/// memory for persistent configuration.
///
/// |    |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
/// |:--:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|:------:|
/// |      NVM Program Active Value [7:0]                                 |
pub struct PROMACT;
impl Command for PROMACT {
    const ADDRESS: Address = BK3::PROMACT.into_address();
}
impl Write for PROMACT {
    const PACKETS: u8 = 1;
}
