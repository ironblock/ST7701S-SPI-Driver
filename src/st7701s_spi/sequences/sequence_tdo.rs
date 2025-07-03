use std::{thread, time};

use crate::st7701s_spi::{
    commands::{BK0Command2, BK1Command2, Command, Command2Selection, CommandsGeneral},
    panel::Mode,
    parameters::{
        BitsPerPixel, ColorOrder, EndPixelFormat, GammaOPBias, Inversion, LEDPolarity, PWMPolarity,
        PixelPinout, ScanDirection, SourceOPInput, SourceOPOutput, VoltageAVCL, VoltageAVDD,
    },
    spi::HalfDuplexSPI,
};

pub fn init(display: &mut HalfDuplexSPI, mode: Mode) {
    let mut cmd2: Command2Selection = Command2Selection::Disabled;

    // Set Command2 for BK0
    display.write_command(CommandsGeneral::set_command_2(Command2Selection::BK0));
    cmd2 = Command2Selection::BK0;

    display.write_command(BK0Command2::display_line_setting(&cmd2, 0x3B, 0x00, 0x00));
    // Note: This will be off by one from the TDO spec:
    // SPI_WriteComm(0xC1); // PORCTRL
    // 0x0B); // V,
    // 0x02); // V,
    display.write_command(BK0Command2::porch_control(&cmd2, &mode));
    display.write_command(BK0Command2::inversion_select(
        &cmd2,
        Inversion::OneDot,
        0x02,
    ));
    display.write_command(Ok(Command {
        address: 0xCC,
        parameters: vec![0x10],
    }));
    display.write_command(BK0Command2::color_control(
        &cmd2,
        PWMPolarity::Low,
        LEDPolarity::Low,
        PixelPinout::Condensed,
        EndPixelFormat::SelfMSB,
    ));
    display.write_command(BK0Command2::positive_gamma_control(
        &cmd2,
        &[
            0x02, 0x13, 0x1B, 0x0D, 0x10, 0x05, 0x08, 0x07, 0x07, 0x24, 0x04, 0x11, 0x0E, 0x2C,
            0x33, 0x1D,
        ],
    ));
    display.write_command(BK0Command2::negative_gamma_control(
        &cmd2,
        &[
            0xB1, 0x05, 0x13, 0x1B, 0x0D, 0x11, 0x05, 0x08, 0x07, 0x07, 0x24, 0x04, 0x11, 0x0E,
            0x2C, 0x33, 0x1D,
        ],
    ));

    // Set Command2 for BK1
    display.write_command(CommandsGeneral::set_command_2(Command2Selection::BK1));
    cmd2 = Command2Selection::BK1;

    display.write_command(BK1Command2::set_vop_amplitude(&cmd2, 0x5d));
    display.write_command(BK1Command2::set_vcom_amplitude(&cmd2, 0x43));
    display.write_command(BK1Command2::set_vgh_voltage(&cmd2, 0x81));
    display.write_command(BK1Command2::test_command_setting(&cmd2));
    display.write_command(BK1Command2::set_vgl_voltage(&cmd2, 0x43));
    display.write_command(BK1Command2::power_control_one(
        &cmd2,
        GammaOPBias::Middle,
        SourceOPInput::Min,
        SourceOPOutput::Min,
    ));

    display.write_command(BK1Command2::power_control_two(
        &cmd2,
        VoltageAVDD::Pos6_6,
        VoltageAVCL::Neg4_4,
    ));
    display.write_command(BK1Command2::set_pre_drive_timing_one(&cmd2, 0x08));
    display.write_command(BK1Command2::set_pre_drive_timing_two(&cmd2, 0x08));
    display.write_command(Ok(Command {
        address: 0xD0,
        parameters: vec![0x88],
    }));
    display.write_command(Ok(Command {
        address: 0xE0,
        parameters: vec![0x00, 0x00, 0x02],
    }));
    display.write_command(Ok(Command {
        address: 0xE1,
        parameters: vec![
            0x03, 0xA0, 0x00, 0x00, 0x04, 0xA0, 0x00, 0x00, 0x00, 0x20, 0x20,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xE2,
        parameters: vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xE3,
        parameters: vec![0x00, 0x00, 0x11, 0x00],
    }));
    display.write_command(Ok(Command {
        address: 0xE4,
        parameters: vec![0x22, 0x00],
    }));
    display.write_command(Ok(Command {
        address: 0xE5,
        parameters: vec![
            0x05, 0xEC, 0xA0, 0xA0, 0x07, 0xEE, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xE6,
        parameters: vec![0x00, 0x00, 0x11, 0x00],
    }));
    display.write_command(Ok(Command {
        address: 0xE7,
        parameters: vec![0x22, 0x00],
    }));
    display.write_command(Ok(Command {
        address: 0xE8,
        parameters: vec![
            0x06, 0xED, 0xA0, 0xA0, 0x08, 0xEF, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xEB,
        parameters: vec![0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x00],
    }));
    display.write_command(Ok(Command {
        address: 0xED,
        parameters: vec![
            0xFF, 0xFF, 0xFF, 0xBA, 0x0A, 0xBF, 0x45, 0xFF, 0xFF, 0x54, 0xFB, 0xA0, 0xAB, 0xFF,
            0xFF, 0xFF,
        ],
    }));
    display.write_command(Ok(Command {
        address: 0xEF,
        parameters: vec![0x10, 0x0D, 0x04, 0x08, 0x3F, 0x1F],
    }));

    // Set Command2 for BK3
    display.write_command(CommandsGeneral::set_command_2(Command2Selection::BK1));
    cmd2 = Command2Selection::BK1;
    display.write_command(Ok(Command {
        address: 0xEF,
        parameters: vec![0x08],
    }));

    // COMMAND2 DISABLE
    display.write_command(CommandsGeneral::set_command_2(Command2Selection::Disabled));
    cmd2 = Command2Selection::Disabled;

    display.write_command(CommandsGeneral::sleep_mode_off());
    thread::sleep(time::Duration::from_millis(120));

    display.write_command(CommandsGeneral::display_on());
    display.write_command(CommandsGeneral::display_data_control(
        ScanDirection::Normal,
        ColorOrder::Rgb,
    ));
    display.write_command(CommandsGeneral::set_color_mode(BitsPerPixel::Rgb666));
}
