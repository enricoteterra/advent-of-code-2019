use std::fmt;
use crate::program::opcode::Opcode;
use crate::program::opcode::Operation;

mod opcode;

pub const MAX_INPUT_SIZE: usize = 256;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<usize>,
    cursor: usize
}

impl Program {
    pub fn new(instructions: &[usize]) -> Program {
        if instructions.len() > MAX_INPUT_SIZE { 
            panic!("input too large: {} codes, maximum is {}", instructions.len(), MAX_INPUT_SIZE )
        }
        Program { 
            instructions: instructions.to_vec(),
            cursor: 0
        }
    }
    pub fn run(&mut self) {
        while self.cursor < self.instructions.len() {
            let opcode = opcode::Opcode::new(&self.instructions[self.cursor..self.cursor+4]);
            self.instructions.splice(self.cursor..self.cursor+4, Self::evaluate(opcode).instructions);
            self.cursor = self.cursor + 4;
        }
    }
    fn evaluate(opcode: Opcode) -> Opcode {
        let result;
        match opcode.operation {
            Operation::ADD => result = opcode.value1 + opcode.value2,
            Operation::MULTIPLY => result = opcode.value1 * opcode.value2,
            _ => result = opcode.instructions[opcode.target] // no change
        }
    
        let mut output: Vec<usize> = opcode.instructions.to_vec();
        output[opcode.target] = result;
        return Opcode::new(&output);
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.instructions)
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Program;

    #[test]
    #[should_panic(expected = "input too large: 500 codes, maximum is 256",)]
    fn it_panics_when_loaded_program_is_too_large() {
        Program::new(&(0.. 500).collect::<Vec<usize>>());
    }

    #[test]
    fn it_runs_once() {
        let mut program = Program::new(&[1,0,0,0]);
        program.run();
        assert_eq!(Program::new(&[2,0,0,0]), program);
    }

    #[test]
    fn it_runs_many_times() {
        let mut program = Program::new(&[1,0,0,0,1,0,1,1]);
        program.run();
        assert_eq!(Program::new(&[2,1,0,0,1,0,1,1]), program);
    }
}
