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

#[test]
fn cpu_add_simple() {
    let mut cpu = CPU::default();

    cpu.registers.a = 1;
    cpu.registers.c = 2;

    let pc = cpu.execute(Instruction::ADD(ArithmeticTarget::C));

    assert_eq!(3, cpu.registers.a);
    assert_eq!(2, cpu.registers.c);

    assert_eq!(1, pc);

    let expected_flags = FlagsRegister {
        zero: false,
        subtract: false,
        half_carry: false,
        carry: false,
    };

    assert_eq!(expected_flags, cpu.registers.f);
}

#[test]
fn cpu_add_zero() {
    let mut cpu = CPU::default();

    cpu.registers.a = 0;
    cpu.registers.c = 0;

    let pc = cpu.execute(Instruction::ADD(ArithmeticTarget::C));

    assert_eq!(0, cpu.registers.a);
    assert_eq!(0, cpu.registers.c);

    assert_eq!(1, pc);

    let expected_flags = FlagsRegister {
        zero: true,
        subtract: false,
        half_carry: false,
        carry: false,
    };

    assert_eq!(expected_flags, cpu.registers.f);
}

#[test]
fn cpu_add_half_carry() {
    let mut cpu = CPU::default();

    cpu.registers.a = 0x0F;
    cpu.registers.c = 1;

    let pc = cpu.execute(Instruction::ADD(ArithmeticTarget::C));

    assert_eq!(0x10, cpu.registers.a);
    assert_eq!(1, cpu.registers.c);

    assert_eq!(1, pc);

    let expected_flags = FlagsRegister {
        zero: false,
        subtract: false,
        half_carry: true,
        carry: false,
    };

    assert_eq!(expected_flags, cpu.registers.f);
}

#[test]
fn cpu_add_carry() {
    let mut cpu = CPU::default();

    cpu.registers.a = 0xFF;
    cpu.registers.c = 2;

    let pc = cpu.execute(Instruction::ADD(ArithmeticTarget::C));

    assert_eq!(1, cpu.registers.a);
    assert_eq!(2, cpu.registers.c);

    assert_eq!(1, pc);

    let expected_flags = FlagsRegister {
        zero: false,
        subtract: false,
        half_carry: true,
        carry: true,
    };

    assert_eq!(expected_flags, cpu.registers.f);
}

#[test]
fn instruction_add_c_read_from_bus() {
    let mut bus = MemoryBus::default();
    bus.write_byte(0, 0x81);

    let actual = Instruction::read_from_bus(&bus, 0);

    let expected = Ok(Instruction::ADD(ArithmeticTarget::C));

    assert_eq!(expected, actual);
}

#[test]
fn instruction_jp_nz_read_from_bus() {
    let mut bus = MemoryBus::default();
    bus.write_byte(0, 0xC2);
    bus.write_byte(1, 0x34);
    bus.write_byte(2, 0x12);

    let actual = Instruction::read_from_bus(&bus, 0);

    let expected = Ok(Instruction::JP(JumpTest::NotZero, 0x1234));

    assert_eq!(expected, actual);
}
