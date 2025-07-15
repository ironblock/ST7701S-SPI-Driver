use std::{thread, time};

use crate::st7701s_spi::{
    commands::{InstructionBK0, InstructionBK1, OldCommand, ExtensionRegister, Opcode},
    panel::Mode,
    parameters::{
        BitsPerPixel, ColorOrder, DataEnable, DataPolarity, EnablePolarity, GammaOPBias,
        HsyncActive, Inversion, ScanDirection, SourceOPInput, SourceOPOutput, TearingEffect,
        VoltageAVCL, VoltageAVDD, VsyncActive,
    },
    spi::HalfDuplexSPI,
};

pub fn init(display: &mut HalfDuplexSPI, mode: Mode) {
    let mut cmd2: ExtensionRegister = ExtensionRegister::Disabled;

    // SOFTWARE RESET
    // 5ms delay
    display.write_command(Opcode::software_reset());
    thread::sleep(time::Duration::from_millis(10));

    // EXIT SLEEP MODE
    // Variable delay (200ms is "safe")
    display.write_command(Opcode::sleep_mode_off());
    thread::sleep(time::Duration::from_millis(300));

    // ENTER BK0 COMMAND2 MODE
    display.write_command(Opcode::set_command_2(ExtensionRegister::BK0));
    cmd2 = ExtensionRegister::BK0;

    display.write_command(InstructionBK0::positive_gamma_control(
        &cmd2,
        &[
            0x00, 0x0E, 0x15, 0x0F, 0x11, 0x08, 0x08, 0x08, 0x08, 0x23, 0x04, 0x13, 0x12, 0x2B,
            0x34, 0x1F,
        ],
    ));
    display.write_command(InstructionBK0::negative_gamma_control(
        &cmd2,
        &[
            0x00, 0x0E, 0x95, 0x0F, 0x13, 0x07, 0x09, 0x08, 0x08, 0x22, 0x04, 0x10, 0x0E, 0x2C,
            0x34, 0x1F,
        ],
    ));
    display.write_command(InstructionBK0::display_line_setting(&cmd2, 0x80, 0x69, 0x02));
    display.write_command(InstructionBK0::porch_control(&cmd2, &mode));
    display.write_command(InstructionBK0::inversion_select(
        &cmd2,
        Inversion::Column,
        0xFF,
    ));
    display.write_command(InstructionBK0::rgb_control(
        &cmd2,
        DataEnable::DE,
        VsyncActive::Low,
        HsyncActive::Low,
        DataPolarity::Rising,
        EnablePolarity::Low,
        &mode,
    ));

    // ENTER BK1 COMMAND2 MODE
    display.write_command(Opcode::set_command_2(ExtensionRegister::BK1));
    cmd2 = ExtensionRegister::BK1;

    display.write_command(InstructionBK1::set_vop_amplitude(&cmd2, 0x45));
    display.write_command(InstructionBK1::set_vcom_amplitude(&cmd2, 0x13));
    display.write_command(InstructionBK1::set_vgh_voltage(&cmd2, 0x07));
    display.write_command(InstructionBK1::test_command_setting(&cmd2));
    display.write_command(InstructionBK1::set_vgl_voltage(&cmd2, 0x07));
    display.write_command(InstructionBK1::power_control_one(
        &cmd2,
        GammaOPBias::Middle,
        SourceOPInput::Min,
        SourceOPOutput::Off,
    ));

    display.write_command(InstructionBK1::power_control_two(
        &cmd2,
        VoltageAVDD::Pos6_6,
        VoltageAVCL::Neg4_4,
    ));
    display.write_command(InstructionBK1::set_pre_drive_timing_one(&cmd2, 0x03));
    display.write_command(InstructionBK1::set_pre_drive_timing_two(&cmd2, 0x03));

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
    display.write_command(Ok(OldCommand {
        address: 0xE0,
        parameters: vec![0x00, 0x00, 0x02],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE1,
        parameters: vec![
            0x0B, 0x00, 0x0D, 0x00, 0x0C, 0x00, 0x0E, 0x00, 0x00, 0x44, 0x44,
        ],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE2,
        parameters: vec![
            0x33, 0x33, 0x44, 0x44, 0x64, 0x00, 0x66, 0x00, 0x65, 0x00, 0x67, 0x00, 0x00,
        ],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE3,
        parameters: vec![0x00, 0x00, 0x33, 0x33],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE4,
        parameters: vec![0x44, 0x44],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE5,
        parameters: vec![
            0x0C, 0x78, 0x3C, 0xA0, 0x0E, 0x78, 0x3C, 0xA0, 0x10, 0x78, 0x3C, 0xA0, 0x12, 0x78,
            0x3C, 0xA0,
        ],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE6,
        parameters: vec![0x00, 0x00, 0x33, 0x33],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE7,
        parameters: vec![0x44, 0x44],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xE8,
        parameters: vec![
            0x0D, 0x78, 0x3C, 0xA0, 0x0F, 0x78, 0x3C, 0xA0, 0x11, 0x78, 0x3C, 0xA0, 0x13, 0x78,
            0x3C, 0xA0,
        ],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xEB,
        parameters: vec![0x02, 0x02, 0x39, 0x39, 0xEE, 0x44, 0x00],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xEC,
        parameters: vec![0x00, 0x00],
    }));
    display.write_command(Ok(OldCommand {
        address: 0xED,
        parameters: vec![
            0xFF, 0xF1, 0x04, 0x56, 0x72, 0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xF3, 0x27, 0x65, 0x40,
            0x1F, 0xFF,
        ],
    }));

    // BK1 COMMAND2 DISABLE
    display.write_command(Opcode::set_command_2(ExtensionRegister::Disabled));
    cmd2 = ExtensionRegister::Disabled;

    display.write_command(Opcode::set_color_mode(BitsPerPixel::Rgb666));
    display.write_command(Opcode::display_data_control(
        ScanDirection::Normal,
        ColorOrder::Rgb,
    ));
    display.write_command(Opcode::tearing_effect_on(TearingEffect::VHBlank));

    display.write_command(Opcode::display_on());
    thread::sleep(time::Duration::from_millis(200));
}
