use crate::bit_value_enum;


bit_value_enum! {
    /// Bits Per Pixel Format
    pub enum BitsPerPixel<2> {
        #[default]
        const RGB565 = 0b00,
        const RGB666 = 0b01,
        const RGB888 = 0b10,
    }
}


bit_value_enum! {
    /// Color Enhancement Level
    pub enum EnhanceLevel<2> {
        #[default]
        const Low    = 0b00,
        const Medium = 0b01,
        const High   = 0b10,
    }
}


/// Individual Color Channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorChannel {
    Red,
    Green,
    Blue,
}

/// Pixel Extrema States
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelExtrema {
    Black,
    White,
}
