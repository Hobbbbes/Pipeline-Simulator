use crate::bus_objects;

pub struct Printer {
    str: String,
    mapping: bus_objects::MemoryMapping,
}

impl Printer {
    pub fn new(addr: u32) -> Self {
        Printer {
            str: String::new(),
            mapping: bus_objects::MemoryMapping {
                start: addr,
                size: 2,
            },
        }
    }

    pub fn flush(&mut self) {
        print!("{}", self.str);
        self.str.clear();
    }
}

impl bus_objects::BusObject for Printer {
    fn mapping(&self) -> bus_objects::MemoryMapping {
        self.mapping
    }
    fn read_byte(&self, _addr: u32) -> u8 {
        0
    }
    fn read_hw(&self, _addr: u32) -> u16 {
        0
    }
    fn read_w(&self, _addr: u32) -> u32 {
        0
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        match addr {
            0 => self.str.push(val as char),
            1 => self.flush(),
            _ => panic!(),
        }
    }
    fn write_hw(&mut self, addr: u32, val: u16) {
        match addr {
            0 => self.str.push(val as u8 as char),
            1 => self.flush(),
            _ => panic!(),
        }
    }
    fn write_w(&mut self, addr: u32, val: u32) {
        match addr {
            0 => self.str.push(val as u8 as char),
            1 => self.flush(),
            _ => panic!(),
        }
    }
}
