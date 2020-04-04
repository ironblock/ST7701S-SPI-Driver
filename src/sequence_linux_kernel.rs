use std::{thread, time};

use crate::instructions::{
    BK0Command2, BK1Command2, BitsPerPixel, ColorOrder, Command, Command2Selection,
    CommandsGeneral, DataEnable, DataPolarity, EnablePolarity, GammaOPBias, HsyncActive, Inversion,
    ScanDirection, SourceOPInput, SourceOPOutput, TearingEffect, VoltageAVCL, VoltageAVDD,
    VsyncActive,
};
use crate::panel::Mode;
use crate::spi::ST7701S;

pub fn init(display: &mut ST7701S, mode: Mode) {
    let mut CMD2: Command2Selection = Command2Selection::Disabled;

    // SOFTWARE RESET
    // 5ms delay
    display.write_command(CommandsGeneral::software_reset());
    thread::sleep(time::Duration::from_millis(10));

    // EXIT SLEEP MODE
    // Variable delay (200ms is "safe")
    display.write_command(CommandsGeneral::sleep_mode_off());
    thread::sleep(time::Duration::from_millis(300));

    // ENTER BK0 COMMAND2 MODE
    display.write_command(CommandsGeneral::set_command_2(Command2Selection::BK0));
    CMD2 = Command2Selection::BK0;

    display.write_command(BK0Command2::positive_gamma_control(
        &CMD2,
        &[
            0x00, 0x0E, 0x15, 0x0F, 0x11, 0x08, 0x08, 0x08, 0x08, 0x23, 0x04, 0x13, 0x12, 0x2B,
            0x34, 0x1F,
        ],
    ));
    display.write_command(BK0Command2::negative_gamma_control(
        &CMD2,
        &[
            0x00, 0x0E, 0x95, 0x0F, 0x13, 0x07, 0x09, 0x08, 0x08, 0x22, 0x04, 0x10, 0x0E, 0x2C,
            0x34, 0x1F,
        ],
    ));
    display.write_command(BK0Command2::display_line_setting(&CMD2, 0x80, 0x69, 0x02));
    display.write_command(BK0Command2::porch_control(&CMD2, &mode));
    display.write_command(BK0Command2::inversion_select(
        &CMD2,
        Inversion::Column,
        0xFF,
    ));
    display.write_command(BK0Command2::rgb_control(
        &CMD2,
        DataEnable::DE,
        VsyncActive::Low,
        HsyncActive::Low,
        DataPolarity::Rising,
        EnablePolarity::Low,
        &mode,
    ));

    // ENTER BK1 COMMAND2 MODE
    display.write_command(CommandsGeneral::set_command_2(Command2Selection::BK1));
    CMD2 = Command2Selection::BK1;

    display.write_command(BK1Command2::set_vop_amplitude(&CMD2, 0x45));
    display.write_command(BK1Command2::set_vcom_amplitude(&CMD2, 0x13));
    display.write_command(BK1Command2::set_vgh_voltage(&CMD2, 0x07));
    display.write_command(BK1Command2::test_command_setting(&CMD2));
    display.write_command(BK1Command2::set_vgl_voltage(&CMD2, 0x07));
    display.write_command(BK1Command2::power_control_one(
        &CMD2,
        GammaOPBias::Middle,
        SourceOPInput::Min,
        SourceOPOutput::Off,
    ));

    display.write_command(BK1Command2::power_control_two(
        &CMD2,
        VoltageAVDD::Pos6_6,
        VoltageAVCL::Neg4_4,
    ));
    display.write_command(BK1Command2::set_pre_drive_timing_one(&CMD2, 0x03));
    display.write_command(BK1Command2::set_pre_drive_timing_two(&CMD2, 0x03));

    // UNKNOWABLE CARGO-CULTED MYSTERY MEAT
    //
    // I copied this command sequence from the Linux MIPI driver for the ST7701,
    // written in C. The author of that driver _also_ had no idea what this
    // command sequence does or means, and claims to have himself copied it from
    // a sample provided by a Sitronix engineer. Since this is a Linux SPI
    // driver for the ST7701S written in Rust, there's ample opportunity for
    // something to not line up.
    //
    // May whatever gods you pray to have mercy on our souls.
    display.write_command(Ok(Command {
        address: 0xE0,
        parameters: vec![0x00, 0x00, 0x02],
    }));
    display.write_command(Ok(Command {
        address: 0xE1,
        parameters: vec![
            0x0B, 0x00, 0x0D, 0x00, 0x0C, 0x00, 0x0E, 0x00, 0x00, 0x44, 0x44,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xE2,
        parameters: vec![
            0x33, 0x33, 0x44, 0x44, 0x64, 0x00, 0x66, 0x00, 0x65, 0x00, 0x67, 0x00, 0x00,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xE3,
        parameters: vec![0x00, 0x00, 0x33, 0x33],
    }));
    display.write_command(Ok(Command {
        address: 0xE4,
        parameters: vec![0x44, 0x44],
    }));
    display.write_command(Ok(Command {
        address: 0xE5,
        parameters: vec![
            0x0C, 0x78, 0x3C, 0xA0, 0x0E, 0x78, 0x3C, 0xA0, 0x10, 0x78, 0x3C, 0xA0, 0x12, 0x78,
            0x3C, 0xA0,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xE6,
        parameters: vec![0x00, 0x00, 0x33, 0x33],
    }));
    display.write_command(Ok(Command {
        address: 0xE7,
        parameters: vec![0x44, 0x44],
    }));
    display.write_command(Ok(Command {
        address: 0xE8,
        parameters: vec![
            0x0D, 0x78, 0x3C, 0xA0, 0x0F, 0x78, 0x3C, 0xA0, 0x11, 0x78, 0x3C, 0xA0, 0x13, 0x78,
            0x3C, 0xA0,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xEB,
        parameters: vec![0x02, 0x02, 0x39, 0x39, 0xEE, 0x44, 0x00],
    }));
    display.write_command(Ok(Command {
        address: 0xEC,
        parameters: vec![0x00, 0x00],
    }));
    display.write_command(Ok(Command {
        address: 0xED,
        parameters: vec![
            0xFF, 0xF1, 0x04, 0x56, 0x72, 0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xF3, 0x27, 0x65, 0x40,
            0x1F, 0xFF,
        ],
    }));

    // BK1 COMMAND2 DISABLE
    display.write_command(CommandsGeneral::set_command_2(Command2Selection::Disabled));
    CMD2 = Command2Selection::Disabled;

    display.write_command(CommandsGeneral::set_color_mode(BitsPerPixel::Rgb666));
    display.write_command(CommandsGeneral::display_data_control(
        ScanDirection::Normal,
        ColorOrder::Rgb,
    ));
    display.write_command(CommandsGeneral::tearing_effect_on(TearingEffect::VHBlank));

    display.write_command(CommandsGeneral::display_on());
    thread::sleep(time::Duration::from_millis(200));
}
