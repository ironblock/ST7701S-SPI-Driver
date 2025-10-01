use crate::st7701s_spi::parameters::general::*;

// FIXME: This isn't necessarily correct, and the datasheet indicates
// that the initial value is "RESERVED". Maybe better to set this to
// something that can never be matched, or change the trait to not
// require an initial value?
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Curve {
    #[default]
    GC1 = 0,
    GC2 = 1,
    GC3 = 2,
    GC4 = 3,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VoltageBias {
    #[default]
    L1 = 0,
    L0 = 1,
    R0 = 2,
    R1 = 3,
}
impl Into<Argument<4>> for VoltageBias {
    fn into(self) -> Argument<4> {
        Argument::new(self as u8)
    }
}

type DAC4Bit = Argument<4>;
type DAC5Bit = Argument<5>;
type DAC6Bit = Argument<6>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VoltageControl {
    aj0: VoltageBias,
    vc0: DAC4Bit,
    aj1: VoltageBias,
    vc4: DAC6Bit,
    aj2: VoltageBias,
    vc8: DAC6Bit,
    vc16: DAC5Bit,
    aj3: VoltageBias,
    vc24: DAC5Bit,
    vc52: DAC4Bit,
    vc80: DAC6Bit,
    vc108: DAC4Bit,
    vc147: DAC4Bit,
    vc175: DAC6Bit,
    vc203: DAC4Bit,
    aj4: VoltageBias,
    vc231: DAC5Bit,
    vc239: DAC5Bit,
    aj5: VoltageBias,
    vc247: DAC6Bit,
    aj6: VoltageBias,
    vc251: DAC6Bit,
    aj7: VoltageBias,
    vc255: DAC5Bit,
}
impl InstructionData for VoltageControl {
    type Packets = [u8; 16];
}

impl EncodeData for VoltageControl {
    fn encode(&self) -> Self::Packets {
        [
            *Packet::new().d6(self.aj0.into()).d0(self.vc0),
            *Packet::new().d6(self.aj1.into()).d0(self.vc4),
            *Packet::new().d6(self.aj2.into()).d0(self.vc8),
            *Packet::new().d0(self.vc16),
            *Packet::new().d6(self.aj3.into()).d0(self.vc24),
            *Packet::new().d0(self.vc52),
            *Packet::new().d0(self.vc80),
            *Packet::new().d0(self.vc108),
            *Packet::new().d0(self.vc147),
            *Packet::new().d0(self.vc175),
            *Packet::new().d0(self.vc203),
            *Packet::new().d6(self.aj4.into()).d0(self.vc231),
            *Packet::new().d0(self.vc239),
            *Packet::new().d6(self.aj5.into()).d0(self.vc247),
            *Packet::new().d6(self.aj6.into()).d0(self.vc251),
            *Packet::new().d6(self.aj7.into()).d0(self.vc255),
        ]
    }
}
