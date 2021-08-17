use std::cmp::Ordering;
use std::option::Option;
use std::vec::Vec;
pub mod memory;
#[derive(Copy, Clone, Debug)]
pub struct MemoryMapping {
    pub start: u32,
    pub size: u32,
}

impl MemoryMapping {
    fn cmp(&self, other: &MemoryMapping) -> Ordering {
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
    bus_objects: Vec<(MemoryMapping, Box<dyn BusObject>)>,
    pos: MemoryMapping,
}

impl Bus {
    pub fn new(
        start: u32,
        size: u32,
        mut bus_objs: Vec<(MemoryMapping, Box<dyn BusObject>)>,
    ) -> Option<Bus> {
        bus_objs.sort_unstable_by(|x, y| {
            let (m1, _) = x;
            let (m2, _) = y;
            m1.cmp(m2)
        });
        let mut end: u32 = 0;
        for (mapping, _) in &bus_objs {
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

    fn get_bus_obj_index_vec(b_obj: &[(MemoryMapping, Box<dyn BusObject>)], addr: u32) -> usize {
        for (index, (mapping, _)) in b_obj.iter().enumerate() {
            if (mapping.start <= addr) && (mapping.start + mapping.size >= addr) {
                return index;
            }
        }
        panic!("Invalid Bus Address");
    }

    fn get_bus_obj_index(&self, addr: u32) -> usize {
        for (index, (mapping, _)) in self.bus_objects.iter().enumerate() {
            if (mapping.start <= addr) && (mapping.start + mapping.size >= addr) {
                return index;
            }
        }
        panic!("Invalid Bus Address");
    }

    fn get_mut_bus_obj(&mut self, addr: u32) -> &mut Box<dyn BusObject> {
        let index = Bus::get_bus_obj_index_vec(&self.bus_objects, addr);
        &mut self.bus_objects[index].1
    }

    fn get_bus_obj(&self, addr: u32) -> &Box<dyn BusObject> {
        &self.bus_objects[self.get_bus_obj_index(addr)].1
    }

    fn get_bus_obj_mapping(&self, addr: u32) -> &MemoryMapping {
        &self.bus_objects[self.get_bus_obj_index(addr)].0
    }

    fn get_bus_obj_and_mapping(&self, addr: u32) -> &(MemoryMapping, Box<dyn BusObject>) {
        &self.bus_objects[self.get_bus_obj_index(addr)]
    }

    fn get_bus_obj_and_mapping_mut(
        &mut self,
        addr: u32,
    ) -> &mut (MemoryMapping, Box<dyn BusObject>) {
        let index = Bus::get_bus_obj_index_vec(&self.bus_objects, addr);
        &mut self.bus_objects[index]
    }
}

impl BusObject for Bus {
    fn mapping(&self) -> MemoryMapping {
        self.pos
    }

    fn read_byte(&self, addr: u32) -> u8 {
        let (mapping, bus_obj) = self.get_bus_obj_and_mapping(addr);
        bus_obj.read_byte(addr - mapping.start)
    }

    fn read_hw(&self, addr: u32) -> u16 {
        let (mapping, bus_obj) = self.get_bus_obj_and_mapping(addr);
        bus_obj.read_hw(addr - mapping.start)
    }
    fn read_w(&self, addr: u32) -> u32 {
        let (mapping, bus_obj) = self.get_bus_obj_and_mapping(addr);
        bus_obj.read_w(addr - mapping.start)
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        let (mapping, bus_obj) = self.get_bus_obj_and_mapping_mut(addr);
        bus_obj.write_byte(addr - mapping.start, val);
    }
    fn write_hw(&mut self, addr: u32, val: u16) {
        let (mapping, bus_obj) = self.get_bus_obj_and_mapping_mut(addr);
        bus_obj.write_hw(addr - mapping.start, val);
    }
    fn write_w(&mut self, addr: u32, val: u32) {
        let (mapping, bus_obj) = self.get_bus_obj_and_mapping_mut(addr);
        bus_obj.write_w(addr - mapping.start, val);
    }
}
