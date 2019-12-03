use std::fmt;

#[derive(PartialEq)]
pub enum Operation {
    NOOP = 0,
    ADD = 1,
    MULTIPLY = 2,
    EXIT = 99
}

pub struct Opcode {
    pub instructions: Vec<usize>,
    pub operation: Operation,
    pub value1: usize,
    pub value2: usize,
    pub target: usize
}

impl Opcode {
    pub fn new(instructions: &[usize]) -> Opcode {
        if instructions.len() < 4 { panic!("instruction set must have at least 4 items! {:?}", instructions) }
        let arg1 = instructions[1];
        let arg2 = instructions[2];
        Opcode {
            instructions: instructions.to_vec(),
            operation: Self::operation_from(instructions[0]),
            value1: instructions[arg1],
            value2: instructions[arg2],
            target: instructions[3]
        }
    }
    fn operation_from(operation_code: usize) -> Operation {
        match operation_code {
            1 => Operation::ADD,
            2 => Operation::MULTIPLY,
            99 => Operation::EXIT,
            _ => Operation::NOOP
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    #[should_panic(expected = "instruction set must have at least 4 items! [0, 1]",)]
    fn it_panics_when_loaded_program_is_too_large() {
        Opcode::new(&(0.. 2).collect::<Vec<usize>>());
    }
}