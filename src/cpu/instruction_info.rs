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
        0b010000 | 0b010001 | 0b010010 | 0b010011 => {
            todo!("Coprocessor Instruction or move from/to coprocessor")
        }
        0b000010 => {
            InstructionInfos::new_j(InstructionInfo::new("j", JTypeInstruction::decode(op_i), j))
        }
        0b000011 => InstructionInfos::new_j(InstructionInfo::new(
            "jal",
            JTypeInstruction::decode(op_i),
            jal,
        )),
        0b100000 => InstructionInfos::new_i(InstructionInfo::new(
            "lb",
            ITypeInstruction::decode(op_i),
            lb,
        )),
        0b100100 => InstructionInfos::new_i(InstructionInfo::new(
            "lbu",
            ITypeInstruction::decode(op_i),
            lbu,
        )),
        0b100001 => InstructionInfos::new_i(InstructionInfo::new(
            "lh",
            ITypeInstruction::decode(op_i),
            lh,
        )),
        0b100101 => InstructionInfos::new_i(InstructionInfo::new(
            "lhu",
            ITypeInstruction::decode(op_i),
            lhu,
        )),
        0b001111 => InstructionInfos::new_i(InstructionInfo::new(
            "lui",
            ITypeInstruction::decode(op_i),
            lui,
        )),
        0b100011 => InstructionInfos::new_i(InstructionInfo::new(
            "lw",
            ITypeInstruction::decode(op_i),
            lw,
        )),
        0b110000 | 0b110001 | 0b110010 | 0b110011 => todo!("Load word to coprocessor"),
        0b100010 => InstructionInfos::new_i(InstructionInfo::new(
            "lwl",
            ITypeInstruction::decode(op_i),
            lwl,
        )),
        0b100110 => InstructionInfos::new_i(InstructionInfo::new(
            "lwr",
            ITypeInstruction::decode(op_i),
            lwr,
        )),
        0b001101 => InstructionInfos::new_i(InstructionInfo::new(
            "ori",
            ITypeInstruction::decode(op_i),
            ori,
        )),
        0b101000 => InstructionInfos::new_i(InstructionInfo::new(
            "sb",
            ITypeInstruction::decode(op_i),
            sb,
        )),
        0b101001 => InstructionInfos::new_i(InstructionInfo::new(
            "sh",
            ITypeInstruction::decode(op_i),
            sh,
        )),
        0b001010 => InstructionInfos::new_i(InstructionInfo::new(
            "slti",
            ITypeInstruction::decode(op_i),
            slti,
        )),
        0b001011 => InstructionInfos::new_i(InstructionInfo::new(
            "sltiu",
            ITypeInstruction::decode(op_i),
            sltiu,
        )),
        0b101011 => InstructionInfos::new_i(InstructionInfo::new(
            "sw",
            ITypeInstruction::decode(op_i),
            sw,
        )),
        0b111000 | 0b111001 | 0b111010 | 0b111011 => todo!("Store word from Coprocessor"),
        0b101010 => InstructionInfos::new_i(InstructionInfo::new(
            "swl",
            ITypeInstruction::decode(op_i),
            swl,
        )),
        0b101110 => InstructionInfos::new_i(InstructionInfo::new(
            "swr",
            ITypeInstruction::decode(op_i),
            swr,
        )),
        0b001110 => InstructionInfos::new_i(InstructionInfo::new(
            "xori",
            ITypeInstruction::decode(op_i),
            xori,
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
        0b001101 => panic!("Break instruction"),
        0b011010 => InstructionInfos::new_r(InstructionInfo::new("div", decoded, div)),
        0b011011 => InstructionInfos::new_r(InstructionInfo::new("divu", decoded, divu)),
        0b001001 => InstructionInfos::new_r(InstructionInfo::new("jalr", decoded, jalr)),
        0b001000 => InstructionInfos::new_r(InstructionInfo::new("jr", decoded, jr)),
        0b010000 => InstructionInfos::new_r(InstructionInfo::new("mfhi", decoded, mfhi)),
        0b010010 => InstructionInfos::new_r(InstructionInfo::new("mflo", decoded, mflo)),
        0b010001 => InstructionInfos::new_r(InstructionInfo::new("mthi", decoded, mthi)),
        0b010011 => InstructionInfos::new_r(InstructionInfo::new("mtlo", decoded, mtlo)),
        0b011000 => InstructionInfos::new_r(InstructionInfo::new("mult", decoded, mult)),
        0b011001 => InstructionInfos::new_r(InstructionInfo::new("multu", decoded, multu)),
        0b100111 => InstructionInfos::new_r(InstructionInfo::new("nor", decoded, nor)),
        0b100101 => InstructionInfos::new_r(InstructionInfo::new("or", decoded, or)),
        0b000000 => InstructionInfos::new_r(InstructionInfo::new("sll", decoded, sll)),
        0b000100 => InstructionInfos::new_r(InstructionInfo::new("sllv", decoded, sllv)),
        0b101010 => InstructionInfos::new_r(InstructionInfo::new("slt", decoded, slt)),
        0b101011 => InstructionInfos::new_r(InstructionInfo::new("sltu", decoded, sltu)),
        0b000011 => InstructionInfos::new_r(InstructionInfo::new("sra", decoded, sra)),
        0b000111 => InstructionInfos::new_r(InstructionInfo::new("srav", decoded, srav)),
        0b000010 => InstructionInfos::new_r(InstructionInfo::new("srl", decoded, srl)),
        0b000110 => InstructionInfos::new_r(InstructionInfo::new("srlv", decoded, srlv)),
        0b100010 => InstructionInfos::new_r(InstructionInfo::new("sub", decoded, sub)),
        0b100011 => InstructionInfos::new_r(InstructionInfo::new("subu", decoded, subu)),
        0b001100 => panic!("Syscall instruction"),
        0b100110 => InstructionInfos::new_r(InstructionInfo::new("xor", decoded, xor)),
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
