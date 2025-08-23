use std::{thread, time};

use log::info;

use crate::st7701s_spi::{
    interface::{bk0::*, bk1::*, core::*, Data},
    panel::Mode,
    parameters::*,
    spi::{Configure, Momentary, Select, SideEffect, StateConstants, StateTracker, Toggle, Transceiver, Transmission, ST7701S},
    state::{Switch},
};

/// This is a 3-wire SPI implementation. Reads and writes share the SDA pin and
/// are performed half-duplex
///
/// Unless otherwise noted, any command pairs that set and unset a "mode" (eg.
/// DISPON/DISPOFF) will have no effect if the display is already in the mode
/// being requested. Therefore, these commands should be safe to use in a
/// "write-only, read-never" workflow.
///
/// NOTE: Some commands take many separate parameter words, most of which have at
/// least 8 bits of variability. Because of this, they aren't enumerated and
/// the vector is passed directly through.

/// The write mode of the interface means the micro controller writes
/// commands and data to the LCD driver. 3-lines serial data packet contains
/// a control bit D/CX and a transmission byte. In 4-lines serial interface,
/// data packet contains just transmission byte and control bit D/CX is
/// transferred by the D/CX pin. If D/CX is “low”, the transmission byte is
/// interpreted as a command byte. If D/CX is “high”, the transmission byte
/// is command register as parameter.

/**
    ## NO OPERATION

    This command is "empty". It has no effect on the display, but it can be used
    to terminate parameter write commands.
*/
type NoOperation = Momentary<NOP>;


/**
    ## SOFTWARE RESET

    Performs a software reset. All register values are reset to their initial
    defaults. The framebuffer is unaffected.

    ### Considerations
    1. Wait at least 5ms before sending another command after the reset
    2. If the display is sleeping (SLPIN), wait at least 120ms before
        attempting to exit sleep mode (SLPOUT).
    3. If the display is already in the process of exiting sleep, a reset
        command will be ignored and have no effect.
*/
type SoftwareReset = SideEffect<SWRESET>;
impl Transmission<()> for SoftwareReset {
    fn transmit(
        &mut self,
        channel: &mut impl Transceiver,
        _: (),
        state: &mut StateTracker,) -> Result<(), &'static str> {
        channel.tx_command::<SWRESET>().and_then(|x| {
            state.reset();

            if SleepMode::is(state, &Switch::On) {
                info!("Software reset while in sleep mode, waiting 120ms for sleep exit");

                thread::sleep(time::Duration::from_millis(120));
            } else {
                info!("Software reset, waiting 5ms for reset to complete");

                thread::sleep(time::Duration::from_millis(5));
            }

            Ok(x)
        })
    }
}

pub type SleepMode = Toggle<SLPIN, SLPOUT>;
impl StateConstants<Switch> for SleepMode {
    const ID: &'static str = "Sleep Mode";
    const INITIAL: Switch = Switch::Off;
}

pub type PartialMode = Toggle<PTLON, NORON>;
impl StateConstants<Switch> for PartialMode {
    const ID: &'static str = "Partial Mode";
    const INITIAL: Switch = Switch::Off;
}

/**
    ## INVERT PICTURE

    Causes the image displayed on the LCD to have its colors inverted (INVON)
    or display normally (INVOFF).
*/
pub type InvertPicture = Toggle<INVON, INVOFF>;
impl StateConstants<Switch> for InvertPicture {
    const ID: &'static str = "Invert Picture";
    const INITIAL: Switch = Switch::Off;
}

pub type AllPixelsWhite = Momentary<ALLPON>;

pub type AllPixelsBlack = Momentary<ALLPOFF>;

pub type GammaCurve = Configure<GAMSET>;
impl StateConstants<gamma::Curve> for GammaCurve {
    const ID: &'static str = "Gamma Curve";
    const INITIAL: gamma::Curve = gamma::Curve::GC3;
}

pub type DisplayOutput = Toggle<DISPON, DISPOFF>;
impl StateConstants<Switch> for DisplayOutput {
    const ID: &'static str = "Display Output";
    const INITIAL: Switch = Switch::Off;
}


pub type TearingEffect = Select<TEON, TEOFF>;
impl StateConstants<Option<(tearing_effect::Blank,)>> for TearingEffect {
    const ID: &'static str = "Tearing Effect";
    const INITIAL: Option<(tearing_effect::Blank,)> = None;
}


/// # DISPLAY DATA ACCESS CONTROL
/// * [ML] - Scan direction
/// * []
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |   --   |   --   |   ML   |   CO   |   --   |   --   |   --   |
pub type DataAccessControl = Configure<MADCTL>;
impl StateConstants<<MADCTL as Data>::Parameters> for DataAccessControl {
    const ID: &'static str = "Data Access Control";
    const INITIAL: <MADCTL as Data>::Parameters = (data_access::ScanDirection::Normal, data_access::ColorOrder::RGB);
}

/**
  ## IDLE MODE
  Turns off Idle Mode. Display is capable of its full 16.7 million color palette
  Turns on Idle Mode. In idle mode the color palette is significantly  reduced.
  The MSB of each color will be rounded up or down, creating a  palette limited
  to 8 colors.
*/
pub type IdleMode = Toggle<IDMON, IDMOFF>;
impl StateConstants<Switch> for IdleMode {
    const ID: &'static str = "Idle Mode";
    const INITIAL: Switch = Switch::Off;
}

/// # SET INTERFACE PIXEL FORMAT
///
/// Defines the format for RGB pixel data.
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |          BPP[2:0]        |   --   |   --   |   --   |   --   |
pub type ColorMode = Configure<COLMOD>;
impl StateConstants<<COLMOD as Data>::Parameters> for ColorMode {
    const ID: &'static str = "Color Mode";
    const INITIAL: <COLMOD as Data>::Parameters = (pixel_format::BitsPerPixel::RGB888,);
}

/// # SET DISPLAY BRIGHTNESS
///
/// Change the display brightness to an 8-bit value.
///
/// 0x00: Lowest brightness
/// 0xFF: Hightest brightness
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|                     Display Brightness Value [7:0]                    |
pub type Brightness = Configure<WRDISBV>;
impl StateConstants<<WRDISBV as Data>::Parameters> for Brightness {
    const ID: &'static str = "Brightness";
    const INITIAL: <WRDISBV as Data>::Parameters = (0x00,);
}

/// # WRITE CTRL DISPLAY
///
/// This command changes more general behavior of the brightness controls.
///
/// [BCTRL] Brightness control on or off
/// [DD] Display dimming (only affects manual brightness settings)
/// [BL] Backlight control on or off
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |   --   |  BCTRL |   --   |   DD   |   BL   |   --   |   --   |

pub type BrightnessControl = Configure<WRCTRLD>;
impl StateConstants<<WRCTRLD as Data>::Parameters> for BrightnessControl {
    const ID: &'static str = "Brightness Control";
    const INITIAL: <WRCTRLD as Data>::Parameters = (brightness::Control::Off, brightness::Dimming::Off, brightness::Backlight::Off);
}

/// # WRITE CONTENT ADAPTIVE BRIGHTNESS CONTROL AND COLOR ENHANCEMENT
///
/// Set parameters for content-based adaptive brightness control, set
/// different color enhancement modes.
///
/// [CE] Color enhancement on or off:
/// [CEMD] Color enhancement mode
/// [CABC] Adaptive brightness control
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   CE   |   --   |    CEMD[1:0]    |   --   |   --   |    CABC[1:0]    |
pub type ColorEnhancement = Configure<WRCACE>;
impl StateConstants<<WRCACE as Data>::Parameters> for ColorEnhancement {
    const ID: &'static str = "Color Enhancement";
    const INITIAL: <WRCACE as Data>::Parameters = (color::Enhancement::Off, color::EnhanceLevel::Low, AdaptiveBrightness::Off);
}

///
/// WRITE CABC MINIMUM BRIGHTNESS
///
/// Sets the minimum brightness value to be used for CABC (see WRCACE).
///
/// [MBV] Minimum Brightness Value
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|                     Minimum Brightness Value [7:0]                    |
pub type MinimumBrightness = Configure<WRCABCMB>;
impl StateConstants<<WRCABCMB as Data>::Parameters> for MinimumBrightness {
    const ID: &'static str = "Minimum Brightness";
    const INITIAL: <WRCABCMB as Data>::Parameters = (0x00,);
}


/// # SET COMMAND2 MODE
/// This is one of the most confusing attributes of the Sitronix chips.
/// BK0, BK1, and BK3 (maybe) all have "Command2" operations that share a
/// common address space. To avoid collisions and to ensure you're sending
/// the command you think you're sending, we use a double-entry bookkeeping
/// approach, where set_command_2 will send the chip the updated Command2
/// setting AND record it back to the local flag, which is required for
/// static type checking in all Command2 operations locally.
pub type SetExtendedCommand = Configure<CND2BKXSEL>;
impl StateConstants<<CND2BKXSEL as Data>::Parameters> for SetExtendedCommand {
    const ID: &'static str = "Set Extended Command";
    const INITIAL: <CND2BKXSEL as Data>::Parameters = (register::ExtendedCommands::Off, register::Bank::BK0);
}

/// # POSITIVE GAMMA CONTROL
/// See note above about parameters
pub const fn positive_gamma_control(
    parameters: [u8; PVGAMCTRL::BYTES],
) -> Operation<{ PVGAMCTRL::BYTES }> {
    Operation::write::<PVGAMCTRL>(parameters)
}

/// # POSITIVE GAMMA CONTROL
/// See note above about parameters
pub const fn negative_gamma_control(
    parameters: [u8; NVGAMCTRL::BYTES],
) -> Operation<{ NVGAMCTRL::BYTES }> {
    Operation::write::<NVGAMCTRL>(parameters)
}

/// # DISPLAY LINE SETTING
pub const fn display_line_setting(
    lde_en: u8,
    line: u8,
    line_delta: u8,
) -> Operation<{ LNESET::BYTES }> {
    Operation::write::<LNESET>([lde_en | line, line_delta])
}

/// # PORCH CONTROL
pub const fn porch_control(mode: &Mode) -> Operation<{ PORCTRL::BYTES }> {
    let front_porch: u8 = (mode.vtotal - mode.vsync_end) as u8;
    let back_porch: u8 = (mode.vsync_start - mode.vdisplay) as u8;

    Operation::write::<PORCTRL>([front_porch, back_porch])
}

/// # INVERSION SELECT
/// * [LINV] - the type of inversion
/// * [RTNI] - minimum number of pclk in each line
pub const fn inversion_select(nlinv: Inversion, rtni: u8) -> Operation<{ INVSET::BYTES }> {
    Operation::write::<INVSET>([nlinv as u8, rtni])
}

/// # RGB CONTROLDE/HV:RGB Mode selection
/// * [DEHV]
///     0: RGB DE mode
///     1: RGB HV mode
/// * [VSP]: Sets the signal polarity of the VSYNC pin.
///     0: Low active
///     1: High active
/// * [HSP]: Sets the signal polarity of the HSYNC pin.
///     0: Low active
///     1: High active
/// * [DP]: Sets the signal polarity of the DOTCLK pin.
///     0: The data is input on the positive edge of DOTCLK
///     1: The data is input on the negative edge of DOTCLK
/// * [EP]: Sets the signal polarity of the ENABLE pin.
///     0: The data DB23-0 is written when ENABLE = “1". Disable data write operation when ENABLE = “0”.
///     1: The data DB23-0 is written when ENABLE = “0”. Disable data write operation when ENABLE = “1”.
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|  DEHV  |   --   |   --   |   --   |   VSP  |   HSP  |   DP   |   EP   |
///|                                  HBP                                  |
///|                                  VBP                                  |
pub const fn rgb_control(
    dehv: DataEnable,
    vsp: VsyncActive,
    hsp: HsyncActive,
    dp: DataPolarity,
    ep: EnablePolarity,
    mode: &Mode,
) -> Operation<{ RGBCTRL::BYTES }> {
    let hbp: u8 = (mode.htotal - mode.hsync_end) as u8;
    let vbp: u8 = (mode.vsync_start - mode.vdisplay) as u8;

    Operation::write::<RGBCTRL>([
        dehv as u8 | vsp as u8 | hsp as u8 | dp as u8 | ep as u8,
        hbp,
        vbp,
    ])
}

/// # COLOR CONTROL
/// * [PWM]: LEDPWM polarity control.
///     0: polarity normal.
///     1: polarity reverse.
/// * [LED]: LED_ON polarity control.
///     0: polarity normal.
///     1: polarity reverse.
/// * [MDT]: RGB pixel format argument.(for 262K).See Table 17.
///     0: pixel format argument normal.
///     1: pixel collect to DB[17:0].
/// * [EPF][2:0]: end of pixel format (for 65k & 262k mode)
///     0: copy self MSB
///     1: copy G MSB
///     2: copy self LSB
///     4: FIX 0
///     5: FIX 1
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |   --   |   PWM  |   LED  |   MDT  |            EPF           |
pub const fn color_control(
    pwm: PWMPolarity,
    led: LEDPolarity,
    mdt: PixelPinout,
    epf: EndPixelFormat,
) -> Operation<{ COLCTRL::BYTES }> {
    Operation::write::<COLCTRL>([pwm as u8 | led as u8 | mdt as u8 | epf as u8])
}

/// # CONFIGURE SUNLIGHT READABLE ENHANCEMENT MODE
///
/// Sets the minimum brightness value to be used for CABC (see WRCACE).
///
/// [MBV] Minimum Brightness Value
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |   --   |   --   |   --   |   SRE  |      SRE_alpha[3:0]      |
pub const fn configure_sunlight_ehancement(
    sre: SunlightReadable,
    mut sre_alpha: u8,
) -> Operation<{ SECTRL::BYTES }> {
    if sre_alpha > 0x0F {
        sre_alpha = 0x0F;
    }

    Operation::write::<SECTRL>([sre as u8 | sre_alpha])
}

// pub const fn set_vop_amplitude(vrha: u8) -> Operation<{VRHS::BYTES}> {
//     Operation::write::<VRHS>([vrha])
// }

pub const fn set_vcom_amplitude(vcom: u8) -> Operation<{ VCOMS::BYTES }> {
    Operation::write::<VCOMS>([vcom])
}

pub const fn set_vgh_voltage(vgh: u8) -> Operation<{ VGHSS::BYTES }> {
    Operation::write::<VGHSS>([vgh])
}
pub const fn test_command_setting() -> Operation<{ TESTCMD::BYTES }> {
    Operation::write::<TESTCMD>([0x80])
}

pub const fn set_vgl_voltage(vgls: u8) -> Operation<{ VGLS::BYTES }> {
    Operation::write::<VGLS>([0x40 | vgls])
}

pub const fn power_control_one(
    ap: GammaOPBias,
    apis: SourceOPInput,
    apos: SourceOPOutput,
) -> Operation<{ PWCTRL1::BYTES }> {
    Operation::write::<PWCTRL1>([ap as u8 | apis as u8 | apos as u8])
}

pub const fn power_control_two(
    avdd: VoltageAVDD,
    avcl: VoltageAVCL,
) -> Operation<{ PWCTRL2::BYTES }> {
    Operation::write::<PWCTRL2>([avdd as u8 | avcl as u8])
}

/// # SET SOURCE PRE DRIVE TIMING CONTROL
/// T2D [3:0]: source pre_drive timing setting.(GND to VDD)
/// Adjust Range : 0 ~ 3 uS 1 step is 0.2uS
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |    1   |    1   |    1   |                T2D                |
pub const fn set_pre_drive_timing_one(t2d: u8) -> Operation<{ SPD1::BYTES }> {
    Operation::write::<SPD1>([0x70 | t2d])
}

/// # SET SOURCE PRE DRIVE TIMING CONTROL
/// Same parameters as SPD1
pub const fn set_pre_drive_timing_two(t2d: u8) -> Operation<{ SPD2::BYTES }> {
    Operation::write::<SPD2>([0x70 | t2d])
}
