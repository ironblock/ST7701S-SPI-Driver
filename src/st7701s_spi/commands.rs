use std::{io, thread, time};

use crate::st7701s_spi::{address::core::*, parameters::display::{InversionSelection, LineSettings, PorchControl}};
use crate::st7701s_spi::address::special::*;
use crate::st7701s_spi::address::{AnyExtension, bk1::*};
use crate::st7701s_spi::address::{Extension, ExtensionBk0, ExtensionBk1, ExtensionBk3, bk3::*};
use crate::st7701s_spi::{
    address::bk0::*,
    device::*,
    parameters::{display::TearingEffectSignal, register::CommandExtension},
    protocol::connection::{Connection, InstructionResult},
    state::abstractions::{Configure, Select, Toggle},
    transmissions::{Parametric as _, Transmission},
};
use crate::st7701s_spi::{
    device::ST7701S,
    parameters::{
        brightness::{Brightness, BrightnessControl},
        color::{ColorChannel, PixelExtrema},
        display::{GammaCurve, VoltageControl},
        general::Switch,
    },
};
use Switch::*;

impl<C: Connection, E> ST7701S<C, E> {
    /// ## No Operation
    ///
    /// This command is "do nothing". It has no effect on the display, but it
    /// can be used to terminate parameter write commands. It is also sometimes
    /// required as a buffer between elements in certain sequences.
    pub fn no_operation(&mut self) -> io::Result<()> {
        self.connection().command::<NOP>()
    }

    /// ## Software Reset
    /// > Reference: p. 188
    ///
    /// Resets all internal registers to their default values. The framebuffer is
    /// not affected.
    ///
    /// ### Considerations
    /// 1. Wait at least 5ms before sending another command after the reset
    /// 2. If the display is sleeping (SLPIN), wait at least 120ms before
    ///    attempting to exit sleep mode (SLPOUT).
    /// 3. If the display is already in the process of exiting sleep, a reset
    ///    command will be ignored and have no effect.
    ///
    /// ### Parameters
    /// It's never stated anywhere why D0 is 1, but it's indicated in both the
    /// primary reference table on p. 184 and again on SWRESET's detail page.
    ///
    /// As an additional contradiction, p. 184 refers to SWRESET as a **command**
    /// (with no arguments), and p. 188 refers to it as a **write**. As only a
    /// write can have arguments and 0x01 is the constant argument in both
    /// references, SWRESET's canonical representation here is as a **write**.
    pub fn software_reset(self) -> ST7701S<C, AnyExtension> {
        const RESET_PARAMETERS: [u8; 1] = [0x01];
        self.connection()
            .write::<SWRESET>(&RESET_PARAMETERS)
            .expect("failed to send software reset command");
        let delay;
        let condition;

        if self.state().mode.sleep.is_on() {
            delay = 120;
            condition = "while in sleep mode";
        } else {
            delay = 5;
            condition = "";
        }

        log::info!(
            "Software reset triggered{}. Pausing commands for {}ms",
            condition,
            delay
        );

        thread::sleep(time::Duration::from_millis(delay));

        self.reset()
    }

    /// ## Read Display ID
    /// > Reference: p. 189
    ///
    /// Reads the display identification information from the device. This may
    /// be useful to verify the display model and manufacturer, but not all
    /// vendors populate this information.
    pub fn read_display_id(
        &mut self,
        buffer: &mut <RDDID as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDDID>(buffer)
    }

    /// ## Read Number of Errors on DSI
    /// > Reference: p. 190
    ///
    /// Returns the number of transmission errors detected on the DSI interface.
    /// This is only relevant for MIPI DSI configurations.
    pub fn read_dsi_errors(
        &mut self,
        buffer: &mut <RDNUMED as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDNUMED>(buffer)
    }

    /// ### Read First Pixel Color Values
    /// > p. 191: `0x06` `RDRED`
    /// > p. 192: `0x07` `RDGREEN`
    /// > p. 193: `0x08` `RDBLUE`
    ///
    /// Returns the value of an individual color channel for the first pixel on
    /// the display. This can be useful for diagnostics, eg. determining if the
    /// frame buffer is displaying a test pattern regardless of the "real world"
    /// physical appearance.
    ///
    /// ### Considerations
    /// 1. The LSB will always be D0
    /// 2. The MSB will depend on the color mode (see `__COLMOD`):
    ///    - 16-bit (RGB565): R: D4, G: D5, B: D4
    ///    - 18-bit (RGB666): R: D6, G: D6, B: D6
    ///    - 24-bit (RGB888): R: D7, G: D7, B: D7
    ///
    /// NOTE: The datasheet claims that the MSB for Green in RGB565 mode is D4,
    /// but this appears to be a mistake.
    pub fn read_first_pixel_values_for(
        &mut self,
        channel: ColorChannel,
        buffer: &mut [u8; 1],
    ) -> io::Result<()> {
        match channel {
            ColorChannel::Red => self.connection().read::<RDRED>(buffer),
            ColorChannel::Green => self.connection().read::<RDGREEN>(buffer),
            ColorChannel::Blue => self.connection().read::<RDBLUE>(buffer),
        }
    }

    /// ### `0x0A` `RDDPM`  Read Display Power Mode
    /// > Reference: p. 194
    pub fn read_display_power_mode(
        &mut self,
        buffer: &mut <RDDPM as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDDPM>(buffer)
    }

    /// ### `0x0B` `RDDMADCTL`  Read Display MADCTL
    /// > Reference: p. 195
    pub fn read_display_madctl(
        &mut self,
        buffer: &mut <RDDMADCTL as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDDMADCTL>(buffer)
    }

    /// ### `0x0C` `RDDCOLMOD`  Read Display Pixel Format
    /// > Reference: p. 196
    pub fn read_display_pixel_format(
        &mut self,
        buffer: &mut <RDDCOLMOD as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDDCOLMOD>(buffer)
    }

    /// ### `0x0D` `RDDIM`  Read Display Image Mode
    /// > Reference: p. 197
    pub fn read_display_image_mode(
        &mut self,
        buffer: &mut <RDDIM as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDDIM>(buffer)
    }

    /// ### `0x0E` `RDDSM`  Read Display Signal Mode
    /// > Reference: p. 198
    pub fn read_display_signal_mode(
        &mut self,
        buffer: &mut <RDDSM as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDDSM>(buffer)
    }

    /// ## Get Scan Line
    /// > Reference: p. 219
    ///
    /// Reads the current scan line being refreshed on the display. Useful for
    /// synchronization and diagnostics.
    pub fn get_scan_line(&mut self, buffer: &mut <GSL as Transmission>::Data) -> InstructionResult {
        self.connection().read::<GSL>(buffer)
    }

    /// ## Configure Display Brightness Value
    /// > Reference: p. 220, 221
    ///
    /// Get or set the display brightness value:
    /// - `0x00`: Dimmest setting
    /// - `0xFF`: Brightest setting
    ///
    /// ### Considerations
    ///   1. Manual brightness control must be enabled (see `__CTRLD`)
    pub fn brightness_value(&'_ mut self) -> Configure<'_, Self, RDDISBV, WRDISBV, Brightness> {
        if cfg!(debug_assertions)
            && self
                .state()
                .config
                .brightness_control
                .manual_control()
                .is_off()
        {
            log::error!("cannot set brightness value when manual brightness control is disabled");
        }

        Configure::new(self, |state| &mut state.config.brightness)
    }

    /// ## Configure Display Brightness Control Modes
    /// > Reference: p. 222, 224
    ///
    /// Configures display control features:
    /// - Automatic or manual brightness control
    /// - Dimming on or off
    /// - Backlight on or off
    ///
    /// ### Considerations
    ///   1. Brightness value must be set separately (see `__DISBV`)
    ///   2. Dimming control can only be set when using manual brightness control)
    pub fn brightness_control(
        &'_ mut self,
    ) -> Configure<'_, Self, RDCTRLD, WRCTRLD, BrightnessControl> {
        Configure::new(self, |state| &mut state.config.brightness_control)
    }

    /// ## Toggle Sleep Mode
    /// > Reference: p. 200, 201
    ///
    pub fn sleep_mode(&'_ mut self) -> Toggle<'_, Self, SLPIN, SLPOUT> {
        Toggle::new(self, |state| &mut state.mode.sleep)
    }

    /// ## Enable Partial Mode
    /// > Reference: p. 202
    ///
    pub fn enable_partial_mode(&mut self) -> InstructionResult {
        self.connection().command::<PTLON>().inspect(|_| {
            self.modify_state(|state| {
                state.mode.partial = On;
            });
        })
    }

    /// ## Enable Normal Mode
    /// > Reference: p. 203
    ///
    pub fn enable_normal_mode(&mut self) -> InstructionResult {
        self.connection().command::<NORON>().inspect(|_| {
            self.modify_state(|state| {
                state.mode.partial = Off;
                state.image.set_all_pixels_black(Off);
                state.image.set_all_pixels_white(Off);
            });
        })
    }

    /// ## Toggle Inverted Colors
    /// > Reference:
    /// > - `INVOFF` p. 204
    /// > - `INVON`  p. 205
    // pub fn invert_colors(&mut self) -> Toggle<Self, INVON, INVOFF> {
    //     Toggle::new(self, |state| &mut state.image.invert_colors())
    // }

    /// ## Set All Pixels Black or White
    /// > Reference:
    /// > - `ALLPOFF` p. 206
    /// > - `ALLPON`  p. 207
    pub fn set_all_pixels(&mut self, extrema: PixelExtrema) -> InstructionResult {
        match extrema {
            PixelExtrema::Black => self.connection().command::<ALLPOFF>().inspect(|_| {
                self.modify_state(|state| {
                    state.image.set_all_pixels_black(On);
                    state.image.set_all_pixels_white(Off);
                })
            }),
            PixelExtrema::White => self.connection().command::<ALLPON>().inspect(|_| {
                self.modify_state(|state| {
                    state.image.set_all_pixels_black(Off);
                    state.image.set_all_pixels_white(On);
                })
            }),
        }
    }

    /// ## Select Gamma Curve
    /// > Reference:
    /// > `GAMSET` p. 208
    ///
    pub fn select_gamma_curve(&mut self, transmission: GammaCurve) -> InstructionResult {
        let gamma_curve = transmission.gc();

        self.connection()
            .write::<GAMSET>(&transmission.as_tx_data())
            .inspect(|_| {
                self.modify_state(|state| {
                    state.image.set_gamma_curve(gamma_curve);
                });
            })
    }

    /// ## Display Output
    /// > Reference:
    /// > `DISPOFF` p. 209
    /// > `DISPON`  p. 210
    pub fn display_output(&'_ mut self) -> Toggle<'_, Self, DISPON, DISPOFF> {
        Toggle::new(self, |state| &mut state.mode.display)
    }

    /// ## Toggle Idle Mode
    /// > Reference: p. 215, 216
    ///
    pub fn idle_mode(&'_ mut self) -> Toggle<'_, Self, IDMON, IDMOFF> {
        Toggle::new(self, |state| &mut state.mode.idle)
    }

    pub fn tearing_effect_line(&'_ mut self) -> Select<'_, Self, TEON, TEOFF, TearingEffectSignal> {
        Select::new(self, |state| &mut state.tearing_effect)
    }

    // TODO: Add adaptive_brightness field to ConfigurationState
    /// ## Configure Content Adaptive Brightness Control and Color Enhancement
    /// > Reference: p. 225, 227
    ///
    /// Get or set parameters for adaptive brightness and color enhancement. Enables or
    /// disables color enhancement and selects the enhancement mode.
    // pub fn adaptive_brightness(&mut self) -> Configure<Self, WRCACE, RDCABC> {
    //     Configure::new(self, |state| &mut state.config.adaptive_brightness)
    // }

    // TODO: Add min_adaptive_brightness field to ConfigurationState
    // /// ## Configure CABC Minimum Brightness
    // /// > Reference: p. 229, 230
    // ///
    // /// Get or set the minimum brightness value for Content Adaptive Brightness Control
    // /// (CABC).
    // pub fn min_adaptive_brightness(&mut self) -> Configure<Self, WRCABCMB, RDCABCMB> {
    //     Configure::new(self, |state| &mut state.config.min_adaptive_brightness)
    // }

    /// ## Read Automatic Brightness Control Self-Diagnostic Result
    /// > Reference: p. 231
    ///
    /// Reads the result of the automatic brightness control self-diagnostic test.
    pub fn read_adaptive_brightness_diagnostic(
        &mut self,
        buffer: &mut <RDABCSDR as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDABCSDR>(buffer)
    }

    /// ## Read Black/White Low Bits
    /// > Reference: p. 232
    ///
    /// Returns the low bits of the black and white color settings for calibration
    /// and diagnostics.
    pub fn read_black_white_low_bits(
        &mut self,
        buffer: &mut <RDBWLB as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDBWLB>(buffer)
    }

    /// ## Read Bkx
    /// > Reference: p. 233
    ///
    /// Reads the Bkx calibration value from the device.
    pub fn read_bkx(&mut self, buffer: &mut <RDBKX as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDBKX>(buffer)
    }

    /// ## Read Bky
    /// > Reference: p. 234
    ///
    /// Reads the Bky calibration value from the device.
    pub fn read_bky(&mut self, buffer: &mut <RDBKY as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDBKY>(buffer)
    }

    /// ## Read Wx
    /// > Reference: p. 235
    ///
    /// Reads the Wx calibration value from the device.
    pub fn read_wx(&mut self, buffer: &mut <RDWX as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDWX>(buffer)
    }

    /// ## Read Wy
    /// > Reference: p. 236
    ///
    /// Reads the Wy calibration value from the device.
    pub fn read_wy(&mut self, buffer: &mut <RDWY as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDWY>(buffer)
    }

    /// ## Read Rx
    /// > Reference: p. 239
    ///
    /// Reads the Rx calibration value from the device.
    pub fn read_rx(&mut self, buffer: &mut <RDRX as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDRX>(buffer)
    }

    /// ## Read Ry
    /// > Reference: p. 239
    ///
    /// Reads the Ry calibration value from the device.
    pub fn read_ry(&mut self, buffer: &mut <RDRY as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDRY>(buffer)
    }

    /// ## Read Gx
    /// > Reference: p. 240
    ///
    /// Reads the Gx calibration value from the device.
    pub fn read_gx(&mut self, buffer: &mut <RDGX as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDGX>(buffer)
    }

    /// ## Read Gy
    /// > Reference: p. 241
    ///
    /// Reads the Gy calibration value from the device.
    pub fn read_gy(&mut self, buffer: &mut <RDGY as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDGY>(buffer)
    }

    /// ## Read Blue/A Color Low Bits
    /// > Reference: p. 242
    ///
    /// Returns the low bits of the blue and A color settings for calibration and
    /// diagnostics.
    pub fn read_blue_low_bits(
        &mut self,
        buffer: &mut <RDBALB as Transmission>::Data,
    ) -> InstructionResult {
        self.connection().read::<RDBALB>(buffer)
    }

    /// ## Read Bx
    /// > Reference: p. 243
    ///
    /// Reads the Bx calibration value from the device.
    pub fn read_bx(&mut self, buffer: &mut <RDBX as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDBX>(buffer)
    }

    /// ## Read By
    /// > Reference: p. 244
    ///
    /// Reads the By calibration value from the device.
    pub fn read_by(&mut self, buffer: &mut <RDBY as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDBY>(buffer)
    }

    /// ## Read Ax
    /// > Reference: p. 245
    ///
    /// Reads the Ax calibration value from the device.
    pub fn read_ax(&mut self, buffer: &mut <RDAX as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDAX>(buffer)
    }

    /// ## Read Ay
    /// > Reference: p. 246
    ///
    /// Reads the Ay calibration value from the device.
    pub fn read_ay(&mut self, buffer: &mut <RDAY as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDAY>(buffer)
    }

    /// ## Read DDB Start
    /// > Reference: p. 247
    ///
    /// Reads the initial value of the Display Data Bus (DDB) for diagnostics.
    pub fn read_ddbs(&mut self, buffer: &mut <RDDDBS as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDDDBS>(buffer)
    }

    /// ## Read DDB Continue
    /// > Reference: p. 249
    ///
    /// Reads the next value of the Display Data Bus (DDB) for diagnostics.
    pub fn read_ddbc(&mut self, buffer: &mut <RDDDBC as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDDDBC>(buffer)
    }

    /// ## Read First Checksum
    /// > Reference: p. 250
    ///
    /// Reads the first checksum value for verifying data integrity.
    pub fn read_fcs(&mut self, buffer: &mut <RDFCS as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDFCS>(buffer)
    }

    /// ## Read Continue Checksum
    /// > Reference: p. 251
    ///
    /// Reads the next checksum value for continued data integrity verification.
    pub fn read_ccs(&mut self, buffer: &mut <RDCCS as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDCCS>(buffer)
    }

    /// ## Read ID1
    /// > Reference: p. 252
    ///
    /// Reads the first identification value from the device.
    pub fn read_id1(&mut self, buffer: &mut <RDID1 as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDID1>(buffer)
    }

    /// ## Read ID2
    /// > Reference: p. 253
    ///
    /// Reads the second identification value from the device.
    pub fn read_id2(&mut self, buffer: &mut <RDID2 as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDID2>(buffer)
    }

    /// ## Read ID3
    /// > Reference: p. 254
    ///
    /// Reads the third identification value from the device.
    pub fn read_id3(&mut self, buffer: &mut <RDID3 as Transmission>::Data) -> InstructionResult {
        self.connection().read::<RDID3>(buffer)
    }

    /// ## Command2 BKx Selection
    /// > Reference: p. 260
    ///
    /// Selects the extended command bank (BK0, BK1, BK3) for subsequent operations.
    /// This command is required before sending any extended command and ensures the
    /// correct register bank is active.
    pub fn select_command_extension<N: Extension>(mut self, extension: N) -> ST7701S<C, N> {
        let mut transmission = CommandExtension::new();

        if let Some(extension) = N::EXTENSION {
            transmission.set_extended_commands(On).set_bank(extension);
        } else {
            transmission.set_extended_commands(Off);
        }

        self.connection()
            .write::<CND2BKXSEL>(&transmission.as_tx_data())
            .inspect(|_| {
                self.modify_state(|state| {
                    state.command_extension = transmission;
                });
            }).expect("failed to select command extension");

        self.set_extension(extension)
    }

    /// ## `Special: 0xFF` `DSTB` Deep Standby Mode Enable
    /// > Reference: p. 285
    ///
    /// Enables deep standby mode, reducing power consumption to a minimum. The
    /// display will not respond to most commands until reactivated.
    pub fn deep_standby_enable(&mut self) -> InstructionResult {
        use crate::st7701s_spi::address::special::DSTB;
        const PARAMS: [u8; 5] = [0x77, 0x01, 0x00, 0x00, 0x13];
        self.connection().write::<DSTB>(&PARAMS)
    }

    /// ## `Special: 0xFF` `DSTBT` Deep Standby Mode Active
    /// > Reference: p. 286
    ///
    /// Indicates whether deep standby mode is currently active. Used for
    /// diagnostics and power management.
    pub fn deep_standby_active(&mut self) -> InstructionResult {
        use crate::st7701s_spi::address::special::DSTBT;
        const PARAMS: [u8; 5] = [0x77, 0x01, 0x00, 0x00, 0x13];
        self.connection().write::<DSTBT>(&PARAMS)
    }
}

impl<C: Connection> ST7701S<C, ExtensionBk0> {
    /// ## `BK0: 0xB0` `PVGAMCTRL` Positive Voltage Gamma Control
    /// > See p. 261
    ///
    /// Configures the positive voltage gamma curve for the display. This command
    /// allows fine-tuning of the display's color response and image quality by
    /// setting multiple voltage control points.
    pub fn positive_gamma_control(&mut self, parameters: &VoltageControl) -> InstructionResult {
        self.connection()
            .write::<PVGAMCTRL>(&parameters.as_tx_data())
    }

    /// ## `BK0: 0xB1` `NVGAMCTRL` Negative Voltage Gamma Control
    /// > Reference: p. 263
    ///
    /// Configures the negative voltage gamma curve for the display. This command
    /// complements PVGAMCTRL and is used to adjust the display's color response for
    /// negative voltages.
    pub fn negative_gamma_control(&mut self, parameters: &VoltageControl) -> InstructionResult {
        self.connection().write::<NVGAMCTRL>(&parameters.as_tx_data())
    }

    /// ## `BK0: 0xB8` `DGMEN` Digital Gamma Enable
    /// > Reference: p. 265
    ///
    /// Enables or disables digital gamma correction. When enabled, the display uses
    /// digital gamma look-up tables for color adjustment.
    pub fn digital_gamma_enable(&mut self, enable: u8) -> InstructionResult {
        self.connection().write::<DGMEN>(&[enable])
    }

    /// ## `BK0: 0xB9` `DGMLUTR` Digital Gamma Look-up Table for Red
    /// > Reference: p. 266
    ///
    /// Sets the digital gamma look-up table for the red color channel. Each entry
    /// defines the gamma correction for a specific input value.
    // pub fn digital_gamma_lut_red(&mut self, lut_data: &GammaLutRed) -> InstructionResult {
    //     self.connection().write::<DGMLUTR>(lut_data)
    // }

    /// ## `BK0: 0xBA` `DGMLUTB` Digital Gamma Look-up Table for Blue
    /// > Reference: p. 267
    ///
    /// Sets the digital gamma look-up table for the blue color channel. Each entry
    /// defines the gamma correction for a specific input value.
    // pub fn digital_gamma_lut_blue(&mut self, lut_data: &GammaLutBlue) -> InstructionResult {
    //     self.connection().write::<DGMLUTB>(lut_data)
    // }

    /// ## `BK0: 0xBC` `PWMCLKSEL` PWM CLK select
    /// > Reference: p. 268
    ///
    /// Selects the clock source for the PWM signal used in backlight control.
    pub fn pwm_clock_select(&mut self, clock_setting: u8) -> InstructionResult {
        self.connection().write::<PWMCLKSEL>(&[clock_setting])
    }

    /// ## `BK0: 0xC0` `LNESET` Display Line Setting
    /// > Reference: p. 269
    ///
    /// Configures the number of display lines and line delta for the panel. This
    /// affects the vertical resolution and timing.
    pub fn line_setting(&mut self, settings: &LineSettings) -> InstructionResult {
        self.connection().write::<LNESET>(&settings.as_tx_data())
    }

    /// ## `BK0: 0xC1` `PORCTRL` Porch Control
    /// > Reference: p. 270
    ///
    /// Sets the front and back porch timing for the display. Proper porch settings
    /// are important for stable image rendering and synchronization.
    pub fn porch_control(&mut self, settings: &PorchControl) -> InstructionResult {
        self.connection().write::<PORCTRL>(&settings.as_tx_data())
    }

    /// ## `BK0: 0xC2` `INVSEL` Inversion Selection & Frame Rate Control
    /// > Reference: p. 271
    ///
    /// Controls display inversion and frame rate settings. Proper inversion settings
    /// ensure correct color representation and can affect display smoothness.
    pub fn inversion_select(&mut self, settings: &InversionSelection) -> InstructionResult {
        self.connection().write::<INVSET>(&settings.as_tx_data())
    }

    /// ## `BK0: 0xC3` `RGBCTRL` RGB control
    /// > Reference: p. 272
    ///
    /// Configures the RGB interface mode and signal polarities. This command is
    /// essential for matching the display's timing and signal requirements to the
    /// host system.
    // pub fn rgb_control(&mut self, settings: &RgbControl) -> InstructionResult {
    //     let params = [
    //         settings.param1,
    //         settings.param2,
    //         settings.param3,
    //         settings.param4,
    //     ];
    //     self.connection().write::<RGBCTRL>(&params)
    // }

    /// ## `BK0: 0xC5` `PARCTRL` Partial Area Control
    /// > Reference: p. 273
    ///
    /// Configures partial display mode settings, allowing only a portion of the
    /// display to be updated for power savings or special effects.
    // pub fn partial_area(&mut self, settings: &PartialControl) -> InstructionResult {
    //     let params = [settings.start_config, settings.end_config];
    //     self.connection().write::<PARCTRL>(&params)
    // }

    /// ## `BK0: 0xC7` `SDIR` X-direction Control
    /// > Reference: p. 274
    ///
    /// Sets the direction of pixel scanning along the X-axis. This is used for
    /// display orientation and mirroring.
    pub fn scan_direction(&mut self, direction: u8) -> InstructionResult {
        self.connection().write::<SDIR>(&[direction])
    }

    /// ## `BK0: 0xC8` `PDOSET` Pseudo-Dot inversion diving setting
    /// > Reference: p. 275
    ///
    /// Configures pseudo-dot inversion settings to improve display uniformity and
    /// reduce artifacts.
    pub fn pseudo_dot_inversion(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<PDOSET>(&[setting])
    }

    /// ## `BK0: 0xCD` `COLCTRL` Color Control
    /// > Reference: p. 276
    ///
    /// Adjusts color control parameters such as PWM polarity, LED polarity, pixel
    /// format, and end pixel format. These settings affect color rendering and
    /// backlight behavior.
    pub fn color_control(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<COLCTRL>(&[setting])
    }

    /// ## `BK0: 0xE0` `SRECTRL` Sunlight Readable Enhancement
    /// > Reference: p. 278
    ///
    /// Enables and configures sunlight readability enhancement features, improving
    /// display visibility in bright environments.
    pub fn sunlight_readable_enhancement(&mut self, params: [u8; 3]) -> InstructionResult {
        self.connection().write::<SRECTRL>(&params)
    }

    /// ## `BK0: 0xE1` `NRCTRL` Noise Reduce Control
    /// > Reference: p. 279
    ///
    /// Sets noise reduction parameters to improve image quality and reduce visual
    /// artifacts.
    pub fn noise_reduction_control(&mut self, params: [u8; 11]) -> InstructionResult {
        self.connection().write::<NRCTRL>(&params)
    }

    /// ## `BK0: 0xE2` `SECTRL` Sharpness and Edge Enhancement
    /// > Reference: p. 280
    ///
    /// Adjusts image sharpness and edge enhancement algorithms to improve perceived
    /// image clarity and detail definition.
    pub fn sharpness_control(&mut self, params: [u8; 13]) -> InstructionResult {
        self.connection().write::<SECTRL>(&params)
    }

    /// ## `BK0: 0xE3` `CCCTRL` Color Calibration
    /// > Reference: p. 281
    ///
    /// Sets color calibration parameters to ensure accurate color reproduction
    /// across different viewing conditions and manufacturing tolerances.
    pub fn color_calibration_control(&mut self, params: [u8; 4]) -> InstructionResult {
        self.connection().write::<CCCTRL>(&params)
    }

    /// ## `BK0: 0xE4` `SKCTRL` Skin Tone Preservation
    /// > Reference: p. 282
    ///
    /// Enables skin tone preservation features for more natural human skin
    /// representation in images and videos.
    pub fn skin_tone_control(&mut self, params: [u8; 2]) -> InstructionResult {
        self.connection().write::<SKCTRL>(&params)
    }

    /// ## `BK0: 0xEA` `NVMSETE` NVM Set Enable
    /// > Reference: p. 283
    ///
    /// Enables or disables Non-Volatile Memory settings for persistent configuration.
    pub fn nvm_set_enable(&mut self, enable: u8) -> InstructionResult {
        self.connection().write::<NVMSETE>(&[enable])
    }

    /// ## `BK0: 0xEE` `CABCCTRL` Content Adaptive Brightness Control
    /// > Reference: p. 284
    ///
    /// Controls Content Adaptive Brightness Control for dynamic backlight adjustment
    /// based on image content to save power and improve visibility.
    pub fn cabc_control(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<CABCCTRL>(&[setting])
    }
}

impl<C: Connection> ST7701S<C, ExtensionBk1> {
    /// ## `BK1: 0xB0` `VRHS` Vop Amplitude Setting
    /// > Reference: p. 287
    ///
    /// Sets the Vop amplitude, which controls the driving voltage for the display
    /// panel. Adjusting this value can affect display brightness and stability.
    pub fn vop_amplitude_setting(&mut self, voltage: u8) -> InstructionResult {
        self.connection().write::<VRHS>(&[voltage])
    }

    /// ## `BK1: 0xB1` `VCOMS` VCOM setting
    /// > Reference: p. 288
    ///
    /// Sets the VCOM voltage level for proper LCD operation and contrast.
    pub fn vcom_setting(&mut self, voltage: u8) -> InstructionResult {
        self.connection().write::<VCOMS>(&[voltage])
    }

    /// ## `BK1: 0xB2` `VGHSS` VGH Setting
    /// > Reference: p. 289
    ///
    /// Configures the VGH (gate high voltage) level for TFT gate driving.
    pub fn vgh_setting(&mut self, voltage: u8) -> InstructionResult {
        self.connection().write::<VGHSS>(&[voltage])
    }

    /// ## `BK1: 0xB3` `TESTCMD` Test Command
    /// > Reference: p. 290
    ///
    /// Issues test commands for diagnostic and verification purposes.
    pub fn test_command(&mut self, command: u8) -> InstructionResult {
        self.connection().write::<TESTCMD>(&[command])
    }

    /// ## `BK1: 0xB5` `VGLS` VGL Setting
    /// > Reference: p. 291
    ///
    /// Configures the VGL (gate low voltage) level for TFT gate driving.
    pub fn vgl_setting(&mut self, voltage: u8) -> InstructionResult {
        self.connection().write::<VGLS>(&[voltage])
    }

    /// ## `BK1: 0xB7` `PWCTRL1` Power Control 1
    /// > Reference: p. 292
    ///
    /// Configures primary power control settings for the display, including voltage
    /// regulators and power sequencing.
    pub fn power_control_1(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<PWCTRL1>(&[setting])
    }

    /// ## `BK1: 0xB8` `PWCTRL2` Power Control 2
    /// > Reference: p. 293
    ///
    /// Sets secondary power control parameters for enhanced power management and
    /// efficiency optimization.
    pub fn power_control_2(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<PWCTRL2>(&[setting])
    }

    /// ## `BK1: 0xBA` `PCLKS1` Panel Clock Setting 1
    /// > Reference: p. 294
    ///
    /// Configures the primary panel clock settings for display timing control.
    pub fn panel_clock_setting_1(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<PCLKS1>(&[setting])
    }

    /// ## `BK1: 0xBB` `PCLKS2` Panel Clock Setting 2
    /// > Reference: p. 295
    ///
    /// Sets secondary panel clock parameters for fine timing adjustments.
    pub fn panel_clock_setting_2(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<PCLKS2>(&[setting])
    }

    /// ## `BK1: 0xBC` `PCLKS3` Panel Clock Setting 3
    /// > Reference: p. 296
    ///
    /// Adjusts tertiary panel clock settings for advanced timing control.
    pub fn panel_clock_setting_3(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<PCLKS3>(&[setting])
    }

    /// ## `BK1: 0xC1` `SPD1` Source Pre-Drive Timing Set 1
    /// > Reference: p. 298
    ///
    /// Configures timing parameters for the source driver pre-drive stage, which
    /// affects signal integrity and display performance.
    pub fn source_pre_drive_timing_1(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<SPD1>(&[setting])
    }

    /// ## `BK1: 0xC2` `SPD2` Speed Control 2
    /// > Reference: p. 298
    ///
    /// Fine-tunes additional speed control parameters for display optimization.
    pub fn speed_control_2(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<SPD2>(&[setting])
    }

    /// ## `BK1: 0xD0` `MIPISET1` MIPI Setting 1
    /// > Reference: p. 299
    ///
    /// Configures primary MIPI interface settings for communication with the host.
    pub fn mipi_setting_1(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<MIPISET1>(&[setting])
    }

    /// ## `BK1: 0xD1` `MIPISET2` MIPI Setting 2
    /// > Reference: p. 300
    ///
    /// Sets detailed MIPI communication parameters for advanced interface control.
    pub fn mipi_setting_2(&mut self, params: [u8; 15]) -> InstructionResult {
        self.connection().write::<MIPISET2>(&params)
    }

    /// ## `BK1: 0xD2` `MIPISET3` MIPI Setting 3
    /// > Reference: p. 301
    ///
    /// Adjusts supplementary MIPI timing and protocol settings.
    pub fn mipi_setting_3(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<MIPISET3>(&[setting])
    }

    /// ## `BK1: 0xD3` `MIPISET4` MIPI Setting 4
    /// > Reference: p. 302
    ///
    /// Finalizes MIPI configuration with additional protocol parameters.
    pub fn mipi_setting_4(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<MIPISET4>(&[setting])
    }
}

impl<C: Connection> ST7701S<C, ExtensionBk3> {
    /// ## `BK3: 0xCA` `NVMSET` NVM Setting
    /// > Reference: p. 304
    ///
    /// Configures Non-Volatile Memory settings for persistent display configuration.
    pub fn nvm_setting(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<NVMSET>(&[setting])
    }

    /// ## `BK3: 0xCC` `PROMACT` PROM Activation
    /// > Reference: p. 305
    ///
    /// Activates PROM (Programmable Read-Only Memory) for factory settings access.
    pub fn prom_activation(&mut self, setting: u8) -> InstructionResult {
        self.connection().write::<PROMACT>(&[setting])
    }
}
