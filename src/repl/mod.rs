use crate::vm::VM;
use std;
use std::io;
use std::io::Write;

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
                _ => {
                    let results = self.parse_hex(buffer);

                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte)
                            }
                        }
                        Err(_) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.", )
                        }
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
