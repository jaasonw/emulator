// Reference: http://z80-heaven.wikidot.com/the-registers-and-memory

// flags
const Z_FLAG: u8 = 7; // Zero flag
const N_FLAG: u8 = 6; // Add/Subtract flag
const H_FLAG: u8 = 5; // Half carry flag
const C_FLAG: u8 = 4; // Carry flag

// 4.194304 MHz
pub const CPU_FREQUENCY: f64 = 4194304.0;

// #[derive(Default)]
pub struct CPU {
    // 8 Bit registers
    a: u8, // Accumulator
    b: u8, // General purpose
    c: u8, // General purpose
    d: u8, // General purpose
    e: u8, // General purpose
    f: u8, // Flags
    h: u8, // General purpose
    l: u8, // General purpose
    i: u8, // Interrupt vector
    r: u8, // Memory refresh

    // exists in the z80, unused in the gameboy
    ixh: u8, // Index register
    ixl: u8, // Index register
    iyh: u8, // Index register
    iyl: u8, // Index register

    // 16 Bit Registers
    pc: u16, // Program counter
    sp: u16, // Stack pointer
    hl: u16, // General purpose
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            i: 0,
            r: 0,
            ixh: 0,
            ixl: 0,
            iyh: 0,
            iyl: 0,
            pc: 0x0000,
            sp: 0xFFFE,
            hl: 0,
        }
    }
}

pub fn run() {
    let mut cpu: CPU = CPU::default();
    println!("Initial CPU state:");
    dump_registers(&cpu);

    cpu.set_a(0x01);
    cpu.set_b(0x02);
    cpu.set_c(0x03);
    cpu.set_d(0x04);
    cpu.set_e(0x05);
    cpu.set_f(0x06);
    cpu.set_h(0x07);
    cpu.set_l(0x08);
    cpu.set_af(0x1234);
    cpu.set_bc(0x5678);
    cpu.set_de(0x9ABC);
    cpu.set_hl(0xDEF0);
    cpu.set_ix(0x1234);
    cpu.set_iy(0x5678);

    println!("CPU state after setting registers:");
    dump_registers(&cpu);
}

pub fn dump_registers(cpu: &CPU) {
    println!("a: {:#04X}", cpu.get_a());
    println!("f: {:#04X}", cpu.get_f());
    println!("b: {:#04X}", cpu.get_b());
    println!("c: {:#04X}", cpu.get_c());
    println!("d: {:#04X}", cpu.get_d());
    println!("e: {:#04X}", cpu.get_e());
    println!("h: {:#04X}", cpu.get_h());
    println!("l: {:#04X}", cpu.get_l());
    println!("i: {:#06X}", cpu.get_i());
    println!("r: {:#06X}", cpu.get_r());
    println!("af: {:#06X}", cpu.get_af());
    println!("bc: {:#06X}", cpu.get_bc());
    println!("de: {:#06X}", cpu.get_de());
    println!("hl: {:#06X}", cpu.get_hl());
    println!("pc: {:#06X}", cpu.get_pc());
    println!("sp: {:#06X}", cpu.get_sp());
    // println!("ix: {:#06X}", cpu.get_ix());
    // println!("iy: {:#06X}", cpu.get_iy());
    // println!("ixh: {:#06X}", cpu.get_ixh());
    // println!("ixl: {:#06X}", cpu.get_ixl());
    // println!("iyh: {:#06X}", cpu.get_iyh());
    // println!("iyl: {:#06X}", cpu.get_iyl());
}


// the only instance of object-oriented programming you will see in this project
// purely because it is more convenient to get and set combined registers with
// .get_af() and .set_af() than it is to get and set the individual registers
impl CPU {
    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }

    pub fn get_f(&self) -> u8 {
        self.f
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }

    pub fn get_i(&self) -> u8 {
        self.i
    }

    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_ixh(&self) -> u8 {
        self.ixh
    }

    pub fn get_ixl(&self) -> u8 {
        self.ixl
    }

    pub fn get_iyh(&self) -> u8 {
        self.iyh
    }

    pub fn get_iyl(&self) -> u8 {
        self.iyl
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn get_hl(&self) -> u16 {
        self.hl
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn get_ix(&self) -> u16 {
        (self.ixh as u16) << 8 | self.ixl as u16
    }

    pub fn get_iy(&self) -> u16 {
        (self.iyh as u16) << 8 | self.iyl as u16
    }

    // setters
    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn set_f(&mut self, value: u8) {
        self.f = value;
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn set_i(&mut self, value: u8) {
        self.i = value;
    }

    pub fn set_r(&mut self, value: u8) {
        self.r = value;
    }

    pub fn set_ixh(&mut self, value: u8) {
        self.ixh = value;
    }

    pub fn set_ixl(&mut self, value: u8) {
        self.ixl = value;
    }

    pub fn set_iyh(&mut self, value: u8) {
        self.iyh = value;
    }

    pub fn set_iyl(&mut self, value: u8) {
        self.iyl = value;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = value as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.hl = value;
    }

    pub fn set_ix(&mut self, value: u16) {
        self.set_ixh((value >> 8) as u8);
        self.set_ixl(value as u8);
    }

    pub fn set_iy(&mut self, value: u16) {
        self.set_iyh((value >> 8) as u8);
        self.set_iyl(value as u8);
    }
}
