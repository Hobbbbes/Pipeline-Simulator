mod bus_objects;
use bus_objects::*;
fn main() {
    let mem_mapping = MemoryMapping {
        start: 0,
        size: 10000,
    };
    let mem = Box::new(memory::Memory::new(Box::new([0; 1000]), mem_mapping));
    let b = bus_objects::Bus::new(0, 0, vec![(mem_mapping, mem)]);
    println!("Hello, world!");
}
