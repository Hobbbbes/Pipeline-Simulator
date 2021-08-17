use crate::bus_objects;
use std::boxed::Box;
pub struct Memory {
    mem: Box<[u8]>,
    mapping: bus_objects::MemoryMapping,
}

impl Memory {
    pub fn new(mem: Box<[u8]>, mapping: bus_objects::MemoryMapping) -> Memory {
        Memory { mem, mapping }
    }
}

impl bus_objects::BusObject for Memory {
    fn mapping(&self) -> bus_objects::MemoryMapping {
        self.mapping
    }
    fn read_byte(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }

    fn read_hw(&self, addr: u32) -> u16 {
        unsafe { *((&self.mem[addr as usize] as *const u8) as *const u16) }
    }

    fn read_w(&self, addr: u32) -> u32 {
        unsafe { *((&self.mem[addr as usize] as *const u8) as *const u32) }
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        self.mem[addr as usize] = val;
    }

    fn write_hw(&mut self, addr: u32, val: u16) {
        unsafe { *((&mut self.mem[addr as usize] as *mut u8) as *mut u16) = val }
    }

    fn write_w(&mut self, addr: u32, val: u32) {
        unsafe { *((&mut self.mem[addr as usize] as *mut u8) as *mut u32) = val }
    }
}
