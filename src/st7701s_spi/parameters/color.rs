#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum BitsPerPixel {
    #[default]
    RGB565 = 0x00,
    RGB666 = 0x01,
    RGB888 = 0x02,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum EnhanceLevel {
    #[default]
    Low = 0x00,
    Medium = 0x01,
    High = 0x02,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AdaptiveBrightness {
    #[default]
    Off = 0x00,
    UserInterface = 0x01,
    StillPicture = 0x02,
    MovingImage = 0x03,
}
