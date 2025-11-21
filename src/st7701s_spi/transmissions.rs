use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};

pub trait Parametric<const N: usize> {
    const INITIAL_VALUE: [u8; N];
    const ARGUMENT_MASK: [BitMask; N];
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Parameters<V, const N: usize>
where
    Self: Parametric<N>,
{
    buffer: [u8; N],
    _variant: PhantomData<V>,
}
impl<V, const N: usize> Parameters<V, N>
where
    Self: Parametric<N>,
{
    pub const fn new() -> Self {
        Self {
            buffer: Self::INITIAL_VALUE,
            _variant: PhantomData,
        }
    }

    pub const fn from_array(buffer: [u8; N]) -> Self {
        Self {
            buffer,
            _variant: PhantomData,
        }
    }

    pub const fn buffer(&self) -> &[u8; N] {
        &self.buffer
    }

    pub const fn buffer_mut(&mut self) -> &mut [u8; N] {
        &mut self.buffer
    }
}
impl<T, const N: usize> Default for Parameters<T, N>
where
    Self: Parametric<N>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BitMask(usize);
impl BitMask {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn from_bit_size(bits: usize) -> Self {
        Self((1 << bits) - 1)
    }

    pub const fn lsh(mut self, shift: usize) -> Self {
        self.0 <<= shift;

        self
    }

    pub const fn merge(self, other: &Self) -> Self {
        assert!((self.0 & other.0) == 0, "Overlapping bit masks");

        Self(self.0 | other.0)
    }

    pub const fn get(&self) -> usize {
        self.0
    }

    pub const fn apply(&self, target: u8) -> u8 {
        target & self.get() as u8
    }
}

impl Display for BitMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010b}", self.0)
    }
}

pub trait BitValue {
    type Target;
    const BIT_SIZE: usize;
    const INITIAL_VALUE: Self::Target;

    const MAX_SIZE: usize = std::mem::size_of::<Self::Target>() * 8;
    const SIZE_MASK: BitMask = BitMask::from_bit_size(Self::BIT_SIZE);

    const IS_VALID: () = const {
        assert!(Self::BIT_SIZE < Self::MAX_SIZE, "BITS exceeds type size");
    };
}

pub trait BitOffset<S: BitValue> {
    const SHIFT: usize;

    const SHIFT_MASK: BitMask = BitMask::from_bit_size(S::BIT_SIZE).lsh(Self::SHIFT);

    const IS_VALID: () = const {
        assert!(Self::SHIFT < S::MAX_SIZE, "SHIFT exceeds type size");
        assert!(
            Self::SHIFT_MASK.get() <= S::MAX_SIZE,
            "Shifted value exceeds range"
        );
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BitField<const SIZE: usize, const INITIAL: u8 = 0>
where
    Self: BitValue,
{
    value: u8,
}
impl<const SIZE: usize, const INITIAL: u8> BitValue for BitField<SIZE, INITIAL> {
    type Target = u8;
    const BIT_SIZE: usize = SIZE;
    const INITIAL_VALUE: Self::Target = INITIAL;
}
impl<const SIZE: usize, const INITIAL: u8> BitField<SIZE, INITIAL> {
    pub const fn new() -> Self {
        const {
            assert!(
                Self::SIZE_MASK.apply(INITIAL) == INITIAL,
                "Value exceeds BitField range"
            )
        };

        Self { value: INITIAL }
    }

    const fn assert_source(source: usize) {
        assert!(
            source == Self::BIT_SIZE,
            "SOURCE does not match target BIT_SIZE"
        );
    }

    const fn assert_value(value: u8) {
        assert!(
            Self::SIZE_MASK.apply(value) == value,
            "Value exceeds BitField range"
        );
    }

    pub const fn from_const<const SOURCE: usize, const VALUE: u8>() -> Self {
        const {
            Self::assert_source(SOURCE);
            Self::assert_value(VALUE);
        };

        Self { value: VALUE }
    }

    pub const fn from_const_source<const SOURCE: usize>(value: u8) -> Self {
        const {
            Self::assert_source(SOURCE);
        };

        Self { value }
    }

    pub const fn from_const_value<const VALUE: u8>() -> Self {
        const {
            Self::assert_value(VALUE);
        };

        Self { value: VALUE }
    }

    pub const fn as_u8(&self) -> u8 {
        self.value
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PacketField<const SHIFT: usize, const BITS: usize, const INITIAL: u8 = 0>
where
    Self: BitValue,
    Self: BitOffset<Self>;
impl<const SHIFT: usize, const BITS: usize, const INITIAL: u8> BitValue
    for PacketField<SHIFT, BITS, INITIAL>
{
    type Target = u8;
    const BIT_SIZE: usize = BITS;
    const INITIAL_VALUE: Self::Target = INITIAL;
}
impl<const SHIFT: usize, const BITS: usize, const INITIAL: u8> BitOffset<Self>
    for PacketField<SHIFT, BITS, INITIAL>
{
    const SHIFT: usize = SHIFT;
}
impl<const SHIFT: usize, const BITS: usize, const INITIAL: u8> PacketField<SHIFT, BITS, INITIAL> {
    pub const fn as_bit_value() -> BitField<BITS, INITIAL> {
        BitField::<BITS, INITIAL>::new()
    }

    pub const fn shift_raw_value(value: u8) -> u8 {
        Self::SHIFT_MASK.apply(value << SHIFT)
    }

    pub const fn shift_bit_value(value: BitField<BITS>) -> u8 {
        value.as_u8() << SHIFT
    }

    pub const fn extract_raw_value(target: u8) -> u8 {
        Self::SHIFT_MASK.apply(target) >> SHIFT
    }

    pub const fn extract_bit_value(target: u8) -> BitField<BITS, INITIAL> {
        BitField::<BITS, INITIAL>::from_const_source::<BITS>(Self::extract_raw_value(target))
    }

    // pub const fn set_raw_value(target: &mut u8, value: u8) {
    //     *target |= Self::shift_raw_value(value);
    // }

    pub const fn set_from_bitfield(target: &mut u8, value: BitField<BITS>) {
        *target |= Self::shift_bit_value(value);
    }

    pub const fn set_const<const VALUE: u8>(target: &mut u8) {
        *target |= Self::shift_bit_value(BitField::<BITS>::from_const_value::<VALUE>());
    }
}

pub type D7<const BITS: usize, const INITIAL: u8 = 0> = PacketField<7, BITS, INITIAL>;
pub type D6<const BITS: usize, const INITIAL: u8 = 0> = PacketField<6, BITS, INITIAL>;
pub type D5<const BITS: usize, const INITIAL: u8 = 0> = PacketField<5, BITS, INITIAL>;
pub type D4<const BITS: usize, const INITIAL: u8 = 0> = PacketField<4, BITS, INITIAL>;
pub type D3<const BITS: usize, const INITIAL: u8 = 0> = PacketField<3, BITS, INITIAL>;
pub type D2<const BITS: usize, const INITIAL: u8 = 0> = PacketField<2, BITS, INITIAL>;
pub type D1<const BITS: usize, const INITIAL: u8 = 0> = PacketField<1, BITS, INITIAL>;
pub type D0<const BITS: usize, const INITIAL: u8 = 0> = PacketField<0, BITS, INITIAL>;

#[macro_export]
macro_rules! bit_value_enum {
    (
        $(#[$META:meta])*
        $VIS:vis enum $NAME:ident<$BITS:literal> {
        $(const $V1:ident = $N1:literal,)*
        #[$DEFAULT:meta]
        $(const $V2:ident = $N2:literal,)+
    }) => {
        pastey::paste! {
            $VIS type [<$NAME Value>] = $crate::st7701s_spi::transmissions::BitField<$BITS>;
            $(#[$META])*
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
            #[repr(u8)]
            $VIS enum $NAME {
                $($V1 = $N1,)*
                #[$DEFAULT]
                $($V2 = $N2,)+
            }
            impl $crate::st7701s_spi::transmissions::BitValue for $NAME {
                type Target = u8;
                const BIT_SIZE: usize = $BITS;
                const INITIAL_VALUE: Self::Target = 0 $(| $N2)?;
            }
            impl $NAME {
                pub const fn as_u8(&self) -> u8 {
                    *self as _
                }

                pub const fn as_bit_value(&self) -> [<$NAME Value>] {
                    match self {
                        $($NAME::$V1 => [<$NAME Value>]::from_const::<$BITS, $N1>(),)*
                        $($NAME::$V2 => [<$NAME Value>]::from_const::<$BITS, $N2>(),)+
                    }
                }

                pub const fn from_raw_value(value: u8) -> Result<Self, &'static str> {
                    match value {
                        $($N1 => Ok($NAME::$V1),)*
                        $($N2 => Ok($NAME::$V2),)+
                        _ => Err("Value does not correspond to any enum variant"),
                    }
                }

                pub const fn from_bit_value(value: [<$NAME Value>]) -> Self {
                    match Self::from_raw_value(value.as_u8()) {
                        Ok(v) => v,
                        Err(_) => unreachable!(),
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
                    Self::from_bit_value(value)
                }
            }
            impl TryFrom<u8> for $NAME {
                type Error = &'static str;

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    Self::from_raw_value(value)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! transmission_mapping {
    (@base_value $BASE:literal) => { $BASE };
    (@base_value)               => { 0 };

    (@value_type ($T:ident<$BITS:literal, $ALIAS:ty>)) => { $ALIAS };
    (@value_type ($T:ident<$BITS:literal>)) =>            { $crate::st7701s_spi::transmissions::BitField<$BITS> };

    (@initial_value ()) => { 0 };
    (@initial_value ($ALIAS:ident,)) => { $ALIAS::INITIAL_VALUE };
    (@initial_value ($ALIAS:ident, $ALIAS_INITIAL:ident,)) => { $ALIAS_INITIAL.as_u8() };
    (@initial_value ($VAL:literal,)) => { $VAL };

    (@argument_mask $($T:tt)+) => {
        BitMask::new(0) $(.merge(& $T::SHIFT_MASK))+
    };

    (
        $(#[$META:meta])*
        $SV:vis struct $NAME:ident<$LENGTH:literal> (
            $(
                $INDEX:literal: (
                    $(
                        $D:ident(
                            $ARG:ident<$BITS:tt>
                            $(as $ALIAS:ident $(= $ALIAS_INITIAL:ident)?)?
                            $(= $VAL:literal)?
                        )
                    ,)*
                ) $(= $BASE:literal)?
            ,)+
        );
    ) => {
        pastey::paste! {
            mod [<$NAME:snake _types>] {
                #[allow(unused_imports)]
                use super::*;

                #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
                pub struct [<$NAME:camel Marker>];

                $(
                    $(
                        pub type [<$ARG:camel Value>] = $crate::transmission_mapping!(@value_type ($ARG<$BITS $(,$ALIAS)?>));
                        pub type [<$ARG:camel Field>] = $crate::st7701s_spi::transmissions::$D<$BITS, {$crate::transmission_mapping!(
                            @initial_value (
                                $($ALIAS, $($ALIAS_INITIAL,)?)?
                                $($VAL,)?
                            )
                        )}>;
                    )*
                )+
            }

            $(#[$META])*
            $SV type [<$NAME:camel>] = $crate::st7701s_spi::transmissions::Parameters<[<$NAME:snake _types>]::[<$NAME:camel Marker>], $LENGTH>;
            impl $crate::st7701s_spi::transmissions::Parametric<$LENGTH> for [<$NAME:camel>] {
                const INITIAL_VALUE: [u8; $LENGTH] = [
                    $(
                        $crate::transmission_mapping!(@base_value $($BASE)?)
                        $(| [<$NAME:snake _types>]::[<$ARG:camel Field>]::INITIAL_VALUE)*
                    ),+
                ];

                const ARGUMENT_MASK: [BitMask; $LENGTH] = [
                    $(
                        $crate::st7701s_spi::transmissions::BitMask::new(0)
                            $(.merge(&[<$NAME:snake _types>]::[<$ARG:camel Field>]::SHIFT_MASK))*
                    ),+
                ];
            }
            impl [<$NAME:camel>] {
                $(
                    $(
                        pub fn [<$ARG:lower>](&self) -> [<$NAME:snake _types>]::[<$ARG:camel Value>] {
                            <transmission_mapping!(
                                @value_type ($ARG<$BITS $(,$ALIAS)?>)
                            )>
                                ::from($D::<$BITS>::extract_bit_value(self.buffer()[$INDEX]))
                        }

                        pub fn [<set_ $ARG:lower>](&mut self, value: [<$NAME:snake _types>]::[<$ARG:camel Value>]) -> &mut Self {
                            $D::<$BITS>
                                ::set_from_bitfield(&mut self.buffer_mut()[$INDEX], value.into());

                            self
                        }
                    )*
                )+
            }
        }
    }
}
