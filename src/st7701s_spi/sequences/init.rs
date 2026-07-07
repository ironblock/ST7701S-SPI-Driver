use std::{io, thread, time};

use crate::st7701s_spi::{
    device::ST7701S,
    instructions::core::{COLMOD, MADCTL},
    parameters::{
        bk0_display::{ColorControl, MDT},
        bk1_power::{CommonVoltage, GateHighVoltage, GateLowVoltage, OperatingVoltage},
        display::{
            InversionSelection, LineSettings, PolarityInversion, PorchControl, VoltageBias,
            VoltageControl,
        },
        general::Switch,
    },
    protocol::connection::{AnyExtension, Bank0, Bank1, Bank3, Connection, ConnectionOwner as _},
};

use Switch::Off;

/// Initializes the panel using the sequence provided by Shanghai Top Display
/// Optoelectronics (TDO) for the TL021WVC02 module.
///
/// Every transmission this function produces is verified byte-for-byte
/// against `TDO_init_sequence.txt` by the `golden_init` integration test.
/// Typed builders are used wherever the register is understood well enough
/// to model; the remainder is sent verbatim via [`ST7701S::write_raw`].
pub fn init_sequence<X: Connection, E>(
    display: ST7701S<X, E>,
) -> io::Result<ST7701S<X, AnyExtension>> {
    let mut display = display.select_command_extension(Bank0);

    // LNESET 0x3B: (0x3B + 1) * 8 = 480 display lines.
    display.line_setting(
        &LineSettings::new()
            .set_extra_line(Off)
            .set_line_const::<0x3B>(),
    )?;

    display.porch_control(
        &PorchControl::new()
            .set_vertical_back_porch_const::<0x0B>()
            .set_vertical_front_porch_const::<0x02>(),
    )?;

    display.inversion_select(
        &InversionSelection::new()
            .set_polarity_inversion(PolarityInversion::OneDot)
            .set_rtni_const::<0x02>(),
    )?;

    // 0xCC does not appear anywhere in the datasheet; the reference sequence
    // sends it between INVSEL and COLCTRL.
    display.write_raw(0xCC, &[0x10])?;

    display.color_control(&ColorControl::new().set_mdt(MDT::CollectToDB))?;

    let positive_gamma = VoltageControl::new()
        .set_aj0(VoltageBias::A)
        .set_vc0_const::<0x02>()
        .set_aj1(VoltageBias::A)
        .set_vc4_const::<0x13>()
        .set_aj2(VoltageBias::A)
        .set_vc8_const::<0x1B>()
        .set_vc16_const::<0x0D>()
        .set_aj3(VoltageBias::A)
        .set_vc24_const::<0x10>()
        .set_vc52_const::<0x05>()
        .set_vc80_const::<0x08>()
        .set_vc108_const::<0x07>()
        .set_vc147_const::<0x07>()
        .set_vc175_const::<0x24>()
        .set_vc203_const::<0x04>()
        .set_aj4(VoltageBias::A)
        .set_vc231_const::<0x11>()
        .set_vc239_const::<0x0E>()
        .set_aj5(VoltageBias::A)
        .set_vc247_const::<0x2C>()
        .set_aj6(VoltageBias::A)
        .set_vc251_const::<0x33>()
        .set_aj7(VoltageBias::A)
        .set_vc255_const::<0x1D>();

    // The negative curve differs from the positive curve only in VC0 and
    // VC24. Field setters replace previous values, so the positive curve can
    // be used as the starting point.
    let negative_gamma = positive_gamma
        .set_vc0_const::<0x05>()
        .set_vc24_const::<0x11>();

    display.positive_gamma_control(&positive_gamma)?;
    display.negative_gamma_control(&negative_gamma)?;

    let mut display = display.select_command_extension(Bank1);

    display.set_operating_voltage(&OperatingVoltage::new().set_amplitude_const::<0x5D>())?;

    display.set_common_voltage(&CommonVoltage::new().set_amplitude_const::<0x43>())?;

    display.set_gate_high_voltage(&GateHighVoltage::new().set_amplitude_volts(12.0))?;

    display.test_command()?;

    display.set_gate_low_voltage(&GateLowVoltage::new().set_amplitude_volts(-8.14))?;

    // The remainder of the BK1 section reproduces the reference bytes
    // verbatim. PWCTRL2/SPD1/SPD2/MIPISET1 have typed models, but their field
    // semantics have not been verified against datasheet v1.2 (and 0xD0 =
    // 0x88 is not even expressible in the current MIPISET1 layout), so the
    // known-good values are sent raw until they are.
    display.write_raw(0xB7, &[0x85])?; // PWCTRL1
    display.write_raw(0xB8, &[0x20])?; // PWCTRL2
    display.write_raw(0xC1, &[0x78])?; // SPD1
    display.write_raw(0xC2, &[0x78])?; // SPD2
    display.write_raw(0xD0, &[0x88])?; // MIPISET1

    // 0xE0-0xEF are undocumented in the public datasheet. Vendor and Linux
    // kernel sequences both ship opaque blobs here; these are the TDO
    // reference values, transcribed from `TDO_init_sequence.txt`.
    display.write_raw(0xE0, &[0x00, 0x00, 0x02])?;
    display.write_raw(
        0xE1,
        &[
            0x03, 0xA0, 0x00, 0x00, 0x04, 0xA0, 0x00, 0x00, 0x00, 0x20, 0x20,
        ],
    )?;
    display.write_raw(0xE2, &[0x00; 13])?;
    display.write_raw(0xE3, &[0x00, 0x00, 0x11, 0x00])?;
    display.write_raw(0xE4, &[0x22, 0x00])?;
    display.write_raw(
        0xE5,
        &[
            0x05, 0xEC, 0xA0, 0xA0, 0x07, 0xEE, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    )?;
    display.write_raw(0xE6, &[0x00, 0x00, 0x11, 0x00])?;
    display.write_raw(0xE7, &[0x22, 0x00])?;
    display.write_raw(
        0xE8,
        &[
            0x06, 0xED, 0xA0, 0xA0, 0x08, 0xEF, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    )?;
    display.write_raw(0xEB, &[0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x00])?;
    display.write_raw(
        0xED,
        &[
            0xFF, 0xFF, 0xFF, 0xBA, 0x0A, 0xBF, 0x45, 0xFF, 0xFF, 0x54, 0xFB, 0xA0, 0xAB, 0xFF,
            0xFF, 0xFF,
        ],
    )?;
    display.write_raw(0xEF, &[0x10, 0x0D, 0x04, 0x08, 0x3F, 0x1F])?;

    // The reference sequence briefly enters BK3 for one undocumented write.
    // Note that 0xEF here is a different (1-byte) register than the 6-byte
    // 0xEF in BK1 above.
    let mut display = display.select_command_extension(Bank3);
    display.write_raw(0xEF, &[0x08])?;

    let mut display = display.select_command_extension(AnyExtension);

    display.sleep_mode().off()?;
    thread::sleep(time::Duration::from_millis(120));

    display.display_output().on()?;

    // MADCTL: normal scan direction, RGB color order.
    // COLMOD: 18-bit RGB666 (0x50 would select 16-bit RGB565).
    // TODO: model these with typed parameters (see `parameters::data_access`).
    display.write::<MADCTL>(&[0x00])?;
    display.write::<COLMOD>(&[0x60])?;

    Ok(display)
}
