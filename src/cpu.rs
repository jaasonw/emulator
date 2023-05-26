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

    // 16 Bit Registers
    pc: u16, // Program counter
    sp: u16, // Stack pointer
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
            pc: 0x0000,
            sp: 0xFFFE,
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
}

pub fn print_flags(cpu: &CPU) {
    println!("z: {}", cpu.get_z_flag());
    println!("n: {}", cpu.get_n_flag());
    println!("h: {}", cpu.get_h_flag());
    println!("c: {}", cpu.get_c_flag());
}

// the only instance of object-oriented programming you will see in this project
// purely because it is more convenient to get and set combined registers with
// .get_af() and .set_af() than it is to get and set the individual registers
impl CPU {
    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn set_z_flag(&mut self, value: bool) {
        self.f = (self.f & !(1 << Z_FLAG)) | ((value as u8) << Z_FLAG);
    }

    pub fn set_n_flag(&mut self, value: bool) {
        self.f = (self.f & !(1 << N_FLAG)) | ((value as u8) << N_FLAG);
    }

    pub fn set_h_flag(&mut self, value: bool) {
        self.f = (self.f & !(1 << H_FLAG)) | ((value as u8) << H_FLAG);
    }

    pub fn set_c_flag(&mut self, value: bool) {
        self.f = (self.f & !(1 << C_FLAG)) | ((value as u8) << C_FLAG);
    }

    pub fn get_z_flag(&self) -> bool {
        (self.f & (1 << Z_FLAG)) != 0
    }

    pub fn get_n_flag(&self) -> bool {
        (self.f & (1 << N_FLAG)) != 0
    }

    pub fn get_h_flag(&self) -> bool {
        (self.f & (1 << H_FLAG)) != 0
    }

    pub fn get_c_flag(&self) -> bool {
        (self.f & (1 << C_FLAG)) != 0
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

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
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
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn get_register_8bit(&mut self, register: &str) -> u8 {
        match register {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            "e" => self.e,
            "f" => self.f,
            "h" => self.h,
            "l" => self.l,
            _ => panic!("Invalid register"),
        }
    }
    pub fn get_register_16bit(&mut self, register: &str) -> u16 {
        match register {
            "af" => self.get_af(),
            "bc" => self.get_bc(),
            "de" => self.get_de(),
            "hl" => self.get_hl(),
            "sp" => self.get_sp(),
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_register_8bit(&mut self, register: &str, value: u8) {
        match register {
            "a" => self.a = value,
            "b" => self.b = value,
            "c" => self.c = value,
            "d" => self.d = value,
            "e" => self.e = value,
            "f" => self.f = value,
            "h" => self.h = value,
            "l" => self.l = value,
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_register_16bit(&mut self, register: &str, value: u16) {
        match register {
            "af" => self.set_af(value),
            "bc" => self.set_bc(value),
            "de" => self.set_de(value),
            "hl" => self.set_hl(value),
            "sp" => self.set_sp(value),
            _ => panic!("Invalid register"),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_z_flag() {
        let mut cpu = CPU::default();
        cpu.set_z_flag(true);
        assert_eq!(cpu.get_z_flag(), true);
        cpu.set_z_flag(false);
        assert_eq!(cpu.get_z_flag(), false);
    }
    #[test]
    fn test_n_flag() {
        let mut cpu = CPU::default();
        cpu.set_n_flag(true);
        assert_eq!(cpu.get_n_flag(), true);
        cpu.set_n_flag(false);
        assert_eq!(cpu.get_n_flag(), false);
    }
    #[test]
    fn test_h_flag() {
        let mut cpu = CPU::default();
        cpu.set_h_flag(true);
        assert_eq!(cpu.get_h_flag(), true);
        cpu.set_h_flag(false);
        assert_eq!(cpu.get_h_flag(), false);
    }
    #[test]
    fn test_c_flag() {
        let mut cpu = CPU::default();
        cpu.set_c_flag(true);
        assert_eq!(cpu.get_c_flag(), true);
        cpu.set_c_flag(false);
        assert_eq!(cpu.get_c_flag(), false);
    }
    #[test]
    fn test_a_register() {
        let mut cpu = CPU::default();
        cpu.set_a(0x12);
        assert_eq!(cpu.get_a(), 0x12);
    }

    #[test]
    fn test_b_register() {
        let mut cpu = CPU::default();
        cpu.set_b(0x12);
        assert_eq!(cpu.get_b(), 0x12);
    }

    #[test]
    fn test_c_register() {
        let mut cpu = CPU::default();
        cpu.set_c(0x12);
        assert_eq!(cpu.get_c(), 0x12);
    }

    #[test]
    fn test_d_register() {
        let mut cpu = CPU::default();
        cpu.set_d(0x12);
        assert_eq!(cpu.get_d(), 0x12);
    }

    #[test]
    fn test_e_register() {
        let mut cpu = CPU::default();
        cpu.set_e(0x12);
        assert_eq!(cpu.get_e(), 0x12);
    }

    #[test]

    fn test_f_register() {
        let mut cpu = CPU::default();
        cpu.set_f(0x12);
        assert_eq!(cpu.get_f(), 0x12);
    }

    #[test]
    fn test_h_register() {
        let mut cpu = CPU::default();
        cpu.set_h(0x12);
        assert_eq!(cpu.get_h(), 0x12);
    }

    #[test]
    fn test_l_register() {
        let mut cpu = CPU::default();
        cpu.set_l(0x12);
        assert_eq!(cpu.get_l(), 0x12);
    }

    #[test]
    fn test_i_register() {
        let mut cpu = CPU::default();
        cpu.set_i(0x12);
        assert_eq!(cpu.get_i(), 0x12);
    }

    #[test]
    fn test_r_register() {
        let mut cpu = CPU::default();
        cpu.set_r(0x12);
        assert_eq!(cpu.get_r(), 0x12);
    }

    #[test]
    fn test_pc_register() {
        let mut cpu = CPU::default();
        cpu.set_pc(0x1234);
        assert_eq!(cpu.get_pc(), 0x1234);
    }

    #[test]
    fn test_sp_register() {
        let mut cpu = CPU::default();
        cpu.set_sp(0x1234);
        assert_eq!(cpu.get_sp(), 0x1234);
    }

    #[test]
    fn test_af_register() {
        let mut cpu = CPU::default();
        cpu.set_af(0x1234);
        assert_eq!(cpu.get_a(), 0x12);
        assert_eq!(cpu.get_f(), 0x34);
        assert_eq!(cpu.get_af(), 0x1234);
    }

    #[test]
    fn test_bc_register() {
        let mut cpu = CPU::default();
        cpu.set_bc(0x1234);
        assert_eq!(cpu.get_b(), 0x12);
        assert_eq!(cpu.get_c(), 0x34);
        assert_eq!(cpu.get_bc(), 0x1234);
    }

    #[test]
    fn test_de_register() {
        let mut cpu = CPU::default();
        cpu.set_de(0x1234);
        assert_eq!(cpu.get_d(), 0x12);
        assert_eq!(cpu.get_e(), 0x34);
        assert_eq!(cpu.get_de(), 0x1234);
    }

    #[test]
    fn test_hl_register() {
        let mut cpu = CPU::default();
        cpu.set_hl(0x1234);
        assert_eq!(cpu.get_h(), 0x12);
        assert_eq!(cpu.get_l(), 0x34);
        assert_eq!(cpu.get_hl(), 0x1234);
    }
}
