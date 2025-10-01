use crate::st7701s_spi::parameters::general::Direction;

pub type ScanDirection = Direction;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ColorOrder {
    #[default]
    RGB = 0x00,
    BGR = 0x01,
}
