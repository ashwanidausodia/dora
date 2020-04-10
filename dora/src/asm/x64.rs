use crate::asm::{Assembler, Register};

impl Register {
    fn low_bits(self) -> u8 {
        self.0 & 0b111
    }

    fn value(self) -> u8 {
        self.0
    }

    fn needs_rex(self) -> bool {
        self.0 > 7
    }
}

pub const RAX: Register = Register(0);
pub const RCX: Register = Register(1);
pub const RDX: Register = Register(2);
pub const RBX: Register = Register(3);
pub const RSP: Register = Register(4);
pub const RBP: Register = Register(5);
pub const RSI: Register = Register(6);
pub const RDI: Register = Register(7);

pub const R8: Register = Register(8);
pub const R9: Register = Register(9);
pub const R10: Register = Register(10);
pub const R11: Register = Register(11);
pub const R12: Register = Register(12);
pub const R13: Register = Register(13);
pub const R14: Register = Register(14);
pub const R15: Register = Register(15);

impl Assembler {
    pub fn pushq_r(&mut self, reg: Register) {
        self.emit_rex32_rm_optional(reg);
        self.emit_u8(0x50 + reg.low_bits());
    }

    pub fn popq_r(&mut self, reg: Register) {
        self.emit_rex32_rm_optional(reg);
        self.emit_u8(0x58 + reg.low_bits());
    }

    pub fn int3(&mut self) {
        self.emit_u8(0xCC);
    }

    pub fn retq(&mut self) {
        self.emit_u8(0xC3);
    }

    pub fn nop(&mut self) {
        self.emit_u8(0x90);
    }

    pub fn setcc_r(&mut self, condition: Condition, dest: Register) {
        if dest.needs_rex() || dest.low_bits() > 3 {
            self.emit_rex(false, false, false, dest.needs_rex());
        }

        self.emit_u8(0x0F);
        self.emit_u8((0x90 + condition.int()) as u8);
        self.emit_modrm_opcode(0, dest);
    }

    pub fn cmovl(&mut self, condition: Condition, dest: Register, src: Register) {
        self.emit_rex32_optional(dest, src);
        self.emit_u8(0x0F);
        self.emit_u8((0x40 + condition.int()) as u8);
        self.emit_modrm_registers(dest, src);
    }

    pub fn cmovq(&mut self, condition: Condition, dest: Register, src: Register) {
        self.emit_rex64_modrm(dest, src);
        self.emit_u8(0x0F);
        self.emit_u8((0x40 + condition.int()) as u8);
        self.emit_modrm_registers(dest, src);
    }

    pub fn lea(&mut self, dest: Register, src: Address) {
        self.emit_rex64_modrm_address(dest, src);
        self.emit_u8(0x8D);
        self.emit_address(dest.low_bits(), src);
    }

    pub fn movq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(src, dest);
        self.emit_u8(0x89);
        self.emit_modrm_registers(src, dest);
    }

    pub fn movq_ri(&mut self, dest: Register, imm: Immediate) {
        if imm.is_int32() {
            self.emit_rex64_rm(dest);
            self.emit_u8(0xC7);
            self.emit_modrm_opcode(0, dest);
            self.emit_u32(imm.int32() as u32);
        } else {
            self.emit_rex64_rm(dest);
            self.emit_u8(0xB8 + dest.low_bits());
            self.emit_u64(imm.int64() as u64);
        }
    }

    pub fn movq_ra(&mut self, dest: Register, src: Address) {
        self.emit_rex64_modrm_address(dest, src);
        self.emit_u8(0x8B);
        self.emit_address(dest.low_bits(), src);
    }

    pub fn movb_ar(&mut self, dest: Address, src: Register) {
        self.emit_rex32_byte_address(src, dest);
        self.emit_u8(0x88);
        self.emit_address(src.low_bits(), dest);
    }

    pub fn movb_ai(&mut self, dest: Address, src: Immediate) {
        assert!(src.is_int8() || src.is_uint8());
        self.emit_rex32_address_optional(dest);
        self.emit_u8(0xc6);
        self.emit_address(0b000, dest);
        self.emit_u8(src.uint8());
    }

    pub fn movq_ar(&mut self, dest: Address, src: Register) {
        self.emit_rex64_modrm_address(src, dest);
        self.emit_u8(0x89);
        self.emit_address(src.low_bits(), dest);
    }

    pub fn movl_ra(&mut self, dest: Register, src: Address) {
        self.emit_rex32_modrm_address(dest, src);
        self.emit_u8(0x8B);
        self.emit_address(dest.low_bits(), src);
    }

    pub fn movl_ar(&mut self, dest: Address, src: Register) {
        self.emit_rex32_modrm_address(src, dest);
        self.emit_u8(0x89);
        self.emit_address(src.low_bits(), dest);
    }

    pub fn movl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(src, dest);
        self.emit_u8(0x89);
        self.emit_modrm_registers(src, dest);
    }

    pub fn movl_ri(&mut self, dest: Register, imm: Immediate) {
        assert!(imm.is_int32());
        self.emit_rex32_rm_optional(dest);
        self.emit_u8(0xB8 + dest.low_bits());
        self.emit_u32(imm.int32() as u32);
    }

    pub fn movzxb_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_byte_optional(dest, src);
        self.emit_u8(0x0f);
        self.emit_u8(0xb6);
        self.emit_modrm_registers(dest, src);
    }

    pub fn movzxb_ra(&mut self, dest: Register, src: Address) {
        self.emit_rex32_modrm_address(dest, src);
        self.emit_u8(0x0f);
        self.emit_u8(0xb6);
        self.emit_address(dest.low_bits(), src);
    }

    pub fn movsxbl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_byte_optional(dest, src);
        self.emit_u8(0x0f);
        self.emit_u8(0xbe);
        self.emit_modrm_registers(dest, src);
    }

    pub fn movsxbl_ra(&mut self, dest: Register, src: Address) {
        self.emit_rex32_modrm_address(dest, src);
        self.emit_u8(0x0f);
        self.emit_u8(0xbe);
        self.emit_address(dest.low_bits(), src);
    }

    pub fn movsxbq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(dest, src);
        self.emit_u8(0x0f);
        self.emit_u8(0xbe);
        self.emit_modrm_registers(dest, src);
    }

    pub fn movsxbq_ra(&mut self, dest: Register, src: Address) {
        self.emit_rex64_modrm_address(dest, src);
        self.emit_u8(0x0f);
        self.emit_u8(0xbe);
        self.emit_address(dest.low_bits(), src);
    }

    pub fn movsxlq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(dest, src);
        self.emit_u8(0x63);
        self.emit_modrm_registers(dest, src);
    }

    pub fn addq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(src, dest);
        self.emit_u8(0x01);
        self.emit_modrm_registers(src, dest);
    }

    pub fn addq_ri(&mut self, dest: Register, imm: Immediate) {
        self.emit_alu64_imm(dest, imm, 0b000, 0x05, false);
    }

    pub fn addl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(src, dest);
        self.emit_u8(0x01);
        self.emit_modrm_registers(src, dest);
    }

    pub fn subq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(src, dest);
        self.emit_u8(0x29);
        self.emit_modrm_registers(src, dest);
    }

    pub fn subq_ri(&mut self, dest: Register, imm: Immediate) {
        self.emit_alu64_imm(dest, imm, 0b101, 0x2D, false);
    }

    pub fn subq_ri32(&mut self, dest: Register, imm: Immediate) {
        self.emit_alu64_imm(dest, imm, 0b101, 0x2D, true);
    }

    pub fn subl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(src, dest);
        self.emit_u8(0x29);
        self.emit_modrm_registers(src, dest);
    }

    pub fn andl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(src, dest);
        self.emit_u8(0x21);
        self.emit_modrm_registers(src, dest);
    }

    pub fn andq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(src, dest);
        self.emit_u8(0x21);
        self.emit_modrm_registers(src, dest);
    }

    pub fn andq_ri(&mut self, dest: Register, imm: Immediate) {
        self.emit_alu64_imm(dest, imm, 0b100, 0x25, false);
    }

    pub fn cmpl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(src, dest);
        self.emit_u8(0x39);
        self.emit_modrm_registers(src, dest);
    }

    pub fn cmpl_ar(&mut self, lhs: Address, rhs: Register) {
        self.emit_rex32_modrm_address(rhs, lhs);
        self.emit_u8(0x39);
        self.emit_address(rhs.low_bits(), lhs);
    }

    pub fn cmpq_rr(&mut self, lhs: Register, rhs: Register) {
        self.emit_rex64_modrm(rhs, lhs);
        self.emit_u8(0x39);
        self.emit_modrm_registers(rhs, lhs);
    }

    pub fn cmpq_ar(&mut self, lhs: Address, rhs: Register) {
        self.emit_rex64_modrm_address(rhs, lhs);
        self.emit_u8(0x39);
        self.emit_address(rhs.low_bits(), lhs);
    }

    pub fn cmpq_ri(&mut self, reg: Register, imm: Immediate) {
        self.emit_alu64_imm(reg, imm, 0b111, 0x3d, false);
    }

    pub fn cmpl_ri(&mut self, reg: Register, imm: Immediate) {
        self.emit_alu32_imm(reg, imm, 0b111, 0x3d, false);
    }

    pub fn orl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(src, dest);
        self.emit_u8(0x09);
        self.emit_modrm_registers(src, dest);
    }

    pub fn orq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(src, dest);
        self.emit_u8(0x09);
        self.emit_modrm_registers(src, dest);
    }

    pub fn xorl_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(src, dest);
        self.emit_u8(0x31);
        self.emit_modrm_registers(src, dest);
    }

    pub fn xorl_ri(&mut self, lhs: Register, rhs: Immediate) {
        assert!(rhs.is_int32());

        if rhs.is_int8() {
            self.emit_rex32_rm_optional(lhs);
            self.emit_u8(0x83);
            self.emit_modrm_opcode(0b110, lhs);
            self.emit_u8(rhs.int8() as u8);
        } else if lhs == RAX {
            self.emit_u8(0x35);
            self.emit_u32(rhs.int32() as u32);
        } else {
            self.emit_rex32_rm_optional(lhs);
            self.emit_u8(0x81);
            self.emit_modrm_opcode(0b110, lhs);
            self.emit_u32(rhs.int32() as u32);
        }
    }

    pub fn xorq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(src, dest);
        self.emit_u8(0x31);
        self.emit_modrm_registers(src, dest);
    }

    pub fn testl_rr(&mut self, lhs: Register, rhs: Register) {
        self.emit_rex32_optional(rhs, lhs);
        self.emit_u8(0x85);
        self.emit_modrm_registers(rhs, lhs);
    }

    pub fn testl_ri(&mut self, lhs: Register, rhs: Immediate) {
        assert!(rhs.is_int32());

        if rhs.is_uint8() {
            if lhs == RAX {
                self.emit_u8(0xa8);
            } else if lhs.value() < 4 {
                self.emit_u8(0xf6);
                self.emit_modrm_opcode(0b000, lhs);
            } else {
                self.emit_rex(false, false, false, lhs.needs_rex());
                self.emit_u8(0xf6);
                self.emit_modrm_opcode(0b000, lhs);
            }
            self.emit_u8(rhs.uint8());
        } else if lhs == RAX {
            self.emit_u8(0xa9);
            self.emit_u32(rhs.int32() as u32);
        } else {
            self.emit_u8(0xf7);
            self.emit_modrm_opcode(0b000, lhs);
            self.emit_u32(rhs.int32() as u32);
        }
    }

    pub fn testl_ar(&mut self, lhs: Address, rhs: Register) {
        self.emit_rex32_modrm_address(rhs, lhs);
        self.emit_u8(0x85);
        self.emit_address(rhs.low_bits(), lhs);
    }

    pub fn testl_ai(&mut self, lhs: Address, rhs: Immediate) {
        assert!(rhs.is_int32());
        self.emit_rex32_address_optional(lhs);
        self.emit_u8(0xf7);
        self.emit_address(0b000, lhs);
        self.emit_u32(rhs.int32() as u32);
    }

    pub fn testq_rr(&mut self, lhs: Register, rhs: Register) {
        self.emit_rex64_modrm(rhs, lhs);
        self.emit_u8(0x85);
        self.emit_modrm_registers(rhs, lhs);
    }

    pub fn testq_ar(&mut self, lhs: Address, rhs: Register) {
        self.emit_rex64_modrm_address(rhs, lhs);
        self.emit_u8(0x85);
        self.emit_address(rhs.low_bits(), lhs);
    }

    pub fn testq_ai(&mut self, lhs: Address, rhs: Immediate) {
        assert!(rhs.is_int32());
        self.emit_rex64_address(lhs);
        self.emit_u8(0xf7);
        self.emit_address(0b000, lhs);
        self.emit_u32(rhs.int32() as u32);
    }

    pub fn imull_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex32_optional(dest, src);
        self.emit_u8(0x0F);
        self.emit_u8(0xAF);
        self.emit_modrm_registers(dest, src);
    }

    pub fn imulq_rr(&mut self, dest: Register, src: Register) {
        self.emit_rex64_modrm(dest, src);
        self.emit_u8(0x0F);
        self.emit_u8(0xAF);
        self.emit_modrm_registers(dest, src);
    }

    pub fn idivl_r(&mut self, reg: Register) {
        self.emit_rex32_rm_optional(reg);
        self.emit_u8(0xF7);
        self.emit_modrm_opcode(0b111, reg);
    }

    pub fn idivq_r(&mut self, src: Register) {
        self.emit_rex64_rm(src);
        self.emit_u8(0xF7);
        self.emit_modrm_opcode(0b111, src);
    }

    pub fn call_r(&mut self, reg: Register) {
        self.emit_rex32_rm_optional(reg);
        self.emit_u8(0xFF);
        self.emit_modrm_opcode(0b010, reg);
    }

    pub fn cdq(&mut self) {
        self.emit_u8(0x99);
    }

    pub fn cqo(&mut self) {
        self.emit_rex64();
        self.emit_u8(0x99);
    }

    pub fn negl(&mut self, reg: Register) {
        self.emit_rex32_rm_optional(reg);
        self.emit_u8(0xF7);
        self.emit_modrm_opcode(0b011, reg);
    }

    pub fn negq(&mut self, reg: Register) {
        self.emit_rex64_rm(reg);
        self.emit_u8(0xF7);
        self.emit_modrm_opcode(0b011, reg);
    }

    pub fn notl(&mut self, reg: Register) {
        self.emit_rex32_rm_optional(reg);
        self.emit_u8(0xF7);
        self.emit_modrm_opcode(0b010, reg);
    }

    pub fn notq(&mut self, reg: Register) {
        self.emit_rex64_rm(reg);
        self.emit_u8(0xF7);
        self.emit_modrm_opcode(0b010, reg);
    }

    fn emit_rex32_rm_optional(&mut self, reg: Register) {
        if reg.needs_rex() {
            self.emit_rex(false, false, false, true);
        }
    }

    fn emit_rex32_byte_optional(&mut self, reg: Register, rm: Register) {
        if reg.needs_rex() || rm.needs_rex() || rm.value() > 3 {
            self.emit_rex(false, reg.needs_rex(), false, rm.needs_rex());
        }
    }

    fn emit_rex32_optional(&mut self, reg: Register, rm: Register) {
        if reg.needs_rex() || rm.needs_rex() {
            self.emit_rex(false, reg.needs_rex(), false, rm.needs_rex());
        }
    }

    fn emit_rex64(&mut self) {
        self.emit_rex(true, false, false, false);
    }

    fn emit_rex64_rm(&mut self, rm: Register) {
        self.emit_rex(true, false, false, rm.needs_rex());
    }

    fn emit_rex64_modrm_address(&mut self, reg: Register, address: Address) {
        let rex = 0x48 | address.rex | if reg.needs_rex() { 0x04 } else { 0 };
        self.emit_u8(rex);
    }

    fn emit_rex64_address(&mut self, address: Address) {
        self.emit_u8(0x48 | address.rex);
    }

    fn emit_rex32_modrm_address(&mut self, reg: Register, address: Address) {
        if address.rex != 0 || reg.needs_rex() {
            self.emit_u8(0x40 | address.rex | if reg.needs_rex() { 0x04 } else { 0 });
        }
    }

    fn emit_rex32_byte_address(&mut self, reg: Register, address: Address) {
        if address.rex != 0 || reg.value() > 3 {
            self.emit_u8(0x40 | address.rex | if reg.needs_rex() { 0x04 } else { 0 });
        }
    }

    fn emit_rex32_address_optional(&mut self, address: Address) {
        if address.rex != 0 {
            self.emit_u8(0x40 | address.rex);
        }
    }

    fn emit_rex64_modrm(&mut self, reg: Register, rm: Register) {
        self.emit_rex(true, reg.needs_rex(), false, rm.needs_rex());
    }

    fn emit_rex(&mut self, w: bool, r: bool, x: bool, b: bool) {
        // w - 64-bit width
        // r - extension of modrm-reg field
        // x - extension of sib index field
        // b - extension of modrm-rm/sib base/opcode reg field
        let opcode = 0x40 | (w as u8) << 3 | (r as u8) << 2 | (x as u8) << 1 | b as u8;
        self.emit_u8(opcode);
    }

    fn emit_modrm_registers(&mut self, reg: Register, rm: Register) {
        self.emit_modrm(0b11, reg.low_bits(), rm.low_bits());
    }

    fn emit_modrm_opcode(&mut self, opcode: u8, reg: Register) {
        self.emit_modrm(0b11, opcode, reg.low_bits());
    }

    fn emit_modrm(&mut self, mode: u8, reg: u8, rm: u8) {
        assert!(mode < 4);
        assert!(reg < 8);
        assert!(rm < 8);
        self.emit_u8(mode << 6 | reg << 3 | rm);
    }

    fn emit_sib(&mut self, scale: u8, index: u8, base: u8) {
        assert!(scale < 4);
        assert!(index < 8);
        assert!(base < 8);
        self.emit_u8(scale << 6 | index << 3 | base);
    }

    fn emit_address(&mut self, reg_or_opcode: u8, address: Address) {
        assert!(reg_or_opcode < 8);

        let bytes = address.encoded_bytes();

        // emit modrm-byte with the given rm value
        self.emit_u8(reg_or_opcode << 3 | bytes[0]);

        for &byte in &bytes[1..] {
            self.emit_u8(byte);
        }
    }

    fn emit_alu64_imm(
        &mut self,
        reg: Register,
        imm: Immediate,
        modrm_reg: u8,
        rax_opcode: u8,
        force32: bool,
    ) {
        assert!(imm.is_int32());
        self.emit_rex64_rm(reg);

        if imm.is_int8() && !force32 {
            self.emit_u8(0x83);
            self.emit_modrm_opcode(modrm_reg, reg);
            self.emit_u8(imm.int8() as u8);
        } else if reg == RAX {
            self.emit_u8(rax_opcode);
            self.emit_u32(imm.int32() as u32);
        } else {
            self.emit_u8(0x81);
            self.emit_modrm_opcode(modrm_reg, reg);
            self.emit_u32(imm.int32() as u32);
        }
    }

    fn emit_alu32_imm(
        &mut self,
        reg: Register,
        imm: Immediate,
        modrm_reg: u8,
        rax_opcode: u8,
        force32: bool,
    ) {
        assert!(imm.is_int32());
        self.emit_rex32_rm_optional(reg);

        if imm.is_int8() && !force32 {
            self.emit_u8(0x83);
            self.emit_modrm_opcode(modrm_reg, reg);
            self.emit_u8(imm.int8() as u8);
        } else if reg == RAX {
            self.emit_u8(rax_opcode);
            self.emit_u32(imm.int32() as u32);
        } else {
            self.emit_u8(0x81);
            self.emit_modrm_opcode(modrm_reg, reg);
            self.emit_u32(imm.int32() as u32);
        }
    }
}

#[derive(Copy, Clone)]
pub enum Condition {
    Overflow,
    NoOverflow,
    Below,
    NeitherAboveNorEqual,
    NotBelow,
    AboveOrEqual,
    Equal,
    Zero,
    NotEqual,
    NotZero,
    BelowOrEqual,
    NotAbove,
    NeitherBelowNorEqual,
    Above,
    Sign,
    NoSign,
    Parity,
    ParityEven,
    NoParity,
    ParityOdd,
    Less,
    NeitherGreaterNorEqual,
    NotLess,
    GreaterOrEqual,
    LessOrEqual,
    NotGreater,
    NeitherLessNorEqual,
    Greater,
}

impl Condition {
    pub fn int(self) -> i32 {
        match self {
            Condition::Overflow => 0b0000,
            Condition::NoOverflow => 0b0001,
            Condition::Below | Condition::NeitherAboveNorEqual => 0b0010,
            Condition::NotBelow | Condition::AboveOrEqual => 0b0011,
            Condition::Equal | Condition::Zero => 0b0100,
            Condition::NotEqual | Condition::NotZero => 0b0101,
            Condition::BelowOrEqual | Condition::NotAbove => 0b0110,
            Condition::NeitherBelowNorEqual | Condition::Above => 0b0111,
            Condition::Sign => 0b1000,
            Condition::NoSign => 0b1001,
            Condition::Parity | Condition::ParityEven => 0b1010,
            Condition::NoParity | Condition::ParityOdd => 0b1011,
            Condition::Less | Condition::NeitherGreaterNorEqual => 0b1100,
            Condition::NotLess | Condition::GreaterOrEqual => 0b1101,
            Condition::LessOrEqual | Condition::NotGreater => 0b1110,
            Condition::NeitherLessNorEqual | Condition::Greater => 0b1111,
        }
    }
}

pub struct Immediate(pub i64);

impl Immediate {
    pub fn is_int8(&self) -> bool {
        let limit = 1i64 << 7;
        -limit <= self.0 && self.0 < limit
    }

    pub fn is_int32(&self) -> bool {
        let limit = 1i64 << 31;
        -limit <= self.0 && self.0 < limit
    }

    pub fn is_uint8(&self) -> bool {
        0 <= self.0 && self.0 < 256
    }

    pub fn uint8(&self) -> u8 {
        self.0 as u8
    }

    pub fn int8(&self) -> i8 {
        self.0 as i8
    }

    pub fn int32(&self) -> i32 {
        self.0 as i32
    }

    pub fn int64(&self) -> i64 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub enum ScaleFactor {
    One,
    Two,
    Four,
    Eight,
}

impl ScaleFactor {
    fn value(self) -> u8 {
        match self {
            ScaleFactor::One => 0,
            ScaleFactor::Two => 1,
            ScaleFactor::Four => 2,
            ScaleFactor::Eight => 3,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Address {
    rex: u8,
    length: u8,
    bytes: [u8; 6],
}

impl Address {
    fn new() -> Address {
        Address {
            rex: 0,
            length: 0,
            bytes: [0; 6],
        }
    }

    fn set_modrm(&mut self, mode: u8, reg: Register) {
        assert!(mode < 4);
        assert_eq!(self.length, 0);

        if reg.needs_rex() {
            self.rex |= 0x41;
        }

        self.bytes[0] = mode << 6 | reg.low_bits();
        self.length += 1;
    }

    fn set_sib(&mut self, scale: ScaleFactor, index: Register, base: Register) {
        assert_eq!(self.length, 1);

        if base.needs_rex() {
            self.rex |= 0x41;
        }

        if index.needs_rex() {
            self.rex |= 0x42;
        }

        self.bytes[1] = scale.value() << 6 | index.low_bits() << 3 | base.low_bits();
        self.length += 1;
    }

    fn set_disp8(&mut self, imm: i8) {
        assert!(self.length == 1 || self.length == 2);
        self.bytes[self.length as usize] = imm as u8;
        self.length += 1;
    }

    fn set_disp32(&mut self, imm: i32) {
        assert!(self.length == 1 || self.length == 2);
        let idx = self.length as usize;
        let imm = imm as u32;
        self.bytes[idx] = imm as u8;
        self.bytes[idx + 1] = (imm >> 8) as u8;
        self.bytes[idx + 2] = (imm >> 16) as u8;
        self.bytes[idx + 3] = (imm >> 24) as u8;
        self.length += 4;
    }

    pub fn offset(base: Register, offset: i32) -> Address {
        let mut address = Address::new();

        let mode = if offset == 0 && base != RBP {
            0b00
        } else if -128 <= offset && offset < 128 {
            0b01
        } else {
            0b10
        };

        address.set_modrm(mode, base);

        if base == RSP {
            address.set_sib(ScaleFactor::One, RSP, base);
        }

        match mode {
            0b00 => {}
            0b01 => address.set_disp8(offset as i8),
            0b10 => address.set_disp32(offset),
            _ => unreachable!(),
        }

        address
    }

    pub fn index(index: Register, factor: ScaleFactor, disp: i32) -> Address {
        let mut address = Address::new();

        address.set_modrm(0b00, RSP);
        assert_ne!(index, RSP);

        address.set_sib(factor, index, RBP);
        address.set_disp32(disp);

        address
    }

    pub fn array(base: Register, index: Register, factor: ScaleFactor, disp: i32) -> Address {
        let mut address = Address::new();

        let mode = if disp == 0 && base != RBP {
            0b00
        } else if -128 <= disp && disp < 128 {
            0b01
        } else {
            0b10
        };

        address.set_modrm(mode, RSP);
        assert_ne!(index, RSP);

        address.set_sib(factor, index, base);

        match mode {
            0b00 => {}
            0b01 => address.set_disp8(disp as i8),
            0b10 => address.set_disp32(disp),
            _ => unreachable!(),
        }

        address
    }

    pub fn rip(disp: i32) -> Address {
        let mut address = Address::new();

        address.set_modrm(0b00, RBP);
        address.set_disp32(disp);

        address
    }

    pub fn encoded_bytes(&self) -> &[u8] {
        &self.bytes[0..self.length as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::asm::*;

    macro_rules! assert_emit {
        (
            $($expr:expr),*;
            $name:ident
        ) => {{
            let mut buf = Assembler::new();
            buf.$name();
            let expected = vec![$($expr,)*];

            assert_eq!(expected, buf.code());
        }};

        (
            $($expr:expr),*;
            $name:ident
            (
                    $($param:expr),+
            )
        ) => {{
            let mut buf = Assembler::new();
            buf.$name($($param,)*);
            let expected = vec![$($expr,)*];
            let data = buf.code();

            if expected != data {
                print!("exp: ");

                for (ind, val) in expected.iter().enumerate() {
                    if ind > 0 { print!(", "); }

                    print!("{:02x}", val);
                }

                print!("\ngot: ");

                for (ind, val) in data.iter().enumerate() {
                    if ind > 0 { print!(", "); }

                    print!("{:02x}", val);
                }

                println!("");

                panic!("emitted code wrong.");
            }
        }};
    }

    #[test]
    fn test_popq_r() {
        assert_emit!(0x58; popq_r(RAX));
        assert_emit!(0x5c; popq_r(RSP));
        assert_emit!(0x41, 0x58; popq_r(R8));
        assert_emit!(0x41, 0x5F; popq_r(R15));
    }

    #[test]
    fn test_pushq_r() {
        assert_emit!(0x50; pushq_r(RAX));
        assert_emit!(0x54; pushq_r(RSP));
        assert_emit!(0x41, 0x50; pushq_r(R8));
        assert_emit!(0x41, 0x57; pushq_r(R15));
    }

    #[test]
    fn test_retq() {
        assert_emit!(0xc3; retq);
    }

    #[test]
    fn test_nop() {
        assert_emit!(0x90; nop);
    }

    #[test]
    fn test_emit_movq_rr() {
        assert_emit!(0x49, 0x89, 0xc7; movq_rr(R15, RAX));
        assert_emit!(0x4c, 0x89, 0xf8; movq_rr(RAX, R15));
        assert_emit!(0x48, 0x89, 0xe5; movq_rr(RBP, RSP));
        assert_emit!(0x48, 0x89, 0xec; movq_rr(RSP, RBP));
    }

    #[test]
    fn test_emit_movl_rr() {
        assert_emit!(0x41, 0x89, 0xc7; movl_rr(R15, RAX));
        assert_emit!(0x44, 0x89, 0xf8; movl_rr(RAX, R15));
        assert_emit!(0x89, 0xc1; movl_rr(RCX, RAX));
    }

    #[test]
    fn test_emit_addq_rr() {
        assert_emit!(0x48, 0x01, 0xD8; addq_rr(RAX, RBX));
        assert_emit!(0x4C, 0x01, 0xE0; addq_rr(RAX, R12));
        assert_emit!(0x49, 0x01, 0xC4; addq_rr(R12, RAX));
        assert_emit!(0x49, 0x01, 0xE7; addq_rr(R15, RSP));
    }

    #[test]
    fn test_emit_addl_rr() {
        assert_emit!(0x01, 0xd8; addl_rr(RAX, RBX));
        assert_emit!(0x44, 0x01, 0xf9; addl_rr(RCX, R15));
    }

    #[test]
    fn test_cdq_cqo() {
        assert_emit!(0x99; cdq);
        assert_emit!(0x48, 0x99; cqo);
    }

    #[test]
    fn test_setcc_r() {
        assert_emit!(0x0f, 0x94, 0xc0; setcc_r(Condition::Equal, RAX));
        assert_emit!(0x41, 0x0f, 0x95, 0xc7; setcc_r(Condition::NotEqual, R15));
        assert_emit!(0x0f, 0x9d, 0xc1; setcc_r(Condition::GreaterOrEqual, RCX));
        assert_emit!(0x0f, 0x9f, 0xc2; setcc_r(Condition::Greater, RDX));
        assert_emit!(0x40, 0x0f, 0x9e, 0xc6; setcc_r(Condition::LessOrEqual, RSI));
        assert_emit!(0x40, 0x0f, 0x9c, 0xc7; setcc_r(Condition::Less, RDI));
    }

    #[test]
    fn test_xorl_rr() {
        assert_emit!(0x44, 0x31, 0xf8; xorl_rr(RAX, R15));
        assert_emit!(0x31, 0xc8; xorl_rr(RAX, RCX));
        assert_emit!(0x41, 0x31, 0xc7; xorl_rr(R15, RAX));
    }

    #[test]
    fn test_xorq_rr() {
        assert_emit!(0x4C, 0x31, 0xf8; xorq_rr(RAX, R15));
        assert_emit!(0x48, 0x31, 0xc8; xorq_rr(RAX, RCX));
        assert_emit!(0x49, 0x31, 0xc7; xorq_rr(R15, RAX));
    }

    #[test]
    fn test_testl_rr() {
        assert_emit!(0x85, 0xc0; testl_rr(RAX, RAX));
        assert_emit!(0x85, 0xc6; testl_rr(RSI, RAX));
        assert_emit!(0x41, 0x85, 0xc7; testl_rr(R15, RAX));
    }

    #[test]
    fn test_testq_rr() {
        assert_emit!(0x48, 0x85, 0xc0; testq_rr(RAX, RAX));
        assert_emit!(0x48, 0x85, 0xc6; testq_rr(RSI, RAX));
        assert_emit!(0x49, 0x85, 0xc7; testq_rr(R15, RAX));
    }

    #[test]
    fn test_subl_rr() {
        assert_emit!(0x29, 0xd8; subl_rr(RAX, RBX));
        assert_emit!(0x44, 0x29, 0xf9; subl_rr(RCX, R15));
    }

    #[test]
    fn test_subq_rr() {
        assert_emit!(0x48, 0x29, 0xd8; subq_rr(RAX, RBX));
        assert_emit!(0x4c, 0x29, 0xf9; subq_rr(RCX, R15));
    }

    #[test]
    fn test_andl_rr() {
        assert_emit!(0x44, 0x21, 0xf8; andl_rr(RAX, R15));
        assert_emit!(0x21, 0xc8; andl_rr(RAX, RCX));
        assert_emit!(0x41, 0x21, 0xc7; andl_rr(R15, RAX));
    }

    #[test]
    fn test_andq_rr() {
        assert_emit!(0x4C, 0x21, 0xf8; andq_rr(RAX, R15));
        assert_emit!(0x48, 0x21, 0xc8; andq_rr(RAX, RCX));
        assert_emit!(0x49, 0x21, 0xc7; andq_rr(R15, RAX));
    }

    #[test]
    fn test_orl_rr() {
        assert_emit!(0x44, 0x09, 0xf8; orl_rr(RAX, R15));
        assert_emit!(0x09, 0xc8; orl_rr(RAX, RCX));
        assert_emit!(0x41, 0x09, 0xc7; orl_rr(R15, RAX));
    }

    #[test]
    fn test_orq_rr() {
        assert_emit!(0x4c, 0x09, 0xf8; orq_rr(RAX, R15));
        assert_emit!(0x48, 0x09, 0xc8; orq_rr(RAX, RCX));
        assert_emit!(0x49, 0x09, 0xc7; orq_rr(R15, RAX));
    }

    #[test]
    fn test_cmpl_rr() {
        assert_emit!(0x44, 0x39, 0xf8; cmpl_rr(RAX, R15));
        assert_emit!(0x41, 0x39, 0xdf; cmpl_rr(R15, RBX));
        assert_emit!(0x39, 0xd8; cmpl_rr(RAX, RBX));
    }

    #[test]
    fn test_cmpq_rr() {
        assert_emit!(0x4C, 0x39, 0xf8; cmpq_rr(RAX, R15));
        assert_emit!(0x49, 0x39, 0xdf; cmpq_rr(R15, RBX));
        assert_emit!(0x48, 0x39, 0xd8; cmpq_rr(RAX, RBX));
    }

    #[test]
    fn test_imull_rr() {
        assert_emit!(0x0f, 0xaf, 0xc3; imull_rr(RAX, RBX));
        assert_emit!(0x41, 0x0f, 0xaf, 0xcf; imull_rr(RCX, R15));
    }

    #[test]
    fn test_imulq_rr() {
        assert_emit!(0x48, 0x0f, 0xaf, 0xc3; imulq_rr(RAX, RBX));
        assert_emit!(0x49, 0x0f, 0xaf, 0xcf; imulq_rr(RCX, R15));
    }

    #[test]
    fn test_idivl_r() {
        assert_emit!(0xf7, 0xf8; idivl_r(RAX));
        assert_emit!(0x41, 0xf7, 0xff; idivl_r(R15));
    }

    #[test]
    fn test_idivq_r() {
        assert_emit!(0x48, 0xf7, 0xf8; idivq_r(RAX));
        assert_emit!(0x49, 0xf7, 0xff; idivq_r(R15));
    }

    #[test]
    fn test_call_r() {
        assert_emit!(0xff, 0xd0; call_r(RAX));
        assert_emit!(0x41, 0xff, 0xd7; call_r(R15));
    }

    #[test]
    fn test_movl_ri() {
        assert_emit!(0xb8, 2, 0, 0, 0; movl_ri(RAX, Immediate(2)));
        assert_emit!(0x41, 0xbe, 3, 0, 0, 0; movl_ri(R14, Immediate(3)));
    }

    #[test]
    fn test_movq_ri() {
        assert_emit!(0x48, 0xc7, 0xc0, 1, 0, 0, 0; movq_ri(RAX, Immediate(1)));
        assert_emit!(0x49, 0xc7, 0xc7, 0xFF, 0xFF, 0xFF, 0xFF; movq_ri(R15, Immediate(-1)));
        assert_emit!(0x48, 0xb8, 0, 0, 0, 0, 1, 0, 0, 0; movq_ri(RAX, Immediate(1 << 32)));
    }

    #[test]
    fn test_subq_ri() {
        assert_emit!(0x48, 0x83, 0xe8, 0x11; subq_ri(RAX, Immediate(0x11)));
        assert_emit!(0x49, 0x83, 0xef, 0x11; subq_ri(R15, Immediate(0x11)));
        assert_emit!(0x48, 0x2d, 0x11, 0x22, 0, 0; subq_ri(RAX, Immediate(0x2211)));
        assert_emit!(0x48, 0x81, 0xe9, 0x11, 0x22, 0, 0; subq_ri(RCX, Immediate(0x2211)));
        assert_emit!(0x49, 0x81, 0xef, 0x11, 0x22, 0, 0; subq_ri(R15, Immediate(0x2211)));
    }

    #[test]
    fn test_addq_ri() {
        assert_emit!(0x48, 0x83, 0xc0, 0x11; addq_ri(RAX, Immediate(0x11)));
        assert_emit!(0x49, 0x83, 0xc7, 0x11; addq_ri(R15, Immediate(0x11)));
        assert_emit!(0x48, 0x05, 0x11, 0x22, 0, 0; addq_ri(RAX, Immediate(0x2211)));
        assert_emit!(0x48, 0x81, 0xc1, 0x11, 0x22, 0, 0; addq_ri(RCX, Immediate(0x2211)));
        assert_emit!(0x49, 0x81, 0xc7, 0x11, 0x22, 0, 0; addq_ri(R15, Immediate(0x2211)));
    }

    #[test]
    fn test_andq_ri() {
        assert_emit!(0x48, 0x83, 0xe0, 0xf8; andq_ri(RAX, Immediate(-8)));
        assert_emit!(0x48, 0x25, 0x80, 0, 0, 0; andq_ri(RAX, Immediate(128)));
        assert_emit!(0x49, 0x83, 0xe1, 0xf8; andq_ri(R9, Immediate(-8)));
        assert_emit!(0x49, 0x81, 0xe1, 0x80, 0, 0, 0; andq_ri(R9, Immediate(128)));
    }

    #[test]
    fn test_cmpq_ri() {
        assert_emit!(0x48, 0x83, 0xf8, 0x7f; cmpq_ri(RAX, Immediate(127)));
        assert_emit!(0x49, 0x83, 0xff, 0x80; cmpq_ri(R15, Immediate(-128)));
        assert_emit!(0x49, 0x83, 0xf9, 0; cmpq_ri(R9, Immediate(0)));
    }

    #[test]
    fn test_cmpl_ri() {
        assert_emit!(0x83, 0xf8, 0; cmpl_ri(RAX, Immediate(0)));
        assert_emit!(0x41, 0x83, 0xff, 0; cmpl_ri(R15, Immediate(0)));
        assert_emit!(0x41, 0x83, 0xf9, 0; cmpl_ri(R9, Immediate(0)));
    }

    #[test]
    fn test_cmov() {
        assert_emit!(0x44, 0x0f, 0x44, 0xf8; cmovl(Condition::Equal, R15, RAX));
        assert_emit!(0x41, 0x0f, 0x45, 0xc5; cmovl(Condition::NotEqual, RAX, R13));
        assert_emit!(0x48, 0x0f, 0x4f, 0xc1; cmovq(Condition::Greater, RAX, RCX));
    }

    #[test]
    fn test_notl() {
        assert_emit!(0xf7, 0xd0; notl(RAX));
        assert_emit!(0x41, 0xf7, 0xd7; notl(R15));
    }

    #[test]
    fn test_notq() {
        assert_emit!(0x48, 0xf7, 0xd0; notq(RAX));
        assert_emit!(0x49, 0xf7, 0xd7; notq(R15));
    }

    #[test]
    fn test_negl() {
        assert_emit!(0xf7, 0xd8; negl(RAX));
        assert_emit!(0x41, 0xf7, 0xdf; negl(R15));
    }

    #[test]
    fn test_negq() {
        assert_emit!(0x48, 0xf7, 0xd8; negq(RAX));
        assert_emit!(0x49, 0xf7, 0xdf; negq(R15));
    }

    #[test]
    fn test_movzxb_rr() {
        assert_emit!(0x0f, 0xb6, 0xc0; movzxb_rr(RAX, RAX));
        assert_emit!(0x40, 0x0f, 0xb6, 0xc7; movzxb_rr(RAX, RDI));
        assert_emit!(0x0f, 0xb6, 0xf8; movzxb_rr(RDI, RAX));
        assert_emit!(0x41, 0x0f, 0xb6, 0xc7; movzxb_rr(RAX, R15));
        assert_emit!(0x44, 0x0f, 0xb6, 0xfb; movzxb_rr(R15, RBX));
        assert_emit!(0x40, 0x0f, 0xb6, 0xce; movzxb_rr(RCX, RSI));
    }

    #[test]
    #[should_panic]
    fn test_address_array_with_rsp_index() {
        Address::array(RAX, RSP, ScaleFactor::Four, 0);
    }

    #[test]
    #[should_panic]
    fn test_address_index_with_rsp_index() {
        Address::index(RSP, ScaleFactor::Four, 0);
    }

    #[test]
    fn test_movq_ra() {
        assert_emit!(0x48, 0x8b, 0x04, 0x24; movq_ra(RAX, Address::offset(RSP, 0)));
        assert_emit!(0x48, 0x8b, 0x44, 0x24, 1; movq_ra(RAX, Address::offset(RSP, 1)));

        assert_emit!(0x4c, 0x8b, 0x3c, 0x24; movq_ra(R15, Address::offset(RSP, 0)));
        assert_emit!(0x4c, 0x8b, 0x7c, 0x24, 1; movq_ra(R15, Address::offset(RSP, 1)));

        assert_emit!(0x4c, 0x8b, 0x7c, 0x24, 0x7f; movq_ra(R15, Address::offset(RSP, 127)));
        assert_emit!(0x4c, 0x8b, 0x7c, 0x24, 0x80; movq_ra(R15, Address::offset(RSP, -128)));

        assert_emit!(0x4c, 0x8b, 0xbc, 0x24, 0x80, 0, 0, 0; movq_ra(R15, Address::offset(RSP, 128)));
        assert_emit!(0x4c, 0x8b, 0xbc, 0x24, 0x7f, 0xff, 0xff, 0xff; movq_ra(R15, Address::offset(RSP, -129)));

        assert_emit!(0x48, 0x8b, 0x45, 0; movq_ra(RAX, Address::offset(RBP, 0)));
        assert_emit!(0x48, 0x8b, 0x45, 1; movq_ra(RAX, Address::offset(RBP, 1)));

        assert_emit!(0x48, 0x8b, 0x04, 0xf8; movq_ra(RAX, Address::array(RAX, RDI, ScaleFactor::Eight, 0)));
        assert_emit!(0x48, 0x8b, 0x44, 0xf8, 1; movq_ra(RAX, Address::array(RAX, RDI, ScaleFactor::Eight, 1)));
        assert_emit!(0x48, 0x8b, 0x44, 0xf8, 0x7f; movq_ra(RAX, Address::array(RAX, RDI, ScaleFactor::Eight, 127)));
        assert_emit!(0x48, 0x8b, 0x44, 0xf8, 0x80; movq_ra(RAX, Address::array(RAX, RDI, ScaleFactor::Eight, -128)));
        assert_emit!(0x48, 0x8b, 0x84, 0xf8, 0x80, 0, 0, 0; movq_ra(RAX, Address::array(RAX, RDI, ScaleFactor::Eight, 128)));
        assert_emit!(0x48, 0x8b, 0x84, 0xf8, 0x7f, 0xff, 0xff, 0xff; movq_ra(RAX, Address::array(RAX, RDI, ScaleFactor::Eight, -129)));

        assert_emit!(0x48, 0x8b, 0x44, 0xed, 0; movq_ra(RAX, Address::array(RBP, RBP, ScaleFactor::Eight, 0)));
        assert_emit!(0x48, 0x8b, 0x04, 0xe8; movq_ra(RAX, Address::array(RAX, RBP, ScaleFactor::Eight, 0)));
        assert_emit!(0x48, 0x8b, 0x44, 0xc5, 0; movq_ra(RAX, Address::array(RBP, RAX, ScaleFactor::Eight, 0)));

        assert_emit!(0x48, 0x8b, 0x04, 0xed, 0, 0, 0, 0; movq_ra(RAX, Address::index(RBP, ScaleFactor::Eight, 0)));
        assert_emit!(0x48, 0x8b, 0x04, 0xed, 1, 0, 0, 0; movq_ra(RAX, Address::index(RBP, ScaleFactor::Eight, 1)));
    }

    #[test]
    fn test_movq_ar() {
        assert_emit!(0x48, 0x89, 0x45, 0; movq_ar(Address::offset(RBP, 0), RAX));
        assert_emit!(0x48, 0x89, 0x44, 0xa8, 1; movq_ar(Address::array(RAX, RBP, ScaleFactor::Four, 1), RAX));
    }

    #[test]
    fn test_movl_ar() {
        assert_emit!(0x89, 0x45, 0; movl_ar(Address::offset(RBP, 0), RAX));
        assert_emit!(0x44, 0x89, 0x7d, 0; movl_ar(Address::offset(RBP, 0), R15));
        assert_emit!(0x45, 0x89, 0x38; movl_ar(Address::offset(R8, 0), R15));
        assert_emit!(0x47, 0x89, 0x3c, 0x88; movl_ar(Address::array(R8, R9, ScaleFactor::Four, 0), R15));
        assert_emit!(0x89, 0x44, 0xa8, 1; movl_ar(Address::array(RAX, RBP, ScaleFactor::Four, 1), RAX));
    }

    #[test]
    fn test_movl_ra() {
        assert_emit!(0x8b, 0x45, 0; movl_ra(RAX, Address::offset(RBP, 0)));
        assert_emit!(0x8b, 0x05, 0, 0, 0, 0; movl_ra(RAX, Address::rip(0)));
    }

    #[test]
    fn test_cmpq_ar() {
        assert_emit!(0x48, 0x39, 0x43, 1; cmpq_ar(Address::offset(RBX, 1), RAX));
        assert_emit!(0x48, 0x39, 0x83, 0, 1, 0, 0; cmpq_ar(Address::offset(RBX, 256), RAX));
        assert_emit!(0x48, 0x39, 0x47, 1; cmpq_ar(Address::offset(RDI, 1), RAX));
        assert_emit!(0x49, 0x39, 0x41, 1; cmpq_ar(Address::offset(R9, 1), RAX));
        assert_emit!(0x4c, 0x39, 0x57, 1; cmpq_ar(Address::offset(RDI, 1), R10));
        assert_emit!(0x48, 0x39, 0x05, 1, 0, 0, 0; cmpq_ar(Address::rip(1), RAX));
    }

    #[test]
    fn test_cmpl_ar() {
        assert_emit!(0x39, 0x43, 1; cmpl_ar(Address::offset(RBX, 1), RAX));
        assert_emit!(0x44, 0x39, 0x53, 1; cmpl_ar(Address::offset(RBX, 1), R10));
    }

    #[test]
    fn test_testl_ar() {
        assert_emit!(0x85, 0x47, 1; testl_ar(Address::offset(RDI, 1), RAX));
        assert_emit!(0x85, 0x07; testl_ar(Address::offset(RDI, 0), RAX));

        assert_emit!(0x41, 0x85, 0x00; testl_ar(Address::offset(R8, 0), RAX));
        assert_emit!(0x41, 0x85, 0x40, 1; testl_ar(Address::offset(R8, 1), RAX));
        assert_emit!(0x45, 0x85, 0x38; testl_ar(Address::offset(R8, 0), R15));

        assert_emit!(0x44, 0x85, 0x38; testl_ar(Address::offset(RAX, 0), R15));
    }

    #[test]
    fn test_testq_ar() {
        assert_emit!(0x48, 0x85, 0x47, 1; testq_ar(Address::offset(RDI, 1), RAX));
        assert_emit!(0x4D, 0x85, 0x38; testq_ar(Address::offset(R8, 0), R15));
    }

    #[test]
    fn test_testl_ai() {
        assert_emit!(0xf7, 0x00, 1, 0, 0, 0; testl_ai(Address::offset(RAX, 0), Immediate(1)));
        assert_emit!(0xf7, 0x40, 1, 1, 0, 0, 0; testl_ai(Address::offset(RAX, 1), Immediate(1)));

        assert_emit!(0x41, 0xf7, 0x00, 1, 0, 0, 0; testl_ai(Address::offset(R8, 0), Immediate(1)));
    }

    #[test]
    fn test_testq_ai() {
        assert_emit!(0x48, 0xf7, 0x00, 1, 0, 0, 0; testq_ai(Address::offset(RAX, 0), Immediate(1)));
        assert_emit!(0x48, 0xf7, 0x40, 1, 1, 0, 0, 0; testq_ai(Address::offset(RAX, 1), Immediate(1)));

        assert_emit!(0x49, 0xf7, 0x00, 1, 0, 0, 0; testq_ai(Address::offset(R8, 0), Immediate(1)));
    }

    #[test]
    fn test_testl_ri() {
        assert_emit!(0xa8, 1; testl_ri(RAX, Immediate(1)));
        assert_emit!(0xf6, 0xc1, 255; testl_ri(RCX, Immediate(255)));
        assert_emit!(0xf6, 0xc2, 1; testl_ri(RDX, Immediate(1)));
        assert_emit!(0xf6, 0xc3, 1; testl_ri(RBX, Immediate(1)));

        assert_emit!(0x40, 0xf6, 0xc6, 1; testl_ri(RSI, Immediate(1)));
        assert_emit!(0x40, 0xf6, 0xc7, 1; testl_ri(RDI, Immediate(1)));
        assert_emit!(0x41, 0xf6, 0xc0, 1; testl_ri(R8, Immediate(1)));
        assert_emit!(0x41, 0xf6, 0xc7, 1; testl_ri(R15, Immediate(1)));

        assert_emit!(0xa9, 0, 1, 0, 0; testl_ri(RAX, Immediate(256)));
        assert_emit!(0xf7, 0xc7, 0, 1, 0, 0; testl_ri(RDI, Immediate(256)));
    }

    #[test]
    fn test_lea() {
        assert_emit!(0x48, 0x8d, 0x00; lea(RAX, Address::offset(RAX, 0)));
        assert_emit!(0x48, 0x8d, 0x40, 1; lea(RAX, Address::offset(RAX, 1)));
        assert_emit!(0x49, 0x8d, 0x00; lea(RAX, Address::offset(R8, 0)));
        assert_emit!(0x4c, 0x8d, 0x00; lea(R8, Address::offset(RAX, 0)));
    }

    #[test]
    fn test_movb_ar() {
        assert_emit!(0x88, 0x04, 0x24; movb_ar(Address::offset(RSP, 0), RAX));
        assert_emit!(0x40, 0x88, 0x34, 0x24; movb_ar(Address::offset(RSP, 0), RSI));
        assert_emit!(0x44, 0x88, 0x04, 0x24; movb_ar(Address::offset(RSP, 0), R8));
    }

    #[test]
    fn test_movzxb_ra() {
        assert_emit!(0x0f, 0xb6, 0x00; movzxb_ra(RAX, Address::offset(RAX, 0)));
        assert_emit!(0x44, 0x0f, 0xb6, 0x00; movzxb_ra(R8, Address::offset(RAX, 0)));
        assert_emit!(0x41, 0x0f, 0xb6, 0x00; movzxb_ra(RAX, Address::offset(R8, 0)));
    }

    #[test]
    fn test_movsxlq_rr() {
        assert_emit!(0x4c, 0x63, 0xf8; movsxlq_rr(R15, RAX));
        assert_emit!(0x49, 0x63, 0xc7; movsxlq_rr(RAX, R15));
    }

    #[test]
    fn test_movsxbl_rr() {
        assert_emit!(0x0f, 0xbe, 0xc0; movsxbl_rr(RAX, RAX));
        assert_emit!(0x41, 0x0f, 0xbe, 0xc0; movsxbl_rr(RAX, R8));
        assert_emit!(0x0f, 0xbe, 0xe0; movsxbl_rr(RSP, RAX));
        assert_emit!(0x44, 0x0f, 0xbe, 0xf8; movsxbl_rr(R15, RAX));

        assert_emit!(0x0f, 0xbe, 0xc3; movsxbl_rr(RAX, RBX));
        assert_emit!(0x40, 0x0f, 0xbe, 0xc4; movsxbl_rr(RAX, RSP));
        assert_emit!(0x40, 0x0f, 0xbe, 0xc7; movsxbl_rr(RAX, RDI));
    }

    #[test]
    fn test_movsxbl_ra() {
        assert_emit!(0x0f, 0xbe, 0x00; movsxbl_ra(RAX, Address::offset(RAX, 0)));
        assert_emit!(0x44, 0x0f, 0xbe, 0x00; movsxbl_ra(R8, Address::offset(RAX, 0)));
        assert_emit!(0x0f, 0xbe, 0x20; movsxbl_ra(RSP, Address::offset(RAX, 0)));
        assert_emit!(0x0f, 0xbe, 0x38; movsxbl_ra(RDI, Address::offset(RAX, 0)));
    }

    #[test]
    fn test_movsxbq_rr() {
        assert_emit!(0x48, 0x0f, 0xbe, 0xc0; movsxbq_rr(RAX, RAX));
        assert_emit!(0x49, 0x0f, 0xbe, 0xc0; movsxbq_rr(RAX, R8));
        assert_emit!(0x48, 0x0f, 0xbe, 0xe0; movsxbq_rr(RSP, RAX));
        assert_emit!(0x4c, 0x0f, 0xbe, 0xf8; movsxbq_rr(R15, RAX));

        assert_emit!(0x48, 0x0f, 0xbe, 0xc3; movsxbq_rr(RAX, RBX));
        assert_emit!(0x48, 0x0f, 0xbe, 0xc4; movsxbq_rr(RAX, RSP));
        assert_emit!(0x48, 0x0f, 0xbe, 0xc7; movsxbq_rr(RAX, RDI));
    }

    #[test]
    fn test_movsxbq_ra() {
        assert_emit!(0x48, 0x0f, 0xbe, 0x00; movsxbq_ra(RAX, Address::offset(RAX, 0)));
        assert_emit!(0x4c, 0x0f, 0xbe, 0x00; movsxbq_ra(R8, Address::offset(RAX, 0)));
        assert_emit!(0x48, 0x0f, 0xbe, 0x20; movsxbq_ra(RSP, Address::offset(RAX, 0)));
        assert_emit!(0x48, 0x0f, 0xbe, 0x38; movsxbq_ra(RDI, Address::offset(RAX, 0)));
    }

    #[test]
    fn test_movb_ai() {
        assert_emit!(0xc6, 0x00, 1; movb_ai(Address::offset(RAX, 0), Immediate(1)));
        assert_emit!(0xc6, 0x00, 0x7f; movb_ai(Address::offset(RAX, 0), Immediate(127)));
        assert_emit!(0xc6, 0x00, 0xff; movb_ai(Address::offset(RAX, 0), Immediate(255)));
        assert_emit!(0xc6, 0x00, 0x80; movb_ai(Address::offset(RAX, 0), Immediate(-128)));
        assert_emit!(0xc6, 0x00, 0x80; movb_ai(Address::offset(RAX, 0), Immediate(128)));
    }

    #[test]
    fn test_xorl_ri() {
        assert_emit!(0x83, 0xf0, 1; xorl_ri(RAX, Immediate(1)));
        assert_emit!(0x41, 0x83, 0xf0, 0x7f; xorl_ri(R8, Immediate(127)));
        assert_emit!(0x41, 0x83, 0xf7, 0x80; xorl_ri(R15, Immediate(-128)));

        assert_emit!(0x35, 0x80, 0, 0, 0; xorl_ri(RAX, Immediate(128)));
        assert_emit!(0x41, 0x81, 0xf0, 0x7f, 0xff, 0xff, 0xff; xorl_ri(R8, Immediate(-129)));
        assert_emit!(0x41, 0x81, 0xf7, 0x80, 0, 0, 0; xorl_ri(R15, Immediate(128)));
    }
}
