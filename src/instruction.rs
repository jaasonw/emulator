use crate::{
    cpu::{self, dump_registers},
    gameboy,
};

pub fn execute_instruction(gb: &mut gameboy::Gameboy, opcode: u16) -> i64 {
    let mut cycles: i64 = 0;

    println!("Executing opcode: {:02X}", opcode);
    match opcode {
        0x00 => {
            // NOP
            cpu::dump_registers(&gb.cpu);
            // print ram
            println!("{:?}", gb.ram);
            panic!("NOP");
            // let _ = Command::new("pause").status();
            cycles += 4;
        }
        0x01 => {
            // LD BC n16
            cpu::dump_registers(&gb.cpu);
            // print ram
            println!("{:?}", gb.ram);
            panic!("NOP");
            cycles += 12;
        }
        0x02 => {
            // LD BC A
            let addr = gb.cpu.get_bc();
            gb.ram[addr as usize] = gb.cpu.get_a();
            cycles += 8;
        }
        0x03 => {
            // INC BC
            increment_16bit(gb, "bc");
            cycles += 8;
        }
        0x04 => {
            // INC B
            increment_8bit(gb, "b");
            cycles += 4;
        }
        0x05 => {
            // DEC B
            decrement_8bit(gb, "b");
            cycles += 4;
        }
        0x06 => {
            // LD B n8
            load_immediate_8bit(gb, "b");
            cycles += 8;
        }
        0x07 => {
            // RLCA
            cycles += 4;
        }
        0x08 => {
            // LD a16 SP
            cycles += 20;
        }
        0x09 => {
            // ADD HL BC
            cycles += 8;
        }
        0x0A => {
            // LD A BC
            cycles += 8;
        }
        0x0B => {
            // DEC BC
            decrement_16bit(gb, "bc");
            cycles += 8;
        }
        0x0C => {
            // INC C
            cycles += 4;
        }
        0x0D => {
            // DEC C
            decrement_8bit(gb, "c");
            cycles += 4;
        }
        0x0E => {
            // LD C n8
            load_immediate_8bit(gb, "c");
            cycles += 8;
        }
        0x0F => {
            // RRCA
            cycles += 4;
        }
        0x10 => {
            // STOP n8
            cycles += 4;
        }
        0x11 => {
            // LD DE n16
            cycles += 12;
        }
        0x12 => {
            // LD DE A
            let addr = gb.cpu.get_de();
            gb.ram[addr as usize] = gb.cpu.get_a();
            cycles += 8;
        }
        0x13 => {
            // INC DE
            increment_16bit(gb, "de");
            cycles += 8;
        }
        0x14 => {
            // INC D
            increment_8bit(gb, "d");
            cycles += 4;
        }
        0x15 => {
            // DEC D
            decrement_8bit(gb, "d");
            cycles += 4;
        }
        0x16 => {
            // LD D n8
            load_immediate_8bit(gb, "d");
            cycles += 8;
        }
        0x17 => {
            // RLA
            cycles += 4;
        }
        0x18 => {
            // JR e8
            let value = gb.cpu.get_pc() as i8;
            gb.cpu.increment_pc();
            jump_relative(gb, i16::try_from(value).unwrap());
            cycles += 12;
        }
        0x19 => {
            // ADD HL DE
            cycles += 8;
        }
        0x1A => {
            // LD A DE
            cycles += 8;
        }
        0x1B => {
            // DEC DE
            decrement_16bit(gb, "de");
            cycles += 8;
        }
        0x1C => {
            // INC E
            increment_8bit(gb, "e");
            cycles += 4;
        }
        0x1D => {
            // DEC E
            decrement_8bit(gb, "e");
            cycles += 4;
        }
        0x1E => {
            // LD E n8
            load_immediate_8bit(gb, "e");
            cycles += 8;
        }
        0x1F => {
            // RRA
            cycles += 4;
        }
        0x20 => {
            // JR NZ e8
            if !gb.cpu.get_z_flag() {
                let value = gameboy::read_byte(gb) as i8;
                println!("Jumping {} steps", value);
                let addr = jump_relative(gb, i16::try_from(value).unwrap());
                println!("Jumped to {:04X}", addr);
                cycles += 12;
            } else {
                cycles += 8;
            }
        }
        0x21 => {
            // LD HL n16
            let value = gameboy::read_short(gb);
            gb.cpu.set_hl(value);
            cycles += 12;
        }
        0x22 => {
            // LD HL+ A
            cycles += 8;
        }
        0x23 => {
            // INC HL
            increment_16bit(gb, "hl");
            cycles += 8;
        }
        0x24 => {
            // INC H
            increment_8bit(gb, "h");
            cycles += 4;
        }
        0x25 => {
            // DEC H
            decrement_8bit(gb, "h");
            cycles += 4;
        }
        0x26 => {
            // LD H n8
            load_immediate_8bit(gb, "h");
            cycles += 8;
        }
        0x27 => {
            // DAA
            cycles += 4;
        }
        0x28 => {
            // JR Z e8
            if gb.cpu.get_z_flag() {
                let value = gameboy::read_byte(gb) as i8;
                jump_relative(gb, i16::try_from(value).unwrap());
                cycles += 12;
            } else {
                cycles += 8;
            }
        }
        0x29 => {
            // ADD HL HL
            cycles += 8;
        }
        0x2A => {
            // LD A HL+
            // Load the contents of memory specified by register pair HL into register A, and then increment the contents of HL.
            let addr = gb.cpu.get_hl();
            gb.cpu.set_a(gb.ram[addr as usize]);
            increment_16bit(gb, "hl");
            cycles += 8;
        }
        0x2B => {
            // DEC HL
            decrement_16bit(gb, "hl");
            cycles += 8;
        }
        0x2C => {
            // INC L
            increment_8bit(gb, "l");
            cycles += 4;
        }
        0x2D => {
            // DEC L
            decrement_8bit(gb, "l");
            cycles += 4;
        }
        0x2E => {
            // LD L n8
            load_immediate_8bit(gb, "l");
            cycles += 8;
        }
        0x2F => {
            // CPL
            cycles += 4;
        }
        0x30 => {
            // JR NC e8
            if !gb.cpu.get_c_flag() {
                let value = gameboy::read_byte(gb) as i8;
                jump_relative(gb, i16::try_from(value).unwrap());
                cycles += 12;
            } else {
                cycles += 8;
            }
        }
        0x31 => {
            // LD SP n16
            let value = gameboy::read_short(gb);
            load_16bit(gb, "sp", value);
            cycles += 12;
        }
        0x32 => {
            // LD HL- A
            // Load register A to the memory address pointed to by HL and then decrement the value of HL
            let address = gb.cpu.get_hl();
            println!("address: {:?}", address);
            gb.ram[address as usize] = gb.cpu.get_a();
            let hl = gb.cpu.get_hl();
            load_16bit(gb, "hl", hl - 1);
            cycles += 8;
        }
        0x33 => {
            // INC SP
            increment_16bit(gb, "sp");
            cycles += 8;
        }
        0x34 => {
            // INC HL
            increment_16bit(gb, "hl");
            cycles += 12;
        }
        0x35 => {
            // DEC HL
            decrement_16bit(gb, "hl");
            cycles += 12;
        }
        0x36 => {
            // LD HL n8
            // Store the contents of 8-bit immediate operand d8 in the memory location specified by register pair HL.
            let value = gameboy::read_byte(gb) as u8;
            let hl = gb.cpu.get_hl();
            gb.ram[hl as usize] = value;
            cycles += 12;
        }
        0x37 => {
            // SCF
            cycles += 4;
        }
        0x38 => {
            // JR C e8
            if gb.cpu.get_c_flag() {
                let value = gameboy::read_byte(gb) as i8;
                jump_relative(gb, i16::try_from(value).unwrap());
                cycles += 12;
            } else {
                cycles += 8;
            }
        }
        0x39 => {
            // ADD HL SP
            cycles += 8;
        }
        0x3A => {
            // LD A HL-
            // Load the contents of memory specified by register pair HL into register A, and then decrement the contents of HL.
            let addr = gb.cpu.get_hl();
            gb.cpu.set_a(gb.ram[addr as usize]);
            decrement_16bit(gb, "hl");
            cycles += 8;
        }
        0x3B => {
            // DEC SP
            decrement_16bit(gb, "sp");
            cycles += 8;
        }
        0x3C => {
            // INC A
            increment_8bit(gb, "a");
            cycles += 4;
        }
        0x3D => {
            // DEC A
            decrement_8bit(gb, "a");
            cycles += 4;
        }
        0x3E => {
            // LD A n8
            load_immediate_8bit(gb, "a");
            cycles += 8;
        }
        0x3F => {
            // CCF
            cycles += 4;
        }
        0x40 => {
            // LD B B
            load_register_8bit(gb, "b", "b");
            cycles += 4;
        }
        0x41 => {
            // LD B C
            load_register_8bit(gb, "b", "c");
            cycles += 4;
        }
        0x42 => {
            // LD B D
            load_register_8bit(gb, "b", "d");
            cycles += 4;
        }
        0x43 => {
            // LD B E
            load_register_8bit(gb, "b", "e");
            cycles += 4;
        }
        0x44 => {
            // LD B H
            load_register_8bit(gb, "b", "h");
            cycles += 4;
        }
        0x45 => {
            // LD B L
            load_register_8bit(gb, "b", "l");
            cycles += 4;
        }
        0x46 => {
            // LD B HL
            let addr = gb.cpu.get_hl();
            let value = gb.ram[addr as usize];
            load_8bit(gb, "b", value);
            cycles += 8;
        }
        0x47 => {
            // LD B A
            load_register_8bit(gb, "b", "a");
            cycles += 4;
        }
        0x48 => {
            // LD C B
            load_register_8bit(gb, "c", "b");
            cycles += 4;
        }
        0x49 => {
            // LD C C
            load_register_8bit(gb, "c", "c");
            cycles += 4;
        }
        0x4A => {
            // LD C D
            load_register_8bit(gb, "c", "d");
            cycles += 4;
        }
        0x4B => {
            // LD C E
            load_register_8bit(gb, "c", "e");
            cycles += 4;
        }
        0x4C => {
            // LD C H
            load_register_8bit(gb, "c", "h");
            cycles += 4;
        }
        0x4D => {
            // LD C L
            load_register_8bit(gb, "c", "l");
            cycles += 4;
        }
        0x4E => {
            // LD C HL
            let addr = gb.cpu.get_hl();
            let value = gb.ram[addr as usize];
            load_8bit(gb, "c", value);
            cycles += 8;
        }
        0x4F => {
            // LD C A
            load_register_8bit(gb, "c", "a");
            cycles += 4;
        }
        0x50 => {
            // LD D B
            load_register_8bit(gb, "d", "b");
            cycles += 4;
        }
        0x51 => {
            // LD D C
            load_register_8bit(gb, "d", "c");
            cycles += 4;
        }
        0x52 => {
            // LD D D
            load_register_8bit(gb, "d", "d");
            cycles += 4;
        }
        0x53 => {
            // LD D E
            load_register_8bit(gb, "d", "e");
            cycles += 4;
        }
        0x54 => {
            // LD D H
            load_register_8bit(gb, "d", "h");
            cycles += 4;
        }
        0x55 => {
            // LD D L
            load_register_8bit(gb, "d", "l");
            cycles += 4;
        }
        0x56 => {
            // LD D HL
            let addr = gb.cpu.get_hl();
            let value = gb.ram[addr as usize];
            load_8bit(gb, "d", value);
            cycles += 8;
        }
        0x57 => {
            // LD D A
            load_register_8bit(gb, "d", "a");
            cycles += 4;
        }
        0x58 => {
            // LD E B
            load_register_8bit(gb, "e", "b");
            cycles += 4;
        }
        0x59 => {
            // LD E C
            load_register_8bit(gb, "e", "c");
            cycles += 4;
        }
        0x5A => {
            // LD E D
            load_register_8bit(gb, "e", "d");
            cycles += 4;
        }
        0x5B => {
            // LD E E
            load_register_8bit(gb, "e", "e");
            cycles += 4;
        }
        0x5C => {
            // LD E H
            load_register_8bit(gb, "e", "h");
            cycles += 4;
        }
        0x5D => {
            // LD E L
            load_register_8bit(gb, "e", "l");
            cycles += 4;
        }
        0x5E => {
            // LD E HL
            let addr = gb.cpu.get_hl();
            let value = gb.ram[addr as usize];
            load_8bit(gb, "e", value);
            cycles += 8;
        }
        0x5F => {
            // LD E A
            load_register_8bit(gb, "e", "a");
            cycles += 4;
        }
        0x60 => {
            // LD H B
            load_register_8bit(gb, "h", "b");
            cycles += 4;
        }
        0x61 => {
            // LD H C
            load_register_8bit(gb, "h", "c");
            cycles += 4;
        }
        0x62 => {
            // LD H D
            load_register_8bit(gb, "h", "d");
            cycles += 4;
        }
        0x63 => {
            // LD H E
            load_register_8bit(gb, "h", "e");
            cycles += 4;
        }
        0x64 => {
            // LD H H
            load_register_8bit(gb, "h", "h");
            cycles += 4;
        }
        0x65 => {
            // LD H L
            load_register_8bit(gb, "h", "l");
            cycles += 4;
        }
        0x66 => {
            // LD H HL
            let addr = gb.cpu.get_hl();
            let value = gb.ram[addr as usize];
            load_8bit(gb, "h", value);
            cycles += 8;
        }
        0x67 => {
            // LD H A
            load_register_8bit(gb, "h", "a");
            cycles += 4;
        }
        0x68 => {
            // LD L B
            load_register_8bit(gb, "l", "b");
            cycles += 4;
        }
        0x69 => {
            // LD L C
            load_register_8bit(gb, "l", "c");
            cycles += 4;
        }
        0x6A => {
            // LD L D
            load_register_8bit(gb, "l", "d");
            cycles += 4;
        }
        0x6B => {
            // LD L E
            load_register_8bit(gb, "l", "e");
            cycles += 4;
        }
        0x6C => {
            // LD L H
            load_register_8bit(gb, "l", "h");
            cycles += 4;
        }
        0x6D => {
            // LD L L
            load_register_8bit(gb, "l", "l");
            cycles += 4;
        }
        0x6E => {
            // LD L HL
            let addr = gb.cpu.get_hl();
            let value = gb.ram[addr as usize];
            load_8bit(gb, "l", value);
            cycles += 8;
        }
        0x6F => {
            // LD L A
            load_register_8bit(gb, "l", "a");
            cycles += 4;
        }
        0x70 => {
            // LD HL B
            // Store the contents of register B in the memory location specified by register pair HL
            let addr = gb.cpu.get_hl();
            let value = gb.cpu.get_b();
            gb.ram[addr as usize] = value;
            cycles += 8;
        }
        0x71 => {
            // LD HL C
            let addr = gb.cpu.get_hl();
            let value = gb.cpu.get_c();
            gb.ram[addr as usize] = value;
            cycles += 8;
        }
        0x72 => {
            // LD HL D
            let addr = gb.cpu.get_hl();
            let value = gb.cpu.get_d();
            gb.ram[addr as usize] = value;
            cycles += 8;
        }
        0x73 => {
            // LD HL E
            let addr = gb.cpu.get_hl();
            let value = gb.cpu.get_e();
            gb.ram[addr as usize] = value;
            cycles += 8;
        }
        0x74 => {
            // LD HL H
            let addr = gb.cpu.get_hl();
            let value = gb.cpu.get_h();
            gb.ram[addr as usize] = value;
            cycles += 8;
        }
        0x75 => {
            // LD HL L
            let addr = gb.cpu.get_hl();
            let value = gb.cpu.get_l();
            gb.ram[addr as usize] = value;
            cycles += 8;
        }
        0x76 => {
            // HALT
            cycles += 4;
        }
        0x77 => {
            // LD HL A
            let addr = gb.cpu.get_hl();
            let value = gb.cpu.get_a();
            gb.ram[addr as usize] = value;
            cycles += 8;
        }
        0x78 => {
            // LD A B
            load_register_8bit(gb, "a", "b");
            cycles += 4;
        }
        0x79 => {
            // LD A C
            load_register_8bit(gb, "a", "c");
            cycles += 4;
        }
        0x7A => {
            // LD A D
            load_register_8bit(gb, "a", "d");
            cycles += 4;
        }
        0x7B => {
            // LD A E
            load_register_8bit(gb, "a", "e");
            cycles += 4;
        }
        0x7C => {
            // LD A H
            load_register_8bit(gb, "a", "h");
            cycles += 4;
        }
        0x7D => {
            // LD A L
            load_register_8bit(gb, "a", "l");
            cycles += 4;
        }
        0x7E => {
            // LD A HL
            let addr = gb.cpu.get_hl();
            let value = gb.ram[addr as usize];
            load_8bit(gb, "a", value);
            cycles += 8;
        }
        0x7F => {
            // LD A A
            load_register_8bit(gb, "a", "a");
            cycles += 4;
        }
        0x80 => {
            // ADD A B
            cycles += 4;
        }
        0x81 => {
            // ADD A C
            cycles += 4;
        }
        0x82 => {
            // ADD A D
            cycles += 4;
        }
        0x83 => {
            // ADD A E
            cycles += 4;
        }
        0x84 => {
            // ADD A H
            cycles += 4;
        }
        0x85 => {
            // ADD A L
            cycles += 4;
        }
        0x86 => {
            // ADD A HL
            cycles += 8;
        }
        0x87 => {
            // ADD A A
            cycles += 4;
        }
        0x88 => {
            // ADC A B
            cycles += 4;
        }
        0x89 => {
            // ADC A C
            cycles += 4;
        }
        0x8A => {
            // ADC A D
            cycles += 4;
        }
        0x8B => {
            // ADC A E
            cycles += 4;
        }
        0x8C => {
            // ADC A H
            cycles += 4;
        }
        0x8D => {
            // ADC A L
            cycles += 4;
        }
        0x8E => {
            // ADC A HL
            cycles += 8;
        }
        0x8F => {
            // ADC A A
            cycles += 4;
        }
        0x90 => {
            // SUB B
            cycles += 4;
        }
        0x91 => {
            // SUB C
            cycles += 4;
        }
        0x92 => {
            // SUB D
            cycles += 4;
        }
        0x93 => {
            // SUB E
            cycles += 4;
        }
        0x94 => {
            // SUB H
            cycles += 4;
        }
        0x95 => {
            // SUB L
            cycles += 4;
        }
        0x96 => {
            // SUB HL
            cycles += 8;
        }
        0x97 => {
            // SUB A
            cycles += 4;
        }
        0x98 => {
            // SBC A B
            cycles += 4;
        }
        0x99 => {
            // SBC A C
            cycles += 4;
        }
        0x9A => {
            // SBC A D
            cycles += 4;
        }
        0x9B => {
            // SBC A E
            cycles += 4;
        }
        0x9C => {
            // SBC A H
            cycles += 4;
        }
        0x9D => {
            // SBC A L
            cycles += 4;
        }
        0x9E => {
            // SBC A HL
            cycles += 8;
        }
        0x9F => {
            // SBC A A
            cycles += 4;
        }
        0xA0 => {
            // AND B
            cycles += 4;
        }
        0xA1 => {
            // AND C
            cycles += 4;
        }
        0xA2 => {
            // AND D
            cycles += 4;
        }
        0xA3 => {
            // AND E
            cycles += 4;
        }
        0xA4 => {
            // AND H
            cycles += 4;
        }
        0xA5 => {
            // AND L
            cycles += 4;
        }
        0xA6 => {
            // AND HL
            cycles += 8;
        }
        0xA7 => {
            // AND A
            cycles += 4;
        }
        0xA8 => {
            // XOR B
            cycles += 4;
        }
        0xA9 => {
            // XOR C
            cycles += 4;
        }
        0xAA => {
            // XOR D
            cycles += 4;
        }
        0xAB => {
            // XOR E
            cycles += 4;
        }
        0xAC => {
            // XOR H
            xor(gb, "h");
            cycles += 4;
        }
        0xAD => {
            // XOR L
            xor(gb, "l");
            cycles += 4;
        }
        0xAE => {
            // XOR HL
            xor(gb, "h");
            xor(gb, "l");
            cycles += 8;
        }
        0xAF => {
            // XOR A
            // gb.cpu.set_a(0);
            xor(gb, "a");
            cycles += 4;
        }
        0xB0 => {
            // OR B
            cycles += 4;
        }
        0xB1 => {
            // OR C
            cycles += 4;
        }
        0xB2 => {
            // OR D
            cycles += 4;
        }
        0xB3 => {
            // OR E
            cycles += 4;
        }
        0xB4 => {
            // OR H
            cycles += 4;
        }
        0xB5 => {
            // OR L
            cycles += 4;
        }
        0xB6 => {
            // OR HL
            cycles += 8;
        }
        0xB7 => {
            // OR A
            cycles += 4;
        }
        0xB8 => {
            // CP B
            cycles += 4;
        }
        0xB9 => {
            // CP C
            cycles += 4;
        }
        0xBA => {
            // CP D
            cycles += 4;
        }
        0xBB => {
            // CP E
            cycles += 4;
        }
        0xBC => {
            // CP H
            cycles += 4;
        }
        0xBD => {
            // CP L
            cycles += 4;
        }
        0xBE => {
            // CP HL
            cycles += 8;
        }
        0xBF => {
            // CP A
            cycles += 4;
        }
        0xC0 => {
            // RET NZ
            // if branch
            cycles += 20;
            // if not branch
            cycles += 8;
        }
        0xC1 => {
            // POP BC
            cycles += 12;
        }
        0xC2 => {
            // JP NZ a16
            // if branch
            cycles += 16;
            // if not branch
            cycles += 12;
        }
        0xC3 => {
            // JP a16
            cycles += 16;
        }
        0xC4 => {
            // CALL NZ a16
            // if branch
            cycles += 24;
            // if not branch
            cycles += 12;
        }
        0xC5 => {
            // PUSH BC
            cycles += 16;
        }
        0xC6 => {
            // ADD A n8
            cycles += 8;
        }
        0xC7 => {
            // RST 00H
            cycles += 16;
        }
        0xC8 => {
            // RET Z
            // if branch
            cycles += 20;
            // if not branch
            cycles += 8;
        }
        0xC9 => {
            // RET
            cycles += 16;
        }
        0xCA => {
            // JP Z a16
            // if branch
            cycles += 16;
            // if not branch
            cycles += 12;
        }
        0xCB => {
            // PREFIX
            cycles += 4;
            let opcode = gameboy::read_byte(gb);
            cycles += execute_prefixed(gb, opcode);
        }
        0xCC => {
            // CALL Z a16
            // if branch
            cycles += 24;
            // if not branch
            cycles += 12;
        }
        0xCD => {
            // CALL a16
            cycles += 24;
        }
        0xCE => {
            // ADC A n8
            cycles += 8;
        }
        0xCF => {
            // RST 08H
            cycles += 16;
        }
        0xD0 => {
            // RET NC
            // if branch
            cycles += 20;
            // if not branch
            cycles += 8;
        }
        0xD1 => {
            // POP DE
            cycles += 12;
        }
        0xD2 => {
            // JP NC a16
            // if branch
            cycles += 16;
            // if not branch
            cycles += 12;
        }
        0xD3 => {
            // ILLEGAL_D3
            cycles += 4;
        }
        0xD4 => {
            // CALL NC a16
            // if branch
            cycles += 24;
            // if not branch
            cycles += 12;
        }
        0xD5 => {
            // PUSH DE
            cycles += 16;
        }
        0xD6 => {
            // SUB n8
            cycles += 8;
        }
        0xD7 => {
            // RST 10H
            cycles += 16;
        }
        0xD8 => {
            // RET C
            // if branch
            cycles += 20;
            // if not branch
            cycles += 8;
        }
        0xD9 => {
            // RETI
            cycles += 16;
        }
        0xDA => {
            // JP C a16
            // if branch
            cycles += 16;
            // if not branch
            cycles += 12;
        }
        0xDB => {
            // ILLEGAL_DB
            cycles += 4;
        }
        0xDC => {
            // CALL C a16
            // if branch
            cycles += 24;
            // if not branch
            cycles += 12;
        }
        0xDD => {
            // ILLEGAL_DD
            cycles += 4;
        }
        0xDE => {
            // SBC A n8
            cycles += 8;
        }
        0xDF => {
            // RST 18H
            cycles += 16;
        }
        0xE0 => {
            // LDH a8 A
            cycles += 12;
        }
        0xE1 => {
            // POP HL
            cycles += 12;
        }
        0xE2 => {
            // LD C A
            // Load to the address specified by the 8-bit C register, data from the 8-bit A register.
            let addr = gb.cpu.get_c();
            let data = gb.cpu.get_a();
            gb.ram[addr as usize] = data;
            cycles += 8;
        }
        0xE3 => {
            // ILLEGAL_E3
            cycles += 4;
        }
        0xE4 => {
            // ILLEGAL_E4
            cycles += 4;
        }
        0xE5 => {
            // PUSH HL
            cycles += 16;
        }
        0xE6 => {
            // AND n8
            cycles += 8;
        }
        0xE7 => {
            // RST 20H
            cycles += 16;
        }
        0xE8 => {
            // ADD SP e8
            cycles += 16;
        }
        0xE9 => {
            // JP HL
            cycles += 4;
        }
        0xEA => {
            // LD a16 A
            // Store the contents of register A in the internal RAM or register specified by the 16-bit immediate operand a16.
            // Load to the absolute address specified by the 16-bit operand nn, data from the 8-bit A register.
            let a16 = gameboy::read_short(gb);
            gb.ram[a16 as usize] = gb.cpu.get_a();
            cycles += 16;
        }
        0xEB => {
            // ILLEGAL_EB
            cycles += 4;
        }
        0xEC => {
            // ILLEGAL_EC
            cycles += 4;
        }
        0xED => {
            // ILLEGAL_ED
            cycles += 4;
        }
        0xEE => {
            // XOR n8
            cycles += 8;
        }
        0xEF => {
            // RST 28H
            cycles += 16;
        }
        0xF0 => {
            // LDH A a8
            cycles += 12;
        }
        0xF1 => {
            // POP AF
            cycles += 12;
        }
        0xF2 => {
            // LD A C
            // Load to the 8-bit A register, data from the address specified by the 8-bit C register.
            let addr = gb.cpu.get_c();
            let data = gb.ram[addr as usize];
            gb.cpu.set_a(data);
            cycles += 8;
        }
        0xF3 => {
            // DI
            cycles += 4;
        }
        0xF4 => {
            // ILLEGAL_F4
            cycles += 4;
        }
        0xF5 => {
            // PUSH AF
            cycles += 16;
        }
        0xF6 => {
            // OR n8
            cycles += 8;
        }
        0xF7 => {
            // RST 30H
            cycles += 16;
        }
        0xF8 => {
            // LD HL SP e8
            cycles += 12;
        }
        0xF9 => {
            // LD SP HL
            cycles += 8;
        }
        0xFA => {
            // LD A a16
            // Load to the 8-bit A register, data from the absolute address specified by the 16-bit operand nn.
            let a16 = gameboy::read_short(gb);
            gb.cpu.set_a(gb.ram[a16 as usize]);
            cycles += 16;
        }
        0xFB => {
            // EI
            cycles += 4;
        }
        0xFC => {
            // ILLEGAL_FC
            cycles += 4;
        }
        0xFD => {
            // ILLEGAL_FD
            cycles += 4;
        }
        0xFE => {
            // CP n8
            cycles += 8;
        }
        0xFF => {
            // RST 38H
            cycles += 16;
        }
        _ => {
            println!("Unknown opcode: {:#04X}", opcode);
        }
    }

    return cycles;
}

fn execute_prefixed(gb: &mut gameboy::Gameboy, opcode: u16) -> i64 {
    let mut cycles: i64 = 0;
    match opcode {
        0x00 => {
            // RLC B
            cycles += 8;
        }
        0x01 => {
            // RLC C
            cycles += 8;
        }
        0x02 => {
            // RLC D
            cycles += 8;
        }
        0x03 => {
            // RLC E
            cycles += 8;
        }
        0x04 => {
            // RLC H
            cycles += 8;
        }
        0x05 => {
            // RLC L
            cycles += 8;
        }
        0x06 => {
            // RLC HL
            cycles += 16;
        }
        0x07 => {
            // RLC A
            cycles += 8;
        }
        0x08 => {
            // RRC B
            cycles += 8;
        }
        0x09 => {
            // RRC C
            cycles += 8;
        }
        0x0A => {
            // RRC D
            cycles += 8;
        }
        0x0B => {
            // RRC E
            cycles += 8;
        }
        0x0C => {
            // RRC H
            cycles += 8;
        }
        0x0D => {
            // RRC L
            cycles += 8;
        }
        0x0E => {
            // RRC HL
            cycles += 16;
        }
        0x0F => {
            // RRC A
            cycles += 8;
        }
        0x10 => {
            // RL B
            cycles += 8;
        }
        0x11 => {
            // RL C
            cycles += 8;
        }
        0x12 => {
            // RL D
            cycles += 8;
        }
        0x13 => {
            // RL E
            cycles += 8;
        }
        0x14 => {
            // RL H
            cycles += 8;
        }
        0x15 => {
            // RL L
            cycles += 8;
        }
        0x16 => {
            // RL HL
            cycles += 16;
        }
        0x17 => {
            // RL A
            cycles += 8;
        }
        0x18 => {
            // RR B
            cycles += 8;
        }
        0x19 => {
            // RR C
            cycles += 8;
        }
        0x1A => {
            // RR D
            cycles += 8;
        }
        0x1B => {
            // RR E
            cycles += 8;
        }
        0x1C => {
            // RR H
            cycles += 8;
        }
        0x1D => {
            // RR L
            cycles += 8;
        }
        0x1E => {
            // RR HL
            cycles += 16;
        }
        0x1F => {
            // RR A
            cycles += 8;
        }
        0x20 => {
            // SLA B
            cycles += 8;
        }
        0x21 => {
            // SLA C
            cycles += 8;
        }
        0x22 => {
            // SLA D
            cycles += 8;
        }
        0x23 => {
            // SLA E
            cycles += 8;
        }
        0x24 => {
            // SLA H
            cycles += 8;
        }
        0x25 => {
            // SLA L
            cycles += 8;
        }
        0x26 => {
            // SLA HL
            cycles += 16;
        }
        0x27 => {
            // SLA A
            cycles += 8;
        }
        0x28 => {
            // SRA B
            cycles += 8;
        }
        0x29 => {
            // SRA C
            cycles += 8;
        }
        0x2A => {
            // SRA D
            cycles += 8;
        }
        0x2B => {
            // SRA E
            cycles += 8;
        }
        0x2C => {
            // SRA H
            cycles += 8;
        }
        0x2D => {
            // SRA L
            cycles += 8;
        }
        0x2E => {
            // SRA HL
            cycles += 16;
        }
        0x2F => {
            // SRA A
            cycles += 8;
        }
        0x30 => {
            // SWAP B
            cycles += 8;
        }
        0x31 => {
            // SWAP C
            cycles += 8;
        }
        0x32 => {
            // SWAP D
            cycles += 8;
        }
        0x33 => {
            // SWAP E
            cycles += 8;
        }
        0x34 => {
            // SWAP H
            cycles += 8;
        }
        0x35 => {
            // SWAP L
            cycles += 8;
        }
        0x36 => {
            // SWAP HL
            cycles += 16;
        }
        0x37 => {
            // SWAP A
            cycles += 8;
        }
        0x38 => {
            // SRL B
            cycles += 8;
        }
        0x39 => {
            // SRL C
            cycles += 8;
        }
        0x3A => {
            // SRL D
            cycles += 8;
        }
        0x3B => {
            // SRL E
            cycles += 8;
        }
        0x3C => {
            // SRL H
            cycles += 8;
        }
        0x3D => {
            // SRL L
            cycles += 8;
        }
        0x3E => {
            // SRL HL
            cycles += 16;
        }
        0x3F => {
            // SRL A
            cycles += 8;
        }
        0x40 => {
            // BIT 0 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(0, gb.cpu.get_b()));
            cycles += 8;
        }
        0x41 => {
            // BIT 0 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(0, gb.cpu.get_c()));
            cycles += 8;
        }
        0x42 => {
            // BIT 0 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(0, gb.cpu.get_d()));
            cycles += 8;
        }
        0x43 => {
            // BIT 0 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(0, gb.cpu.get_e()));
            cycles += 8;
        }
        0x44 => {
            // BIT 0 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(0, gb.cpu.get_h()));
            cycles += 8;
        }
        0x45 => {
            // BIT 0 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(0, gb.cpu.get_l()));
            cycles += 8;
        }
        0x46 => {
            // BIT 0 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(0, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x47 => {
            // BIT 0 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(0, gb.cpu.get_a()));
            cycles += 8;
        }
        0x48 => {
            // BIT 1 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(1, gb.cpu.get_b()));
            cycles += 8;
        }
        0x49 => {
            // BIT 1 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(1, gb.cpu.get_c()));
            cycles += 8;
        }
        0x4A => {
            // BIT 1 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(1, gb.cpu.get_d()));
            cycles += 8;
        }
        0x4B => {
            // BIT 1 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(1, gb.cpu.get_e()));
            cycles += 8;
        }
        0x4C => {
            // BIT 1 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(1, gb.cpu.get_h()));
            cycles += 8;
        }
        0x4D => {
            // BIT 1 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(1, gb.cpu.get_l()));
            cycles += 8;
        }
        0x4E => {
            // BIT 1 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(1, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x4F => {
            // BIT 1 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(1, gb.cpu.get_a()));
            cycles += 8;
        }
        0x50 => {
            // BIT 2 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(2, gb.cpu.get_b()));
            cycles += 8;
        }
        0x51 => {
            // BIT 2 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(2, gb.cpu.get_c()));
            cycles += 8;
        }
        0x52 => {
            // BIT 2 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(2, gb.cpu.get_d()));
            cycles += 8;
        }
        0x53 => {
            // BIT 2 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(2, gb.cpu.get_e()));
            cycles += 8;
        }
        0x54 => {
            // BIT 2 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(2, gb.cpu.get_h()));
            cycles += 8;
        }
        0x55 => {
            // BIT 2 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(2, gb.cpu.get_l()));
            cycles += 8;
        }
        0x56 => {
            // BIT 2 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(2, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x57 => {
            // BIT 2 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(2, gb.cpu.get_a()));
            cycles += 8;
        }
        0x58 => {
            // BIT 3 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(3, gb.cpu.get_b()));
            cycles += 8;
        }
        0x59 => {
            // BIT 3 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(3, gb.cpu.get_c()));
            cycles += 8;
        }
        0x5A => {
            // BIT 3 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(3, gb.cpu.get_d()));
            cycles += 8;
        }
        0x5B => {
            // BIT 3 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(3, gb.cpu.get_e()));
            cycles += 8;
        }
        0x5C => {
            // BIT 3 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(3, gb.cpu.get_h()));
            cycles += 8;
        }
        0x5D => {
            // BIT 3 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(3, gb.cpu.get_l()));
            cycles += 8;
        }
        0x5E => {
            // BIT 3 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(3, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x5F => {
            // BIT 3 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(3, gb.cpu.get_a()));
            cycles += 8;
        }
        0x60 => {
            // BIT 4 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(4, gb.cpu.get_b()));
            cycles += 8;
        }
        0x61 => {
            // BIT 4 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(4, gb.cpu.get_c()));
            cycles += 8;
        }
        0x62 => {
            // BIT 4 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(4, gb.cpu.get_d()));
            cycles += 8;
        }
        0x63 => {
            // BIT 4 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(4, gb.cpu.get_e()));
            cycles += 8;
        }
        0x64 => {
            // BIT 4 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(4, gb.cpu.get_h()));
            cycles += 8;
        }
        0x65 => {
            // BIT 4 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(4, gb.cpu.get_l()));
            cycles += 8;
        }
        0x66 => {
            // BIT 4 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(4, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x67 => {
            // BIT 4 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(4, gb.cpu.get_a()));
            cycles += 8;
        }
        0x68 => {
            // BIT 5 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(5, gb.cpu.get_b()));
            cycles += 8;
        }
        0x69 => {
            // BIT 5 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(5, gb.cpu.get_c()));
            cycles += 8;
        }
        0x6A => {
            // BIT 5 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(5, gb.cpu.get_d()));
            cycles += 8;
        }
        0x6B => {
            // BIT 5 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(5, gb.cpu.get_e()));
            cycles += 8;
        }
        0x6C => {
            // BIT 5 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(5, gb.cpu.get_h()));
            cycles += 8;
        }
        0x6D => {
            // BIT 5 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(5, gb.cpu.get_l()));
            cycles += 8;
        }
        0x6E => {
            // BIT 5 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(5, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x6F => {
            // BIT 5 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(5, gb.cpu.get_a()));
            cycles += 8;
        }
        0x70 => {
            // BIT 6 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(6, gb.cpu.get_b()));
            cycles += 8;
        }
        0x71 => {
            // BIT 6 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(6, gb.cpu.get_c()));
            cycles += 8;
        }
        0x72 => {
            // BIT 6 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(6, gb.cpu.get_d()));
            cycles += 8;
        }
        0x73 => {
            // BIT 6 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(6, gb.cpu.get_e()));
            cycles += 8;
        }
        0x74 => {
            // BIT 6 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(6, gb.cpu.get_h()));
            cycles += 8;
        }
        0x75 => {
            // BIT 6 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(6, gb.cpu.get_l()));
            cycles += 8;
        }
        0x76 => {
            // BIT 6 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(6, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x77 => {
            // BIT 6 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(6, gb.cpu.get_a()));
            cycles += 8;
        }
        0x78 => {
            // BIT 7 B
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(7, gb.cpu.get_b()));
            cycles += 8;
        }
        0x79 => {
            // BIT 7 C
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(7, gb.cpu.get_c()));
            cycles += 8;
        }
        0x7A => {
            // BIT 7 D
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(7, gb.cpu.get_d()));
            cycles += 8;
        }
        0x7B => {
            // BIT 7 E
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(7, gb.cpu.get_e()));
            cycles += 8;
        }
        0x7C => {
            // BIT 7 H
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(7, gb.cpu.get_h()));
            cycles += 8;
        }
        0x7D => {
            // BIT 7 L
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(7, gb.cpu.get_l()));
            cycles += 8;
        }
        0x7E => {
            // BIT 7 HL
            gb.cpu
                .set_z_flag(!get_bit_at_position_16bit(7, gb.cpu.get_hl()));
            cycles += 12;
        }
        0x7F => {
            // BIT 7 A
            gb.cpu
                .set_z_flag(!get_bit_at_position_8bit(7, gb.cpu.get_a()));
            cycles += 8;
        }
        0x80 => {
            // RES 0 B
            cycles += 8;
        }
        0x81 => {
            // RES 0 C
            cycles += 8;
        }
        0x82 => {
            // RES 0 D
            cycles += 8;
        }
        0x83 => {
            // RES 0 E
            cycles += 8;
        }
        0x84 => {
            // RES 0 H
            cycles += 8;
        }
        0x85 => {
            // RES 0 L
            cycles += 8;
        }
        0x86 => {
            // RES 0 HL
            cycles += 16;
        }
        0x87 => {
            // RES 0 A
            cycles += 8;
        }
        0x88 => {
            // RES 1 B
            cycles += 8;
        }
        0x89 => {
            // RES 1 C
            cycles += 8;
        }
        0x8A => {
            // RES 1 D
            cycles += 8;
        }
        0x8B => {
            // RES 1 E
            cycles += 8;
        }
        0x8C => {
            // RES 1 H
            cycles += 8;
        }
        0x8D => {
            // RES 1 L
            cycles += 8;
        }
        0x8E => {
            // RES 1 HL
            cycles += 16;
        }
        0x8F => {
            // RES 1 A
            cycles += 8;
        }
        0x90 => {
            // RES 2 B
            cycles += 8;
        }
        0x91 => {
            // RES 2 C
            cycles += 8;
        }
        0x92 => {
            // RES 2 D
            cycles += 8;
        }
        0x93 => {
            // RES 2 E
            cycles += 8;
        }
        0x94 => {
            // RES 2 H
            cycles += 8;
        }
        0x95 => {
            // RES 2 L
            cycles += 8;
        }
        0x96 => {
            // RES 2 HL
            cycles += 16;
        }
        0x97 => {
            // RES 2 A
            cycles += 8;
        }
        0x98 => {
            // RES 3 B
            cycles += 8;
        }
        0x99 => {
            // RES 3 C
            cycles += 8;
        }
        0x9A => {
            // RES 3 D
            cycles += 8;
        }
        0x9B => {
            // RES 3 E
            cycles += 8;
        }
        0x9C => {
            // RES 3 H
            cycles += 8;
        }
        0x9D => {
            // RES 3 L
            cycles += 8;
        }
        0x9E => {
            // RES 3 HL
            cycles += 16;
        }
        0x9F => {
            // RES 3 A
            cycles += 8;
        }
        0xA0 => {
            // RES 4 B
            cycles += 8;
        }
        0xA1 => {
            // RES 4 C
            cycles += 8;
        }
        0xA2 => {
            // RES 4 D
            cycles += 8;
        }
        0xA3 => {
            // RES 4 E
            cycles += 8;
        }
        0xA4 => {
            // RES 4 H
            cycles += 8;
        }
        0xA5 => {
            // RES 4 L
            cycles += 8;
        }
        0xA6 => {
            // RES 4 HL
            cycles += 16;
        }
        0xA7 => {
            // RES 4 A
            cycles += 8;
        }
        0xA8 => {
            // RES 5 B
            cycles += 8;
        }
        0xA9 => {
            // RES 5 C
            cycles += 8;
        }
        0xAA => {
            // RES 5 D
            cycles += 8;
        }
        0xAB => {
            // RES 5 E
            cycles += 8;
        }
        0xAC => {
            // RES 5 H
            cycles += 8;
        }
        0xAD => {
            // RES 5 L
            cycles += 8;
        }
        0xAE => {
            // RES 5 HL
            cycles += 16;
        }
        0xAF => {
            // RES 5 A
            cycles += 8;
        }
        0xB0 => {
            // RES 6 B
            cycles += 8;
        }
        0xB1 => {
            // RES 6 C
            cycles += 8;
        }
        0xB2 => {
            // RES 6 D
            cycles += 8;
        }
        0xB3 => {
            // RES 6 E
            cycles += 8;
        }
        0xB4 => {
            // RES 6 H
            cycles += 8;
        }
        0xB5 => {
            // RES 6 L
            cycles += 8;
        }
        0xB6 => {
            // RES 6 HL
            cycles += 16;
        }
        0xB7 => {
            // RES 6 A
            cycles += 8;
        }
        0xB8 => {
            // RES 7 B
            cycles += 8;
        }
        0xB9 => {
            // RES 7 C
            cycles += 8;
        }
        0xBA => {
            // RES 7 D
            cycles += 8;
        }
        0xBB => {
            // RES 7 E
            cycles += 8;
        }
        0xBC => {
            // RES 7 H
            cycles += 8;
        }
        0xBD => {
            // RES 7 L
            cycles += 8;
        }
        0xBE => {
            // RES 7 HL
            cycles += 16;
        }
        0xBF => {
            // RES 7 A
            cycles += 8;
        }
        0xC0 => {
            // SET 0 B
            cycles += 8;
        }
        0xC1 => {
            // SET 0 C
            cycles += 8;
        }
        0xC2 => {
            // SET 0 D
            cycles += 8;
        }
        0xC3 => {
            // SET 0 E
            cycles += 8;
        }
        0xC4 => {
            // SET 0 H
            cycles += 8;
        }
        0xC5 => {
            // SET 0 L
            cycles += 8;
        }
        0xC6 => {
            // SET 0 HL
            cycles += 16;
        }
        0xC7 => {
            // SET 0 A
            cycles += 8;
        }
        0xC8 => {
            // SET 1 B
            cycles += 8;
        }
        0xC9 => {
            // SET 1 C
            cycles += 8;
        }
        0xCA => {
            // SET 1 D
            cycles += 8;
        }
        0xCB => {
            // SET 1 E
            cycles += 8;
        }
        0xCC => {
            // SET 1 H
            cycles += 8;
        }
        0xCD => {
            // SET 1 L
            cycles += 8;
        }
        0xCE => {
            // SET 1 HL
            cycles += 16;
        }
        0xCF => {
            // SET 1 A
            cycles += 8;
        }
        0xD0 => {
            // SET 2 B
            cycles += 8;
        }
        0xD1 => {
            // SET 2 C
            cycles += 8;
        }
        0xD2 => {
            // SET 2 D
            cycles += 8;
        }
        0xD3 => {
            // SET 2 E
            cycles += 8;
        }
        0xD4 => {
            // SET 2 H
            cycles += 8;
        }
        0xD5 => {
            // SET 2 L
            cycles += 8;
        }
        0xD6 => {
            // SET 2 HL
            cycles += 16;
        }
        0xD7 => {
            // SET 2 A
            cycles += 8;
        }
        0xD8 => {
            // SET 3 B
            cycles += 8;
        }
        0xD9 => {
            // SET 3 C
            cycles += 8;
        }
        0xDA => {
            // SET 3 D
            cycles += 8;
        }
        0xDB => {
            // SET 3 E
            cycles += 8;
        }
        0xDC => {
            // SET 3 H
            cycles += 8;
        }
        0xDD => {
            // SET 3 L
            cycles += 8;
        }
        0xDE => {
            // SET 3 HL
            cycles += 16;
        }
        0xDF => {
            // SET 3 A
            cycles += 8;
        }
        0xE0 => {
            // SET 4 B
            cycles += 8;
        }
        0xE1 => {
            // SET 4 C
            cycles += 8;
        }
        0xE2 => {
            // SET 4 D
            cycles += 8;
        }
        0xE3 => {
            // SET 4 E
            cycles += 8;
        }
        0xE4 => {
            // SET 4 H
            cycles += 8;
        }
        0xE5 => {
            // SET 4 L
            cycles += 8;
        }
        0xE6 => {
            // SET 4 HL
            cycles += 16;
        }
        0xE7 => {
            // SET 4 A
            cycles += 8;
        }
        0xE8 => {
            // SET 5 B
            cycles += 8;
        }
        0xE9 => {
            // SET 5 C
            cycles += 8;
        }
        0xEA => {
            // SET 5 D
            cycles += 8;
        }
        0xEB => {
            // SET 5 E
            cycles += 8;
        }
        0xEC => {
            // SET 5 H
            cycles += 8;
        }
        0xED => {
            // SET 5 L
            cycles += 8;
        }
        0xEE => {
            // SET 5 HL
            cycles += 16;
        }
        0xEF => {
            // SET 5 A
            cycles += 8;
        }
        0xF0 => {
            // SET 6 B
            cycles += 8;
        }
        0xF1 => {
            // SET 6 C
            cycles += 8;
        }
        0xF2 => {
            // SET 6 D
            cycles += 8;
        }
        0xF3 => {
            // SET 6 E
            cycles += 8;
        }
        0xF4 => {
            // SET 6 H
            cycles += 8;
        }
        0xF5 => {
            // SET 6 L
            cycles += 8;
        }
        0xF6 => {
            // SET 6 HL
            cycles += 16;
        }
        0xF7 => {
            // SET 6 A
            cycles += 8;
        }
        0xF8 => {
            // SET 7 B
            cycles += 8;
        }
        0xF9 => {
            // SET 7 C
            cycles += 8;
        }
        0xFA => {
            // SET 7 D
            cycles += 8;
        }
        0xFB => {
            // SET 7 E
            cycles += 8;
        }
        0xFC => {
            // SET 7 H
            cycles += 8;
        }
        0xFD => {
            // SET 7 L
            cycles += 8;
        }
        0xFE => {
            // SET 7 HL
            cycles += 16;
        }
        0xFF => {
            // SET 7 A
            cycles += 8;
        }
        _ => {
            println!("Unknown opcode: {:#04X}", opcode);
        }
    }
    return cycles;
}

// Jump/Call functions
fn jump_relative(gb: &mut gameboy::Gameboy, value: i16) -> u16 {
    let signed_pc = i16::try_from(gb.cpu.get_pc()).unwrap();
    let jump_pc = i16::try_from(signed_pc + value).unwrap();
    gb.cpu.set_pc(u16::try_from(jump_pc).unwrap());
    return u16::try_from(jump_pc).unwrap();
}

// bit and shift functions
fn get_bit_at_position_8bit(n: u32, num: u8) -> bool {
    return (num & (1 << (7 - n))) != 0;
}

fn get_bit_at_position_16bit(n: u32, num: u16) -> bool {
    return (num & (1 << (15 - n))) != 0;
}

// Load functions
fn load_8bit(gb: &mut gameboy::Gameboy, register: &str, value: u8) {
    gb.cpu.set_register_8bit(register, value);
}

fn load_16bit(gb: &mut gameboy::Gameboy, register: &str, value: u16) {
    gb.cpu.set_register_16bit(register, value);
}

fn load_immediate_8bit(gb: &mut gameboy::Gameboy, register: &str) {
    let value = u8::try_from(gameboy::read_byte(gb)).unwrap();
    load_8bit(gb, register, value);
}

fn load_immediate_16bit(gb: &mut gameboy::Gameboy, register: &str) {
    let value = u16::try_from(gameboy::read_short(gb)).unwrap();
    load_16bit(gb, register, value);
}

fn load_register_8bit(gb: &mut gameboy::Gameboy, dest: &str, src: &str) {
    let src_value = gb.cpu.get_register_8bit(src);
    load_8bit(gb, dest, src_value);
}

fn load_register_16bit(gb: &mut gameboy::Gameboy, dest: &str, src: &str) {
    let src_value = gb.cpu.get_register_16bit(src);
    load_16bit(gb, dest, src_value);
}

// ALU functions
fn increment_8bit(gb: &mut gameboy::Gameboy, register: &str) -> u8 {
    let current = gb.cpu.get_register_8bit(register);
    let result = current.wrapping_add(1);
    load_8bit(gb, register, result);
    return result;
}

fn decrement_8bit(gb: &mut gameboy::Gameboy, register: &str) -> u8 {
    let current = gb.cpu.get_register_8bit(register);
    let result = current.wrapping_sub(1);
    load_8bit(gb, register, result);
    return result;
}

fn xor(gb: &mut gameboy::Gameboy, register: &str) -> u8 {
    let a = gb.cpu.get_a();
    let value = gb.cpu.get_register_8bit(register);
    let result = a ^ value;
    if result == 0 {
        gb.cpu.set_z_flag(true);
    } else {
        gb.cpu.set_z_flag(false);
    }
    gb.cpu.set_register_8bit(register, result);
    return result;
}

fn increment_16bit(gb: &mut gameboy::Gameboy, register: &str) -> u16 {
    let current = gb.cpu.get_register_16bit(register);
    let result = current.wrapping_add(1);
    load_16bit(gb, register, result);
    return result;
}

fn decrement_16bit(gb: &mut gameboy::Gameboy, register: &str) -> u16 {
    let current = gb.cpu.get_register_16bit(register);
    let result = current.wrapping_sub(1);
    load_16bit(gb, register, result);
    return result;
}

mod tests {
    use super::*;

    #[test]
    fn test_jump_relative() {
        let mut gb = gameboy::create_gameboy();
        gb.cpu.set_pc(0x0000);
        jump_relative(&mut gb, 5);
        assert_eq!(gb.cpu.get_pc(), 0x0005);
        jump_relative(&mut gb, -5);
        assert_eq!(gb.cpu.get_pc(), 0x0000);
    }

    #[test]
    fn test_xor() {
        let mut gb = gameboy::create_gameboy();

        // xor a with itself is always 0, z flag should be set
        gb.cpu.set_a(0x00);
        xor(&mut gb, "a");
        assert_eq!(gb.cpu.get_a(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);
        gb.cpu.set_a(0x01);
        xor(&mut gb, "a");
        assert_eq!(gb.cpu.get_a(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);

        // xor a with b is 0 only if a == b, z flag should be set
        gb.cpu.set_a(0x00);
        gb.cpu.set_b(0x00);
        xor(&mut gb, "b");
        assert_eq!(gb.cpu.get_b(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);

        // xor a with b is 1 otherwise, z flag should be unset
        gb.cpu.set_a(0x01);
        gb.cpu.set_b(0x00);
        xor(&mut gb, "b");
        assert_eq!(gb.cpu.get_b(), 0x01);
        assert_eq!(gb.cpu.get_z_flag(), false);

        // repeat for other registers
        gb.cpu.set_a(0x00);
        gb.cpu.set_c(0x00);
        gb.cpu.set_d(0x00);
        gb.cpu.set_e(0x00);
        gb.cpu.set_h(0x00);
        gb.cpu.set_l(0x00);
        xor(&mut gb, "c");
        assert_eq!(gb.cpu.get_c(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);
        xor(&mut gb, "d");
        assert_eq!(gb.cpu.get_d(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);
        xor(&mut gb, "e");
        assert_eq!(gb.cpu.get_e(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);
        xor(&mut gb, "h");
        assert_eq!(gb.cpu.get_h(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);
        xor(&mut gb, "l");
        assert_eq!(gb.cpu.get_l(), 0x00);
        assert_eq!(gb.cpu.get_z_flag(), true);

        gb.cpu.set_a(0x01);
        gb.cpu.set_c(0x00);
        gb.cpu.set_d(0x00);
        gb.cpu.set_e(0x00);
        gb.cpu.set_h(0x00);
        gb.cpu.set_l(0x00);
        xor(&mut gb, "c");
        assert_eq!(gb.cpu.get_c(), 0x01);
        assert_eq!(gb.cpu.get_z_flag(), false);
        xor(&mut gb, "d");
        assert_eq!(gb.cpu.get_d(), 0x01);
        assert_eq!(gb.cpu.get_z_flag(), false);
        xor(&mut gb, "e");
        assert_eq!(gb.cpu.get_e(), 0x01);
        assert_eq!(gb.cpu.get_z_flag(), false);
        xor(&mut gb, "h");
        assert_eq!(gb.cpu.get_h(), 0x01);
        assert_eq!(gb.cpu.get_z_flag(), false);
        xor(&mut gb, "l");
        assert_eq!(gb.cpu.get_l(), 0x01);
        assert_eq!(gb.cpu.get_z_flag(), false);
    }

    #[test]
    fn test_get_bit_at() {
        // 0b11111110
        let val: u8 = 0xFE;
        let bit = get_bit_at_position_8bit(7, val) as u8;
        assert_eq!(bit, 0);
    }

    #[test]
    fn test_increment_8bit() {
        let mut gb = gameboy::create_gameboy();
        gb.cpu.set_a(0x00);
        increment_8bit(&mut gb, "a");
        assert_eq!(gb.cpu.get_a(), 0x01);
    }

    #[test]
    fn test_decrement_8bit() {
        let mut gb = gameboy::create_gameboy();
        gb.cpu.set_a(0x01);
        decrement_8bit(&mut gb, "a");
        assert_eq!(gb.cpu.get_a(), 0x00);
    }

    #[test]
    fn test_increment_16bit() {
        let mut gb = gameboy::create_gameboy();
        gb.cpu.set_bc(0x0000);
        increment_16bit(&mut gb, "bc");
        assert_eq!(gb.cpu.get_bc(), 0x0001);
    }

    #[test]
    fn test_decrement_16bit() {
        let mut gb = gameboy::create_gameboy();
        gb.cpu.set_bc(0x0001);
        decrement_16bit(&mut gb, "bc");
        assert_eq!(gb.cpu.get_bc(), 0x0000);
    }
}
