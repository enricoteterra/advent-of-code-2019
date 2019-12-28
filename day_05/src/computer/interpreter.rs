use crate::computer::*;

struct Interpreter {
    pub memory: Vec<usize>,
    pub pointer: usize,
}

impl Interpreter {
    pub fn new(maybe_initial_memory: Option<Vec<usize>>) -> Interpreter {
        Interpreter {
            memory: maybe_initial_memory.unwrap_or(Vec::new()),
            pointer: 0,
        }
    }
    pub fn read(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            match instruction.opcode.code {
                Operation::ADD => {
                    let arg1 = instruction.params[0].value;
                    let arg2 = instruction.params[1].value;
                    let target = instruction.params[2].value;
                    self.memory[target] = self.memory[arg1] + self.memory[arg2];
                }
                Operation::MULTIPLY => {
                    let arg1 = instruction.params[0].value;
                    let arg2 = instruction.params[1].value;
                    let target = instruction.params[2].value;
                    self.memory[target] = self.memory[arg1] * self.memory[arg2];
                }
                Operation::STORE => {
                    let value = instruction.params[0].value;
                    let target = instruction.params[1].value;
                    self.memory[target] = value;
                }
                Operation::EXIT => break,
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_interprets_add_operation() {
        let memory: Vec<usize> = "1,9,10,3,2,3,11,0,99,30,40,50"
            .split(",")
            .map(|it| it.parse().unwrap())
            .collect();
        let mut interpreter = Interpreter::new(Some(memory));
        let instructions = vec![
            Instruction::new(
                Operation::ADD,
                vec![
                    Parameter::new(Mode::POSITION, 9),
                    Parameter::new(Mode::POSITION, 10),
                    Parameter::new(Mode::POSITION, 3),
                ],
            ),
            Instruction::new(
                Operation::MULTIPLY,
                vec![
                    Parameter::new(Mode::POSITION, 3),
                    Parameter::new(Mode::POSITION, 11),
                    Parameter::new(Mode::POSITION, 0),
                ],
            ),
            Instruction::new(Operation::EXIT, vec![]),
        ];
        interpreter.read(instructions);
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            interpreter.memory.to_vec()
        );
    }

    #[test]
    fn it_interprets_store_operation() {
        let memory: Vec<usize> = "3,1".split(",").map(|it| it.parse().unwrap()).collect();
        let mut interpreter = Interpreter::new(Some(memory));
        let instructions = vec![Instruction::new(
            Operation::STORE,
            vec![
                Parameter::new(Mode::POSITION, 3),
                Parameter::new(Mode::POSITION, 1),
            ],
        )];
        interpreter.read(instructions);
        assert_eq!(vec![3, 3], interpreter.memory.to_vec());
    }
}
