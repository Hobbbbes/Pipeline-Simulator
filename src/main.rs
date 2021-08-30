mod bus_objects;
use bus_objects::*;
mod cpu;

mod commandline_arguments;
use commandline_arguments::CommandLineArguments;
use std::vec;
extern crate goblin;
use goblin::elf32;
fn main() {
    let c = CommandLineArguments::new();
    println!("{:?}", c);
    //let mem = Box::new(memory::Memory::new(Box::new([0; 1000]), mem_mapping));
    //let mut b = bus_objects::Bus::new(0, 0, vec![mem]).unwrap();
    let (entry, mut b) = prepare_bus(&c);
    let mut cpu = cpu::MipsCpu::new(&mut b, entry.entry_point);
    cpu.set_stack_start(c.stack_overwrite());
    cpu.init_gp(entry.init_gp);
    //add(&mut cpu, RTypeInstruction::new());
    while cpu.bus.read_byte(c.exit_pos()) == 0 {
        cpu.step();
    }
    cpu.bus.write_byte(c.printer_pos(), 1);
}

struct ElfInfo {
    entry_point: u32,
    init_gp: u32,
}

fn prepare_bus(c: &CommandLineArguments) -> (ElfInfo, Bus) {
    let (entry, mut ram) = load_elf_into_ram(c.executable());
    ram.push(Box::new(memory::Memory::new(
        vec![0; (c.stack_size() * 1024) as usize].into_boxed_slice(),
        bus_objects::MemoryMapping {
            start: c.stack_overwrite() - (c.stack_size() * 1024),
            size: c.stack_size() * 1024,
        },
    )));
    ram.push(Box::new(printer::Printer::new(c.printer_pos())));
    ram.push(Box::new(exit::Exit::new(c.exit_pos())));
    (entry, bus_objects::Bus::new(0, 0, ram).unwrap())
}
fn load_elf_into_ram(filename: &str) -> (ElfInfo, vec::Vec<Box<dyn bus_objects::BusObject>>) {
    let elf = std::fs::read(filename).expect("Failed to read file");
    let mut vec: Vec<Box<dyn bus_objects::BusObject>> = vec![];
    let mut info: ElfInfo = ElfInfo {
        entry_point: 0,
        init_gp: 0,
    };
    match goblin::elf::Elf::parse(&elf) {
        Ok(binary) => {
            info.entry_point = binary.entry as u32;
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
            for sh in binary.section_headers {
                if sh.sh_type == 0x7000_0006 {
                    //if type MIPS_REGINFO
                    let mut arr: [u8; 4] = [0; 4];
                    for i in 0..3 {
                        arr[i] = elf[sh.sh_offset as usize + i + 20];
                    }
                    info.init_gp = u32::from_le_bytes(arr);
                }
            }
        }
        Err(e) => println!("Error {}", e),
    }
    (info, vec)
}
