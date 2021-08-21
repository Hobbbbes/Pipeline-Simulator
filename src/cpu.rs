pub mod instructions;
use super::bus_objects;
const MIPS_REGISTER_NAMES: [&str; 32] = [
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "fp",
    "ra",
];

pub struct MipsCpu<'a> {
    general_registers: [u32; 31],
    fpu_register: [u32; 2],

    pc: u32,
    instruction_buffer_register: u32,
    bus: &'a dyn bus_objects::BusObject,

    branch: bool,
    branch_target: u32,
}

impl<'a> MipsCpu<'a> {
    pub fn new(bus: &'a dyn bus_objects::BusObject, pc: u32) -> Self {
        MipsCpu {
            general_registers: [0; 31],
            fpu_register: [0.0 as u32; 2],
            pc,
            instruction_buffer_register: 0,
            bus,
            branch: false,
            branch_target: 0,
        }
    }
    fn get_register(&self, index: u8) -> u32 {
        match index {
            0 => 0,
            r => self.general_registers[(r as usize) - 1],
        }
    }
    fn set_register(&mut self, index: u8, value: u32) {
        if index != 0 {
            self.general_registers[(index as usize) - 1] = value;
        }
    }
}
