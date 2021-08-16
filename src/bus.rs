
use std::vec::Vec;
use std::option::Option;
use std::cmp::Ordering;
pub struct MemoryMapping {
    start : u32,
    size : u32
}

impl MemoryMapping {
    fn cmp(&self,other : &MemoryMapping) -> Ordering {
        if self.start > other.start {
            Ordering::Greater
        } else if self.start < other.start {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

pub trait BusObject {
    fn mapping(&self) -> MemoryMapping;

    fn read_byte(&self,addr : u32) -> u8;
    fn read_hw(&self,addr : u32) -> u16;
    fn read_w(&self,addr : u32) -> u32;

    fn write_byte(&mut self,addr : u32, val : u8);
    fn write_hw(&mut self,addr : u32, val : u16);
    fn write_w(&mut self,addr : u32, val : u32);
}

pub struct Bus{
    bus_objects : Vec<(MemoryMapping,Box<dyn BusObject>)>,
    pos : MemoryMapping
}

impl Bus {
    fn new(start : u32, size : u32, bus_objs : Vec<(MemoryMapping,Box<dyn BusObject>)> ) -> Option<Bus>{
        bus_objs.sort_unstable_by(|x,y| {
            let (m1,_) = x;
            let (m2,_) = y;
            m1.cmp(m2)
        });
        let mut end : u32 = 0;
        for (mapping,_) in bus_objs {
            if (mapping.start + mapping.size) < end {
                return Option::None;
            } else {
                end = mapping.start + mapping.size;
            }
        }
        Option::from(
            Bus{bus_objects: bus_objs, pos : MemoryMapping{start,size}}
        )
    }

    fn get_bus_obj_index(&self,addr : u32) -> usize {
        
    }

    fn get_mut_bus_obj(&mut self,addr: u32) -> &mut Box<dyn BusObject> {

    }

    fn get_bus_obj(&self, addr: u32) -> &Box<dyn BusObject> {

    }
}

impl BusObject for Bus {
    fn mapping(&self) -> MemoryMapping {
        return self.pos;
    }

    fn read_byte(addr: u32)
}