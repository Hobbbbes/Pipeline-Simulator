use std::cmp::Ordering;
use std::option::Option;
use std::vec::Vec;
pub mod exit;
pub mod memory;
pub mod printer;
#[derive(Copy, Clone, Debug)]
pub struct MemoryMapping {
    pub start: u32,
    pub size: u32,
}

impl MemoryMapping {
    fn _cmp(&self, other: &MemoryMapping) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

pub trait BusObject {
    fn mapping(&self) -> MemoryMapping;

    fn read_byte(&self, addr: u32) -> u8;
    fn read_hw(&self, addr: u32) -> u16;
    fn read_w(&self, addr: u32) -> u32;

    fn write_byte(&mut self, addr: u32, val: u8);
    fn write_hw(&mut self, addr: u32, val: u16);
    fn write_w(&mut self, addr: u32, val: u32);
}

pub struct Bus {
    bus_objects: Vec<Box<dyn BusObject>>,
    pos: MemoryMapping,
}

impl Bus {
    pub fn new(start: u32, size: u32, mut bus_objs: Vec<Box<dyn BusObject>>) -> Option<Bus> {
        bus_objs.sort_by_key(|x| x.mapping().start);
        //bus_objs.sort_unstable_by(|x, y| x.mapping().cmp(&y.mapping()));
        let mut end: u32 = 0;
        for x in &bus_objs {
            let mapping = x.mapping();
            if (mapping.start + mapping.size) < end {
                return Option::None;
            } else {
                end = mapping.start + mapping.size;
            }
        }
        Option::from(Bus {
            bus_objects: bus_objs,
            pos: MemoryMapping { start, size },
        })
    }

    fn get_bus_obj_index_vec(b_obj: &[Box<dyn BusObject>], addr: u32) -> usize {
        for (index, x) in b_obj.iter().enumerate() {
            let mapping = x.mapping();
            if (mapping.start <= addr) && (mapping.start + mapping.size >= addr) {
                return index;
            }
        }
        panic!("Invalid Bus Address {:#04X?}", addr);
    }

    fn get_bus_obj_index(&self, addr: u32) -> usize {
        for (index, x) in self.bus_objects.iter().enumerate() {
            let mapping = x.mapping();
            if (mapping.start <= addr) && (mapping.start + mapping.size >= addr) {
                return index;
            }
        }
        panic!("Invalid Bus Address {:#04X?}", addr);
    }

    fn get_mut_bus_obj(&mut self, addr: u32) -> &mut Box<dyn BusObject> {
        let index = Bus::get_bus_obj_index_vec(&self.bus_objects, addr);
        &mut self.bus_objects[index]
    }

    fn get_bus_obj(&self, addr: u32) -> &dyn BusObject {
        &*self.bus_objects[self.get_bus_obj_index(addr)]
    }
}

impl BusObject for Bus {
    fn mapping(&self) -> MemoryMapping {
        self.pos
    }

    fn read_byte(&self, addr: u32) -> u8 {
        let bus_obj = self.get_bus_obj(addr);
        bus_obj.read_byte(addr - bus_obj.mapping().start)
    }

    fn read_hw(&self, addr: u32) -> u16 {
        let bus_obj = self.get_bus_obj(addr);
        bus_obj.read_hw(addr - bus_obj.mapping().start)
    }
    fn read_w(&self, addr: u32) -> u32 {
        let bus_obj = self.get_bus_obj(addr);
        bus_obj.read_w(addr - bus_obj.mapping().start)
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        let bus_obj = self.get_mut_bus_obj(addr);
        bus_obj.write_byte(addr - bus_obj.mapping().start, val);
    }
    fn write_hw(&mut self, addr: u32, val: u16) {
        let bus_obj = self.get_mut_bus_obj(addr);
        bus_obj.write_hw(addr - bus_obj.mapping().start, val);
    }
    fn write_w(&mut self, addr: u32, val: u32) {
        let bus_obj = self.get_mut_bus_obj(addr);
        bus_obj.write_w(addr - bus_obj.mapping().start, val);
    }
}
