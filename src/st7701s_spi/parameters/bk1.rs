use crate::state_struct;

state_struct! {
    /// MIPI Setting 2 Parameters
    /// Used for BK1 MIPISET2 (0xD1) command (15 parameters)
    pub struct MipiSetting2 {
        pub param1: u8,
        pub param2: u8,
        pub param3: u8,
        pub param4: u8,
        pub param5: u8,
        pub param6: u8,
        pub param7: u8,
        pub param8: u8,
        pub param9: u8,
        pub param10: u8,
        pub param11: u8,
        pub param12: u8,
        pub param13: u8,
        pub param14: u8,
        pub param15: u8,
    }
}
