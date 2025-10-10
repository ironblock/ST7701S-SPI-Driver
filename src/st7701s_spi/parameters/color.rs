use crate::{bit_value_enum};

bit_value_enum! {
    pub enum BitsPerPixel<2> {
        #[default]
        const RGB565 = 0b00,
        const RGB666 = 0b01,
        const RGB888 = 0b10,
    }
}

bit_value_enum! {
    pub enum EnhanceLevel<2> {
        #[default]
        const Low    = 0b00,
        const Medium = 0b01,
        const High   = 0b10,
    }
}

bit_value_enum! {
    pub enum AdaptiveBrightness<2> {
        #[default]
        const Off           = 0b00,
        const UserInterface = 0b01,
        const StillPicture  = 0b10,
        const MovingImage   = 0b11,
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