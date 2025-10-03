use std::{
    fmt::Debug,
    ops::{Deref},
};

use crate::st7701s_spi::address::{ReadInstruction, WriteInstruction};

const fn type_bit_size<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

const fn bit_mask(bits: usize) -> usize {
    (2 << bits) - 1
}

const fn shifted_bit_mask(bits: usize, shift: usize) -> usize {
    bit_mask(bits) << shift
}

const fn merge_bit_masks<const N: usize>(masks: [(usize, usize); N]) -> usize {
    let mut mask = 0;
    let mut i = 0;

    while i < masks.len() {
        let (bits, shift) = masks[i];
        let next_mask = shifted_bit_mask(bits, shift);

        assert!((mask & next_mask) == 0, "Overlapping bit masks");

        mask |= next_mask;
        i += 1;
    }

    mask
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


pub fn d7<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<7, BITS> {
    Position::new(argument.into())
}
pub fn d6<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<6, BITS> {
    Position::new(argument.into())
}
pub fn d5<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<5, BITS> {
    Position::new(argument.into())
}
pub fn d4<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<4, BITS> {
    Position::new(argument.into())
}
pub fn d3<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<3, BITS> {
    Position::new(argument.into())
}
pub fn d2<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<2, BITS> {
    Position::new(argument.into())
}
pub fn d1<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<1, BITS> {
    Position::new(argument.into())
}
pub fn d0<const BITS: usize>(argument: impl Into<Argument<BITS>>) -> Position::<0, BITS> {
    Position::new(argument.into())
}

pub trait PacketList {
    fn merge(self) -> u8;
}

#[macro_export]
macro_rules! enum_argument {
    ($VIS:vis enum $NAME:ident<$BITS:literal> {
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
        impl From<$NAME> for Argument<$BITS> {
            fn from(value: $NAME) -> Self {
                Argument::new(value as u8)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_packet {
    ($($T:ident),+) => {
        #[allow(unused_parens, non_snake_case)]
        impl <$($T),*> PacketList for ($($T),+)
        where
            $($T: BitRange,)+
        {
            fn merge(self) -> u8 {
                const {  merge_bit_masks([$(($T::BITS, $T::SHIFT)),*]) };
                let ($($T),+) = self;

                0 $(| *$T)+
            }
        }
    };
}

impl_packet!(A);
impl_packet!(A, B);
impl_packet!(A, B, C);
impl_packet!(A, B, C, D);
impl_packet!(A, B, C, D, E);
impl_packet!(A, B, C, D, E, F);
impl_packet!(A, B, C, D, E, F, G);
impl_packet!(A, B, C, D, E, F, G, H);

pub trait EncodeData {
    fn encode(&self) -> impl AsRef<[u8]>;
}

pub trait DecodeData<R: ReadInstruction> {
    fn decode(packets: &R::Buffer) -> Self;
}

#[derive(Default, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Switch {
    #[default]
    Off = 0,
    On = 1,
}

#[derive(Default, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Normal = 0,
    Reverse = 1,
}

#[derive(Default, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Logic {
    #[default]
    Low = 0,
    High = 1,
}

#[derive(Default, Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Edge {
    #[default]
    Falling = 0,
    Rising = 1,
}
