use crate::gameboy;

pub fn executeInstruction(gb: &mut gameboy::Gameboy, opcode: u16) -> i64 {
    let mut cycles: i64 = 0;

    match opcode {
        0x00 => {
            // NOP
            cycles += 4;
        }
        0x01 => {
            // LD BC n16
            cycles += 12;
        }
        0x02 => {
            // LD BC A
            cycles += 8;
        }
        0x03 => {
            // INC BC
            cycles += 8;
        }
        0x04 => {
            // INC B
            cycles += 4;
        }
        0x05 => {
            // DEC B
            cycles += 4;
        }
        0x06 => {
            // LD B n8
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
            cycles += 8;
        }
        0x0C => {
            // INC C
            cycles += 4;
        }
        0x0D => {
            // DEC C
            cycles += 4;
        }
        0x0E => {
            // LD C n8
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
            cycles += 8;
        }
        0x13 => {
            // INC DE
            cycles += 8;
        }
        0x14 => {
            // INC D
            cycles += 4;
        }
        0x15 => {
            // DEC D
            cycles += 4;
        }
        0x16 => {
            // LD D n8
            cycles += 8;
        }
        0x17 => {
            // RLA
            cycles += 4;
        }
        0x18 => {
            // JR e8
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
            cycles += 8;
        }
        0x1C => {
            // INC E
            cycles += 4;
        }
        0x1D => {
            // DEC E
            cycles += 4;
        }
        0x1E => {
            // LD E n8
            cycles += 8;
        }
        0x1F => {
            // RRA
            cycles += 4;
        }
        0x20 => {
            // JR NZ e8
            // if branch
            cycles += 12;
            // if not branch
            cycles += 8;
        }
        0x21 => {
            // LD HL n16
            cycles += 12;
        }
        0x22 => {
            // LD HL A
            cycles += 8;
        }
        0x23 => {
            // INC HL
            cycles += 8;
        }
        0x24 => {
            // INC H
            cycles += 4;
        }
        0x25 => {
            // DEC H
            cycles += 4;
        }
        0x26 => {
            // LD H n8
            cycles += 8;
        }
        0x27 => {
            // DAA
            cycles += 4;
        }
        0x28 => {
            // JR Z e8
            // if branch
            cycles += 12;
            // if not branch
            cycles += 8;
        }
        0x29 => {
            // ADD HL HL
            cycles += 8;
        }
        0x2A => {
            // LD A HL
            cycles += 8;
        }
        0x2B => {
            // DEC HL
            cycles += 8;
        }
        0x2C => {
            // INC L
            cycles += 4;
        }
        0x2D => {
            // DEC L
            cycles += 4;
        }
        0x2E => {
            // LD L n8
            cycles += 8;
        }
        0x2F => {
            // CPL
            cycles += 4;
        }
        0x30 => {
            // JR NC e8
            // if branch
            cycles += 12;
            // if not branch
            cycles += 8;
        }
        0x31 => {
            // LD SP n16
            cycles += 12;
        }
        0x32 => {
            // LD HL A
            cycles += 8;
        }
        0x33 => {
            // INC SP
            cycles += 8;
        }
        0x34 => {
            // INC HL
            cycles += 12;
        }
        0x35 => {
            // DEC HL
            cycles += 12;
        }
        0x36 => {
            // LD HL n8
            cycles += 12;
        }
        0x37 => {
            // SCF
            cycles += 4;
        }
        0x38 => {
            // JR C e8
            // if branch
            cycles += 12;
            // if not branch
            cycles += 8;
        }
        0x39 => {
            // ADD HL SP
            cycles += 8;
        }
        0x3A => {
            // LD A HL
            cycles += 8;
        }
        0x3B => {
            // DEC SP
            cycles += 8;
        }
        0x3C => {
            // INC A
            cycles += 4;
        }
        0x3D => {
            // DEC A
            cycles += 4;
        }
        0x3E => {
            // LD A n8
            cycles += 8;
        }
        0x3F => {
            // CCF
            cycles += 4;
        }
        0x40 => {
            // LD B B
            cycles += 4;
        }
        0x41 => {
            // LD B C
            cycles += 4;
        }
        0x42 => {
            // LD B D
            cycles += 4;
        }
        0x43 => {
            // LD B E
            cycles += 4;
        }
        0x44 => {
            // LD B H
            cycles += 4;
        }
        0x45 => {
            // LD B L
            cycles += 4;
        }
        0x46 => {
            // LD B HL
            cycles += 8;
        }
        0x47 => {
            // LD B A
            cycles += 4;
        }
        0x48 => {
            // LD C B
            cycles += 4;
        }
        0x49 => {
            // LD C C
            cycles += 4;
        }
        0x4A => {
            // LD C D
            cycles += 4;
        }
        0x4B => {
            // LD C E
            cycles += 4;
        }
        0x4C => {
            // LD C H
            cycles += 4;
        }
        0x4D => {
            // LD C L
            cycles += 4;
        }
        0x4E => {
            // LD C HL
            cycles += 8;
        }
        0x4F => {
            // LD C A
            cycles += 4;
        }
        0x50 => {
            // LD D B
            cycles += 4;
        }
        0x51 => {
            // LD D C
            cycles += 4;
        }
        0x52 => {
            // LD D D
            cycles += 4;
        }
        0x53 => {
            // LD D E
            cycles += 4;
        }
        0x54 => {
            // LD D H
            cycles += 4;
        }
        0x55 => {
            // LD D L
            cycles += 4;
        }
        0x56 => {
            // LD D HL
            cycles += 8;
        }
        0x57 => {
            // LD D A
            cycles += 4;
        }
        0x58 => {
            // LD E B
            cycles += 4;
        }
        0x59 => {
            // LD E C
            cycles += 4;
        }
        0x5A => {
            // LD E D
            cycles += 4;
        }
        0x5B => {
            // LD E E
            cycles += 4;
        }
        0x5C => {
            // LD E H
            cycles += 4;
        }
        0x5D => {
            // LD E L
            cycles += 4;
        }
        0x5E => {
            // LD E HL
            cycles += 8;
        }
        0x5F => {
            // LD E A
            cycles += 4;
        }
        0x60 => {
            // LD H B
            cycles += 4;
        }
        0x61 => {
            // LD H C
            cycles += 4;
        }
        0x62 => {
            // LD H D
            cycles += 4;
        }
        0x63 => {
            // LD H E
            cycles += 4;
        }
        0x64 => {
            // LD H H
            cycles += 4;
        }
        0x65 => {
            // LD H L
            cycles += 4;
        }
        0x66 => {
            // LD H HL
            cycles += 8;
        }
        0x67 => {
            // LD H A
            cycles += 4;
        }
        0x68 => {
            // LD L B
            cycles += 4;
        }
        0x69 => {
            // LD L C
            cycles += 4;
        }
        0x6A => {
            // LD L D
            cycles += 4;
        }
        0x6B => {
            // LD L E
            cycles += 4;
        }
        0x6C => {
            // LD L H
            cycles += 4;
        }
        0x6D => {
            // LD L L
            cycles += 4;
        }
        0x6E => {
            // LD L HL
            cycles += 8;
        }
        0x6F => {
            // LD L A
            cycles += 4;
        }
        0x70 => {
            // LD HL B
            cycles += 8;
        }
        0x71 => {
            // LD HL C
            cycles += 8;
        }
        0x72 => {
            // LD HL D
            cycles += 8;
        }
        0x73 => {
            // LD HL E
            cycles += 8;
        }
        0x74 => {
            // LD HL H
            cycles += 8;
        }
        0x75 => {
            // LD HL L
            cycles += 8;
        }
        0x76 => {
            // HALT
            cycles += 4;
        }
        0x77 => {
            // LD HL A
            cycles += 8;
        }
        0x78 => {
            // LD A B
            cycles += 4;
        }
        0x79 => {
            // LD A C
            cycles += 4;
        }
        0x7A => {
            // LD A D
            cycles += 4;
        }
        0x7B => {
            // LD A E
            cycles += 4;
        }
        0x7C => {
            // LD A H
            cycles += 4;
        }
        0x7D => {
            // LD A L
            cycles += 4;
        }
        0x7E => {
            // LD A HL
            cycles += 8;
        }
        0x7F => {
            // LD A A
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
            cycles += 4;
        }
        0xAD => {
            // XOR L
            cycles += 4;
        }
        0xAE => {
            // XOR HL
            cycles += 8;
        }
        0xAF => {
            // XOR A
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
