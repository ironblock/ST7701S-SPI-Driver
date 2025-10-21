#![warn(missing_docs, missing_debug_implementations, rust_2024_compatibility)]

//! # ST7701S SPI Driver
//!
//! This software is provides a basic Linux driver able to initialize and
//! control displays with a Sitronix ST7701S controller over SPI.
//!
//! ## Hardware Compatibility
//! The primary development hardware used for testing is an original
//! BeagleBone Black (Texas Instruments AM335X) connected to a Shanghai Top
//! Display Optoelectronics Co. TL021WVC02-B1323B LCD module. The module is
//! controlled via SPI and driven by 18 RGB signal lines (sometimes called a
//! "3SPI+18RGB" interface).
//!
//! Given that this is a 480x480 display connected to a slow, single-core ARM
//! device, there may exist race conditions or other timing issues that won't be
//! exposed on that hardware.

use linux_embedded_hal::SpidevDevice;
use log::info;
use st7701s::st7701s_spi::{device::ST7701S, panel::TDOMODE, protocol::spi::*, sequences::init::{self, init_sequence}};

fn main() {
    info!("Initializing SPI driver for ST7701S panel");
    let mut device = SpidevDevice::open("/dev/spidev1.0").expect("Failed to open SPI device");
    device.configure(&ThreeWireSPI::DEFAULT_OPTIONS).expect("Failed to configure SPI device");

    let mut display= ST7701S::new(ThreeWireSPI { device });

    init_sequence(&mut display);
}
