use super::instructions::*;
use crate::cpu;
pub struct InstructionInfo<T> {
    pub memonic: &'static str,
    pub decodedInstruction: T,
    pub f: fn(&mut cpu::MipsCpu<'_>, T),
}

impl<T> InstructionInfo<T> {
    pub fn new(memonic: &'static str, decodedInstruction: T, f: fn(&mut cpu::MipsCpu, T)) -> Self {
        InstructionInfo {
            memonic,
            decodedInstruction,
            f,
        }
    }
}

pub enum InstructionInfos {
    IType(InstructionInfo<ITypeInstruction>),
    JType(InstructionInfo<JTypeInstruction>),
    RType(InstructionInfo<RTypeInstruction>),
}

impl InstructionInfos {
    pub fn new_i(i: InstructionInfo<ITypeInstruction>) -> Self {
        InstructionInfos::IType(i)
    }
    pub fn new_j(i: InstructionInfo<JTypeInstruction>) -> Self {
        InstructionInfos::JType(i)
    }

    pub fn new_r(i: InstructionInfo<RTypeInstruction>) -> Self {
        InstructionInfos::RType(i)
    }
}

pub fn decode_opcode(opI: OpDecodedInstruction) -> InstructionInfos {
    match opI.op {
        0b000000 => decode_rtype(opI),
        0b001000 => InstructionInfos::new_i(InstructionInfo::new(
            "addi",
            ITypeInstruction::decode(opI),
            addi,
        )),
        0b001001 => InstructionInfos::new_i(InstructionInfo::new(
            "addiu",
            ITypeInstruction::decode(opI),
            addiu,
        )),
        0b001100 => InstructionInfos::new_i(InstructionInfo::new(
            "andi",
            ITypeInstruction::decode(opI),
            andi,
        )),
        0b000100 => InstructionInfos::new_i(InstructionInfo::new(
            "beq",
            ITypeInstruction::decode(opI),
            beq,
        )),
    }
}

pub fn decode_rtype(opI: OpDecodedInstruction) -> InstructionInfos {
    let decoded = RTypeInstruction::decode(opI);
    match decoded.funct {
        0b100000 => InstructionInfos::new_r(InstructionInfo::new("add", decoded, add)),
        0b100001 => InstructionInfos::new_r(InstructionInfo::new("addu", decoded, addu)),
        0b100100 => InstructionInfos::new_r(InstructionInfo::new("and", decoded, and)),
    }
}
