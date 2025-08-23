
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Switch {
    Off = 0,
    On = 1,
}
impl Switch {
    pub const fn as_d(&self, n: u8) -> u8 {
        (*self as u8) << n
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Direction {
    Normal = 0,
    Reverse = 1,
}
impl Direction {
    pub const fn as_d(&self, n: u8) -> u8 {
        (*self as u8) << n
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Logic {
    Low = 0,
    High = 1,
}
impl Logic {
    pub const fn as_d(&self, n: u8) -> u8 {
        (*self as u8) << n
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Edge {
    Falling = 0,
    Rising = 1,
}
impl Edge {
    pub const fn as_d(&self, n: u8) -> u8 {
        (*self as u8) << n
    }
}

// #[derive(Debug)]
// pub enum Power {
//     L1,
//     L2,
//     L3,
//     L4,
//     L5,
// }

// impl Power {
//     const fn level(state: State) -> Self {
//         use Switch::*;

//         match state {
//             State {
//                 partial_mode: Off,
//                 idle_mode: Off,
//                 sleep_mode: Off,
//                 ..
//             } => Self::L1,
//             State {
//                 partial_mode: On,
//                 idle_mode: Off,
//                 sleep_mode: Off,
//                 ..
//             } => Self::L2,
//             State {
//                 partial_mode: Off,
//                 idle_mode: On,
//                 sleep_mode: Off,
//                 ..
//             } => Self::L3,
//             State {
//                 partial_mode: On,
//                 idle_mode: On,
//                 sleep_mode: Off,
//                 ..
//             } => Self::L4,
//             State { sleep_mode: On, .. } => Self::L5,
//         }
//     }
// }
