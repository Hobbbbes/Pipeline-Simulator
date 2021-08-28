mod bus_objects;
use bus_objects::*;
mod cpu;
use cpu::instructions::*;
use goblin::elf32;
extern crate goblin;

static mut START_ADDR: u32 = 0;

fn main() {
    let mem_mapping = MemoryMapping {
        start: 0,
        size: 10000,
    };
    //let mem = Box::new(memory::Memory::new(Box::new([0; 1000]), mem_mapping));
    //let mut b = bus_objects::Bus::new(0, 0, vec![mem]).unwrap();
    let mut b =
        prepare_bus("/mnt/home/calvin/Code_Projekts/C/MipsTestProgramms/build/print_test.elf");
    let mut cpu = cpu::MipsCpu::new(&mut b, unsafe { START_ADDR });
    //add(&mut cpu, RTypeInstruction::new());
    loop {
        cpu.step();
    }
    println!("Hello, world!");
}

fn prepare_bus(filename: &str) -> Bus {
    let ram = load_elf_into_ram(filename);
    bus_objects::Bus::new(0, 0, vec![ram]).unwrap()
}
fn load_elf_into_ram(filename: &str) -> Box<memory::Memory> {
    let elf = std::fs::read(filename).expect("Failed to read file");
    match goblin::elf::Elf::parse(&elf) {
        Ok(binary) => {
            let entry = binary.entry;
            unsafe {
                START_ADDR = entry as u32;
            }
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
                }
            }
        }
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
