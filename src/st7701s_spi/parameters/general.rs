use std::{
    fmt::Debug,
    ops::{Deref},
};

use crate::st7701s_spi::{address::{ReadInstruction, WriteInstruction}, transmissions::BitValue};

const fn type_bit_size<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

const fn bit_mask(bits: usize) -> usize {
    (2 << bits) - 1
}

const fn shifted_bit_mask(bits: usize, shift: usize) -> usize {
    bit_mask(bits) << shift
}


const fn value_within_mask(bits: usize, value: usize) {
    assert!((value & bit_mask(bits)) == value, "Value exceeds bit size");
}

const fn shifted_value_within_mask(bits: usize, shift: usize, value: usize) {
    assert!(
        (value & shifted_bit_mask(bits, shift)) == value,
        "Value exceeds bit size when shifted"
    );
}

const fn within_type_size<T, const BITS: usize>() {
    assert!(type_bit_size::<T>() >= BITS, "BITS exceeds size of T");
}

const fn valid_type_size<T, const BITS: usize>() -> usize {
    within_type_size::<T, BITS>();
    BITS
}

const fn is_bit_position_in_bounds<T, const SHIFT: usize>() {
    assert!(type_bit_size::<T>() >= SHIFT, "SHIFT exceeds size of T");
}

const fn is_shifted_range_in_bounds<T, const BITS: usize, const SHIFT: usize>() {
    assert!(
        type_bit_size::<T>() >= BITS + SHIFT,
        "BITS will overflow T when shifted by SHIFT"
    );
}

const fn valid_shift_type<T, const SHIFT: usize>() -> usize {
    is_bit_position_in_bounds::<T, SHIFT>();
    SHIFT
}

const fn valid_shifted_type_size<T, const BITS: usize, const SHIFT: usize>() -> usize {
    is_shifted_range_in_bounds::<T, BITS, SHIFT>();
    BITS
}

pub trait BitSize: Deref<Target = u8> {
    const BITS: usize;
}

pub trait BitRange: Deref<Target = u8> {
    const MASK: usize;
    const BITS: usize;
    const SHIFT: usize;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Argument<const BITS: usize>(u8)
where
    Self: BitSize;
impl<const BITS: usize> BitSize for Argument<BITS> {
    const BITS: usize = const { valid_type_size::<u8, BITS>() };
}
impl<const BITS: usize> Argument<BITS> {
    pub const fn new(value: u8) -> Self {
        if cfg!(debug_assertions) {
            value_within_mask(BITS, value as usize);
        }

        Self(value)
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}
impl<const BITS: usize> Deref for Argument<BITS> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position<const SHIFT: usize, const BITS: usize>(u8)
where
    Self: BitRange;
impl<const SHIFT: usize, const BITS: usize> BitRange for Position<SHIFT, BITS> {
    const MASK: usize = const { shifted_bit_mask(BITS, SHIFT) };
    const BITS: usize = const { valid_shifted_type_size::<u8, BITS, SHIFT>() };
    const SHIFT: usize = const { valid_shift_type::<u8, SHIFT>() };
}
impl<const SHIFT: usize, const BITS: usize> Position<SHIFT, BITS> {
    pub const fn new(argument: Argument<BITS>) -> Self {
        let value = argument.as_u8() << SHIFT;

        if cfg!(debug_assertions) {
            shifted_value_within_mask(BITS, SHIFT, value as usize);
        }

        Self(value)
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}
impl<const SHIFT: usize, const BITS: usize> Deref for Position<SHIFT, BITS> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {

        &self.0
    }
}

pub trait PacketList {
    fn merge(self) -> u8;
}

#[macro_export]
macro_rules! enum_argument {
    ($VIS:vis enum $NAME:ident[$HI:literal : $LO:literal] {
        $($V1:ident = $N1:literal,)*
        #[$DEFAULT:meta]
        $($V2:ident = $N2:literal,)+
    }) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
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
            pub const fn as_bit_value(&self) -> BitValue<$HI, $LO> {
                BitValue::new(self.as_u8())
            }
        }
        impl From<$NAME> for BitValue<$HI, $LO> {
            fn from(value: $NAME) -> Self {
                value.as_bit_value()
            }
        }
        impl TryFrom<u8> for $NAME {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($N1 => Ok(Self::$V1),)*
                    $($N2 => Ok(Self::$V2),)*
                    _ => Err(()),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! state_struct {
    ($VIS:vis struct $NAME:ident {
        $($FVIS:vis $FIELD:ident: $TYPE:ty = $VAL:expr,)+
    }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        $VIS struct $NAME {
            $($FVIS $FIELD: $TYPE,)+
        }
        impl $NAME {
            $VIS const fn new() -> Self {
                Self {
                    $($FIELD: $VAL),+
                }
            }
        }
        impl Default for $NAME {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

pub trait EncodeData<W: WriteInstruction> {
    fn encode(&self) -> W::Buffer;
}

pub trait DecodeData<R: ReadInstruction> {
    fn decode(data: &R::Buffer) -> Self;
}

enum_argument! {
    pub enum Switch[0:0] {
        #[default]
        Off = 0,
        On = 1,
    }
}

enum_argument! {
    pub enum Direction[0:0] {
        #[default]
        Normal = 0,
        Reverse = 1,
    }
}

enum_argument! {
    pub enum Logic[0:0] {
        #[default]
        Low = 0,
        High = 1,
    }
}

enum_argument! {
    pub enum Edge[0:0] {
        #[default]
        Falling = 0,
        Rising = 1,
    }
}
