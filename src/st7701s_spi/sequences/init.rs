use std::{any::Any, io};

use crate::st7701s_spi::{
    device::ST7701S,
    parameters::bk0_display::{ColorControl, MDT},
    protocol::connection::{Bank0, Connection, Extension},
};

pub fn init_sequence<X: Connection, E>(
    display: ST7701S<X, E>,
) -> io::Result<ST7701S<X, impl Extension>> {
    let mut display = display.select_command_extension(Bank0);

    // display.line_setting(&LineSettings::new().set_extra_line(On).set_line(27))?;

    // display.porch_control(
    //     &PorchControl::new()
    //         .set_vertical_back_porch(11)
    //         .set_vertical_front_porch(2),
    // )?;

    // display.inversion_select(
    //     &InversionSelection::new()
    //         .set_polarity_inversion(PolarityInversion::OneDot)
    //         .set_rtni(2),
    // )?;

    // SPI_WriteComm(0xCC); // ?????????
    // SPI_WriteData(0x10);

    display.color_control(ColorControl::new().set_mdt(MDT::CollectToDB))?;

    // let gamma_voltage = VoltageControl::new()
    //     .set_aj0(VoltageBias::A)
    //     .set_vc0(0x02)
    //     .set_aj1(VoltageBias::A)
    //     .set_vc4(0x13)
    //     .set_aj2(VoltageBias::A)
    //     .set_vc8(0x1B)
    //     .set_vc16(0x0D)
    //     .set_aj3(VoltageBias::A)
    //     .set_vc24(0x10)
    //     .set_vc52(0x05)
    //     .set_vc80(0x08)
    //     .set_vc108(0x07)
    //     .set_vc147(0x07)
    //     .set_vc175(0x24)
    //     .set_vc203(0x04)
    //     .set_aj4(VoltageBias::A)
    //     .set_vc231(0x11)
    //     .set_vc239(0x0E)
    //     .set_aj5(VoltageBias::A)
    //     .set_vc247(0x2C)
    //     .set_aj6(VoltageBias::A)
    //     .set_vc251(0x33)
    //     .set_aj7(VoltageBias::A)
    //     .set_vc255(0x1D);

    // display.positive_gamma_control(&gamma_voltage)?;
    // display.negative_gamma_control(&gamma_voltage)?;

    // let mut display = display.select_command_extension(Bk1);

    // display.set_operating_voltage(&OperatingVoltage::new().set_amplitude(0x5D))?;

    // display.set_common_voltage(&CommonVoltage::new().set_amplitude(0x43))?;

    // display.set_gate_high_voltage(&GateHighVoltage::new().set_amplitude_volts(12.0))?;

    // display.test_command()?;

    // display.set_gate_low_voltage(&GateLowVoltage::new().set_amplitude_volts(-8.14))?;

    // SPI_WriteComm(0xB7); // power control 1
    // SPI_WriteData(0x85);
    // display.power_control_1(settings)

    // SPI_WriteComm(0xB8);// power control 2
    // SPI_WriteData(0x20);

    // SPI_WriteComm(0xC1); // pre drive timing set 1
    // SPI_WriteData(0x78);

    // SPI_WriteComm(0xC2); source eq2 setting
    // SPI_WriteData(0x78);

    // SPI_WriteComm(0xD0); // mipi setting 1? what?
    // SPI_WriteData(0x88); // ???

    // SPI_WriteComm(0xE0); // SSCTRL - spread spectrum
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x02);

    // SPI_WriteComm(0xE1); // Noise Reduction Control
    // SPI_WriteData(0x03);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x04);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x20);
    // SPI_WriteData(0x20);

    // SPI_WriteComm(0xE2); // Sharpness
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xE3); // Color calibration
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x11);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xE4); // Skin tone preservation
    // SPI_WriteData(0x22);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xE5); // ????????
    // SPI_WriteData(0x05);
    // SPI_WriteData(0xEC);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0x07);
    // SPI_WriteData(0xEE);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xE6); // ??????????
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x11);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xE7); // ???????
    // SPI_WriteData(0x22);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xE8); // ?????????
    // SPI_WriteData(0x06);
    // SPI_WriteData(0xED);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0x08);
    // SPI_WriteData(0xEF);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xEB); // ????????
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x40);
    // SPI_WriteData(0x40);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);

    // SPI_WriteComm(0xED); // ???????
    // SPI_WriteData(0xFF);
    // SPI_WriteData(0xFF);
    // SPI_WriteData(0xFF);
    // SPI_WriteData(0xBA);
    // SPI_WriteData(0x0A);
    // SPI_WriteData(0xBF);
    // SPI_WriteData(0x45);
    // SPI_WriteData(0xFF);
    // SPI_WriteData(0xFF);
    // SPI_WriteData(0x54);
    // SPI_WriteData(0xFB);
    // SPI_WriteData(0xA0);
    // SPI_WriteData(0xAB);
    // SPI_WriteData(0xFF);
    // SPI_WriteData(0xFF);
    // SPI_WriteData(0xFF);

    // SPI_WriteComm(0xEF); // ???????
    // SPI_WriteData(0x10);
    // SPI_WriteData(0x0D);
    // SPI_WriteData(0x04);
    // SPI_WriteData(0x08);
    // SPI_WriteData(0x3F);
    // SPI_WriteData(0x1F);

    // SPI_WriteComm(0xFF);
    // SPI_WriteData(0x77);
    // SPI_WriteData(0x01);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x13); // BK3 command2!!! Not being used in my code yet

    // SPI_WriteComm(0xEF); // ????????
    // SPI_WriteData(0x08);

    // SPI_WriteComm(0xFF);
    // SPI_WriteData(0x77);
    // SPI_WriteData(0x01);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00); // Disable command2

    // #if 0
    // WriteComm (0xFF);
    // WriteData (0x77);
    // WriteData (0x01);
    // WriteData (0x00);
    // WriteData (0x00);
    // WriteData (0x12); // ????

    // WriteComm (0xD1);
    // WriteData (0x81);
    // WriteData (0x08);
    // WriteData (0x03);
    // WriteData (0x20);
    // WriteData (0x08);
    // WriteData (0x01);
    // WriteData (0xA0);
    // WriteData (0x01);
    // WriteData (0xE0);
    // WriteData (0xA0);
    // WriteData (0x01);
    // WriteData (0xE0);
    // WriteData (0x03);
    // WriteData (0x20);
    // WriteComm (0xD2);
    // WriteData (0x08);
    // #endif
    // /////////////////Bring up the internal test picture///////////////////////////////////

    // SPI_WriteComm(0x11); // SLEEP OUT

    // Delay(120);

    // SPI_WriteComm(0x29); // DISPLAY ON

    // SPI_WriteComm(0x36); // MADCTL
    // SPI_WriteData(0x00); // normal scan, rgb

    // SPI_WriteComm(0x3A); // pixel format
    // SPI_WriteData(0x60);//0x60 18bit   0x50 16bit
    // #endif

    Ok(display)
}
