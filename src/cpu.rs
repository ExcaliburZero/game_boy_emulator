#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub bus: MemoryBus,
}

impl CPU {
    pub fn step(&mut self) {
        let next_pc = match Instruction::read_from_bus(&self.bus, self.pc) {
            Ok(instruction) => self.execute(instruction),
            Err(msg) => {
                panic!("{}", msg)
            }
        };

        self.pc = next_pc;
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        let instruction_len = instruction.len();

        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(instruction_len)
                    }
                    _ => {
                        /* TODO: implement */
                        self.pc
                    }
                }
            }
            Instruction::JP(test, address) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition, address)
            }
            _ => {
                /* TODO: implement other instructions */
                self.pc
            }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn jump(&self, should_jump: bool, address: u16) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            // let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            // let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            // (most_significant_byte << 8) | least_significant_byte
            address
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }

    pub fn read_address(&self, address: u16) -> u16 {
        // Gameboy is little endian so read `address + 1` as most significant bit
        // and `address` as least significant bit
        let least_significant_byte = self.read_byte(address) as u16;
        let most_significant_byte = self.read_byte(address + 1) as u16;
        (most_significant_byte << 8) | least_significant_byte
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }
}

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        let b = self.b as u16;
        let c = self.c as u16;

        (b << 8) | c
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        let zero = if flag.zero { 1 } else { 0 };
        let subtract = if flag.subtract { 1 } else { 0 };
        let half_carry = if flag.half_carry { 1 } else { 0 };
        let carry = if flag.carry { 1 } else { 0 };

        zero << ZERO_FLAG_BYTE_POSITION
            | subtract << SUBTRACT_FLAG_BYTE_POSITION
            | half_carry << HALF_CARRY_FLAG_BYTE_POSITION
            | carry << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

/// A GameBoy (DMG) assembly instruction.
///
/// See the following link for a table of the instruction opcodes:
/// <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
    RLC(PrefixTarget),
    JP(JumpTest, u16),
}

impl Instruction {
    /// Reads the Instruction at the given address in the MemoryBus.
    ///
    /// If not a valid/known instruction then an error message is returned.
    pub fn read_from_bus(bus: &MemoryBus, pc: u16) -> Result<Instruction, String> {
        let mut instruction_byte = bus.read_byte(pc);

        let mut instruction_address = pc;
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_address += 1;
            instruction_byte = bus.read_byte(instruction_address);
        }

        if let Some(instruction) = Instruction::from_byte(bus, instruction_address, prefixed) {
            Ok(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            Err(format!("Unkown instruction found for: {}", description))
        }
    }

    fn from_byte(bus: &MemoryBus, pc: u16, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::read_prefixed(bus, pc)
        } else {
            Instruction::read_not_prefixed(bus, pc)
        }
    }

    fn read_prefixed(bus: &MemoryBus, pc: u16) -> Option<Instruction> {
        let byte = bus.read_byte(pc);
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn read_not_prefixed(bus: &MemoryBus, pc: u16) -> Option<Instruction> {
        let byte = bus.read_byte(pc);
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0xC2 => Some(Instruction::JP(JumpTest::NotZero, bus.read_address(pc + 1))),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    /// Returns the number of bytes that the instruction takes up. Useful for knowing how far to
    /// move the program counter after executing the instruction for non-jumping instructions.
    pub fn len(&self) -> u16 {
        use Instruction::*;

        match self {
            ADD(_) => 1,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Eq, PartialEq)]
pub enum IncDecTarget {
    BC,
    DE,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PrefixTarget {
    B,
}

#[derive(Debug, Eq, PartialEq)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
