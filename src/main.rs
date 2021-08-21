mod bus_objects;
use bus_objects::*;
mod cpu;
use cpu::instructions::*;
fn main() {
    let mem_mapping = MemoryMapping {
        start: 0,
        size: 10000,
    };
    let mem = Box::new(memory::Memory::new(Box::new([0; 1000]), mem_mapping));
    let b = bus_objects::Bus::new(0, 0, vec![(mem_mapping, mem)]).unwrap();
    let mut cpu = cpu::MipsCpu::new(&b, 0);
    cpu.add(RTypeInstruction::new());
    println!("Hello, world!");
}
