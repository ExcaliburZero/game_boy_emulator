extern crate game_boy_emulator;

#[test]
fn set_get_bc_register() {
    let mut registers = game_boy_emulator::cpu::Registers::default();

    registers.set_bc(23);

    assert_eq!(23, registers.get_bc());
}
