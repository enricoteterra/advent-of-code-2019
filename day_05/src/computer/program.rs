pub struct Program {
    pub memory: Vec<usize>,
    pub pointer: usize,
}

impl Program {
    pub fn new(maybe_initial_state: Option<Vec<usize>>) -> Program {
        Program {
            memory: maybe_initial_state.unwrap_or(Vec::new()),
            pointer: 0,
        }
    }
    pub fn run(&mut self) {
        while self.pointer < self.memory.len() {
            if self.pointer + 4 > self.memory.len() {
                break;
            }

            let address_of_next_instruction = self.pointer;
            let operation = self.memory[address_of_next_instruction];
            let address_of_arg_a = self.memory[address_of_next_instruction + 1];
            let address_of_arg_b = self.memory[address_of_next_instruction + 2];
            let address_of_arg_c = self.memory[address_of_next_instruction + 3];
            match operation {
                1 => {
                    self.memory[address_of_arg_c] =
                        self.memory[address_of_arg_a] + self.memory[address_of_arg_b]
                }
                2 => {
                    self.memory[address_of_arg_c] =
                        self.memory[address_of_arg_a] * self.memory[address_of_arg_b]
                }
                99 => break,
                _ => (),
            }
            self.pointer += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_program_from_memory() {
        let mut program_without_operations = Program::new(None);
        program_without_operations.run();
        assert_eq!(0, program_without_operations.pointer);

        let mut program_with_some_operations = Program::new(Some([1, 2, 2, 2].to_vec()));
        program_with_some_operations.run();
        assert_eq!(4, program_with_some_operations.pointer);

        let mut program_with_many_operations = Program::new(Some((1..98).collect::<Vec<usize>>()));
        program_with_many_operations.run();
        assert_eq!(96, program_with_many_operations.pointer);
    }

    #[test]
    fn it_writes_result_of_addition_back_into_memory() {
        let mut program_with_some_operations = Program::new(Some([1, 0, 0, 0].to_vec()));
        program_with_some_operations.run();
        assert_eq!([2, 0, 0, 0].to_vec(), program_with_some_operations.memory);
    }

    #[test]
    fn it_writes_result_of_multiplication_back_into_memory() {
        let mut program_with_some_operations = Program::new(Some([2, 0, 0, 1].to_vec()));
        program_with_some_operations.run();
        assert_eq!([2, 4, 0, 1].to_vec(), program_with_some_operations.memory);
    }

    #[test]
    fn it_performs_addition_and_multiplication_multiple_times() {
        let mut program_with_some_operations =
            Program::new(Some([1, 0, 0, 0, 2, 0, 0, 0].to_vec()));
        program_with_some_operations.run();
        assert_eq!(
            [4, 0, 0, 0, 2, 0, 0, 0].to_vec(),
            program_with_some_operations.memory
        );
    }

    #[test]
    fn it_stops_program_at_the_exit_indicator() {
        let mut program_with_exit_indicator =
            Program::new(Some([1, 0, 0, 0, 99, 2, 0, 0, 0].to_vec()));
        program_with_exit_indicator.run();
        assert_eq!(
            [2, 0, 0, 0, 99, 2, 0, 0, 0].to_vec(),
            program_with_exit_indicator.memory
        );

        let mut program_with_2nd_exit_indicator =
            Program::new(Some([1, 0, 0, 0, 2, 0, 0, 0, 99].to_vec()));
        program_with_2nd_exit_indicator.run();
        assert_eq!(
            [4, 0, 0, 0, 2, 0, 0, 0, 99].to_vec(),
            program_with_2nd_exit_indicator.memory
        );
    }
}
