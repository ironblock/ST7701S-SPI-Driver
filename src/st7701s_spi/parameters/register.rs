use crate::{
    bit_value_enum,
    st7701s_spi::{parameters::general::Switch, transmissions::{BitValue, BitMask, BitOffset, D4, D0}},
    transmission_mapping,
};

bit_value_enum! {
    pub enum Bank<2> {
        #[default]
        const BK0 = 0,
        const BK1 = 1,
        const BK3 = 3,
    }
}

transmission_mapping! {
    pub struct CommandExtension<5>(
        0: () = 0b0111_0111,
        1: () = 0b0000_0001,
        2: () = 0b0000_0000,
        3: () = 0b0000_0000,
        4: (D4(extended_commands<1> as Switch), D0(bank<2> as Bank),),
    );
}
