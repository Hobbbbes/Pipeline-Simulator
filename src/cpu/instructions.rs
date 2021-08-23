use crate::cpu;
use std::todo;
pub struct OpDecodedInstruction {
    pub op: u8,
    other: u32,
}

impl OpDecodedInstruction {
    pub fn decode(i: u32) -> Self {
        OpDecodedInstruction {
            op: ((i & 0xFC000000) >> 26) as u8,
            other: i,
        }
    }
}

pub trait FromOpDecodedInstruction {
    fn decode(i: OpDecodedInstruction) -> Self;
}

pub struct ITypeInstruction {
    pub op: u8,
    rs: u8,
    rt: u8,
    immediate: u16,
}

impl FromOpDecodedInstruction for ITypeInstruction {
    fn decode(i: OpDecodedInstruction) -> Self {
        let im = i.other & 0x0000FFFF;
        let rt = (i.other & 0x001F0000) >> 16;
        let rs = (i.other & 0x03E00000) >> 21;
        ITypeInstruction {
            op: i.op,
            rs: rs as u8,
            rt: rt as u8,
            immediate: im as u16,
        }
    }
}

pub struct JTypeInstruction {
    pub op: u8,
    target: u32,
}

impl FromOpDecodedInstruction for JTypeInstruction {
    fn decode(i: OpDecodedInstruction) -> Self {
        JTypeInstruction {
            op: i.op,
            target: i.other & 0x03FFFFFF,
        }
    }
}

pub struct RTypeInstruction {
    pub op: u8,
    rs: u8,
    rt: u8,
    rd: u8,
    shamt: u8,
    pub funct: u8,
}

impl RTypeInstruction {
    pub fn new() -> Self {
        RTypeInstruction {
            op: 0,
            rs: 0,
            rt: 0,
            rd: 0,
            shamt: 0,
            funct: 0,
        }
    }
}

impl FromOpDecodedInstruction for RTypeInstruction {
    fn decode(i: OpDecodedInstruction) -> Self {
        let rs = (i.other & 0x03E00000) >> 21;
        let rt = (i.other & 0x001F0000) >> 16;
        let rd = (i.other & 0x0000F800) >> 11;
        let shamt = (i.other & 0x000007C0) >> 6;
        let funct = i.other & 0x0000003F;
        RTypeInstruction {
            op: i.op,
            rs: rs as u8,
            rt: rt as u8,
            rd: rd as u8,
            shamt: shamt as u8,
            funct: funct as u8,
        }
    }
}

//add 2 signed integers
//throw Integer overflow exception on overflow
pub fn add(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    let x = cpu.get_register(i.rs);
    let y = cpu.get_register(i.rt);
    cpu.set_register(i.rd, ((x as i32) + (y as i32)) as u32);
}
//add content of one register to immediate sign extended value
//throw Integer overflow exception on overflow
pub fn addi(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    let x = cpu.get_register(i.rs) as i32;
    let imm = i.immediate as i16 as i32;
    cpu.set_register(i.rt, (x + imm) as u32);
}

pub fn addiu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn addu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn and(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn andi(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn beq(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}
