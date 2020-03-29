use std::convert::TryInto;

use crate::panel::Mode;

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
#[derive(Clone)]
pub struct Command {
    pub address: u8,
    pub parameters: Vec<u8>,
}

impl Command {
    fn new(address: u8) -> Command {
        Command {
            address: address,
            parameters: Vec::new(),
        }
    }

    fn arg(mut self, arg: u8) -> Command {
        self.parameters.push(arg);
        self
    }

    fn args(mut self, args: &[u8]) -> Command {
        self.parameters.extend_from_slice(args);
        self
    }

    pub fn serialize_address(&self) -> [u8; 2] {
        [(self.address as u8), 0x00]
    }

    pub fn serialize_parameter(parameter: u8) -> [u8; 2] {
        [parameter, 0x01]
    }
}

#[derive(Copy, Clone)]
pub enum GammaCurve {
    /// Gamma Curve 1 (G=2.2)
    One = 0x01,
    /// Reserved
    Two = 0x02,
    /// Reserved
    Three = 0x04,
    /// Reserved
    Four = 0x08,
}

#[derive(Copy, Clone)]
pub enum TearingEffect {
    /// V-blanking only
    VBlank = 0x00,
    /// V-blanking and H-blanking
    VHBlank = 0x01,
}

#[derive(Copy, Clone)]
pub enum DataEnable {
    DE = 0x00,
    HV = 0x80,
}

#[derive(Copy, Clone)]
pub enum VsyncActive {
    Low = 0x00,
    High = 0x08,
}
#[derive(Copy, Clone)]
pub enum HsyncActive {
    Low = 0x00,
    High = 0x04,
}
#[derive(Copy, Clone)]
pub enum DataPolarity {
    Rising = 0x00,
    Falling = 0x02,
}
#[derive(Copy, Clone)]
pub enum EnablePolarity {
    Low = 0x00,
    High = 0x01,
}

#[derive(Copy, Clone)]
pub enum PWMPolarity {
    Low = 0x00,
    High = 0x20,
}

#[derive(Copy, Clone)]
pub enum LEDPolarity {
    Low = 0x00,
    High = 0x10,
}

#[derive(Copy, Clone)]
pub enum PixelPinout {
    Normal = 0x00,
    Condensed = 0x08,
}
#[derive(Copy, Clone)]
pub enum EndPixelFormat {
    SelfMSB = 0x00,
    GreenMSB = 0x01,
    SelfLSB = 0x02,
    Zero = 0x04,
    One = 0x05,
}

#[derive(Copy, Clone)]
pub enum ScanDirection {
    Normal = 0x00,
    Reverse = 0x10,
}

#[derive(Copy, Clone)]
pub enum ColorOrder {
    /// RGB mode
    Rgb = 0x00,
    /// BGR mode
    Bgr = 0x08,
}

#[derive(Copy, Clone)]
pub enum BitsPerPixel {
    /// 16 bits per pixel (RGB565)
    Rgb565 = 0x50,
    /// 18 bits per pixel (RGB666)
    Rgb666 = 0x60,
    /// 24 bits per pixel (RGB888)
    Rgb888 = 0x70,
}

#[derive(Copy, Clone)]
pub enum BrightnessControl {
    /// Ignore display brightness value and soft-set it to 0x00
    Off = 0x00,
    /// Use display brightness value normally
    On = 0x20,
}

#[derive(Copy, Clone)]
pub enum DisplayDimming {
    /// Ignore display brightness value and soft-set it to 0x00
    Off = 0x00,
    /// Use display brightness value normally
    On = 0x08,
}

#[derive(Copy, Clone)]
pub enum Backlight {
    /// Disable backlight circuit. Control lines must be low.
    Off = 0x00,
    /// Enable backlight circuit. Normal behavior.
    On = 0x04,
}

#[derive(Copy, Clone)]
pub enum Enhancement {
    /// Disable color enhancement
    Off = 0x00,
    /// Enable color enhancement
    On = 0x80,
}

#[derive(Copy, Clone)]
pub enum EnhancementMode {
    Low = 0x00,
    Medium = 0x10,
    High = 0x30,
}

#[derive(Copy, Clone)]
pub enum AdaptiveBrightness {
    /// Off
    Off = 0x00,
    /// User Interface Mode
    UserInterface = 0x01,
    /// Still Picture Mode
    StillPicture = 0x02,
    /// Moving Image Mode
    MovingImage = 0x03,
}

#[derive(Copy, Clone)]
pub enum Inversion {
    OneDot = 0x00,
    TwoDot = 0x01,
    Column = 0x07,
}

#[derive(Copy, Clone)]
pub enum GammaOPBias {
    Off = 0x00,
    Min = 0x40,
    Middle = 0x80,
    Max = 0xC0,
}

#[derive(Copy, Clone)]
pub enum SourceOPInput {
    Off = 0x00,
    Min = 0x04,
    Middle = 0x08,
    Max = 0x0C,
}

#[derive(Copy, Clone)]
pub enum SourceOPOutput {
    Off = 0x00,
    Min = 0x01,
    Middle = 0x02,
    Max = 0x03,
}

#[derive(Copy, Clone)]
pub enum VoltageAVDD {
    Pos6_2 = 0x00,
    Pos6_4 = 0x10,
    Pos6_6 = 0x20,
    Pos6_8 = 0x30,
}

#[derive(Copy, Clone)]
pub enum VoltageAVCL {
    Neg4_4 = 0x00,
    Neg4_6 = 0x01,
    Neg4_8 = 0x02,
    Neg5_0 = 0x03,
}

#[derive(Copy, Clone)]
pub enum SunlightReadable {
    /// DEFAULT: Sunlight readable mode off
    Off = 0x00,
    /// Enable sunlight readable mode
    On = 0x10,
}

#[derive(PartialEq)]
pub enum Command2Selection {
    Disabled = 0x00,
    BK0 = 0x10,
    BK1 = 0x11,
}

#[derive(Copy, Clone)]
pub enum CommandsGeneral {
    NOP = 0x00,        // No-op
    SWRESET = 0x01,    // Software Reset
    RDDID = 0x04,      // Read Display ID
    RDNUMED = 0x05,    // Read Number of Errors on DSI
    RDRED = 0x06,      // Read the first pixel of Red Color
    RDGREEN = 0x07,    // Read the first pixel of Green Color
    RDBLUE = 0x08,     // Read the first pixel of Blue Color
    RDDPM = 0x0A,      // Read Display Power Mode
    RDDMADCTL = 0x0B,  // Read Display MADCTL
    RDDCOLMOD = 0x0C,  // Read Display Pixel Format
    RDDIM = 0x0D,      // Read Display Image Mode
    RDDSM = 0x0E,      // Read Display Signal Mode
    RDDSDR = 0x0F,     // Read Display Self-Diagnostic Result
    SLPIN = 0x10,      // Sleep in
    SLPOUT = 0x11,     // Sleep Out
    PTLON = 0x12,      // Partial Display Mode On
    NORON = 0x13,      // Normal Display Mode On
    INVOFF = 0x20,     // Display Inversion Off
    INVON = 0x21,      // Display Inversion On
    ALLPOFF = 0x22,    // All Pixel Off
    ALLPON = 0x23,     // All Pixel ON
    GAMSET = 0x26,     // Gamma Set
    DISPOFF = 0x28,    // Display Off
    DISPON = 0x29,     // Display On
    TEOFF = 0x34,      // Tearing Effect Line OFF
    TEON = 0x35,       // Tearing Effect Line ON
    MADCTL = 0x36,     // Display data access control
    IDMOFF = 0x38,     // Idle Mode Off
    IDMON = 0x39,      // Idle Mode On
    COLMOD = 0x3A,     // Interface Pixel Format
    GSL = 0x45,        // Get Scan Line
    WRDISBV = 0x51,    // Write Display Brightness
    RDDISBV = 0x52,    // Read Display Brightness Value
    WRCTRLD = 0x53,    // Write CTRL Display
    RDCTRLD = 0x54,    // Read CTRL Value Display
    WRCACE = 0x55,     // Write Content Adaptive Brightness Control and Color Enhancement
    RDCABC = 0x56,     // Read Content Adaptive Brightness Control
    WRCABCMB = 0x5E,   // Write CABC Minimum Brightness
    RDCABCMB = 0x5F,   // Read CABC Minimum Brightness
    RDABCSDR = 0x68,   // Read Automatic Brightness Control Self-Diagnostic Result
    RDBWLB = 0x70,     // Read Black/White Low Bits
    RDBkx = 0x71,      // Read Bkx
    RDBky = 0x72,      // Read Bky
    RDWx = 0x73,       // Read Wx
    RDWy = 0x74,       // Read Wy
    RDRGLB = 0x75,     // Read Red/Green Low Bits
    RDRx = 0x76,       // Read Rx
    RDRy = 0x77,       // Read Ry
    RDGx = 0x78,       // Read Gx
    RDGy = 0x79,       // Read Gy
    RDBALB = 0x7A,     // Read Blue/A Color Low Bits
    RDBx = 0x7B,       // Read Bx
    CND2BKxSEL = 0xFF, // Set Command2 mode for BK Register
}

#[derive(Copy, Clone)]
pub enum BK0Command2 {
    PVGAMCTRL = 0xB0, // Positive Voltage Gamma Control
    NVGAMCTRL = 0xB1, // Negative Voltage Gamma Control
    DGMEN = 0xB8,     // Digital Gamma Enable
    DGMLUTR = 0xB9,   // Digital Gamma Look-up Table for Red
    DGMLUTB = 0xBA,   // Digital Gamma Look-up Table for Blue
    PWMCLKSEL = 0xBC, // PWM CLK select
    LNESET = 0xC0,    // Display Line Setting
    PORCTRL = 0xC1,   // Porch Control
    INVSET = 0xC2,    // Inversion selection & Frame Rate Control
    RGBCTRL = 0xC3,   // RGB control
    PARCTRL = 0xC5,   // Partial Mode Control
    SDIR = 0xC7,      // X-direction Control
    PDOSET = 0xC8,    // Pseudo-Dot inversion diving setting
    COLCTRL = 0xCD,   // Color Control
    SRECTRL = 0xE0,   // Sunlight Readable Enhancement
    NRCTRL = 0xE1,    // Noise Reduce Control
    SECTRL = 0xE2,    // Sharpness Control
    CCCTRL = 0xE3,    // Color Calibration Control
    SKCTRL = 0xE4,    // Skin Tone Preservation CONTROL
}

#[derive(Copy, Clone)]
pub enum BK1Command2 {
    VRHS = 0xB0,     // Vop Amplitude setting
    VCOMS = 0xB1,    // VCOM amplitude setting
    VGHSS = 0xB2,    // VGH Voltage setting
    TESTCMD = 0xB3,  // TEST Command Setting
    VGLS = 0xB5,     // VGL Voltage setting
    PWCTRL1 = 0xB7,  // Power Control 1
    PWCTRL2 = 0xB8,  // Power Control 2
    PCLKS1 = 0xBA,   // Power pumping clk selection 1
    PCLKS3 = 0xBC,   // Power pumping clk selection 3
    SPD1 = 0xC1,     //  Source pre_drive timing set1
    SPD2 = 0xC2,     //  Source pre_drive timing set2
    MIPISET1 = 0xD0, // MIPI Setting 1
    MIPISET2 = 0xD1, // MIPI Setting 2
    MIPISET3 = 0xD2, // MIPI Setting 3
    MIPISET4 = 0xD3, // MIPI Setting 4
}

impl CommandsGeneral {
    /// # NO OPERATION
    ///
    /// This command is "empty". It has no effect on the display, but it can be
    /// used to terminate parameter write commands.
    pub fn no_operation() -> Result<Command, &'static str> {
        Ok(Command::new(Self::NOP as u8))
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
    pub fn software_reset() -> Result<Command, &'static str> {
        Ok(Command::new(Self::SWRESET as u8))
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
    pub fn sleep_mode_on() -> Result<Command, &'static str> {
        Ok(Command::new(Self::SLPIN as u8))
    }

    /// # SLEEP OUT
    ///
    /// This command turns off the minimum power state set by SLPIN.
    ///
    /// The driver may send PCLK, HS, and CS information before SLPOUT, and this
    /// data will be valid for the two frames before the command if Normal Mode
    /// is active.
    pub fn sleep_mode_off() -> Result<Command, &'static str> {
        Ok(Command::new(Self::SLPOUT as u8))
    }
    /// # PARTIAL MODE ON
    ///
    /// This command turns on Partial Mode. See PARTIAL AREA (30h) command.
    pub fn partial_mode_on() -> Result<Command, &'static str> {
        Ok(Command::new(Self::PTLON as u8))
    }
    /// # NORMAL MODE ON (DEFAULT)
    ///
    /// This command turns on Normal Mode and turns off Partial Mode.
    pub fn normal_mode_on() -> Result<Command, &'static str> {
        Ok(Command::new(Self::NORON as u8))
    }
    /// # DISPLAY INVERSION OFF (DEFAULT)
    ///
    /// This command restores normal pixel values.
    pub fn invert_display_off() -> Result<Command, &'static str> {
        Ok(Command::new(Self::INVOFF as u8))
    }
    /// # DISPLAY INVERSION ON
    ///
    /// This command inverts the display (white becomes black, red becomes blue).
    pub fn invert_display_on() -> Result<Command, &'static str> {
        Ok(Command::new(Self::INVON as u8))
    }
    /// # ALL PIXELS OFF (BLACK)
    ///
    /// This command sets all pixel values to black.
    ///
    /// ALLPOFF may be used in Sleep Mode, Normal Mode, or Partial Mode.
    pub fn all_pixels_off() -> Result<Command, &'static str> {
        Ok(Command::new(Self::ALLPOFF as u8))
    }
    /// # ALL PIXELS ON (WHITE)
    ///
    /// This command sets all pixel values to white.
    ///
    /// ALLPOFF may be used in Sleep Mode, Normal Mode, or Partial Mode.
    pub fn all_pixels_on() -> Result<Command, &'static str> {
        Ok(Command::new(Self::ALLPON as u8))
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
    pub fn gamma_curve_select(GC: GammaCurve) -> Result<Command, &'static str> {
        Ok(Command::new(Self::GAMSET as u8).arg(GC as u8))
    }
    /// # DISPLAY OFF (DEFAULT?)
    ///
    /// This command is used to enter Display Off Mode. In this mode, display
    /// data is disabled and all pixels are blanked.
    ///
    /// NOTE: It's possible that this is the default value.
    pub fn display_off() -> Result<Command, &'static str> {
        Ok(Command::new(Self::DISPOFF as u8))
    }
    /// # DISPLAY ON
    ///
    /// WARNING: I have no idea how this behaves. The Sitronix docs monkey copied
    /// and pasted the description for DISPOFF. At a guess, it should turn the
    /// display back on.
    pub fn display_on() -> Result<Command, &'static str> {
        Ok(Command::new(Self::DISPON as u8))
    }
    /// # TEARING EFFECT LINE OFF
    ///
    /// This command is used to turn off the display module's Tearing Effect
    /// output signal (vsync?) on the TE signal line (active low).
    pub fn tearing_effect_off() -> Result<Command, &'static str> {
        Ok(Command::new(Self::TEOFF as u8))
    }
    /// # TEARING EFFECT LINE ON
    ///
    /// This command is used to turn on the display module's Tearing Effect
    /// output signal line.
    ///
    ///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
    ///|   --   |   --   |   --   |   --   |   --   |   --   |   --   |   TE   |
    pub fn tearing_effect_on(TE: TearingEffect) -> Result<Command, &'static str> {
        Ok(Command::new(Self::TEON as u8).arg(TE as u8))
    }
    /// # DISPLAY DATA ACCESS CONTROL
    /// * [ML] - Scan direction
    /// * []
    ///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
    ///|   --   |   --   |   --   |   ML   |   CO   |   --   |   --   |   --   |
    pub fn display_data_control(
        ML: ScanDirection,
        CO: ColorOrder,
    ) -> Result<Command, &'static str> {
        Ok(Command::new(Self::MADCTL as u8).arg(ML as u8 | CO as u8))
    }
    /// # IDLE MODE OFF
    ///
    /// Turns off Idle Mode. Display is capable of its full 16.7 million color
    /// palette
    pub fn idle_mode_off() -> Result<Command, &'static str> {
        Ok(Command::new(Self::IDMOFF as u8))
    }
    /// # IDLE MODE ON
    ///
    /// Turns on Idle Mode. In idle mode the color palette is significantly
    /// reduced. The MSB of each color will be rounded up or down, creating a
    /// palette limited to 8 colors.
    pub fn idle_mode_on() -> Result<Command, &'static str> {
        Ok(Command::new(Self::IDMON as u8))
    }
    /// # SET INTERFACE PIXEL FORMAT
    ///
    /// Defines the format for RGB pixel data.
    ///
    ///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
    ///|   --   |          BPP[2:0]        |   --   |   --   |   --   |   --   |
    pub fn set_color_mode(BPP: BitsPerPixel) -> Result<Command, &'static str> {
        Ok(Command::new(Self::COLMOD as u8).arg(BPP as u8))
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
    pub fn set_display_brightness(DBV: u8) -> Result<Command, &'static str> {
        Ok(Command::new(Self::WRDISBV as u8).arg(DBV as u8))
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
    pub fn configure_brightness(
        BCTRL: BrightnessControl,
        DD: DisplayDimming,
        BL: Backlight,
    ) -> Result<Command, &'static str> {
        Ok(Command::new(Self::WRCTRLD as u8).arg(BCTRL as u8 | DD as u8 | BL as u8))
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
    pub fn configure_color_enhancement(
        CE: Enhancement,
        CEMD: EnhancementMode,
        CABC: AdaptiveBrightness,
    ) -> Result<Command, &'static str> {
        Ok(Command::new(Self::WRCACE as u8).arg(CE as u8 | CEMD as u8 | CABC as u8))
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
    pub fn set_minimum_brightness(MBV: u8) -> Result<Command, &'static str> {
        Ok(Command::new(Self::WRCABCMB as u8).arg(MBV as u8))
    }

    pub fn read_display_pixel_format() -> Result<Command, &'static str> {
        Ok(Command::new(Self::RDDCOLMOD as u8))
    }

    pub fn read_self_diagnostics() -> Result<Command, &'static str> {
        Ok(Command::new(Self::RDDSDR as u8))
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
    pub fn set_command_2(set: Command2Selection) -> Result<Command, &'static str> {
        Ok(Command::new(Self::CND2BKxSEL as u8).args(&[0x77, 0x01, 0x00, 0x00, set as u8]))
    }
}

impl BK0Command2 {
    pub fn validate<F>(CMD2: &Command2Selection, build_command: F) -> Result<Command, &'static str>
    where
        F: Fn() -> Command,
    {
        match CMD2 {
            Command2Selection::BK0 => Ok(build_command()),
            _ => Err("Cannot run command '{}': BK0 Command 2 mode not set"),
        }
    }

    /// # POSITIVE GAMMA CONTROL
    /// See note above about parameters
    pub fn positive_gamma_control(
        CMD2: &Command2Selection,
        parameters: &[u8],
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(Self::PVGAMCTRL as u8).args(parameters)
        })
    }

    /// # POSITIVE GAMMA CONTROL
    /// See note above about parameters
    pub fn negative_gamma_control(
        CMD2: &Command2Selection,
        parameters: &[u8],
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(Self::NVGAMCTRL as u8).args(parameters)
        })
    }

    /// # DISPLAY LINE SETTING
    pub fn display_line_setting(
        CMD2: &Command2Selection,
        LDE_EN: u8,
        Line: u8,
        Line_delta: u8,
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(Self::LNESET as u8).args(&[LDE_EN | Line, Line_delta])
        })
    }

    /// # PORCH CONTROL
    pub fn porch_control(CMD2: &Command2Selection, mode: &Mode) -> Result<Command, &'static str> {
        let front_porch: u8 = (mode.vtotal - mode.vsync_end).try_into().unwrap();
        let back_porch: u8 = (mode.vsync_start - mode.vdisplay).try_into().unwrap();
        Self::validate(CMD2, || {
            Command::new(Self::PORCTRL as u8).args(&[front_porch, back_porch])
        })
    }

    /// # INVERSION SELECT
    /// * [LINV] - the type of inversion
    /// * [RTNI] - minimum number of pclk in each line
    pub fn inversion_select(
        CMD2: &Command2Selection,
        NLINV: Inversion,
        RTNI: u8,
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(Self::INVSET as u8).args(&[0x30 | NLINV as u8, RTNI])
        })
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
    pub fn rgb_control(
        CMD2: &Command2Selection,
        DEHV: DataEnable,
        VSP: VsyncActive,
        HSP: HsyncActive,
        DP: DataPolarity,
        EP: EnablePolarity,
        mode: &Mode,
    ) -> Result<Command, &'static str> {
        let HBP: u8 = (mode.htotal - mode.hsync_end).try_into().unwrap();
        let VBP: u8 = (mode.vsync_start - mode.vdisplay).try_into().unwrap();

        Self::validate(CMD2, || {
            Command::new(Self::RGBCTRL as u8).args(&[
                DEHV as u8 | VSP as u8 | HSP as u8 | DP as u8 | EP as u8,
                HBP,
                VBP,
            ])
        })
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
    pub fn color_control(
        CMD2: &Command2Selection,
        PWM: PWMPolarity,
        LED: LEDPolarity,
        MDT: PixelPinout,
        EPF: EndPixelFormat,
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(Self::COLCTRL as u8).arg(PWM as u8 | LED as u8 | MDT as u8 | EPF as u8)
        })
    }

    /// # CONFIGURE SUNLIGHT READABLE ENHANCEMENT MODE
    ///
    /// Sets the minimum brightness value to be used for CABC (see WRCACE).
    ///
    /// [MBV] Minimum Brightness Value
    ///
    ///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
    ///|   --   |   --   |   --   |   --   |   SRE  |      SRE_alpha[3:0]      |
    pub fn configure_sunlight_ehancement(
        CMD2: &Command2Selection,
        SRE: SunlightReadable,
        mut SRE_alpha: u8,
    ) -> Result<Command, &'static str> {
        if SRE_alpha > 0x0F {
            SRE_alpha = 0x0F;
        }
        Self::validate(CMD2, || {
            Command::new(Self::SECTRL as u8).arg(SRE as u8 | SRE_alpha)
        })
    }
}

impl BK1Command2 {
    pub fn validate<F>(CMD2: &Command2Selection, build_command: F) -> Result<Command, &'static str>
    where
        F: Fn() -> Command,
    {
        match CMD2 {
            Command2Selection::BK1 => Ok(build_command()),
            _ => Err("Cannot run command '{}': BK0 Command 2 mode not set"),
        }
    }

    pub fn set_vop_amplitude(CMD2: &Command2Selection, VRHA: u8) -> Result<Command, &'static str> {
        Self::validate(CMD2, || Command::new(BK1Command2::VRHS as u8).arg(VRHA))
    }

    pub fn set_vcom_amplitude(CMD2: &Command2Selection, VCOM: u8) -> Result<Command, &'static str> {
        Self::validate(CMD2, || Command::new(BK1Command2::VCOMS as u8).arg(VCOM))
    }

    pub fn set_vgh_voltage(CMD2: &Command2Selection, VGH: u8) -> Result<Command, &'static str> {
        Self::validate(CMD2, || Command::new(BK1Command2::VGHSS as u8).arg(VGH))
    }
    pub fn test_command_setting(CMD2: &Command2Selection) -> Result<Command, &'static str> {
        Self::validate(CMD2, || Command::new(BK1Command2::TESTCMD as u8).arg(0x80))
    }

    pub fn set_vgl_voltage(CMD2: &Command2Selection, VGLS: u8) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(BK1Command2::VGLS as u8).arg(0x40 | VGLS)
        })
    }

    pub fn power_control_one(
        CMD2: &Command2Selection,
        AP: GammaOPBias,
        APIS: SourceOPInput,
        APOS: SourceOPOutput,
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(BK1Command2::PWCTRL1 as u8).arg(AP as u8 | APIS as u8 | APOS as u8)
        })
    }

    pub fn power_control_two(
        CMD2: &Command2Selection,
        AVDD: VoltageAVDD,
        AVCL: VoltageAVCL,
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(BK1Command2::PWCTRL2 as u8).arg(AVDD as u8 | AVCL as u8)
        })
    }

    /// # SET SOURCE PRE DRIVE TIMING CONTROL
    /// T2D [3:0]: source pre_drive timing setting.(GND to VDD)
    /// Adjust Range : 0 ~ 3 uS 1 step is 0.2uS
    ///|   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
    ///|   --   |    1   |    1   |    1   |                T2D                |
    pub fn set_pre_drive_timing_one(
        CMD2: &Command2Selection,
        T2D: u8,
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(BK1Command2::SPD1 as u8).arg(0x70 | T2D)
        })
    }

    /// # SET SOURCE PRE DRIVE TIMING CONTROL
    /// Same parameters as SPD1
    pub fn set_pre_drive_timing_two(
        CMD2: &Command2Selection,
        T2D: u8,
    ) -> Result<Command, &'static str> {
        Self::validate(CMD2, || {
            Command::new(BK1Command2::SPD2 as u8).arg(0x70 | T2D)
        })
    }
}
