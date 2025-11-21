use crate::{
    bit_value_enum,
    st7701s_spi::{
        parameters::general::{Direction, Edge, Logic, Switch},
        transmissions::{BitValue, BitMask, BitOffset, D7, D6, D0, D4, D3, D2, D1, D5},
    },
    transmission_mapping,
};
use Edge::Rising;
use Logic::{High, Low};
use Switch::Off;


bit_value_enum! {
    /// RGB Interface Mode Selection
    pub enum RgbInterfaceMode<3> {
        #[default]
        const Mode18Bit = 0b101,
        const Mode16Bit = 0b110,
        const Mode6Bit  = 0b111,
    }
}

transmission_mapping! {
    /// ## RGB Interface Control
    /// Configures RGB interface timing and polarity settings
    /// > Reference: `RGBCTRL` p. 272
    pub struct RgbControl<4>(
        0: (
            D7(hsync_back_porch_enable<1> as Switch = Off),
            D6(hsync_front_porch_enable<1> as Switch = Off),
            D0(rgb_interface_mode<3> as RgbInterfaceMode),
        ),
        1: (
            D4(de_polarity<1> as Logic = High),
            D3(hsync_polarity<1> as Logic = Low),
            D2(vsync_polarity<1> as Logic = Low),
            D1(dotclk_polarity<1> as Edge = Rising),
        ),
        2: (D0(hsync_back_porch<7> = 0x0A),),
        3: (D0(hsync_front_porch<7> = 0x0A),),
    );
}


transmission_mapping! {
    /// ## Partial Area Control
    /// Configures partial display update region
    /// > Reference: `PARCTRL` p. 273
    pub struct PartialControl<2>(
        0: (D0(partial_start_row_high<8>),),
        1: (D0(partial_end_row_high<8>),),
    );
}


transmission_mapping! {
    /// ## X-Direction Scan Control
    /// Controls horizontal scan direction for display mirroring
    /// > Reference: `SDIR` p. 274
    pub struct ScanDirectionControl<1>(
        0: (D0(x_direction<1> as Direction),),
    );
}


bit_value_enum! {
    /// Pseudo-dot inversion mode
    pub enum PseudoDotMode<2> {
        #[default]
        const Column = 0b00,
        const TwoDot = 0b01,
        const OneDot = 0b10,
    }
}

transmission_mapping! {
    /// ## Pseudo-Dot Inversion Setting
    /// Configures pseudo-dot inversion for display uniformity
    /// > Reference: `PDOSET` p. 275
    pub struct PseudoDotInversion<1>(
        0: (D0(pseudo_dot_mode<2> as PseudoDotMode),),
    );
}


bit_value_enum! {
    pub enum MDT<1> {
        #[default]
        const Normal      = 0,
        const CollectToDB = 1,
    }
}
bit_value_enum! {
    /// Pixel Format
    pub enum EndPixelFormat<3> {
        #[default]
        const CopySelfMSB  = 0x00,
        const CopyGreenMSB = 0x01,
        const CopySelfLSB  = 0x02,
        const FixZero      = 0x04,
        const FixOne       = 0x05,
    }
}

transmission_mapping! {
    /// ## Color Control Settings
    /// Configures PWM/LED polarity and pixel format
    /// > Reference: `COLCTRL` p. 276
    pub struct ColorControl<1>(
        0: (
            D5(pwm_polarity<1> as Direction),
            D4(led_polarity<1> as Direction),
            D3(mdt<1> as MDT),
            D0(end_pixel_format<3> as EndPixelFormat),
        ),
    );
}


transmission_mapping! {
    /// ## Sunlight Readable Enhancement Control
    /// Improves display visibility in bright ambient light
    /// > Reference: `SRECTRL` p. 278
    pub struct SunlightEnhancement<3>(
        0: (
            D7(sre_enable<1> as Switch = Off),
            D0(sre_level<5> = 0x00),
        ),
        1: (D0(white_level<8> = 0xFF),),
        2: (D0(black_level<8> = 0x00),),
    );
}


transmission_mapping! {
    /// ## Noise Reduction Control
    /// Configures noise reduction algorithms for image quality
    /// > Reference: `NRCTRL` p. 279
    pub struct NoiseReduction<11>(
        0:  (
            D7(nr_enable<1> as Switch = Off),
            D0(nr_strength<4> = 0x0),
        ),
        1:  (D0(nr_y_strength<8> = 0x00),),
        2:  (D0(nr_cb_strength<8> = 0x00),),
        3:  (D0(nr_cr_strength<8> = 0x00),),
        4:  (D0(nr_edge_threshold<8> = 0x00),),
        5:  (D0(nr_edge_gain<8> = 0x00),),
        6:  (D0(nr_texture_threshold<8> = 0x00),),
        7:  (D0(nr_texture_gain<8> = 0x00),),
        8:  (D0(nr_color_threshold<8> = 0x00),),
        9:  (D0(nr_color_gain<8> = 0x00),),
        10: (D0(nr_reserved<8> = 0x00),),
    );
}


transmission_mapping! {
    /// ## Sharpness and Edge Enhancement Control
    /// Adjusts edge detection and sharpening algorithms
    /// > Reference: `SECTRL` p. 280
    pub struct SharpnessControl<13>(
        0:  (
            D7(sharpness_enable<1> as Switch = Off),
            D0(sharpness_strength<4> = 0x0),
        ),
        1:  (D0(edge_threshold_1<8> = 0x00),),
        2:  (D0(edge_threshold_2<8> = 0x00),),
        3:  (D0(edge_threshold_3<8> = 0x00),),
        4:  (D0(edge_gain_1<8> = 0x00),),
        5:  (D0(edge_gain_2<8> = 0x00),),
        6:  (D0(edge_gain_3<8> = 0x00),),
        7:  (D0(edge_gain_4<8> = 0x00),),
        8:  (D0(detail_threshold<8> = 0x00),),
        9:  (D0(detail_gain<8> = 0x00),),
        10: (D0(overshoot_limit<8> = 0x00),),
        11: (D0(undershoot_limit<8> = 0x00),),
        12: (D0(sharpness_reserved<8> = 0x00),),
    );
}


transmission_mapping! {
    /// ## Color Calibration Control
    /// Fine-tunes color reproduction and white balance
    /// > Reference: `CCCTRL` p. 281
    pub struct ColorCalibration<4>(
        0: (D0(red_gain<8> = 0x80),),
        1: (D0(green_gain<8> = 0x80),),
        2: (D0(blue_gain<8> = 0x80),),
        3: (
            D6(color_temp_mode<2> = 0b00),
            D0(color_saturation<4> = 0x8),
        ),
    );
}


transmission_mapping! {
    /// ## Skin Tone Control
    /// Preserves natural skin tones in images
    /// > Reference: `SKCTRL` p. 282
    pub struct SkinToneControl<2>(
        0: (
            D7(skin_tone_enable<1> as Switch = Off),
            D0(skin_tone_range<4> = 0x0),
        ),
        1: (D0(skin_tone_gain<8> = 0x00),),
    );
}


/// Digital Gamma Look-up Table for Red channel (64 bytes)
/// > Reference: `DGMLUTR` p. 266
pub type GammaLutRed = [u8; 64];

/// Digital Gamma Look-up Table for Blue channel (64 bytes)
/// > Reference: `DGMLUTB` p. 267
pub type GammaLutBlue = [u8; 64];
