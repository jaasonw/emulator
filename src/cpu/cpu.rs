pub struct CPU {
    // Reference: http://z80-heaven.wikidot.com/the-registers-and-memory
    // 8 Bit registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,

    // 16 Bit Registers
    pub pc: u16,
    pub sp: u16,
    pub hl: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
            hl: 0,
        }
    }

    // 16 Bit Registers that are made up of 8 bit registers
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }
}
