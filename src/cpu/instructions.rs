use crate::cpu;

pub struct ITypeInstruction {
    op: u8,
    rs: u8,
    rt: u8,
    immediate: u16,
}

impl ITypeInstruction {
    pub fn decode(i: u32) -> Self {
        let im = i & 0x0000FFFF;
        let rt = (i & 0x001F0000) >> 16;
        let rs = (i & 0x03E00000) >> 21;
        let op = (i & 0xFC000000) >> 26;
        ITypeInstruction {
            op: op as u8,
            rs: rs as u8,
            rt: rt as u8,
            immediate: im as u16,
        }
    }
}

pub struct JTypeInstruction {
    op: u8,
    target: u32,
}

impl JTypeInstruction {}

pub struct RTypeInstruction {
    op: u8,
    rs: u8,
    rt: u8,
    rd: u8,
    shamt: u8,
    funct: u8,
}

pub enum Instruction {
    IType(ITypeInstruction),
    JType(JTypeInstruction),
    RType(RTypeInstruction),
}

impl cpu::MipsCpu<'_> {}
