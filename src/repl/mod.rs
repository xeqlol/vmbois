use crate::assembler::parser::program::parse_program;
use crate::vm::VM;
use nom::types::CompleteStr;
use std;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Default)]
#[allow(dead_code)]
pub struct Repl {
    command_buffer: Vec<String>,
    vm: VM,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium");

        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!("> ");
            io::stdout().flush().expect("Unable to flush stdout");
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Cya boi");

                    std::process::exit(0);
                }
                ".history" => self.handle_history(),
                ".program" => self.handle_program(),
                ".register" => self.handle_registers(),
                ".load_file" => self.handle_load_file(),
                ".clear" => self.handle_clear(),
                _ => {
                    let parsed_program = parse_program(CompleteStr(buffer));

                    if parsed_program.is_err() {
                        println!("Unable to parse input");
                        continue;
                    }

                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes();

                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }

                    self.vm.run_once();
                }
            }
        }
    }

    fn handle_history(&self) {
        for command in &self.command_buffer {
            println!("{}", command);
        }
    }

    fn handle_program(&self) {
        println!("Listing instructions currently in VM's program vector:");

        for instruction in &self.vm.program {
            println!("{}", instruction);
        }

        println!("End of program listing")
    }

    fn handle_registers(&self) {
        println!("Listing registers and all contents:");
        println!("{:#?}", self.vm.registers);
        println!("End of regicster listing");
    }

    fn handle_clear(&mut self) {
        println!("Clearing program vector");
        self.vm.program = vec![];
    }

    fn handle_load_file(&mut self) {
        print!("Please enter the path to the file you wish to load: ");
        io::stdout().flush().expect("Unable to flush stdout");

        let mut tmp = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("Unable to read line from user");

        let tmp = tmp.trim();
        let filename = Path::new(&tmp);
        let mut f = File::open(Path::new(&filename)).expect("File not found");
        let mut contents = String::new();

        f.read_to_string(&mut contents)
            .expect("There was an error reading from the file");
        
        let program = match parse_program(CompleteStr(&contents)) {
            Ok((_, program)) => program,
            Err(e) => {
                println!("Unable to parse input: {:?}", e);

                return;
            }
        };

        self.vm.program.append(&mut program.to_bytes());
    }

    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];

        for hex_string in split {
            let result = u8::from_str_radix(&hex_string, 16)?;

            results.push(result);
        }

        Ok(results)
    }
}
