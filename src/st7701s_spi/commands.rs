use std::{thread, time};

use crate::st7701s_spi::{
    interface::{bk0::*, bk1::*, core::*, *}, panel::Mode, parameters::*, spi::ST7701S, state::{toggle, Switch}
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

impl ST7701S {
    /**
    ## NO OPERATION

    This command is "empty". It has no effect on the display, but it can be used
    to terminate parameter write commands.
    */
    pub fn no_operation(&mut self) {
        self.command::<NOP>();
    }

    pub fn software_reset(&mut self) {
        self.write::<SWRESET>(());

        if self.state.is_some_and(|state| !matches(state.sleep_mode, Switch::On)) {
            thread::sleep(time::Duration::from_millis(120));
        } else {
            thread::sleep(time::Duration::from_millis(120));
        }
    }
}

/// # SOFTWARE RESET
///
/// The display module performs a software reset. Registers are written with
/// the default "reset" values.
///
///   - Frame buffer contents are unaffected by this command
///   - After a SWRESET command, sleep at least 5ms before the next command
///   - If the display is sleeping when a SWRESET is sent, the sleep
///     duration should be at least 120ms before sending the next command.
///   - SWRESET cannot be sent during SLPOUT
///   - (MIPI ONLY) Send a shutdown packet before SWRESET
pub const fn software_reset() -> Operation<{ SWRESET::BYTES }> {
    Operation::write::<SWRESET>(SWRESET::encode_data())
}

/// # SLEEP IN
///
/// This command causes the display module to enter a minimum power state.
/// The buck converter, display oscilator, and panel scanning are all shut
/// down.
///
/// The control interface, display data, and registers remain active.
///
/// The driver may send PCLK, HS, and CS information after SLPIN, and this
/// data will be valid for the next two frames if Normal Mode is active.
///
/// Dimming will not work when changing from sleep out to sleep in.
///
/// Normally, sleep state can be read with RDDST, but MISO must be connected.
///
pub const fn sleep_mode(mode: Switch) -> Operation<0> {
    toggle::<SLPIN, SLPOUT>(mode)
}

/// # PARTIAL MODE ON
///
/// This command turns on Partial Mode. See PARTIAL AREA (30h) command.
pub const fn partial_mode(mode: Switch) -> Operation<0> {
    toggle::<PTLON, NORON>(mode)
}

/// # DISPLAY INVERSION OFF (DEFAULT)
///
/// This command restores normal pixel values.
pub const fn invert_display(mode: Switch) -> Operation<0> {
    toggle::<INVON, INVOFF>(mode)
}

/// # ALL PIXELS OFF (BLACK)
///
/// This command sets all pixel values to black.
///
/// ALLPOFF may be used in Sleep Mode, Normal Mode, or Partial Mode.
pub const fn all_pixels_black() -> Operation<0> {
    Operation::command::<ALLPOFF>()
}

/// # ALL PIXELS ON (WHITE)
///
/// This command sets all pixel values to white.
///
/// ALLPOFF may be used in Sleep Mode, Normal Mode, or Partial Mode.
pub const fn all_pixels_white() -> Operation<0> {
    Operation::command::<ALLPON>()
}

/// # GAMMA CURVE SELECT
///
/// This command selects a predefined gamma curve from one of four values.
///
/// WARNING: It's not clear from the Sitronix documentation what any values
/// are aside from 01.
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |   --   |   --   |   --   |   --   |         GC[3:0]          |
pub const fn gamma_curve_select(gc: gamma::Curve) -> Operation<{ GAMSET::BYTES }> {
    Operation::write::<GAMSET>(GAMSET::encode_data((gc,)))
}

/// # DISPLAY OFF (DEFAULT?)
///
/// This command is used to enter Display Off Mode. In this mode, display
/// data is disabled and all pixels are blanked.
///
/// NOTE: It's possible that this is the default value.
pub const fn display_output(mode: Switch) -> Operation<0> {
    toggle::<DISPON, DISPOFF>(mode)
}

pub const fn tearing_effect(te: Option<tearing_effect::Blank>) -> Operation<{ TEON::BYTES }> {
    if let Some(p1) = te {
        Operation::write::<TEON>(TEON::encode_data((p1,)))
    } else {
        Operation::command::<TEOFF>()
    }
}

/// # DISPLAY DATA ACCESS CONTROL
/// * [ML] - Scan direction
/// * []
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |   --   |   --   |   ML   |   CO   |   --   |   --   |   --   |
pub const fn display_data_control(
    ml: data_access::ScanDirection,
    co: data_access::ColorOrder,
) -> Operation<{ MADCTL::BYTES }> {
    Operation::write::<MADCTL>(MADCTL::encode_data((ml, co)))
}

/// # IDLE MODE OFF
///
/// Turns off Idle Mode. Display is capable of its full 16.7 million color
/// palette
/// Turns on Idle Mode. In idle mode the color palette is significantly
/// reduced. The MSB of each color will be rounded up or down, creating a
/// palette limited to 8 colors.
pub const fn idle_mode(mode: Switch) -> Operation<0> {
    toggle::<IDMON, IDMOFF>(mode)
}

/// # SET INTERFACE PIXEL FORMAT
///
/// Defines the format for RGB pixel data.
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |          BPP[2:0]        |   --   |   --   |   --   |   --   |
pub const fn set_color_mode(bpp: BitsPerPixel) -> Operation<{ COLMOD::BYTES }> {
    Operation::write::<COLMOD>([bpp as u8])
}

/// # WRDISBV
///
/// Change the display brightness to an 8-bit value.
///
/// 0x00: Lowest brightness
/// 0xFF: Hightest brightness
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|                     Display Brightness Value [7:0]                    |
pub const fn set_display_brightness(dbv: u8) -> Operation<{ WRDISBV::BYTES }> {
    Operation::write::<WRDISBV>([dbv as u8])
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
pub const fn configure_brightness(
    bctrl: BrightnessControl,
    dd: DisplayDimming,
    bl: Backlight,
) -> Operation<{ WRCTRLD::BYTES }> {
    Operation::write::<WRCTRLD>([bctrl as u8 | dd as u8 | bl as u8])
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
pub const fn configure_color_enhancement(
    ce: Enhancement,
    cemd: EnhancementMode,
    cabc: AdaptiveBrightness,
) -> Operation<{ WRCACE::BYTES }> {
    Operation::write::<WRCACE>([ce as u8 | cemd as u8 | cabc as u8])
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
pub const fn set_minimum_brightness(mbv: u8) -> Operation<{ WRCABCMB::BYTES }> {
    Operation::write::<WRCABCMB>([mbv as u8])
}

pub const fn read_display_pixel_format() -> Operation<0> {
    Operation::read::<RDDCOLMOD>(todo!())
}

pub const fn read_self_diagnostics() -> Operation<0> {
    Operation::read::<RDDSDR>(todo!())
}

/// # SET COMMAND2 MODE
/// This is one of the most confusing attributes of the Sitronix chips.
/// BK0, BK1, and BK3 (maybe) all have "Command2" operations that share a
/// common address space. To avoid collisions and to ensure you're sending
/// the command you think you're sending, we use a double-entry bookkeeping
/// approach, where set_command_2 will send the chip the updated Command2
/// setting AND record it back to the local flag, which is required for
/// static type checking in all Command2 operations locally.
pub const fn select_command_extension(
    cn2: Switch,
    bkxsel: Bank,
) -> Operation<{ CND2BKXSEL::BYTES }> {
    Operation::write::<CND2BKXSEL>(CND2BKXSEL::encode_data((cn2,bkxsel,)))
}

/// # POSITIVE GAMMA CONTROL
/// See note above about parameters
pub const fn positive_gamma_control(
    parameters: [u8;  PVGAMCTRL::BYTES ],
) -> Operation<{ PVGAMCTRL::BYTES }> {
    Operation::write::<PVGAMCTRL>(parameters)
}

/// # POSITIVE GAMMA CONTROL
/// See note above about parameters
pub const fn negative_gamma_control(
    parameters: [u8; NVGAMCTRL::BYTES ],
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

pub const fn set_vcom_amplitude(vcom: u8) -> Operation<{VCOMS::BYTES}> {
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
