//! Regression tests for the packet/bitfield layer (`transmission_mapping!`
//! and `bit_value_enum!`).
//!
//! These pin down three historical bugs, each of which silently corrupted
//! bytes on the wire:
//!
//! 1. `bit_value_enum!` computed `INITIAL_VALUE` as the OR of every variant
//!    discriminant instead of the `#[default]` variant's value.
//! 2. `transmission_mapping!` composed packet initial values from
//!    *unshifted* field initials, colliding with the low bits of the byte.
//! 3. Field setters OR-ed into the byte without clearing the field first,
//!    so defaults and earlier writes leaked into later values.

use st7701s::st7701s_spi::parameters::{
    bk0_display::RgbControl,
    bk1_power::{GateHighVoltage, GateLowVoltage, MipiSetting1},
    display::{LineSettings, PorchControl},
    general::Switch,
    register::{Bank, CommandExtension},
};
use st7701s::st7701s_spi::transmissions::BitValue;

#[test]
fn enum_initial_value_is_the_default_variant() {
    // Previously: OR of all variants (Switch = 1, Bank = 3).
    assert_eq!(Switch::INITIAL_VALUE, Switch::Off.as_u8());
    assert_eq!(Bank::INITIAL_VALUE, Bank::BK0.as_u8());
}

#[test]
fn packet_initial_values_are_shifted_into_field_positions() {
    // MIPISET1: mipi_enable defaults to On at D7 => 0x80, all other fields 0.
    // Previously rendered as 0x03 (unshifted On at D0 + unshifted defaults).
    assert_eq!(MipiSetting1::new().buffer(), &[0x80]);

    // RGBCTRL byte 1: DE active-high at D4 (0x10) + DOTCLK rising at D1
    // (0x02). Previously both contributed unshifted 0x01.
    assert_eq!(RgbControl::new().buffer(), &[0x05, 0x12, 0x0A, 0x0A]);
}

#[test]
fn setters_replace_previous_field_contents() {
    // VBP defaults to 0x04; setting 0x0B must yield 0x0B, not 0x04 | 0x0B.
    let porch = PorchControl::new().set_vertical_back_porch_const::<0x0B>();
    assert_eq!(porch.buffer(), &[0x0B, 0x02]);

    // Setting the same field twice: the second value must win outright.
    let porch = porch.set_vertical_back_porch_const::<0x04>();
    assert_eq!(porch.buffer(), &[0x04, 0x02]);
}

#[test]
fn setters_do_not_disturb_neighboring_fields() {
    // line<7> lives at D0..D6, extra_line<1> at D7. Writing one must leave
    // the other alone (and clear only its own bits).
    let lneset = LineSettings::new()
        .set_extra_line(Switch::Off)
        .set_line_const::<0x3B>();
    assert_eq!(lneset.buffer(), &[0x3B, 0x00]);

    let lneset = lneset.set_extra_line(Switch::On);
    assert_eq!(lneset.buffer(), &[0xBB, 0x00]);
}

#[test]
fn command_extension_selects_the_requested_bank() {
    // Previously every combination rendered as 0x13 (BK3 + garbage), so the
    // driver never actually selected BK0 or BK1.
    let select = |bank| {
        CommandExtension::new()
            .set_extended_commands(Switch::On)
            .set_bank(bank)
    };

    assert_eq!(select(Bank::BK0).buffer(), &[0x77, 0x01, 0x00, 0x00, 0x10]);
    assert_eq!(select(Bank::BK1).buffer(), &[0x77, 0x01, 0x00, 0x00, 0x11]);
    assert_eq!(select(Bank::BK3).buffer(), &[0x77, 0x01, 0x00, 0x00, 0x13]);

    let disable = CommandExtension::new().set_extended_commands(Switch::Off);
    assert_eq!(disable.buffer(), &[0x77, 0x01, 0x00, 0x00, 0x00]);
}

#[test]
fn gate_voltage_registers_include_their_fixed_bits() {
    // Reference sequence: VGHSS = 0x81 (12.0 V), VGLS = 0x43 (-8.14 V).
    let vgh = GateHighVoltage::new().set_amplitude_volts(12.0);
    assert_eq!(vgh.buffer(), &[0x81]);

    let vgl = GateLowVoltage::new().set_amplitude_volts(-8.14);
    assert_eq!(vgl.buffer(), &[0x43]);
}
