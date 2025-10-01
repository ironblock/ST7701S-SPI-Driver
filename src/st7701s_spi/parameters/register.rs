use std::ops::Deref;

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
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Bank {
    #[default]
    BK0 = 0,
    BK1 = 1,
    BK3 = 3,
}
