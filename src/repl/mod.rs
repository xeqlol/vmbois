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
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}
