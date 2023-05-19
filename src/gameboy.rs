// // mod cpu;
use crate::cpu;

pub struct Gameboy {
    cpu: cpu::CPU,
    pub ram: [u8; 0xFFFF],
}

impl Default for Gameboy {
    fn default() -> Self {
        Gameboy {
            cpu: cpu::CPU::default(),
            ram: [0; 0xFFFF],
        }
    }
}

impl Gameboy {
    pub fn run(&mut self) {
        // println!("gameboy: {:?}", self.ram[0xfffe]);
    }
}

pub fn step(gb: &mut &Gameboy) {

}
