use super::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        while self.execute_instruction() {}
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");

                return false;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;

                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];

                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize];

                self.pc += target as usize;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize];

                self.pc -= target as usize;
            }
            op @ _ => {
                println!("Unexpected {} opcode at {}", op, self.pc);

                return false;
            }
        }

        true
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;

        result
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0x00, 0x00, 0x00, 0x00];;
        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0xFF, 0x00, 0x00, 0x00];;
        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0x01, 0x00, 0x01, 0xF4];
        test_vm.run();

        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0x01, 0x00, 0x00, 0x02, 0x01, 0x01, 0x00, 0x02, 0x02, 0x00, 0x01, 0x02,
        ];
        test_vm.run();

        assert_eq!(test_vm.registers[0], 2);
        assert_eq!(test_vm.registers[1], 2);
        assert_eq!(test_vm.registers[2], 4);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0x01, 0x00, 0x00, 0x02, 0x01, 0x01, 0x00, 0x02, 0x03, 0x00, 0x01, 0x02,
        ];
        test_vm.run();

        assert_eq!(test_vm.registers[0], 2);
        assert_eq!(test_vm.registers[1], 2);
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0x01, 0x00, 0x00, 0x02, 0x01, 0x01, 0x00, 0x02, 0x04, 0x00, 0x01, 0x02,
        ];
        test_vm.run();

        assert_eq!(test_vm.registers[0], 2);
        assert_eq!(test_vm.registers[1], 2);
        assert_eq!(test_vm.registers[2], 4);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0x01, 0x00, 0x00, 0x05, 0x01, 0x01, 0x00, 0x02, 0x05, 0x00, 0x01, 0x02,
        ];
        test_vm.run();

        assert_eq!(test_vm.registers[0], 5);
        assert_eq!(test_vm.registers[1], 2);
        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0x06, 0x00, 0x00, 0x00];
        test_vm.registers[0] = 1;
        test_vm.run_once();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![0x07, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![0x08, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 0);
    }
}
