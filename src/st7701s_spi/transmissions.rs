    // TODO IMPLEMENT
    pub const fn merge_bit_masks<const N: usize>(masks: [usize; N]) -> usize {
        let mut mask = 0;
        let mut i = 0;

        while i < N {
            assert!((mask & masks[i]) == 0, "Overlapping bit masks");

            mask |= masks[i];
            i += 1;
        }

        mask
    }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitMask(usize);
impl BitMask {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn from_bit_size(bits: usize) -> Self {
        Self((1 << bits) - 1)
    }

    pub const fn lsh(&mut self, shift: usize) {
        self.0 <<= shift;
    }

    pub const fn get(&self) -> usize {
        self.0
    }

    pub const fn apply(&self, target: u8) -> u8 {
        target & self.get() as u8
    }
}

pub trait BitSize<T, const BITS: usize> {
    const MAX_SIZE: usize = std::mem::size_of::<T>() * 8;
    const VALUE_MASK: BitMask = BitMask::from_bit_size(BITS);

    const IS_VALID: () = const {
        assert!(BITS < Self::MAX_SIZE, "BITS exceeds type size");
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitValue<const BITS: usize>(u8);
impl <const BITS: usize> BitValue<BITS> {
    pub const fn new<const VALUE: u8>() -> Self {
        const { assert!(Self::VALUE_MASK.apply(VALUE) == VALUE, "Value exceeds BitValue range") };

        Self(VALUE)
    }

    pub const fn from_bit_sized<const SOURCE: usize>(value: u8) -> Self {
        const { assert!(SOURCE == BITS, "SOURCE does not match target BITS") };
        Self(value)
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}
impl<const BITS: usize> BitSize<u8, BITS> for BitValue<BITS> {
    const IS_VALID: () = const {
        assert!(BITS < Self::MAX_SIZE, "BITS exceeds type size");
    };
}
impl<const BITS: usize> From<BitValue<BITS>> for u8 {
    fn from(value: BitValue<BITS>) -> Self {
        value.0
    }
}
impl<const BITS: usize> TryFrom<u8> for BitValue<BITS> {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let masked_value = BitValue::<BITS>::VALUE_MASK.apply(value);
        if masked_value == value {
            Ok(Self(masked_value))
        } else {
            Err("Value exceeds BitValue range")
        }
    }
}

// pub trait BitValueTranslator<const BITS: usize>: From<BitValue<BITS>> + Into<BitValue<BITS>> {
//     type Value: BitSize<u8, BITS>;
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitField<const BITS: usize, const SHIFT: usize>;
impl <const BITS: usize, const SHIFT: usize> BitSize<u8, BITS> for BitField<BITS, SHIFT> {
    const IS_VALID: () = const {
        assert!(BITS < Self::MAX_SIZE, "BITS exceeds type size");
        assert!(SHIFT < Self::MAX_SIZE, "SHIFT exceeds type size");
        assert!(BITS >= SHIFT, "BITS may not be smaller than SHIFT");
        assert!(
            (Self::SHIFT_MASK >> SHIFT) == Self::VALUE_MASK.get(),
            "value exceeds range"
        );
    };
}
impl<const BITS: usize, const SHIFT: usize> BitField<BITS, SHIFT> {
    pub const SHIFT_MASK: usize = Self::VALUE_MASK.get() << SHIFT;

    pub const fn apply_mask(value: u8) -> u8 {
        value & Self::SHIFT_MASK as u8
    }

    pub const fn shift_raw_value(value: u8) -> u8 {
        Self::apply_mask(value) << SHIFT
    }

    pub const fn shift_bit_value(value: BitValue<BITS>) -> u8 {
        value.as_u8() << SHIFT
    }

    pub const fn extract_raw_value(target: u8) -> u8 {
        Self::apply_mask(target) >> SHIFT
    }

    pub const fn extract_bit_value(target: u8) -> BitValue<BITS> {
        BitValue::from_bit_sized::<BITS>(Self::extract_raw_value(target))
    }

    pub const fn set_raw_value(target: &mut u8, value: u8) {
        *target |= Self::shift_raw_value(value);
    }

    pub const fn set_bit_value(target: &mut u8, value: BitValue<BITS>) {
        *target |= Self::shift_bit_value(value);
    }

}

pub type D7<const BITS: usize> = BitField<BITS, 7>;
pub type D6<const BITS: usize> = BitField<BITS, 6>;
pub type D5<const BITS: usize> = BitField<BITS, 5>;
pub type D4<const BITS: usize> = BitField<BITS, 4>;
pub type D3<const BITS: usize> = BitField<BITS, 3>;
pub type D2<const BITS: usize> = BitField<BITS, 2>;
pub type D1<const BITS: usize> = BitField<BITS, 1>;
pub type D0<const BITS: usize> = BitField<BITS, 0>;

#[macro_export]
macro_rules! bit_value_enum {
    ($VIS:vis enum $NAME:ident<$BITS:literal> {
        $(const $V1:ident = $N1:literal,)*
        #[$DEFAULT:meta]
        $(const $V2:ident = $N2:literal,)+
    }) => {
        pastey::paste! {
            $VIS type [<$NAME Value>] = $crate::st7701s_spi::transmissions::BitValue<$BITS>;
            #[derive(std::fmt::Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(u8)]
            $VIS enum $NAME {
                $($V1 = $N1,)*
                #[$DEFAULT]
                $($V2 = $N2,)+
            }
            impl $NAME {
                pub const fn as_u8(&self) -> u8 {
                    *self as _
                }
                pub const fn as_bit_value(&self) -> [<$NAME Value>] {
                    match self {
                        $($NAME::$V1 => [<$NAME Value>]::new::<$N1>(),)*
                        $($NAME::$V2 => [<$NAME Value>]::new::<$N2>(),)+
                    }
                }

                pub const fn from_raw_value(value: [<$NAME Value>]) -> Result<Self, &'static str> {
                    match value.as_u8() {
                        $($N1 => Ok($NAME::$V1),)*
                        $($N2 => Ok($NAME::$V2),)*
                        _ => Err("Value does not correspond to any enum variant"),
                    }
                }
            }
            impl From<$NAME> for [<$NAME Value>] {
                fn from(value: $NAME) -> Self {
                    value.as_bit_value()
                }
            }
            impl From<[<$NAME Value>]> for $NAME {
                fn from(value: [<$NAME Value>]) -> Self {
                    match Self::from_raw_value(value) {
                        Ok(v) => v,
                        Err(_) => unreachable!(),
                    }
                }
            }
            impl TryFrom<u8> for $NAME {
                type Error = ();

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    Self::from_raw_value($crate::st7701s_spi::transmissions::BitValue::from_bit_sized::<$BITS>(value)).map_err(|_| ())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! transmission_mapping {
    (@type_alias ($ARG:ident<$BITS:literal>, $ALIAS:ty)) => { $ALIAS };
    (@type_alias ($ARG:ident<$BITS:literal>))  => { BitValue<$BITS> };
    (
        $(#[$META:meta])*
        $SV:vis struct $NAME:ident<$LENGTH:literal> (
            $($INDEX:literal: (
                $($D:ident( $ARG:ident<$BITS:literal> $(as $ALIAS:ty)? $(= $VAL:expr)?),)+
            ),)*
        )
    ) => {
        pastey::paste! {
            pub mod [<$NAME:snake _arguments>] {
                use $crate::st7701s_spi::transmissions::*;

                $($(pub type [<$ARG:camel>] = $D<$BITS>;)+)+

                pub const PACKET_MASKS: [usize; $LENGTH] = [
                    $(merge_bit_masks([$([<$ARG:camel>]::SHIFT_MASK),+])),+
                ];

                pub const INITIAL_VALUES: [u8; $LENGTH] = [
                    $(0 $($(| $VAL)*)*),+
                ];
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            $SV struct [<$NAME:camel>]([u8; $LENGTH]);
            impl [<$NAME:camel>] {

                $(
                    $(
                        pub fn [<$ARG:lower>](&self) -> transmission_mapping!(@type_alias ($ARG<$BITS>$(,$ALIAS)?)) {
                                <transmission_mapping!(@type_alias ($ARG<$BITS>$(,$ALIAS)?))>::from([<$NAME:snake _arguments>]::[<$ARG:camel>]
                                    ::extract_bit_value(self.0[$INDEX]))
                        }

                        pub fn [<set_ $ARG:lower>](&mut self, value: transmission_mapping!(@type_alias ($ARG<$BITS>$(,$ALIAS)?)))  {
                            [<$NAME:snake _arguments>]::[<$ARG:camel>]
                                ::set_bit_value(&mut self.0[$INDEX], value.into())
                        }
                    )+
                )+

                pub const fn new() -> Self {
                    Self::from_packets([<$NAME:snake _arguments>]::INITIAL_VALUES)
                }

                pub const fn from_packets(packets: [u8; $LENGTH]) -> Self {
                    Self(packets)
                }

                pub const fn as_packets(self) -> [u8; $LENGTH] {
                    self.0
                }
            }
        }
    };
}
