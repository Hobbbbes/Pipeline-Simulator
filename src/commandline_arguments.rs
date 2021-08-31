extern crate clap;
use clap::{App, Arg};

#[derive(Debug)]
pub struct CommandLineArguments {
    //Overwrites the position of the stack. Default: 0xFFFFFFFF
    stack_overwrite: u32,

    //Stack size in Kilobytes. Default: 1024
    stack_size: u32,

    exit_pos: u32,
    printer_pos: u32,

    disassemble: bool,
    //Path to a statically linked freestanding ELF Binary with MIPS R3000 machine code
    executable: String,
}

impl CommandLineArguments {
    pub fn new() -> Self {
        let matches = App::new("Pipeline Simulator for R3000 MIPS Processor")
            .version("0.1")
            .author("Calvin Katt")
            .about("Simulates a MIPS R3000 CPU")
            .arg(Arg::with_name("Disassemble").long("disassemble").short("d").help("Disassemble the executed instructions"))
            .arg(
                Arg::with_name("Stack Overwrite")
                    .long("stackoverwrite")
                    .value_name("ADDR")
                    .help("Sets the address of the stack pointer at the start of the program.")
                    .takes_value(true)
                    .default_value("0xFFFFFFFF"),
            )
            .arg(
                Arg::with_name("Stack Size")
                    .long("stacksize")
                    .value_name("SIZE")
                    .help("Sets the size of the stack in Kilobytes")
                    .takes_value(true)
                    .default_value("1024"),
            )
            .arg(
                Arg::with_name("Printer Position").long("printerpos").short("p").help("Sets the memory address of the two byte printer device").takes_value(true).default_value("0x2")
            )
            .arg(Arg::with_name("Exit Position").long("exitpos").short("e").help("Sets the memory address of the one byte exit device used to signal the exit of the programm to the simulator").takes_value(true).default_value("0x1"))
            .arg(Arg::with_name("EXECUTABLE").help(
                "Path to a statically linked freestanding ELF Binary with MIPS R3000 machine code",
            ).required(true)).get_matches();
        let exec_path = matches.value_of("EXECUTABLE").unwrap();
        let stack_overwrite = u32::from_str_radix(
            matches
                .value_of("Stack Overwrite")
                .unwrap()
                .trim_start_matches("0x"),
            16,
        )
        .unwrap();
        let stack_size = matches
            .value_of("Stack Size")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let exit_pos = u32::from_str_radix(
            matches
                .value_of("Exit Position")
                .unwrap()
                .trim_start_matches("0x"),
            16,
        )
        .unwrap();
        let printer_pos = u32::from_str_radix(
            matches
                .value_of("Printer Position")
                .unwrap()
                .trim_start_matches("0x"),
            16,
        )
        .unwrap();
        let disassemble = matches.is_present("Disassemble");
        CommandLineArguments {
            stack_overwrite,
            stack_size,
            exit_pos,
            printer_pos,
            disassemble,
            executable: String::from(exec_path),
        }
    }

    #[inline]
    pub fn executable(&self) -> &str {
        &self.executable
    }

    #[inline]
    pub fn stack_size(&self) -> u32 {
        self.stack_size
    }

    #[inline]
    pub fn stack_overwrite(&self) -> u32 {
        self.stack_overwrite
    }

    #[inline]
    pub fn exit_pos(&self) -> u32 {
        self.exit_pos
    }

    #[inline]
    pub fn printer_pos(&self) -> u32 {
        self.printer_pos
    }

    #[inline]
    pub fn disassemble(&self) -> bool {
        self.disassemble
    }
}
