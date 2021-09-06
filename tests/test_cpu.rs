extern crate game_boy_emulator;

use game_boy_emulator::cpu::*;

#[test]
fn set_get_bc_register() {
    let mut registers = Registers::default();

    registers.set_bc(23);

    assert_eq!(23, registers.get_bc());
}

#[test]
fn byte_to_flag_register() {
    let byte: u8 = 0b10100000;

    let expected = FlagsRegister {
        zero: true,
        subtract: false,
        half_carry: true,
        carry: false,
    };

    let actual = FlagsRegister::from(byte);

    assert_eq!(expected, actual);
}

#[test]
fn flag_register_to_byte() {
    let flag_register = FlagsRegister {
        zero: true,
        subtract: false,
        half_carry: true,
        carry: false,
    };

    let expected: u8 = 0b10100000;

    let actual = u8::from(flag_register);

    assert_eq!(expected, actual);
}
