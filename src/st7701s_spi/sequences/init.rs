use std::io;

use crate::st7701s_spi::{
    address::{Extension, ExtensionBk0, ExtensionBk1},
    device::ST7701S,
    parameters::{
        display::{
            InversionSelection, LineSettings, PolarityInversion, PorchControl, VoltageBias,
            VoltageControl,
        },
        general::Switch,
    },
    protocol::connection::Connection,
};
use Switch::*;

pub fn init_sequence<C: Connection, E: Extension>(
    display: ST7701S<C, E>,
) -> Result<ST7701S<C, impl Extension>, io::Error> {
    let mut display = display.select_command_extension(ExtensionBk0);

    display.line_setting(
        &LineSettings::new()
            .set_extra_line(On)
            .set_line_const::<27>(),
    )?;

    display.porch_control(
        &PorchControl::new()
            .set_vertical_back_porch_const::<11>()
            .set_vertical_front_porch_const::<2>(),
    )?;

    display.inversion_select(
        &InversionSelection::new()
            .set_polarity_inversion(PolarityInversion::OneDot)
            .set_rtni_const::<2>(),
    )?;

    // SPI_WriteComm(0xCC); // ?????????
    // SPI_WriteData(0x10);

    // SPI_WriteComm(0xCD); // COLCTRL
    // SPI_WriteData(0x08); // 00001000 MDT 1, pixel collect to DB[17:0]

    let gamma_voltage = VoltageControl::new().set_aj0(VoltageBias::A)
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

    display.positive_gamma_control(&gamma_voltage)?;
    display.negative_gamma_control(&gamma_voltage)?;

    let display = display.select_command_extension(ExtensionBk1);

    // SPI_WriteComm(0xB0); VOP amplitude
    // SPI_WriteData(0x5d);//5d

    // SPI_WriteComm(0xB1); 	//VCOM amplitude setting
    // SPI_WriteData(0x43); //43

    // SPI_WriteComm(0xB2); 	//VGH Voltage setting
    // SPI_WriteData(0x81);	//12V

    // SPI_WriteComm(0xB3); // Test command
    // SPI_WriteData(0x80); // required static

    // SPI_WriteComm(0xB5); 	//VGL Voltage setting
    // SPI_WriteData(0x43);	//-8.3V

    // SPI_WriteComm(0xB7); // power control 1
    // SPI_WriteData(0x85);

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
