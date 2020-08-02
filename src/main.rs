extern crate enum_primitive;
extern crate num;
extern crate spidev;

mod instructions;
mod panel;
mod sequence_linux_kernel;
mod sequence_tdo;
mod spi;

use panel::TDOMode;
use sequence_tdo::init;
use spi::ST7701S;
use std::env;

/// ST7701S supports two kinds of RGB interface, DE mode (mode 1) and HV mode
/// (mode 2), and 16bit/18bit and 24 bit data format. When DE mode is selected
/// and the VSYNC, HSYNC, DOTCLK, DE, D23:0 pins can be used; when HV mode is
/// selected and the VSYNC, HSYNC, DOTCLK, D23:0 pins can be used. When using
/// RGB interface, only serial interface can be selected.
fn main() {
    println!("Initializing SPI driver for ST7701S panel");

    let args: Vec<String> = env::args().collect();
    let spi_argument: Option<String> = Some(String::from(&args[0]));
    let mut spi_path: String = String::from("/dev/spidev0.0");

    if !spi_argument.is_some() {
        spi_path = spi_argument.unwrap();
    }

    println!("Using SPI device path {}", spi_path);

    let mut display = ST7701S::new(spi_path);
    let mode = TDOMode;

    init(&mut display, mode);
}
