//Written to at end of programm

use crate::bus_objects;

pub struct Exit {
    pub exit: bool,
    mapping: bus_objects::MemoryMapping,
}

impl Exit {
    pub fn new(addr: u32) -> Self {
        Exit {
            exit: false,
            mapping: bus_objects::MemoryMapping {
                start: addr,
                size: 1,
            },
        }
    }
}

impl bus_objects::BusObject for Exit {
    fn mapping(&self) -> bus_objects::MemoryMapping {
        self.mapping
    }
    fn read_byte(&self, _addr: u32) -> u8 {
        self.exit as u8
    }
    fn read_hw(&self, _addr: u32) -> u16 {
        self.exit as u16
    }
    fn read_w(&self, _addr: u32) -> u32 {
        self.exit as u32
    }

    fn write_byte(&mut self, _addr: u32, val: u8) {
        if val > 0 {
            self.exit = true;
        }
    }
    fn write_hw(&mut self, _addr: u32, val: u16) {
        if val > 0 {
            self.exit = true;
        }
    }
    fn write_w(&mut self, _addr: u32, val: u32) {
        if val > 0 {
            self.exit = true;
        }
    }
}
