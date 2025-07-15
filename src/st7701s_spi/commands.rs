use crate::st7701s_spi::{
    interface::{BK0, BK1, Command, Core, Instruction, Transmission},
    panel::Mode,
    parameters::{
        AdaptiveBrightness, Backlight, BitsPerPixel, BrightnessControl, ColorOrder, DataEnable,
        DataPolarity, DisplayDimming, EnablePolarity, EndPixelFormat, Enhancement, EnhancementMode,
        GammaCurve, GammaOPBias, HsyncActive, Inversion, LEDPolarity, PWMPolarity, PixelPinout,
        ScanDirection, SourceOPInput, SourceOPOutput, SunlightReadable, TearingEffect, VoltageAVCL,
        VoltageAVDD, VsyncActive,
    },
    state::{Toggle, ToggleCommands},
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
// pub struct OldCommand {
//     pub address: u8,
//     pub parameters: Vec<u8>,
// }

// impl OldCommand {
//     fn new(address: u8) -> OldCommand {
//         OldCommand {
//             address,
//             parameters: Vec::new(),
//         }
//     }

//     fn arg(mut self, arg: u8) -> OldCommand {
//         self.parameters.push(arg);
//         self
//     }

//     fn args(mut self, args: &[u8]) -> OldCommand {
//         self.parameters.extend_from_slice(args);
//         self
//     }

//     pub fn serialize_address(&self) -> [u8; 2] {
//         [self.address, 0x00]
//     }

//     pub fn serialize_parameter(parameter: u8) -> [u8; 2] {
//         [parameter, 0x01]
//     }
// }

#[repr(u8)]
#[rustfmt::skip]
pub enum ExtensionRegister {
    Disabled = 0x00,
    BK0 = 0x10,
    BK1 = 0x11,
    BK3 = 0x13,
}

//  const fn encode<S>(mut packets: [u8; S])-> [u16; S]  {
//     const COMMAND_BIT: u16 = 0b0_0000_0000;
//     let mut transmission: [u16; S] = [0x00; S];

//     transmission
// }

// pub type Address = &'static u8;
// pub type Parameters<const S: usize> = [u8; S];

// pub struct Command(Address);
// pub struct Transmission<const S: usize>(Address, Parameters<S>);

/// # NO OPERATION
///
/// This command is "empty". It has no effect on the display, but it can be
/// used to terminate parameter write commands.
const fn no_operation() -> Command {
    Instruction::Core(Core::NOP).to_command()
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
pub const fn software_reset() -> Transmission<1> {
    const RESET_DATA: [u8; 1] = [0b0000_0001];

    Instruction::Core(Core::SWRESET).to_write(RESET_DATA)
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
pub const fn sleep_mode(mode: Toggle) -> Command {
    const SLEEP: ToggleCommands = ToggleCommands::new(
        Instruction::Core(Core::SLPIN),
        Instruction::Core(Core::SLPOUT),
    );

    SLEEP.toggle(mode)
}

/// # PARTIAL MODE ON
///
/// This command turns on Partial Mode. See PARTIAL AREA (30h) command.
pub const fn partial_mode(mode: Toggle) -> Command {
    const PARTIAL: ToggleCommands = ToggleCommands::new(
        Instruction::Core(Core::PTLON),
        Instruction::Core(Core::NORON),
    );

    PARTIAL.toggle(mode)
}

/// # DISPLAY INVERSION OFF (DEFAULT)
///
/// This command restores normal pixel values.
pub const fn invert_display(mode: Toggle) -> Command {
    const INVERT: ToggleCommands = ToggleCommands::new(
        Instruction::Core(Core::PTLON),
        Instruction::Core(Core::NORON),
    );

    INVERT.toggle(mode)
}

/// # ALL PIXELS OFF (BLACK)
///
/// This command sets all pixel values to black.
///
/// ALLPOFF may be used in Sleep Mode, Normal Mode, or Partial Mode.
pub const fn all_pixels_off() -> Command {
    Instruction::Core(Core::ALLPOFF).to_command()
}

/// # ALL PIXELS ON (WHITE)
///
/// This command sets all pixel values to white.
///
/// ALLPOFF may be used in Sleep Mode, Normal Mode, or Partial Mode.
pub const fn all_pixels_on() -> Command {
    Instruction::Core(Core::ALLPON).to_command()
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
pub const fn gamma_curve_select(gc: GammaCurve) -> Transmission<1> {
    let data = match gc {
        GammaCurve::One => 0x01,
        GammaCurve::Two => 0x02,
        GammaCurve::Three => 0x04,
        GammaCurve::Four => 0x08,
    };
    Instruction::Core(Core::GAMSET).to_write([data])
}

/// # DISPLAY OFF (DEFAULT?)
///
/// This command is used to enter Display Off Mode. In this mode, display
/// data is disabled and all pixels are blanked.
///
/// NOTE: It's possible that this is the default value.
pub const fn display_output(mode: Toggle) -> Command {
    const OUTPUT: ToggleCommands = ToggleCommands::new(
        Instruction::Core(Core::DISPON),
        Instruction::Core(Core::DISPOFF),
    );

    OUTPUT.toggle(mode)
}

pub const fn tearing_effect(te: Option<TearingEffect>) -> Transmission {
    if let Some(x) = te {
        let data: u8 = match x {
            TearingEffect::VBlank => 0x00,
            TearingEffect::VHBlank => 0x01,
        };

        Instruction::Core(Core::TEON).to_write([data])
    } else {
        Instruction::Core(Core::TEOFF).to_command()
    }
}

/// # DISPLAY DATA ACCESS CONTROL
/// * [ML] - Scan direction
/// * []
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |   --   |   --   |   ML   |   CO   |   --   |   --   |   --   |
pub const fn display_data_control(ml: ScanDirection, co: ColorOrder) -> Transmission<1> {
    Instruction::Core(Core::MADCTL).to_write([ml as u8 | co as u8])
}

/// # IDLE MODE OFF
///
/// Turns off Idle Mode. Display is capable of its full 16.7 million color
/// palette
pub const fn idle_mode_off() -> Command {
    Instruction::Core(Core::IDMOFF).to_command()
}

/// # IDLE MODE ON
///
/// Turns on Idle Mode. In idle mode the color palette is significantly
/// reduced. The MSB of each color will be rounded up or down, creating a
/// palette limited to 8 colors.
pub const fn idle_mode_on() -> Command {
    Instruction::Core(Core::IDMON).to_command()
}

/// # SET INTERFACE PIXEL FORMAT
///
/// Defines the format for RGB pixel data.
///
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |          BPP[2:0]        |   --   |   --   |   --   |   --   |
pub const fn set_color_mode(bpp: BitsPerPixel) -> Transmission<1> {
    Instruction::Core(Core::COLMOD).to_write([bpp as u8])
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
pub const fn set_display_brightness(dbv: u8) -> Transmission<1> {
    Instruction::Core(Core::WRDISBV).to_write([dbv as u8])
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
) -> Transmission<1> {
    Instruction::Core(Core::WRCTRLD).to_write([bctrl as u8 | dd as u8 | bl as u8])
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
) -> Transmission<1> {
    Instruction::Core(Core::WRCACE).to_write([ce as u8 | cemd as u8 | cabc as u8])
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
pub const fn set_minimum_brightness(mbv: u8) -> Transmission<1> {
    Instruction::Core(Core::WRCABCMB).to_write([mbv as u8])
}

pub const fn read_display_pixel_format() -> Command {
    Instruction::Core(Core::RDDCOLMOD).to_command()
}

pub const fn read_self_diagnostics() -> Command {
    Instruction::Core(Core::RDDSDR).to_command()
}

/// # SET COMMAND2 MODE
/// This is one of the most confusing attributes of the Sitronix chips.
/// BK0, BK1, and BK3 (maybe) all have "Command2" instructions that share a
/// common address space. To avoid collisions and to ensure you're sending
/// the command you think you're sending, we use a double-entry bookkeeping
/// approach, where set_command_2 will send the chip the updated Command2
/// setting AND record it back to the local flag, which is required for
/// static type checking in all Command2 instructions locally.
///
/// eg. for a BK1 Command2 instruction, "current" must be set to
/// Command2Selection::BK1.
// pub const fn set_command_2(set: ExtensionRegister) -> Transmission<5> {
//     Instruction::Core(
//         Core::CND2BKxSEL).to_write(
//         [0x77, 0x01, 0x00, 0x00, set as u8],
//     ))
// }

/// # POSITIVE GAMMA CONTROL
/// See note above about parameters
pub const fn positive_gamma_control(
    cmd2: &ExtensionRegister,
    parameters: [u8; 1],
) -> Transmission<1> {
    Instruction::BK0(BK0::PVGAMCTRL).to_write(parameters)
}

/// # POSITIVE GAMMA CONTROL
/// See note above about parameters
pub const fn negative_gamma_control(
    cmd2: &ExtensionRegister,
    parameters: [u8; 1],
) -> Transmission<1> {
    Instruction::BK0(BK0::NVGAMCTRL).to_write(parameters)
}

/// # DISPLAY LINE SETTING
pub const fn display_line_setting(
    cmd2: &ExtensionRegister,
    lde_en: u8,
    line: u8,
    line_delta: u8,
) -> Transmission<2> {
    Instruction::BK0(BK0::LNESET).to_write([lde_en | line, line_delta])
}

/// # PORCH CONTROL
pub const fn porch_control(cmd2: &ExtensionRegister, mode: &Mode) -> Transmission<2> {
    let front_porch: u8 = (mode.vtotal - mode.vsync_end) as u8;
    let back_porch: u8 = (mode.vsync_start - mode.vdisplay) as u8;

    Instruction::BK0(BK0::PORCTRL).to_write([front_porch, back_porch])
}

/// # INVERSION SELECT
/// * [LINV] - the type of inversion
/// * [RTNI] - minimum number of pclk in each line
pub const fn inversion_select(
    cmd2: &ExtensionRegister,
    nlinv: Inversion,
    rtni: u8,
) -> Transmission<2> {
    Instruction::BK0(BK0::INVSET).to_write([nlinv as u8, rtni])
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
    cmd2: &ExtensionRegister,
    dehv: DataEnable,
    vsp: VsyncActive,
    hsp: HsyncActive,
    dp: DataPolarity,
    ep: EnablePolarity,
    mode: &Mode,
) -> Transmission<3> {
    let hbp: u8 = (mode.htotal - mode.hsync_end) as u8;
    let vbp: u8 = (mode.vsync_start - mode.vdisplay) as u8;

    Instruction::BK0(BK0::RGBCTRL).to_write([
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
    cmd2: &ExtensionRegister,
    pwm: PWMPolarity,
    led: LEDPolarity,
    mdt: PixelPinout,
    epf: EndPixelFormat,
) -> Transmission<1> {
    Instruction::BK0(BK0::COLCTRL).to_write([pwm as u8 | led as u8 | mdt as u8 | epf as u8])
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
    cmd2: &ExtensionRegister,
    sre: SunlightReadable,
    mut sre_alpha: u8,
) -> Transmission<1> {
    if sre_alpha > 0x0F {
        sre_alpha = 0x0F;
    }

    Instruction::BK0(BK0::SECTRL).to_write([sre as u8 | sre_alpha])
}

pub const fn set_vop_amplitude(cmd2: &ExtensionRegister, vrha: u8) -> Transmission<1> {
    Instruction::BK1(BK1::VRHS).to_write([vrha])
}

pub const fn set_vcom_amplitude(cmd2: &ExtensionRegister, vcom: u8) -> Transmission<1> {
    Instruction::BK1(BK1::VCOMS).to_write([vcom])
}

pub const fn set_vgh_voltage(cmd2: &ExtensionRegister, vgh: u8) -> Transmission<1> {
    Instruction::BK1(BK1::VGHSS).to_write([vgh])
}
pub const fn test_command_setting(cmd2: &ExtensionRegister) -> Transmission<1> {
    Instruction::BK1(BK1::TESTCMD).to_write([0x80])
}

pub const fn set_vgl_voltage(cmd2: &ExtensionRegister, vgls: u8) -> Transmission<1> {
    Instruction::BK1(BK1::VGLS).to_write([0x40 | vgls])
}

pub const fn power_control_one(
    cmd2: &ExtensionRegister,
    ap: GammaOPBias,
    apis: SourceOPInput,
    apos: SourceOPOutput,
) -> Transmission<1> {
    Instruction::BK1(BK1::PWCTRL1).to_write([ap as u8 | apis as u8 | apos as u8])
}

pub const fn power_control_two(
    cmd2: &ExtensionRegister,
    avdd: VoltageAVDD,
    avcl: VoltageAVCL,
) -> Transmission<1> {
    Instruction::BK1(BK1::PWCTRL2).to_write([avdd as u8 | avcl as u8])
}

/// # SET SOURCE PRE DRIVE TIMING CONTROL
/// T2D [3:0]: source pre_drive timing setting.(GND to VDD)
/// Adjust Range : 0 ~ 3 uS 1 step is 0.2uS
///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
///|   --   |    1   |    1   |    1   |                T2D                |
pub const fn set_pre_drive_timing_one(cmd2: &ExtensionRegister, t2d: u8) -> Transmission<1> {
    Instruction::BK1(BK1::SPD1).to_write([0x70 | t2d])
}

/// # SET SOURCE PRE DRIVE TIMING CONTROL
/// Same parameters as SPD1
pub const fn set_pre_drive_timing_two(cmd2: &ExtensionRegister, t2d: u8) -> Transmission<1> {
    Instruction::BK1(BK1::SPD2).to_write([0x70 | t2d])
}
