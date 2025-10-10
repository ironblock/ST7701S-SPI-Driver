use crate::{bit_value_enum, st7701s_spi::address::{ReadInstruction, WriteInstruction}};

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

bit_value_enum! {
    pub enum Switch<1> {
        #[default]
        const Off = 0,
        const On = 1,
    }
}

bit_value_enum! {
    pub enum Direction<1> {
        #[default]
        const Normal = 0,
        const Reverse = 1,
    }
}

bit_value_enum! {
    pub enum Logic<1> {
        #[default]
        const Low = 0,
        const High = 1,
    }
}

bit_value_enum! {
    pub enum Edge<1> {
        #[default]
        const Falling = 0,
        const Rising = 1,
    }
}
