mod bus_objects;
use bus_objects::*;
mod cpu;
use cpu::instructions::*;
use std::fs::File;
use std::io::Read;
fn main() {
    let mem_mapping = MemoryMapping {
        start: 0,
        size: 10000,
    };
    let mem = Box::new(memory::Memory::new(Box::new([0; 1000]), mem_mapping));
    let mut b = bus_objects::Bus::new(0, 0, vec![mem]).unwrap();
    let mut cpu = cpu::MipsCpu::new(&mut b, 0);
    add(&mut cpu, RTypeInstruction::new());
    cpu.step();
    println!("Hello, world!");
}

fn prepare_bus(filename: &str) -> Bus {
    let ram = load_elf_into_ram(filename);
    bus_objects::Bus::new(0, 0, vec![ram]).unwrap()
}
fn load_elf_into_ram(filename: &str) -> Box<memory::Memory> {
    let elf = std::fs::read(filename).expect("Failed to read file");
    match goblin::elf::Elf::parse(&elf) {
        Ok(binary) => println!("parsed binary"),
        Err(e) => println!("Error {}", e),
    }
    Box::new(memory::Memory::new(
        Box::new([0; 1000]),
        MemoryMapping {
            start: 0,
            size: 10000,
        },
    ))
}
