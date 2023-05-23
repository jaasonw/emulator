// // mod cpu;
use crate::cpu;
use crate::instruction;

pub struct Gameboy {
    pub cpu: cpu::CPU,
    pub ram: [u8; 0xFFFF],
}

pub fn create_gameboy() -> Gameboy {
    let mut gb = Gameboy {
        cpu: cpu::CPU::default(),
        ram: [0; 0xFFFF],
    };
    gb.ram[0xfffe] = 0x00;
    gb
}

// reads the byte at the current program counter
pub fn read_byte(gb: &mut Gameboy) -> u16 {
    let byte: u8 = gb.ram[gb.cpu.get_pc() as usize];
    gb.cpu.increment_pc();
    return byte.into();
}

pub fn step(gb: &mut Gameboy) -> i64 {
    let mut cycles = 0;
    // fetch
    let opcode = read_byte(gb);
    cycles += 4;
    // decode
    cycles += 2;
    // execute
    cycles += instruction::execute_instruction(gb, opcode);

    return cycles;
}
