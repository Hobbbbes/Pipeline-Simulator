mod instruction_info;
pub mod instructions;
use crate::bus_objects;
use crate::bus_objects::BusObject;
use instruction_info::*;
use instructions::*;
const MIPS_REGISTER_NAMES: [&str; 32] = [
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
    "s8|fp", "ra",
];

pub struct MipsCpu<'a> {
    //Always big endian
    general_registers: [u32; 31],

    //Depending on host architecture
    hi: u32,
    //Depending on host architecture
    lo: u32,
    //fpu_register: [u32; 2],

    //Depending on host architecture
    pc: u32,
    pub bus: &'a mut bus_objects::Bus,

    branch: bool,
    //Depending on host architecture
    branch_target: u32,
}

impl<'a> MipsCpu<'a> {
    pub fn new(bus: &'a mut bus_objects::Bus, pc: u32) -> Self {
        MipsCpu {
            general_registers: [0; 31],
            hi: 0,
            lo: 0,
            pc,
            bus,
            branch: false,
            branch_target: 0,
        }
    }

    #[inline]
    pub fn set_stack_start(&mut self, v: u32) {
        self.set_register(29, v);
        self.set_register(30, v);
    }

    pub fn init_gp(&mut self, v: u32) {
        self.set_register_nc(28, v);
    }

    #[inline]
    fn get_register(&self, index: u8) -> u32 {
        match index {
            0 => 0,
            r => u32::from_be(self.general_registers[(r as usize) - 1]),
        }
    }
    #[inline]
    fn get_register_nc(&self, index: u8) -> u32 {
        match index {
            0 => 0,
            r => self.general_registers[(r as usize) - 1],
        }
    }

    #[inline]
    fn set_register(&mut self, index: u8, value: u32) {
        self.set_register_nc(index, u32::from_be(value));
    }
    //Set Register without conversion to Big Endian
    #[inline]
    fn set_register_nc(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.general_registers[(index as usize) - 1] = value;
        }
    }
    pub fn step(&mut self) {
        let i_w = u32::from_be(self.bus.read_w(self.pc));
        let first_stage = OpDecodedInstruction::decode(i_w);
        let second_stage = decode_opcode(first_stage);
        let branch = self.branch;
        match second_stage {
            InstructionInfos::RType(i) => {
                println!(
                    "{:#04X?}: {} {},{},{}",
                    self.pc,
                    i.memonic,
                    MIPS_REGISTER_NAMES[i.decoded_instruction.rd() as usize],
                    MIPS_REGISTER_NAMES[i.decoded_instruction.rs() as usize],
                    MIPS_REGISTER_NAMES[i.decoded_instruction.rt() as usize]
                );
                self.execute(i);
            }
            InstructionInfos::IType(i) => {
                println!(
                    "{:#04X?}: {} {},{},{}",
                    self.pc,
                    i.memonic,
                    MIPS_REGISTER_NAMES[i.decoded_instruction.rt() as usize],
                    MIPS_REGISTER_NAMES[i.decoded_instruction.rs() as usize],
                    i.decoded_instruction.immediate() as i16
                );
                self.execute(i);
            }
            InstructionInfos::JType(i) => {
                println!(
                    "{:#04X?}: {} {:#04X?}",
                    self.pc,
                    i.memonic,
                    (i.decoded_instruction.target() << 2) as i32
                );
                self.execute(i);
            }
        }
        match branch {
            true => {
                self.pc = self.branch_target;
                self.branch = false;
            }
            false => self.pc += 4,
        }
    }

    fn execute<T>(&mut self, i: InstructionInfo<T>) {
        let f = i.f;
        f(self, i.decoded_instruction);
    }
}
