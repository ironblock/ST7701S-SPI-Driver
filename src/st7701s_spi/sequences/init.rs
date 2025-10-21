use std::io;

use crate::st7701s_spi::{
    device::ST7701S,
    parameters::{
        general::Switch,
        register::{Bank, CommandExtension},
    }, protocol::connection::Connection,
};
use Switch::*;

pub fn init_sequence<C: Connection, E>(display: &mut ST7701S<C, E>) -> io::Result<()> {
    display.select_command_extension(
        CommandExtension::new()
            .set_extended_commands(On)
            .set_bank(Bank::BK0),
    )?;

    Ok(())

    // SPI_WriteComm(0xC0); // LNESET
    // device.line_setting(settings);
    // SPI_WriteData(0x3B); // LDE_EN
    // SPI_WriteData(0x00); // Line Delta

    // SPI_WriteComm(0xC1); // PORCTRL
    // SPI_WriteData(0x0B); // VBP
    // SPI_WriteData(0x02); // VFP

    // SPI_WriteComm(0xC2); // INVSEL
    // SPI_WriteData(0x00); // This should have fixed bits but doesn't??
    // SPI_WriteData(0x02); // RTNI

    // SPI_WriteComm(0xCC); // ?????????
    // SPI_WriteData(0x10);

    // SPI_WriteComm(0xCD); // COLCTRL
    // SPI_WriteData(0x08); // 00001000 MDT 1, pixel collect to DB[17:0]

    // SPI_WriteComm ( 0xB0); // Positive Voltage Gamma Control
    // SPI_WriteData ( 0x02); //
    // SPI_WriteData ( 0x13);
    // SPI_WriteData ( 0x1B);
    // SPI_WriteData ( 0x0D);
    // SPI_WriteData ( 0x10);
    // SPI_WriteData ( 0x05);
    // SPI_WriteData ( 0x08);
    // SPI_WriteData ( 0x07);
    // SPI_WriteData ( 0x07);
    // SPI_WriteData ( 0x24);
    // SPI_WriteData ( 0x04);
    // SPI_WriteData ( 0x11);
    // SPI_WriteData ( 0x0E);
    // SPI_WriteData ( 0x2C);
    // SPI_WriteData ( 0x33);
    // SPI_WriteData ( 0x1D);

    // SPI_WriteComm ( 0xB1); //Negative Voltage Gamma Control
    // SPI_WriteData ( 0x05);
    // SPI_WriteData ( 0x13);
    // SPI_WriteData ( 0x1B);
    // SPI_WriteData ( 0x0D);
    // SPI_WriteData ( 0x11);
    // SPI_WriteData ( 0x05);
    // SPI_WriteData ( 0x08);
    // SPI_WriteData ( 0x07);
    // SPI_WriteData ( 0x07);
    // SPI_WriteData ( 0x24);
    // SPI_WriteData ( 0x04);
    // SPI_WriteData ( 0x11);
    // SPI_WriteData ( 0x0E);
    // SPI_WriteData ( 0x2C);
    // SPI_WriteData ( 0x33);
    // SPI_WriteData ( 0x1D);

    // SPI_WriteComm(0xFF);
    // SPI_WriteData(0x77);
    // SPI_WriteData(0x01);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x00);
    // SPI_WriteData(0x11); // BK1 command2

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
}
