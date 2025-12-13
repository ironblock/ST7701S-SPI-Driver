use crate::st7701s_spi::protocol::connection::{Bank0, Bank1, Bank3, Connection, Extension};
use crate::st7701s_spi::protocol::connection::{ConnectionOwner as _, RxData};

use std::{io, thread, time};

use crate::st7701s_spi::instructions::{
    bk0::{
        CABCCTRL, CCCTRL, COLCTRL, DGMEN, DGMLUTB, DGMLUTR, INVSET, LNESET, NRCTRL, NVGAMCTRL,
        NVMSETE, PARCTRL, PDOSET, PORCTRL, PVGAMCTRL, PWMCLKSEL, RGBCTRL, SDIR, SECTRL, SKCTRL,
        SRECTRL,
    },
    bk1::{
        MIPISET1, MIPISET2, MIPISET3, MIPISET4, PCLKS1, PCLKS2, PCLKS3, PWCTRL2, SPD1, SPD2,
        TESTCMD, VCOMS, VGHSS, VGLS, VRHS,
    },
    bk3::{NVMSET, PROMACT},
    core::{
        ALLPOFF, ALLPON, DISPOFF, DISPON, GAMSET, GSL, IDMOFF, IDMON, INVOFF, INVON, NOP, NORON,
        PTLON, RDABCSDR, RDAX, RDAY, RDBALB, RDBKX, RDBKY, RDBLUE, RDBWLB, RDBX, RDBY, RDCABC,
        RDCABCMB, RDCCS, RDCTRLD, RDDCOLMOD, RDDDBC, RDDDBS, RDDID, RDDIM, RDDISBV, RDDMADCTL,
        RDDPM, RDDSM, RDFCS, RDGREEN, RDGX, RDGY, RDID1, RDID2, RDID3, RDNUMED, RDRED, RDRX, RDRY,
        RDWX, RDWY, SLPIN, SLPOUT, SWRESET, TEOFF, TEON, WRCABCMB, WRCACE, WRCTRLD, WRDISBV,
    },
    special::{CND2BKXSEL, DSTB, DSTBT},
};

use crate::st7701s_spi::parameters::{
    bk0_display::{
        ColorCalibration, ColorControl, GammaLutBlue, GammaLutRed, NoiseReduction, PartialControl,
        PseudoDotInversion, RgbControl, ScanDirectionControl, SharpnessControl, SkinToneControl,
        SunlightEnhancement,
    },
    bk1_power::{
        CommonVoltage, GateHighVoltage, GateLowVoltage, MipiSetting1, MipiSetting2, MipiSetting3,
        MipiSetting4, OperatingVoltage, PanelClockSetting1, PanelClockSetting2, PanelClockSetting3,
        PowerControl2, SourcePreDriveTiming1, SourcePreDriveTiming2,
    },
    display::{InversionSelection, LineSettings, PorchControl},
};
use crate::st7701s_spi::{
    device::ST7701S,
    parameters::{
        brightness::{AdaptiveBrightness, Brightness, BrightnessControl, MinAdaptiveBrightness},
        color::{ColorChannel, PixelExtrema},
        display::{GammaCurve, VoltageControl},
        general::Switch,
    },
};
use crate::st7701s_spi::{
    parameters::{display::TearingEffectSignal, register::CommandExtension},
    state::abstractions::{Configure, Select, Toggle},
};
use Switch::{Off, On};

#[allow(clippy::missing_errors_doc, reason = "IO errors are self-explanatory")]
impl<X: Connection, E> ST7701S<X, E> {
    /// ## No Operation
    ///
    /// This command is "do nothing". It has no effect on the display, but it
    /// can be used to terminate parameter write commands. It is also sometimes
    /// required as a buffer between elements in certain sequences.
    pub fn no_operation(&mut self) -> io::Result<()> {
        self.command::<NOP>()
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
    pub fn software_reset(self) -> io::Result<ST7701S<X>> {
        const RESET_PARAMETERS: [u8; 1] = [0x01];
        self.write::<SWRESET>(&RESET_PARAMETERS)?;
        let (delay, condition) = if self.state().mode.sleep.is_on() {
            (120, "while in sleep mode")
        } else {
            (5, "")
        };

        log::info!("Software reset triggered{condition}. Pausing commands for {delay}ms");

        thread::sleep(time::Duration::from_millis(delay));

        Ok(self.reset())
    }

    /// ## Read Display ID
    /// > Reference: p. 189
    ///
    /// Reads the display identification information from the device. This may
    /// be useful to verify the display model and manufacturer, but not all
    /// vendors populate this information.
    pub fn read_display_id(&mut self, buffer: &mut <RDDID as RxData>::Data) -> io::Result<()> {
        self.read::<RDDID>(buffer)
    }

    /// ## Read Number of Errors on DSI
    /// > Reference: p. 190
    ///
    /// Returns the number of transmission errors detected on the DSI interface.
    /// This is only relevant for MIPI DSI configurations.
    pub fn read_dsi_errors(&mut self, buffer: &mut <RDNUMED as RxData>::Data) -> io::Result<()> {
        self.read::<RDNUMED>(buffer)
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
            ColorChannel::Red => self.read::<RDRED>(buffer),
            ColorChannel::Green => self.read::<RDGREEN>(buffer),
            ColorChannel::Blue => self.read::<RDBLUE>(buffer),
        }
    }

    /// ### `0x0A` `RDDPM`  Read Display Power Mode
    /// > Reference: p. 194
    pub fn read_display_power_mode(
        &mut self,
        buffer: &mut <RDDPM as RxData>::Data,
    ) -> io::Result<()> {
        self.read::<RDDPM>(buffer)
    }

    /// ### `0x0B` `RDDMADCTL`  Read Display MADCTL
    /// > Reference: p. 195
    pub fn read_display_madctl(
        &mut self,
        buffer: &mut <RDDMADCTL as RxData>::Data,
    ) -> io::Result<()> {
        self.read::<RDDMADCTL>(buffer)
    }

    /// ### `0x0C` `RDDCOLMOD`  Read Display Pixel Format
    /// > Reference: p. 196
    pub fn read_display_pixel_format(
        &mut self,
        buffer: &mut <RDDCOLMOD as RxData>::Data,
    ) -> io::Result<()> {
        self.read::<RDDCOLMOD>(buffer)
    }

    /// ### `0x0D` `RDDIM`  Read Display Image Mode
    /// > Reference: p. 197
    pub fn read_display_image_mode(
        &mut self,
        buffer: &mut <RDDIM as RxData>::Data,
    ) -> io::Result<()> {
        self.read::<RDDIM>(buffer)
    }

    /// ### `0x0E` `RDDSM`  Read Display Signal Mode
    /// > Reference: p. 198
    pub fn read_display_signal_mode(
        &mut self,
        buffer: &mut <RDDSM as RxData>::Data,
    ) -> io::Result<()> {
        self.read::<RDDSM>(buffer)
    }

    /// ## Get Scan Line
    /// > Reference: p. 219
    ///
    /// Reads the current scan line being refreshed on the display. Useful for
    /// synchronization and diagnostics.
    pub fn get_scan_line(&mut self, buffer: &mut <GSL as RxData>::Data) -> io::Result<()> {
        self.read::<GSL>(buffer)
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
    pub fn brightness_value(&'_ mut self) -> Configure<'_, X, E, RDDISBV, WRDISBV, Brightness> {
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
    ) -> Configure<'_, X, E, RDCTRLD, WRCTRLD, BrightnessControl> {
        Configure::new(self, |state| &mut state.config.brightness_control)
    }

    /// ## Toggle Sleep Mode
    /// > Reference: p. 200, 201
    ///
    pub fn sleep_mode(&'_ mut self) -> Toggle<'_, X, E, SLPIN, SLPOUT> {
        Toggle::new(self, |state| &mut state.mode.sleep)
    }

    /// ## Enable Partial Mode
    /// > Reference: p. 202
    ///
    pub fn enable_partial_mode(&mut self) -> io::Result<()> {
        self.command::<PTLON>().inspect(|()| {
            self.modify_state(|state| {
                state.mode.partial = On;
            });
        })
    }

    /// ## Enable Normal Mode
    /// > Reference: p. 203
    ///
    pub fn enable_normal_mode(&mut self) -> io::Result<()> {
        self.command::<NORON>().inspect(|()| {
            self.modify_state(|state| {
                state.mode.partial = Off;
                state.image = state.image.set_all_pixels_black(Off);
                state.image = state.image.set_all_pixels_white(Off);
            });
        })
    }

    /// ## Toggle Inverted Colors
    /// > Reference:
    /// > - `INVOFF` p. 204
    /// > - `INVON`  p. 205
    pub fn invert_colors_on(&mut self) -> io::Result<()> {
        self.command::<INVON>().inspect(|()| {
            self.modify_state(|state| {
                state.image = state.image.set_invert_colors(On);
            });
        })
    }

    pub fn invert_colors_off(&mut self) -> io::Result<()> {
        self.command::<INVOFF>().inspect(|()| {
            self.modify_state(|state| {
                state.image = state.image.set_invert_colors(Off);
            });
        })
    }

    /// ## Set All Pixels Black or White
    /// > Reference:
    /// > - `ALLPOFF` p. 206
    /// > - `ALLPON`  p. 207
    pub fn set_all_pixels(&mut self, extrema: PixelExtrema) -> io::Result<()> {
        match extrema {
            PixelExtrema::Black => self.command::<ALLPOFF>().inspect(|()| {
                self.modify_state(|state| {
                    state.image = state.image.set_all_pixels_black(On);
                    state.image = state.image.set_all_pixels_white(Off);
                });
            }),
            PixelExtrema::White => self.command::<ALLPON>().inspect(|()| {
                self.modify_state(|state| {
                    state.image = state.image.set_all_pixels_black(Off);
                    state.image = state.image.set_all_pixels_white(On);
                });
            }),
        }
    }

    /// ## Select Gamma Curve
    /// > Reference:
    /// > `GAMSET` p. 208
    ///
    pub fn select_gamma_curve(&mut self, transmission: GammaCurve) -> io::Result<()> {
        let gamma_curve = transmission.gc();

        self.write::<GAMSET>(transmission.buffer()).map(|()| {
            self.modify_state(|state| {
                state.image = state.image.set_gamma_curve(gamma_curve);
            });
        })
    }

    /// ## Display Output
    /// > Reference:
    /// > `DISPOFF` p. 209
    /// > `DISPON`  p. 210
    pub fn display_output(&'_ mut self) -> Toggle<'_, X, E, DISPON, DISPOFF> {
        Toggle::new(self, |state| &mut state.mode.display)
    }

    /// ## Toggle Idle Mode
    /// > Reference: p. 215, 216
    ///
    pub fn idle_mode(&'_ mut self) -> Toggle<'_, X, E, IDMON, IDMOFF> {
        Toggle::new(self, |state| &mut state.mode.idle)
    }

    pub fn tearing_effect_line(&'_ mut self) -> Select<'_, X, E, TEON, TEOFF, TearingEffectSignal> {
        Select::new(self, |state| &mut state.tearing_effect)
    }

    /// ## Configure Content Adaptive Brightness Control and Color Enhancement
    /// > Reference: p. 225, 227
    ///
    /// Get or set parameters for adaptive brightness and color enhancement. Enables or
    /// disables color enhancement and selects the enhancement mode.
    pub fn adaptive_brightness(
        &'_ mut self,
    ) -> Configure<'_, X, E, WRCACE, RDCABC, AdaptiveBrightness> {
        Configure::new(self, |state| &mut state.config.adaptive_brightness)
    }

    /// ## Configure CABC Minimum Brightness
    /// > Reference: p. 229, 230
    ///
    /// Get or set the minimum brightness value for Content Adaptive Brightness Control
    /// (CABC).
    pub fn min_adaptive_brightness(
        &'_ mut self,
    ) -> Configure<'_, X, E, WRCABCMB, RDCABCMB, MinAdaptiveBrightness> {
        Configure::new(self, |state| &mut state.config.min_adaptive_brightness)
    }

    /// ## Read Automatic Brightness Control Self-Diagnostic Result
    /// > Reference: p. 231
    ///
    /// Reads the result of the automatic brightness control self-diagnostic test.
    pub fn read_adaptive_brightness_diagnostic(
        &mut self,
        buffer: &mut <RDABCSDR as RxData>::Data,
    ) -> io::Result<()> {
        self.read::<RDABCSDR>(buffer)
    }

    /// ## Read Black/White Low Bits
    /// > Reference: p. 232
    ///
    /// Returns the low bits of the black and white color settings for calibration
    /// and diagnostics.
    pub fn read_black_white_low_bits(
        &mut self,
        buffer: &mut <RDBWLB as RxData>::Data,
    ) -> io::Result<()> {
        self.read::<RDBWLB>(buffer)
    }

    /// ## Read Bkx
    /// > Reference: p. 233
    ///
    /// Reads the Bkx calibration value from the device.
    pub fn read_bkx(&mut self, buffer: &mut <RDBKX as RxData>::Data) -> io::Result<()> {
        self.read::<RDBKX>(buffer)
    }

    /// ## Read Bky
    /// > Reference: p. 234
    ///
    /// Reads the Bky calibration value from the device.
    pub fn read_bky(&mut self, buffer: &mut <RDBKY as RxData>::Data) -> io::Result<()> {
        self.read::<RDBKY>(buffer)
    }

    /// ## Read Wx
    /// > Reference: p. 235
    ///
    /// Reads the Wx calibration value from the device.
    pub fn read_wx(&mut self, buffer: &mut <RDWX as RxData>::Data) -> io::Result<()> {
        self.read::<RDWX>(buffer)
    }

    /// ## Read Wy
    /// > Reference: p. 236
    ///
    /// Reads the Wy calibration value from the device.
    pub fn read_wy(&mut self, buffer: &mut <RDWY as RxData>::Data) -> io::Result<()> {
        self.read::<RDWY>(buffer)
    }

    /// ## Read Rx
    /// > Reference: p. 239
    ///
    /// Reads the Rx calibration value from the device.
    pub fn read_rx(&mut self, buffer: &mut <RDRX as RxData>::Data) -> io::Result<()> {
        self.read::<RDRX>(buffer)
    }

    /// ## Read Ry
    /// > Reference: p. 239
    ///
    /// Reads the Ry calibration value from the device.
    pub fn read_ry(&mut self, buffer: &mut <RDRY as RxData>::Data) -> io::Result<()> {
        self.read::<RDRY>(buffer)
    }

    /// ## Read Gx
    /// > Reference: p. 240
    ///
    /// Reads the Gx calibration value from the device.
    pub fn read_gx(&mut self, buffer: &mut <RDGX as RxData>::Data) -> io::Result<()> {
        self.read::<RDGX>(buffer)
    }

    /// ## Read Gy
    /// > Reference: p. 241
    ///
    /// Reads the Gy calibration value from the device.
    pub fn read_gy(&mut self, buffer: &mut <RDGY as RxData>::Data) -> io::Result<()> {
        self.read::<RDGY>(buffer)
    }

    /// ## Read Blue/A Color Low Bits
    /// > Reference: p. 242
    ///
    /// Returns the low bits of the blue and A color settings for calibration and
    /// diagnostics.
    pub fn read_blue_low_bits(&mut self, buffer: &mut <RDBALB as RxData>::Data) -> io::Result<()> {
        self.read::<RDBALB>(buffer)
    }

    /// ## Read Bx
    /// > Reference: p. 243
    ///
    /// Reads the Bx calibration value from the device.
    pub fn read_bx(&mut self, buffer: &mut <RDBX as RxData>::Data) -> io::Result<()> {
        self.read::<RDBX>(buffer)
    }

    /// ## Read By
    /// > Reference: p. 244
    ///
    /// Reads the By calibration value from the device.
    pub fn read_by(&mut self, buffer: &mut <RDBY as RxData>::Data) -> io::Result<()> {
        self.read::<RDBY>(buffer)
    }

    /// ## Read Ax
    /// > Reference: p. 245
    ///
    /// Reads the Ax calibration value from the device.
    pub fn read_ax(&mut self, buffer: &mut <RDAX as RxData>::Data) -> io::Result<()> {
        self.read::<RDAX>(buffer)
    }

    /// ## Read Ay
    /// > Reference: p. 246
    ///
    /// Reads the Ay calibration value from the device.
    pub fn read_ay(&mut self, buffer: &mut <RDAY as RxData>::Data) -> io::Result<()> {
        self.read::<RDAY>(buffer)
    }

    /// ## Read DDB Start
    /// > Reference: p. 247
    ///
    /// Reads the initial value of the Display Data Bus (DDB) for diagnostics.
    pub fn read_ddbs(&mut self, buffer: &mut <RDDDBS as RxData>::Data) -> io::Result<()> {
        self.read::<RDDDBS>(buffer)
    }

    /// ## Read DDB Continue
    /// > Reference: p. 249
    ///
    /// Reads the next value of the Display Data Bus (DDB) for diagnostics.
    pub fn read_ddbc(&mut self, buffer: &mut <RDDDBC as RxData>::Data) -> io::Result<()> {
        self.read::<RDDDBC>(buffer)
    }

    /// ## Read First Checksum
    /// > Reference: p. 250
    ///
    /// Reads the first checksum value for verifying data integrity.
    pub fn read_fcs(&mut self, buffer: &mut <RDFCS as RxData>::Data) -> io::Result<()> {
        self.read::<RDFCS>(buffer)
    }

    /// ## Read Continue Checksum
    /// > Reference: p. 251
    ///
    /// Reads the next checksum value for continued data integrity verification.
    pub fn read_ccs(&mut self, buffer: &mut <RDCCS as RxData>::Data) -> io::Result<()> {
        self.read::<RDCCS>(buffer)
    }

    /// ## Read ID1
    /// > Reference: p. 252
    ///
    /// Reads the first identification value from the device.
    pub fn read_id1(&mut self, buffer: &mut <RDID1 as RxData>::Data) -> io::Result<()> {
        self.read::<RDID1>(buffer)
    }

    /// ## Read ID2
    /// > Reference: p. 253
    ///
    /// Reads the second identification value from the device.
    pub fn read_id2(&mut self, buffer: &mut <RDID2 as RxData>::Data) -> io::Result<()> {
        self.read::<RDID2>(buffer)
    }

    /// ## Read ID3
    /// > Reference: p. 254
    ///
    /// Reads the third identification value from the device.
    pub fn read_id3(&mut self, buffer: &mut <RDID3 as RxData>::Data) -> io::Result<()> {
        self.read::<RDID3>(buffer)
    }

    /// ## Command2 `BKx` Selection
    /// > Reference: p. 260
    ///
    /// Selects the extended command bank (BK0, BK1, BK3) for subsequent operations.
    /// This command is required before sending any extended command and ensures the
    /// correct register bank is active.
    pub fn select_command_extension<N: Extension>(
        mut self,
        extension: N,
    ) -> io::Result<ST7701S<X, N>> {
        let transmission = N::EXTENSION.map_or_else(
            || CommandExtension::new().set_extended_commands(Off),
            |bank| {
                CommandExtension::new()
                    .set_extended_commands(On)
                    .set_bank(bank)
            },
        );

        self.write::<CND2BKXSEL>(transmission.buffer())
            .inspect(|()| {
                self.modify_state(|state| {
                    state.command_extension = transmission;
                });
            })?;

        Ok(self.set_extension(extension))
    }

    /// ## `Special: 0xFF` `DSTB` Deep Standby Mode Enable
    /// > Reference: p. 285
    ///
    /// Enables deep standby mode, reducing power consumption to a minimum. The
    /// display will not respond to most commands until reactivated.
    pub fn deep_standby_enable(&mut self) -> io::Result<()> {
        const PARAMS: [u8; 5] = [0x77, 0x01, 0x00, 0x00, 0x13];
        self.write::<DSTB>(&PARAMS)
    }

    /// ## `Special: 0xFF` `DSTBT` Deep Standby Mode Active
    /// > Reference: p. 286
    ///
    /// Indicates whether deep standby mode is currently active. Used for
    /// diagnostics and power management.
    pub fn deep_standby_active(&mut self) -> io::Result<()> {
        const PARAMS: [u8; 5] = [0x77, 0x01, 0x00, 0x00, 0x13];
        self.write::<DSTBT>(&PARAMS)
    }
}

#[allow(clippy::missing_errors_doc, reason = "IO errors are self-explanatory")]
impl<X: Connection> ST7701S<X, Bank0> {
    /// ## `BK0: 0xB0` `PVGAMCTRL` Positive Voltage Gamma Control
    /// > See p. 261
    ///
    /// Configures the positive voltage gamma curve for the display. This command
    /// allows fine-tuning of the display's color response and image quality by
    /// setting multiple voltage control points.
    pub fn positive_gamma_control(&mut self, parameters: &VoltageControl) -> io::Result<()> {
        self.write::<PVGAMCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xB1` `NVGAMCTRL` Negative Voltage Gamma Control
    /// > Reference: p. 263
    ///
    /// Configures the negative voltage gamma curve for the display. This command
    /// complements PVGAMCTRL and is used to adjust the display's color response for
    /// negative voltages.
    pub fn negative_gamma_control(&mut self, parameters: &VoltageControl) -> io::Result<()> {
        self.write::<NVGAMCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xB8` `DGMEN` Digital Gamma Enable
    /// > Reference: p. 265
    ///
    /// Enables or disables digital gamma correction. When enabled, the display uses
    /// digital gamma look-up tables for color adjustment.
    pub fn digital_gamma_enable(&mut self, enable: u8) -> io::Result<()> {
        self.write::<DGMEN>(&[enable])
    }

    /// ## `BK0: 0xB9` `DGMLUTR` Digital Gamma Look-up Table for Red
    /// > Reference: p. 266
    ///
    /// Sets the digital gamma look-up table for the red color channel. Each entry
    /// defines the gamma correction for a specific input value.
    pub fn digital_gamma_lut_red(&mut self, lut_data: &GammaLutRed) -> io::Result<()> {
        self.write::<DGMLUTR>(lut_data)
    }

    /// ## `BK0: 0xBA` `DGMLUTB` Digital Gamma Look-up Table for Blue
    /// > Reference: p. 267
    ///
    /// Sets the digital gamma look-up table for the blue color channel. Each entry
    /// defines the gamma correction for a specific input value.
    pub fn digital_gamma_lut_blue(&mut self, lut_data: &GammaLutBlue) -> io::Result<()> {
        self.write::<DGMLUTB>(lut_data)
    }

    /// ## `BK0: 0xBC` `PWMCLKSEL` PWM CLK select
    /// > Reference: p. 268
    ///
    /// Selects the clock source for the PWM signal used in backlight control.
    pub fn pwm_clock_select(&mut self, clock_setting: u8) -> io::Result<()> {
        self.write::<PWMCLKSEL>(&[clock_setting])
    }

    /// ## `BK0: 0xC0` `LNESET` Display Line Setting
    /// > Reference: p. 269
    ///
    /// Configures the number of display lines and line delta for the panel. This
    /// affects the vertical resolution and timing.
    pub fn line_setting(&mut self, parameters: &LineSettings) -> io::Result<()> {
        self.write::<LNESET>(parameters.buffer())
    }

    /// ## `BK0: 0xC1` `PORCTRL` Porch Control
    /// > Reference: p. 270
    ///
    /// Sets the front and back porch timing for the display. Proper porch settings
    /// are important for stable image rendering and synchronization.
    pub fn porch_control(&mut self, parameters: &PorchControl) -> io::Result<()> {
        self.write::<PORCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xC2` `INVSEL` Inversion Selection & Frame Rate Control
    /// > Reference: p. 271
    ///
    /// Controls display inversion and frame rate settings. Proper inversion settings
    /// ensure correct color representation and can affect display smoothness.
    pub fn inversion_select(&mut self, parameters: &InversionSelection) -> io::Result<()> {
        self.write::<INVSET>(parameters.buffer())
    }

    /// ## `BK0: 0xC3` `RGBCTRL` RGB control
    /// > Reference: p. 272
    ///
    /// Configures the RGB interface mode and signal polarities. This command is
    /// essential for matching the display's timing and signal requirements to the
    /// host system.
    pub fn rgb_control(&mut self, parameters: &RgbControl) -> io::Result<()> {
        self.write::<RGBCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xC5` `PARCTRL` Partial Area Control
    /// > Reference: p. 273
    ///
    /// Configures partial display mode settings, allowing only a portion of the
    /// display to be updated for power savings or special effects.
    pub fn partial_area(&mut self, parameters: &PartialControl) -> io::Result<()> {
        self.write::<PARCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xC7` `SDIR` X-direction Control
    /// > Reference: p. 274
    ///
    /// Sets the direction of pixel scanning along the X-axis. This is used for
    /// display orientation and mirroring.
    pub fn scan_direction(&mut self, parameters: &ScanDirectionControl) -> io::Result<()> {
        self.write::<SDIR>(parameters.buffer())
    }

    /// ## `BK0: 0xC8` `PDOSET` Pseudo-Dot inversion diving setting
    /// > Reference: p. 275
    ///
    /// Configures pseudo-dot inversion settings to improve display uniformity and
    /// reduce artifacts.
    pub fn pseudo_dot_inversion(&mut self, parameters: &PseudoDotInversion) -> io::Result<()> {
        self.write::<PDOSET>(parameters.buffer())
    }

    /// ## `BK0: 0xCD` `COLCTRL` Color Control
    /// > Reference: p. 276
    ///
    /// Adjusts color control parameters such as PWM polarity, LED polarity, pixel
    /// format, and end pixel format. These settings affect color rendering and
    /// backlight behavior.
    pub fn color_control(&mut self, parameters: &ColorControl) -> io::Result<()> {
        self.write::<COLCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xE0` `SRECTRL` Sunlight Readable Enhancement
    /// > Reference: p. 278
    ///
    /// Enables and configures sunlight readability enhancement features, improving
    /// display visibility in bright environments.
    pub fn sunlight_readable_enhancement(
        &mut self,
        parameters: &SunlightEnhancement,
    ) -> io::Result<()> {
        self.write::<SRECTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xE1` `NRCTRL` Noise Reduce Control
    /// > Reference: p. 279
    ///
    /// Sets noise reduction parameters to improve image quality and reduce visual
    /// artifacts.
    pub fn noise_reduction_control(&mut self, parameters: &NoiseReduction) -> io::Result<()> {
        self.write::<NRCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xE2` `SECTRL` Sharpness and Edge Enhancement
    /// > Reference: p. 280
    ///
    /// Adjusts image sharpness and edge enhancement algorithms to improve perceived
    /// image clarity and detail definition.
    pub fn sharpness_control(&mut self, parameters: &SharpnessControl) -> io::Result<()> {
        self.write::<SECTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xE3` `CCCTRL` Color Calibration
    /// > Reference: p. 281
    ///
    /// Sets color calibration parameters to ensure accurate color reproduction
    /// across different viewing conditions and manufacturing tolerances.
    pub fn color_calibration_control(&mut self, parameters: &ColorCalibration) -> io::Result<()> {
        self.write::<CCCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xE4` `SKCTRL` Skin Tone Preservation
    /// > Reference: p. 282
    ///
    /// Enables skin tone preservation features for more natural human skin
    /// representation in images and videos.
    pub fn skin_tone_control(&mut self, parameters: &SkinToneControl) -> io::Result<()> {
        self.write::<SKCTRL>(parameters.buffer())
    }

    /// ## `BK0: 0xEA` `NVMSETE` NVM Set Enable
    /// > Reference: p. 283
    ///
    /// Enables or disables Non-Volatile Memory settings for persistent configuration.
    pub fn nvm_set_enable(&mut self, enable: u8) -> io::Result<()> {
        self.write::<NVMSETE>(&[enable])
    }

    /// ## `BK0: 0xEE` `CABCCTRL` Content Adaptive Brightness Control
    /// > Reference: p. 284
    ///
    /// Controls Content Adaptive Brightness Control for dynamic backlight adjustment
    /// based on image content to save power and improve visibility.
    pub fn cabc_control(&mut self, setting: u8) -> io::Result<()> {
        self.write::<CABCCTRL>(&[setting])
    }
}

#[allow(clippy::missing_errors_doc, reason = "IO errors are self-explanatory")]
impl<X: Connection> ST7701S<X, Bank1> {
    /// ## `BK1: 0xB0` `VRHS` VOP Amplitude Setting
    /// > Reference: p. 283
    ///
    /// Sets the positive voltage amplitude (VOP) for the voltage regulator.
    /// `Vop = 3.5375 + (VRHA[7:0] x 0.0125);`
    pub fn set_operating_voltage(&mut self, parameters: &OperatingVoltage) -> io::Result<()> {
        self.write::<VRHS>(parameters.buffer())
    }

    /// ## `BK1: 0xB1` `VCOMS` VCOM Setting
    /// > Reference: p. 284
    ///
    /// Configures the VCOM voltage level for optimal display performance and
    /// contrast.
    pub fn set_common_voltage(&mut self, parameters: &CommonVoltage) -> io::Result<()> {
        self.write::<VCOMS>(parameters.buffer())
    }

    /// ## `BK1: 0xB2` `VGHSS` VGH Voltage Setting
    /// > Reference: p. 285
    ///
    /// Sets the VGH (gate high) voltage level.
    pub fn set_gate_high_voltage(&mut self, parameters: &GateHighVoltage) -> io::Result<()> {
        self.write::<VGHSS>(parameters.buffer())
    }

    /// ## `BK1: 0xB3` `TESTCMD` Test Command
    /// > Reference: p. 286
    ///
    /// Unknown purpose, not documented.
    pub fn test_command(&mut self) -> io::Result<()> {
        const VALUE: u8 = 0x80;
        self.write::<TESTCMD>(&[VALUE])
    }

    /// ## `BK1: 0xB5` `VGLS` VGL Voltage Setting
    /// > Reference: p. 287
    ///
    /// Sets the VGL (gate low) voltage level.
    pub fn set_gate_low_voltage(&mut self, parameters: &GateLowVoltage) -> io::Result<()> {
        self.write::<VGLS>(parameters.buffer())
    }

    // ## `BK1: 0xB7` `PWCTRL1` Power Control 1
    // > Reference: p. 288
    //
    // Primary power control settings including AVDD, AVEE, and VGH/VGL multipliers.
    // pub fn power_control_1(&mut self, parameters: &PowerControl1) -> io::Result<()> {
    //     self.write::<PWCTRL1>(parameters)
    // }

    /// ## `BK1: 0xB8` `PWCTRL2` Power Control 2
    /// > Reference: p. 289
    ///
    /// Secondary power control settings for fine-tuning voltage generation.
    pub fn power_control_2(&mut self, parameters: &PowerControl2) -> io::Result<()> {
        self.write::<PWCTRL2>(parameters.buffer())
    }

    /// ## `BK1: 0xBA` `PCLKS1` Panel Clock Setting 1
    /// > Reference: p. 294
    ///
    /// Configures the primary panel clock settings for display timing control.
    pub fn panel_clock_setting_1(&mut self, parameters: &PanelClockSetting1) -> io::Result<()> {
        self.write::<PCLKS1>(parameters.buffer())
    }

    /// ## `BK1: 0xBB` `PCLKS2` Panel Clock Setting 2
    /// > Reference: p. 295
    ///
    /// Sets secondary panel clock parameters for fine timing adjustments.
    pub fn panel_clock_setting_2(&mut self, parameters: &PanelClockSetting2) -> io::Result<()> {
        self.write::<PCLKS2>(parameters.buffer())
    }

    /// ## `BK1: 0xBC` `PCLKS3` Panel Clock Setting 3
    /// > Reference: p. 296
    ///
    /// Adjusts tertiary panel clock settings for advanced timing control.
    pub fn panel_clock_setting_3(&mut self, parameters: &PanelClockSetting3) -> io::Result<()> {
        self.write::<PCLKS3>(parameters.buffer())
    }

    /// ## `BK1: 0xC1` `SPD1` Source Pre-Drive Timing Set 1
    /// > Reference: p. 298
    ///
    /// Configures timing parameters for the source driver pre-drive stage, which
    /// affects signal integrity and display performance.
    pub fn source_pre_drive_timing_1(
        &mut self,
        parameters: &SourcePreDriveTiming1,
    ) -> io::Result<()> {
        self.write::<SPD1>(parameters.buffer())
    }

    /// ## `BK1: 0xC2` `SPD2` Source Pre-Drive Timing Set 2
    /// > Reference: p. 299
    ///
    /// Fine-tunes additional source pre-drive timing parameters for display optimization.
    pub fn source_pre_drive_timing_2(
        &mut self,
        parameters: &SourcePreDriveTiming2,
    ) -> io::Result<()> {
        self.write::<SPD2>(parameters.buffer())
    }

    /// ## `BK1: 0xD0` `MIPISET1` MIPI Setting 1
    /// > Reference: p. 299
    ///
    /// Configures primary MIPI interface settings for communication with the host.
    pub fn mipi_setting_1(&mut self, parameters: &MipiSetting1) -> io::Result<()> {
        self.write::<MIPISET1>(parameters.buffer())
    }

    /// ## `BK1: 0xD1` `MIPISET2` MIPI Setting 2
    /// > Reference: p. 300
    ///
    /// Sets detailed MIPI communication parameters for advanced interface control.
    pub fn mipi_setting_2(&mut self, parameters: &MipiSetting2) -> io::Result<()> {
        self.write::<MIPISET2>(parameters.buffer())
    }

    /// ## `BK1: 0xD2` `MIPISET3` MIPI Setting 3
    /// > Reference: p. 301
    ///
    /// Configures additional MIPI interface parameters.
    pub fn mipi_setting_3(&mut self, parameters: &MipiSetting3) -> io::Result<()> {
        self.write::<MIPISET3>(parameters.buffer())
    }

    /// ## `BK1: 0xD3` `MIPISET4` MIPI Setting 4
    /// > Reference: p. 302
    ///
    /// Sets final MIPI interface settings for complete configuration.
    pub fn mipi_setting_4(&mut self, parameters: &MipiSetting4) -> io::Result<()> {
        self.write::<MIPISET4>(parameters.buffer())
    }
}

#[allow(clippy::missing_errors_doc, reason = "IO errors are self-explanatory")]
impl<X: Connection> ST7701S<X, Bank3> {
    /// ## `BK3: 0xCA` `NVMSET` NVM Setting
    /// > Reference: p. 304
    ///
    /// Configures Non-Volatile Memory settings for persistent display configuration.
    pub fn nvm_setting(&mut self, setting: u8) -> io::Result<()> {
        self.write::<NVMSET>(&[setting])
    }

    /// ## `BK3: 0xCC` `PROMACT` PROM Activation
    /// > Reference: p. 305
    ///
    /// Activates PROM (Programmable Read-Only Memory) for factory settings access.
    pub fn prom_activation(&mut self, setting: u8) -> io::Result<()> {
        self.write::<PROMACT>(&[setting])
    }
}
