use super::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
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

        use super::instruction::Opcode::*;

        match self.decode_opcode() {
            HLT => return self.handle_hlt(),
            LOAD => self.handle_load(),
            ADD => self.handle_add(),
            SUB => self.handle_sub(),
            MUL => self.handle_mul(),
            DIV => self.handle_div(),
            JMP => self.handle_jmp(),
            JMPF => self.handle_jmpf(),
            JMPB => self.handle_jmpb(),
            EQ => self.handle_eq(),
            NEQ => self.handle_neq(),
            GT => self.handle_gt(),
            LT => self.handle_lt(),
            GTQ => self.handle_gtq(),
            LTQ => self.handle_ltq(),
            JEQ => self.handle_jeq(),
            JNEQ => self.handle_jneq(),
            op @ _ => {
                println!("Unexpected {} opcode at {}", op, self.pc);

                return false;
            }
        }

        true
    }

    fn handle_hlt(&self) -> bool {
        println!("HLT encountered");

        return false;
    }

    fn handle_load(&mut self) {
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as u16;

        self.registers[register] = number as i32;
    }

    // TODO (xeqlol): keep DRY
    fn handle_add(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.registers[self.next_8_bits() as usize] = register1 + register2;
    }

    fn handle_sub(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.registers[self.next_8_bits() as usize] = register1 - register2;
    }

    fn handle_mul(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.registers[self.next_8_bits() as usize] = register1 * register2;
    }

    fn handle_div(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.registers[self.next_8_bits() as usize] = register1 / register2;
        self.remainder = (register1 % register2) as u32;
    }

    fn handle_jmp(&mut self) {
        let target = self.registers[self.next_8_bits() as usize];

        self.pc = target as usize;
    }

    fn handle_jmpf(&mut self) {
        let target = self.registers[self.next_8_bits() as usize];

        self.pc += target as usize;
    }

    fn handle_jmpb(&mut self) {
        let target = self.registers[self.next_8_bits() as usize];

        self.pc -= target as usize;
    }

    fn handle_eq(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.equal_flag = register1 == register2;

        self.next_8_bits();
    }

    fn handle_neq(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.equal_flag = register1 != register2;

        self.next_8_bits();
    }

    fn handle_gt(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.equal_flag = register1 > register2;

        self.next_8_bits();
    }

    fn handle_lt(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.equal_flag = register1 < register2;

        self.next_8_bits();
    }

    fn handle_gtq(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.equal_flag = register1 >= register2;

        self.next_8_bits();
    }

    fn handle_ltq(&mut self) {
        let (register1, register2) = self.read_next_2_registers();

        self.equal_flag = register1 <= register2;

        self.next_8_bits();
    }

    fn handle_jeq(&mut self) {
        let target = self.registers[self.next_8_bits() as usize];

        if self.equal_flag {
            self.pc = target as usize;
        }
    }

    fn handle_jneq(&mut self) {
        let target = self.registers[self.next_8_bits() as usize];

        if !self.equal_flag {
            self.pc = target as usize;
        }
    }

    fn read_next_2_registers(&mut self) -> (i32, i32) {
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];

        (register1, register2)
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

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![0x09, 0x00, 0x01, 0x00, 0x09, 0x00, 0x01, 0x00];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;
        test_vm.program = vec![0x0A, 0x00, 0x01, 0x00, 0x0A, 0x00, 0x01, 0x00];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.program = vec![0x0B, 0x00, 0x01, 0x00, 0x0B, 0x00, 0x01, 0x00];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;
        test_vm.program = vec![0x0C, 0x00, 0x01, 0x00, 0x0C, 0x00, 0x01, 0x00];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_gtq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![
            0x0D, 0x00, 0x01, 0x00, 0x0D, 0x00, 0x01, 0x00, 0x0D, 0x00, 0x01, 0x00,
        ];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_ltq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.program = vec![
            0x0E, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x01, 0x00,
        ];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![0x0F, 0x00, 0x00, 0x00];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_opcode_jneq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = false;
        test_vm.program = vec![0x10, 0x00, 0x00, 0x00];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}
