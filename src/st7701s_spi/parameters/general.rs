use std::{fmt::Debug, ops::Deref};

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

pub trait ValidBitSize {
    const SIZE: usize;
}

pub trait BitRange {
    const SIZE: usize;
    const SHIFT: usize;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Argument<const BITS: usize>(u8);
impl<const BITS: usize> ValidBitSize for Argument<BITS> {
    const SIZE: usize = const { valid_type_size::<u8, BITS>() };
}
impl<const BITS: usize> Argument<BITS> {
    pub const fn new(value: u8) -> Self
    where
        Self: ValidBitSize,
    {
        if cfg!(debug_assertions) {
            value_within_mask(BITS, value as usize);
        }

        Self(value)
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}
// impl<const BITS: usize> Deref for Argument<BITS> {
//     type Target = u8;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// #[derive(Default, Debug, PartialEq)]
// pub struct ShiftedArgument<T: BitSize, const SHIFT: usize>(T);
// impl<T: BitSize, const SHIFT: usize> BitRange for ShiftedArgument<T, SHIFT> {
//     const SIZE: usize = T::SIZE;
//     const SHIFT: usize = SHIFT;

//     const CHECK_SHIFT: () = check_shift::<T, T::SIZE, SHIFT>();
//     const CHECK_SHIFT_SIZE: () = check_shift_size::<T, T::SIZE, SHIFT>();

//     const SHIFT_MASK: usize = const { shift_mask(T::SIZE, SHIFT) };
// }
// impl<T: BitSize, const SHIFT: usize> ShiftedArgument<T, SHIFT> {
//     pub const fn new(arg: T) -> Self {
//         Self(arg << Self::SHIFT)
//     }
// }
// impl<T: BitSize, const SHIFT: usize> From<u8> for ShiftedArgument<T, SHIFT> {
//     fn from(value: u8) -> Self {
//         Self(Argument::new(value))
//     }
// }

// pub const fn d0<T: BitSize>(arg: T) -> ShiftedArgument<T, 0> {
//     ShiftedArgument::new(arg)
// }
// pub const fn d1<T: BitSize>(arg: T) -> ShiftedArgument<T, 1> {
//     ShiftedArgument::new(arg)
// }
// pub const fn d2<T: BitSize>(arg: T) -> ShiftedArgument<T, 2> {
//     ShiftedArgument::new(arg)
// }
// pub const fn d3<T: BitSize>(arg: T) -> ShiftedArgument<T, 3> {
//     ShiftedArgument::new(arg)
// }
// pub const fn d4<T: BitSize>(arg: T) -> ShiftedArgument<T, 4> {
//     ShiftedArgument::new(arg)
// }
// pub const fn d5<T: BitSize>(arg: T) -> ShiftedArgument<T, 5> {
//     ShiftedArgument::new(arg)
// }
// pub const fn d6<T: BitSize>(arg: T) -> ShiftedArgument<T, 6> {
//     ShiftedArgument::new(arg)
// }
// pub const fn d7<T: BitSize>(arg: T) -> ShiftedArgument<T, 7> {
//     ShiftedArgument::new(arg)
// }

// pub struct Packet(u8);
// impl Packet {
//     pub const fn new<const N: usize>(value: u8) -> Self {
//         Self(value)
//     }

//     pub const fn from_array(arguments: [ShiftedArgument<_, _>]) -> Self {
//         let mut combined_mask: usize = 0;
//         let mut value = 0;
//         let mut i = 0;

//         while i < arguments.len() {
//             if cfg!(debug_assertions) {
//                 assert!(
//                     arguments[i].SHIFT_MASK & combined_mask == 0,
//                     "Arguments overlap"
//                 );
//                 combined_mask |= arguments[i].SHIFT_MASK;
//             }

//             value |= arguments[i].value();
//             i += 1;
//         }

//         Self(value)
//     }
// }
// impl From<u8> for Packet {
//     fn from(value: u8) -> Self {
//         Self::new(value)
//     }
// }

struct BitPosition<const BITS: usize, const SHIFT: usize>(u8);
impl<const BITS: usize, const SHIFT: usize> BitRange for BitPosition<BITS, SHIFT> {
    const SIZE: usize = const { valid_shifted_type_size::<u8, BITS, SHIFT>() };
    const SHIFT: usize = const { valid_shift_type::<u8, SHIFT>() };
}
impl<const BITS: usize, const SHIFT: usize> BitPosition<BITS, SHIFT> {
    pub const fn new(argument: Argument<BITS>) -> Self {
        let value = argument.as_u8();
        if cfg!(debug_assertions) {
            shifted_value_within_mask(BITS, SHIFT, value as usize);
        }

        Self(value << SHIFT)
    }
}
impl<const BITS: usize, const SHIFT: usize> Deref for BitPosition<BITS, SHIFT> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type D7<const BITS: usize> = BitPosition<BITS, 7>;
type D6<const BITS: usize> = BitPosition<BITS, 6>;
type D5<const BITS: usize> = BitPosition<BITS, 5>;
type D4<const BITS: usize> = BitPosition<BITS, 4>;
type D3<const BITS: usize> = BitPosition<BITS, 3>;
type D2<const BITS: usize> = BitPosition<BITS, 2>;
type D1<const BITS: usize> = BitPosition<BITS, 1>;
type D0<const BITS: usize> = BitPosition<BITS, 0>;

// impl<const BITS: usize> BitPosition for D7<BITS> {
//     const PLACE: usize = 7;
// }
// impl<const BITS: usize> BitPosition for D6<BITS> {
//     const PLACE: usize = 6;
// }
// impl<const BITS: usize> BitPosition for D5<BITS> {
//     const PLACE: usize = 5;
// }
// impl<const BITS: usize> BitPosition for D4<BITS> {
//     const PLACE: usize = 4;
// }
// impl<const BITS: usize> BitPosition for D3<BITS> {
//     const PLACE: usize = 3;
// }
// impl<const BITS: usize> BitPosition for D2<BITS> {
//     const PLACE: usize = 2;
// }
// impl<const BITS: usize> BitPosition for D1<BITS> {
//     const PLACE: usize = 1;
// }
// impl<const BITS: usize> BitPosition for D0<BITS> {
//     const PLACE: usize = 0;
// }

// impl <T, const BITS: usize> BitRange<BITS, {T::PLACE}> for T where T: BitPosition {}

// impl<A> BitParameters for (A,) where A: BitRange {}
// impl<A, B> BitParameters for (A, B)
// where
//     A: BitRange,
//     B: BitRange,
// {
// }
// impl<A, B, C> BitParameters for (A, B, C)
// where
//     A: BitRange,
//     B: BitRange,
//     C: BitRange,
// {
// }
// impl<A, B, C, D> BitParameters for (A, B, C, D)
// where
//     A: BitRange,
//     B: BitRange,
//     C: BitRange,
//     D: BitRange,
// {
// }

// pub mod packet_data {
//     use crate::st7701s_spi::parameters::general::{shifted_bit_mask, BitRange};

//     const fn bit_range_mask<R: BitRange>() -> usize {
//         shifted_bit_mask(R::SIZE, R::SHIFT)
//     }

//     const fn bit_masks_are_mutually_exclusive<const N: usize>(masks: [impl BitRange; N]) {
//         let mut combined_mask: usize = 0;
//         let mut i: usize = 0;

//         while i < masks.len() {
//             let shifted_mask = bit_range_mask:: <(masks[i]) as BitRange >();
//             assert!((shifted_mask & combined_mask) == 0, "Bit masks overlap");
//             combined_mask |= shifted_mask;
//             i += 1;
//         }
//     }
//     pub trait PacketList {
//         const VALID: ();
//         // fn merge(self) -> u8;
//     }

//     impl<A> PacketList for (A,)
//     where
//         A: BitRange,
//     {
//         const VALID: () = const { bit_masks_are_mutually_exclusive([(A::SIZE, A::SHIFT)]) };
//     }

//     impl<A, B> PacketList for (A, B)
//     where
//         A: BitRange,
//         B: BitRange,
//     {
//         const VALID: () =
//             const { bit_masks_are_mutually_exclusive([(A::SIZE, A::SHIFT), (B::SIZE, B::SHIFT)]) };
//     }
//     impl<A, B, C> PacketList for (A, B, C)
//     where
//         A: BitRange,
//         B: BitRange,
//         C: BitRange,
//     {
//         const VALID: () = const {
//             bit_masks_are_mutually_exclusive([
//                 (A::SIZE, A::SHIFT),
//                 (B::SIZE, B::SHIFT),
//                 (C::SIZE, C::SHIFT),
//             ])
//         };
//     }
//     impl<A, B, C, D> PacketList for (A, B, C, D)
//     where
//         A: BitRange,
//         B: BitRange,
//         C: BitRange,
//         D: BitRange,
//     {
//         const VALID: () = const {
//             bit_masks_are_mutually_exclusive([
//                 (A::SIZE, A::SHIFT),
//                 (B::SIZE, B::SHIFT),
//                 (C::SIZE, C::SHIFT),
//                 (D::SIZE, D::SHIFT),
//             ])
//         };
//     }
//     impl<A, B, C, D, E> PacketList for (A, B, C, D, E)
//     where
//         A: BitRange,
//         B: BitRange,
//         C: BitRange,
//         D: BitRange,
//         E: BitRange,
//     {
//         const VALID: () = const {
//             bit_masks_are_mutually_exclusive([
//                 (A::SIZE, A::SHIFT),
//                 (B::SIZE, B::SHIFT),
//                 (C::SIZE, C::SHIFT),
//                 (D::SIZE, D::SHIFT),
//                 (E::SIZE, E::SHIFT),
//             ])
//         };
//     }
//     impl<A, B, C, D, E, F> PacketList for (A, B, C, D, E, F)
//     where
//         A: BitRange,
//         B: BitRange,
//         C: BitRange,
//         D: BitRange,
//         E: BitRange,
//         F: BitRange,
//     {
//         const VALID: () = const {
//             bit_masks_are_mutually_exclusive([
//                 (A::SIZE, A::SHIFT),
//                 (B::SIZE, B::SHIFT),
//                 (C::SIZE, C::SHIFT),
//                 (D::SIZE, D::SHIFT),
//                 (E::SIZE, E::SHIFT),
//                 (F::SIZE, F::SHIFT),
//             ])
//         };
//     }
// }

trait Pack {
    const D7: usize = 0;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Packet {
    mask: u8,
    value: u8,
}
impl Packet {
    pub const fn new() -> Self {
        // while i < arguments.len() {
        //     if cfg!(debug_assertions) {
        //         assert!(
        //             (shifted_bit_mask(<arguments[i] as dyn BitRange>::SIZE, arguments[i].SHIFT) & combined_mask) == 0,
        //             "Arguments overlap"
        //         );
        //         combined_mask |= arguments[i].SHIFT_MASK;
        //     }

        //     value |= arguments[i].value();
        //     i += 1;
        // }

        Self { mask: 0, value: 0 }
    }

    const fn shift_and_merge<const SHIFT: usize, const BITS: usize>(
        self,
        argument: Argument<BITS>,
    ) -> Self {
        let shifted: u8 = argument.as_u8() << SHIFT as u8;
        let mut mask: u8 = 0;

        if cfg!(debug_assertions) {
            mask = shifted_bit_mask(BITS, SHIFT) as u8;
            assert!(self.mask & mask == 0, "Argument overlaps existing bits");
        }

        Self {
            value: self.value | shifted,
            mask: self.mask | mask,
        }
    }

    pub const fn d7<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<7, BITS>(argument)
    }

    pub const fn d6<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<6, BITS>(argument)
    }

    pub const fn d5<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<5, BITS>(argument)
    }

    pub const fn d4<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<4, BITS>(argument)
    }

    pub const fn d3<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<3, BITS>(argument)
    }

    pub const fn d2<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<2, BITS>(argument)
    }

    pub const fn d1<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<1, BITS>(argument)
    }

    pub const fn d0<const BITS: usize>(self, argument: Argument<BITS>) -> Self {
        self.shift_and_merge::<0, BITS>(argument)
    }
}
impl Deref for Packet {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait Packets: AsRef<[u8]> + IntoIterator<Item = u8> {}
pub trait InstructionData {
    type Packets: Packets;
}

pub trait EncodeData: InstructionData {
    fn encode(&self) -> Self::Packets;
}

pub trait DecodeData: InstructionData {
    fn decode(packets: Self::Packets) -> Self;
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
