use crate::{
    bit_value_enum,
    st7701s_spi::{
        address::{ExtensionBk1, bk1::PWCTRL1},
        device::ST7701S,
        protocol::connection::{Connection, InstructionResult},
    },
    transmission_mapping,
};


bit_value_enum! {
    pub enum BiasCurrent<2> {
        #[default]
        const Off = 0b00,
        const Min = 0b01,
        const Mid = 0b10,
        const Max = 0b11,
    }
}

// transmission_mapping! {
//     /// ## Power Control 1
//     /// Primary power supply configuration
//     /// > Reference: `PWCTRL1` p. 292
//     pub struct BiasCurrentParameters<1>(
//         0: (
//             D6(gamma_bias<2> as BiasCurrent = Mid),
//             D2(source_input_bias<2> as BiasCurrent = Max),
//             D0(source_output_bias<2> as BiasCurrent = Off),
//         ),
//     );
// }

// impl<C: Connection> ST7701S<C, ExtensionBk1> {
//     pub fn select_bias_current(&self, transmission: &BiasCurrentParameters) -> InstructionResult {
//         self.write::<PWCTRL1>(transmission)
//     }
// }
