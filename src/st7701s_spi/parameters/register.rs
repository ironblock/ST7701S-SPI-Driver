use std::ops::Deref;

use crate::{bit_value_enum,
    st7701s_spi::{parameters::general::Switch, transmissions::*},
    transmission_mapping};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address {
    register: u8,
    bank: Option<Bank>,
}
impl Address {
    pub const fn new(register: u8, bank: Option<Bank>) -> Self {
        Self { register, bank }
    }

    pub const fn register(&self) -> u8 {
        self.register
    }

    pub const fn bank(&self) -> &Option<Bank> {
        &self.bank
    }
}
impl Deref for Address {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.register
    }
}

/**
    ## Extended Address Banks

    The ST7701S exposes some extended command sets based on the setting of an
    internal register, referred to in the datasheet as **Command2 BKx**.

    > Section 12.3.1 `CND2BKxSEL`, page 260
*/
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
        1: (D0(packet_1<8> = BitValue::new::<0b0111_0111>()),),
        2: (D0(packet_2<8> = BitValue::new::<0b0000_0001>()),),
        3: (D0(packet_3<8> = BitValue::new::<0b0000_0000>()),),
        4: (D0(packet_4<8> = BitValue::new::<0b0000_0000>()),),
        5: (D4(enable_extension<1> as Switch), D0(bank<2> as Bank),),
    )
}