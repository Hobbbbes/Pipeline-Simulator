mod bus_objects;
use bus_objects::*;
mod cpu;
use goblin::elf32;
mod commandline_arguments;
use commandline_arguments::CommandLineArguments;
use std::vec;
extern crate goblin;

fn main() {
    let c = CommandLineArguments::new();
    println!("{:?}", c);
    //let mem = Box::new(memory::Memory::new(Box::new([0; 1000]), mem_mapping));
    //let mut b = bus_objects::Bus::new(0, 0, vec![mem]).unwrap();
    let (entry, mut b) = prepare_bus(&c);
    let mut cpu = cpu::MipsCpu::new(&mut b, entry);
    cpu.set_stack_start(c.stack_overwrite());
    //add(&mut cpu, RTypeInstruction::new());
    loop {
        cpu.step();
    }
    //println!("Hello, world!");
}

fn prepare_bus(c: &CommandLineArguments) -> (u32, Bus) {
    let (entry, mut ram) = load_elf_into_ram(c.executable());
    ram.push(Box::new(memory::Memory::new(
        vec![0; c.stack_size() as usize].into_boxed_slice(),
        bus_objects::MemoryMapping {
            start: c.stack_overwrite() - c.stack_size(),
            size: c.stack_size(),
        },
    )));
    (entry, bus_objects::Bus::new(0, 0, ram).unwrap())
}
fn load_elf_into_ram(filename: &str) -> (u32, vec::Vec<Box<dyn bus_objects::BusObject>>) {
    let elf = std::fs::read(filename).expect("Failed to read file");
    let mut vec: Vec<Box<dyn bus_objects::BusObject>> = vec![];
    let mut entry: u32 = 0;
    match goblin::elf::Elf::parse(&elf) {
        Ok(binary) => {
            entry = binary.entry as u32;
            println!("{:X}", entry);
            for ph in binary.program_headers {
                println!("{:?}", ph);
                if ph.p_type == elf32::program_header::PT_LOAD {
                    let mut mem = Box::new(memory::Memory::new(
                        vec![0; ph.p_memsz as usize].into_boxed_slice(),
                        bus_objects::MemoryMapping {
                            start: ph.p_vaddr as u32,
                            size: ph.p_memsz as u32,
                        },
                    ));
                    for i in 0..ph.p_filesz {
                        mem.write_byte(i as u32, elf[(i + ph.p_offset) as usize])
                    }
                    vec.push(mem);
                }
            }
        }
        Err(e) => println!("Error {}", e),
    }
    (entry, vec)
}
