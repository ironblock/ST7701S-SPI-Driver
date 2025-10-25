use crate::bit_value_enum;

#[macro_export]
macro_rules! state_struct {
    ($VIS:vis struct $NAME:ident {
        $($FVIS:vis $FIELD:ident: $TYPE:ty = $VAL:expr,)+
    }) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

bit_value_enum! {
    pub enum Switch<1> {
        #[default]
        const Off = 0,
        const On = 1,
    }
}

impl Switch {
    pub fn is_on(&self) -> bool {
        *self == Self::On
    }

    pub fn is_off(&self) -> bool {
        *self == Self::Off
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Volts(pub f32);