use crate::{enum_argument, st7701s_spi::{transmissions::*}};

enum_argument! {
    pub enum BitsPerPixel[1:0] {
        #[default]
        RGB565 = 0b00,
        RGB666 = 0b01,
        RGB888 = 0b10,
    }
}

enum_argument! {
    pub enum EnhanceLevel[1:0] {
        #[default]
        Low    = 0b00,
        Medium = 0b01,
        High   = 0b10,
    }
}

enum_argument! {
    pub enum AdaptiveBrightness[1:0] {
        #[default]
        Off           = 0b00,
        UserInterface = 0b01,
        StillPicture  = 0b10,
        MovingImage   = 0b11,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorChannel {
    Red,
    Green,
    Blue,
}

pub enum PixelExtrema {
    Black,
    White,
}