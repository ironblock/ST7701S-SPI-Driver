extern crate enum_primitive;
extern crate num;
extern crate spidev;

pub mod instructions;
pub mod panel;
pub mod sequence_linux_kernel;
pub mod sequence_tdo;
pub mod spi;

use panel::TDO_MODE;
use sequence_tdo::init;
use spi::ST7701S;
use std::env;

/// ST7701S supports two kinds of RGB interface, DE mode (mode 1) and HV mode
/// (mode 2), and 16bit/18bit and 24 bit data format. When DE mode is selected
/// and the VSYNC, HSYNC, DOTCLK, DE, D23:0 pins can be used; when HV mode is
/// selected and the VSYNC, HSYNC, DOTCLK, D23:0 pins can be used. When using
/// RGB interface, only serial interface can be selected.
fn main() {
    let tag: String = String::from("[ST7701S]");
    println!("{} Initializing SPI driver", tag);

    let args: Vec<String> = env::args().collect();
    let mut spi_path: String = String::from("/dev/spi/0.0");
    let mut spi_argument: Option<String> = None;

    if args.len() > 1 {
        spi_argument = Some(String::from(&args[1]));
    }

    if spi_argument.is_some() {
        spi_path = spi_argument.unwrap();
        println!("{} Using custom SPI device path \"{}\"", tag, spi_path);
    } else {
        println!("{} Using default SPI device path {}", tag, spi_path);
    }

    let mut display = ST7701S::new(spi_path);
    let mode = TDO_MODE;

    init(&mut display, mode);
}
