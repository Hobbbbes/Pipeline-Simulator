use crate::cpu;
use std::todo;

#[derive(Clone, Copy)]
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
    pub rt: u8, //for regimm instruction function field
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

pub fn bgez(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn bgezal(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn bgtz(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn blez(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn bltz(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn bltzal(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn bne(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn div(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn divu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn j(cpu: &mut cpu::MipsCpu<'_>, i: JTypeInstruction) {
    todo!()
}

pub fn jal(cpu: &mut cpu::MipsCpu<'_>, i: JTypeInstruction) {
    todo!()
}

pub fn jalr(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn jr(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn lb(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn lbu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn lh(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn lhu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn lui(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn lw(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn lwl(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn lwr(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn mfhi(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn mflo(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn mthi(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn mtlo(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn mult(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn multu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn nor(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn or(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn ori(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn sb(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn sh(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn sll(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn sllv(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn slt(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn slti(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn sltiu(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn sltu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn sra(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn srav(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn srl(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn srlv(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn sub(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn subu(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn sw(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn swl(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn swr(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}

pub fn xor(cpu: &mut cpu::MipsCpu<'_>, i: RTypeInstruction) {
    todo!()
}

pub fn xori(cpu: &mut cpu::MipsCpu<'_>, i: ITypeInstruction) {
    todo!()
}
