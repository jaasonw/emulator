// Reference: http://z80-heaven.wikidot.com/the-registers-and-memory
#[derive(Default)]
struct CPU {
    // 8 Bit registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    i: u8,
    r: u8,
    ixh: u8,
    ixl: u8,
    iyh: u8,
    iyl: u8,

    // 16 Bit Registers
    pc: u16,
    sp: u16,
    hl: u16,
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

fn dump_registers(cpu: &CPU) {
    println!("a: {:X}", cpu.get_a());
    println!("b: {:X}", cpu.get_b());
    println!("c: {:X}", cpu.get_c());
    println!("d: {:X}", cpu.get_d());
    println!("e: {:X}", cpu.get_e());
    println!("f: {:X}", cpu.get_f());
    println!("h: {:X}", cpu.get_h());
    println!("l: {:X}", cpu.get_l());
    println!("i: {:X}", cpu.get_i());
    println!("r: {:X}", cpu.get_r());
    println!("ixh: {:X}", cpu.get_ixh());
    println!("ixl: {:X}", cpu.get_ixl());
    println!("iyh: {:X}", cpu.get_iyh());
    println!("iyl: {:X}", cpu.get_iyl());
    println!("pc: {:X}", cpu.get_pc());
    println!("sp: {:X}", cpu.get_sp());
    println!("af: {:X}", cpu.get_af());
    println!("bc: {:X}", cpu.get_bc());
    println!("de: {:X}", cpu.get_de());
    println!("hl: {:X}", cpu.get_hl());
    println!("ix: {:X}", cpu.get_ix());
    println!("iy: {:X}", cpu.get_iy());
}

impl CPU {
    fn get_a(&self) -> u8 {
        self.a
    }

    fn get_b(&self) -> u8 {
        self.b
    }

    fn get_c(&self) -> u8 {
        self.c
    }

    fn get_d(&self) -> u8 {
        self.d
    }

    fn get_e(&self) -> u8 {
        self.e
    }

    fn get_f(&self) -> u8 {
        self.f
    }

    fn get_h(&self) -> u8 {
        self.h
    }

    fn get_l(&self) -> u8 {
        self.l
    }

    fn get_i(&self) -> u8 {
        self.i
    }

    fn get_r(&self) -> u8 {
        self.r
    }

    fn get_ixh(&self) -> u8 {
        self.ixh
    }

    fn get_ixl(&self) -> u8 {
        self.ixl
    }

    fn get_iyh(&self) -> u8 {
        self.iyh
    }

    fn get_iyl(&self) -> u8 {
        self.iyl
    }

    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn get_sp(&self) -> u16 {
        self.sp
    }

    fn get_hl(&self) -> u16 {
        self.hl
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn get_ix(&self) -> u16 {
        (self.ixh as u16) << 8 | self.ixl as u16
    }

    fn get_iy(&self) -> u16 {
        (self.iyh as u16) << 8 | self.iyl as u16
    }

    // setters
    fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    fn set_f(&mut self, value: u8) {
        self.f = value;
    }

    fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    fn set_i(&mut self, value: u8) {
        self.i = value;
    }

    fn set_r(&mut self, value: u8) {
        self.r = value;
    }

    fn set_ixh(&mut self, value: u8) {
        self.ixh = value;
    }

    fn set_ixl(&mut self, value: u8) {
        self.ixl = value;
    }

    fn set_iyh(&mut self, value: u8) {
        self.iyh = value;
    }

    fn set_iyl(&mut self, value: u8) {
        self.iyl = value;
    }

    fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = value as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.hl = value;
    }

    fn set_ix(&mut self, value: u16) {
        self.ixh = (value >> 8) as u8;
        self.ixl = value as u8;
    }

    fn set_iy(&mut self, value: u16) {
        self.iyh = (value >> 8) as u8;
        self.iyl = value as u8;
    }
}
