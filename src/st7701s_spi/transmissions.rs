pub const fn merge_bit_masks(masks: &[usize]) -> usize {
    let mut mask = 0;
    let mut i = 0;

    while i < masks.len() {
        assert!((mask & masks[i]) == 0, "Overlapping bit masks");

        mask |= masks[i];
        i += 1;
    }

    mask
}

pub trait BitSize<const HI: u8, const LO: u8 = 0> {
    const VALID: () = const {
        assert!(HI < 8);
        assert!(LO < 8);
        assert!(HI >= LO);
    };
    const SIZE: u8 = HI - LO + 1;
    const MASK: u8 = (1 << Self::SIZE) - 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitValue<const HI: u8, const LO: u8 = 0>(u8)
where
    Self: BitSize<HI, LO> + Into<u8>;

impl<const HI: u8, const LO: u8> BitValue<HI, LO> {
    pub const fn new(value: u8) -> Self {
        if cfg!(debug_assertions) {
            assert!((value & Self::MASK) == value, "value exceeds bit range");
        }

        Self(value)
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}
impl<const HI: u8, const LO: u8> BitSize<HI, LO> for BitValue<HI, LO> {}
impl<const HI: u8, const LO: u8> From<BitValue<HI, LO>> for u8 {
    fn from(value: BitValue<HI, LO>) -> Self {
        value.0
    }
}
impl<const HI: u8, const LO: u8> TryFrom<u8> for BitValue<HI, LO> {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (value & Self::MASK) == value {
            Ok(Self::new(value))
        } else {
            Err("value exceeds bit range")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitField<const HI: u8, const LO: u8 = 0> {
    D7,
    D6,
    D5,
    D4,
    D3,
    D2,
    D1,
    D0,
}
impl<const HI: u8, const LO: u8> BitSize<HI, LO> for BitField<HI, LO> {}
impl<const HI: u8, const LO: u8> BitField<HI, LO> {
    pub const fn offset(&self) -> u8 {
        match self {
            Self::D7 => 7,
            Self::D6 => 6,
            Self::D5 => 5,
            Self::D4 => 4,
            Self::D3 => 3,
            Self::D2 => 2,
            Self::D1 => 1,
            Self::D0 => 0,
        }
    }

    pub const fn shifted_mask(&self) -> u8 {
        Self::MASK << self.offset()
    }

    pub const fn assert_value_in_range(&self, value: u8) {
        assert!((value & Self::MASK) == value, "value exceeds bit range");
    }

    pub const fn shift_value(&self, value: &BitValue<HI, LO>) -> u8 {
        if cfg!(debug_assertions) {
            self.assert_value_in_range(value.as_u8());
        }

        value.as_u8() << self.offset()
    }

    pub const fn set_bits(&self, packet: &mut u8, value: &BitValue<HI, LO>) {
        *packet |= self.shift_value(value);
    }

    pub const fn get_bits(&self, packet: &u8) -> u8 {
        (*packet >> self.offset()) & Self::MASK
    }
}

#[macro_export]
macro_rules! transmission_mapping {
    (
        $(#[$META:meta])*
        $SV:vis struct $NAME:ident<$LENGTH:literal> (
            $($INDEX:literal: (
                $($D:ident( $ARG:ident[$HI:literal:$LO:literal] as $ALIAS:ty $(= $VAL:expr)?),)+
            ),)*
        )
    ) => {
        paste! {

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            $SV struct [<$NAME:camel>]([u8; $LENGTH]);

            impl [<$NAME:camel>] {
                const INITIAL: [u8; $LENGTH] = const {
                    #[allow(unused_mut)]
                    let mut transmission = [0; $LENGTH];

                    $($($(transmission[$INDEX] |= Self::[<$ARG:upper _RANGE>].shift_value($VAL);)*)*)*

                    transmission
                };

                $(
                    const [<PACKET_ $INDEX _MASK>]: usize = merge_bit_masks(&[$(Self::[<$ARG:upper _RANGE>].shifted_mask() as usize),+]);
                    $(
                        const [<$ARG:upper _RANGE>]: BitField<$HI, $LO> = BitField::$D;
                        const [<$ARG:upper _INDEX>]: usize = $INDEX;
                        pub fn [<$ARG:lower>](&self) -> $ALIAS {
                            $ALIAS::try_from(
                                Self::[<$ARG:upper _RANGE>]
                                    .get_bits(&self.0[Self::[<$ARG:upper _INDEX>]]),
                            ).unwrap()
                        }

                        pub fn [<set_ $ARG:lower>](&mut self, value: $ALIAS)  {
                            let bit_value = BitValue::from(value);

                            Self::[<$ARG:upper _RANGE>]
                                .set_bits(&mut self.0[Self::[<$ARG:upper _INDEX>]], &bit_value);
                        }
                    )+
                )+

                pub const fn new() -> Self {
                    Self::from_packets(Self::INITIAL)
                }

                pub const fn from_packets(packets: [u8; $LENGTH]) -> Self {
                    Self(packets)
                }

                pub const fn as_packets(&self) -> &[u8; $LENGTH] {
                    &self.0
                }
            }
        }
    };
}
