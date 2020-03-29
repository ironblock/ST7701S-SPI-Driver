extern crate enum_primitive;
extern crate num;
extern crate spidev;
use std::{thread, time};

mod instructions;
mod panel;
mod spi;

use instructions::{BK0Command2, BK1Command2, Command, Command2Selection, CommandsGeneral};
use panel::CVTRB;
use spi::ST7701S;

/// ST7701S supports two kinds of RGB interface, DE mode (mode 1) and HV mode
/// (mode 2), and 16bit/18bit and 24 bit data format. When DE mode is selected
/// and the VSYNC, HSYNC, DOTCLK, DE, D23:0 pins can be used; when HV mode is
/// selected and the VSYNC, HSYNC, DOTCLK, D23:0 pins can be used. When using
/// RGB interface, only serial interface can be selected.
fn main() {
    println!("Initializing SPI driver for ST7701S panel");

    let mut CMD2: Command2Selection = Command2Selection::Disabled;
    let mut display = ST7701S::new(String::from("/dev/spidev1.0"));
    let mode = CVTRB;

    // SOFTWARE RESET
    // 5ms delay
    display.write_command(CommandsGeneral::software_reset());
    thread::sleep(time::Duration::from_millis(5));

    // EXIT SLEEP MODE
    // Variable delay (200ms is "safe")
    display.write_command(CommandsGeneral::sleep_mode_off());
    thread::sleep(time::Duration::from_millis(200));

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
        instructions::Inversion::Column,
        0xFF,
    ));
    display.write_command(BK0Command2::rgb_control(
        &CMD2,
        instructions::DataEnable::DE,
        instructions::VsyncActive::Low,
        instructions::HsyncActive::Low,
        instructions::DataPolarity::Rising,
        instructions::EnablePolarity::Low,
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
        instructions::GammaOPBias::Middle,
        instructions::SourceOPInput::Min,
        instructions::SourceOPOutput::Off,
    ));

    display.write_command(BK1Command2::power_control_two(
        &CMD2,
        instructions::VoltageAVDD::Pos6_6,
        instructions::VoltageAVCL::Neg4_4,
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

    // display.write_command(CommandsGeneral::set_color_mode(
    //     instructions::BitsPerPixel::Rgb666,
    // ));
    // display.write_command(CommandsGeneral::display_data_control(
    //     instructions::ScanDirection::Normal,
    //     instructions::ColorOrder::Rgb,
    // ));
    display.write_command(CommandsGeneral::tearing_effect_on(
        instructions::TearingEffect::VHBlank,
    ));

    display.write_command(CommandsGeneral::display_on());
    thread::sleep(time::Duration::from_millis(200));
}
