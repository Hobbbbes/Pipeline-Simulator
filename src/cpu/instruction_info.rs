use super::instructions::*;
use crate::cpu;
pub struct InstructionInfo<T> {
    pub memonic: &'static str,
    pub decoded_instruction: T,
    pub f: fn(&mut cpu::MipsCpu<'_>, T),
}

impl<T> InstructionInfo<T> {
    pub fn new(memonic: &'static str, decoded_instruction: T, f: fn(&mut cpu::MipsCpu, T)) -> Self {
        InstructionInfo {
            memonic,
            decoded_instruction,
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

pub fn decode_opcode(op_i: OpDecodedInstruction) -> InstructionInfos {
    match op_i.op {
        0b000000 => decode_rtype(op_i),
        0b000001 => decode_regimm(op_i),
        0b001000 => InstructionInfos::new_i(InstructionInfo::new(
            "addi",
            ITypeInstruction::decode(op_i),
            addi,
        )),
        0b001001 => InstructionInfos::new_i(InstructionInfo::new(
            "addiu",
            ITypeInstruction::decode(op_i),
            addiu,
        )),
        0b001100 => InstructionInfos::new_i(InstructionInfo::new(
            "andi",
            ITypeInstruction::decode(op_i),
            andi,
        )),
        0b000100 => InstructionInfos::new_i(InstructionInfo::new(
            "beq",
            ITypeInstruction::decode(op_i),
            beq,
        )),
        0b000111 => InstructionInfos::new_i(InstructionInfo::new(
            "bgtz",
            ITypeInstruction::decode(op_i),
            bgtz,
        )),
        0b000110 => InstructionInfos::new_i(InstructionInfo::new(
            "blez",
            ITypeInstruction::decode(op_i),
            blez,
        )),
        0b000101 => InstructionInfos::new_i(InstructionInfo::new(
            "bne",
            ITypeInstruction::decode(op_i),
            bne,
        )),
        _ => panic!("unknown instruction"),
    }
}

pub fn decode_rtype(op_i: OpDecodedInstruction) -> InstructionInfos {
    let decoded = RTypeInstruction::decode(op_i);
    match decoded.funct {
        0b100000 => InstructionInfos::new_r(InstructionInfo::new("add", decoded, add)),
        0b100001 => InstructionInfos::new_r(InstructionInfo::new("addu", decoded, addu)),
        0b100100 => InstructionInfos::new_r(InstructionInfo::new("and", decoded, and)),
        _ => panic!("unknown instruction"),
    }
}

//decodes register immediate branch instructions in IType that only need one register
pub fn decode_regimm(op_i: OpDecodedInstruction) -> InstructionInfos {
    let decoded = ITypeInstruction::decode(op_i);
    match decoded.rt {
        0b00001 => InstructionInfos::new_i(InstructionInfo::new("bgez", decoded, bgez)),
        0b10001 => InstructionInfos::new_i(InstructionInfo::new("bgezal", decoded, bgezal)),
        0b00000 => InstructionInfos::new_i(InstructionInfo::new("bltz", decoded, bltz)),
        0b10000 => InstructionInfos::new_i(InstructionInfo::new("bltzal", decoded, bltzal)),
        _ => panic!("unknown instruction"),
    }
}
