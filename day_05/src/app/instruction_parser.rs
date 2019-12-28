use crate::app::opcode_parser::parse_opcode;
use crate::computer::*;

pub fn parse(input: &str) -> Instruction {
    let code: Vec<usize> = input.split(",").map(|it| it.parse().unwrap()).collect();
    let opcode = parse_opcode(code[0]);
    return Instruction {
        params: parse_parameter_values(&opcode, code[1..].to_vec()),
        opcode: opcode,
    };
}

fn parse_parameter_values(opcode: &Opcode, params: Vec<usize>) -> Vec<Parameter> {
    match opcode.code {
        Operation::MULTIPLY | Operation::ADD => params[0..=2]
            .iter()
            .zip(opcode.parameter_modes.to_vec())
            .map(|(param, mode)| Parameter::new(mode, *param))
            .collect::<Vec<Parameter>>(),
        Operation::STORE => vec![Parameter::new(Mode::POSITION, params[0])],
        Operation::OUT => vec![Parameter::new(Mode::POSITION, params[0])],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_multiply_instruction_string_with_some_immediate_param() {
        assert_eq!(
            Instruction::new(
                Operation::MULTIPLY,
                vec![
                    Parameter::new(Mode::IMMEDIATE, 2),
                    Parameter::new(Mode::POSITION, 3),
                    Parameter::new(Mode::POSITION, 4),
                ]
            ),
            parse("10002,2,3,4")
        );
    }

    #[test]
    fn it_parses_add_instruction_string_with_all_immediate_params() {
        assert_eq!(
            Instruction::new(
                Operation::ADD,
                vec![
                    Parameter::new(Mode::IMMEDIATE, 2),
                    Parameter::new(Mode::IMMEDIATE, 2),
                    Parameter::new(Mode::IMMEDIATE, 2),
                ]
            ),
            parse("11101,2,2,2")
        );
    }

    #[test]
    fn it_parses_add_instruction_string_with_all_positional_params() {
        assert_eq!(
            Instruction::new(
                Operation::ADD,
                vec![
                    Parameter::new(Mode::POSITION, 1),
                    Parameter::new(Mode::POSITION, 1),
                    Parameter::new(Mode::POSITION, 1),
                ]
            ),
            parse("1,1,1,1")
        );
    }

    #[test]
    fn it_parses_store_instruction_string() {
        assert_eq!(
            Instruction::new(Operation::STORE, vec![Parameter::new(Mode::POSITION, 1)]),
            parse("3,1")
        );
    }

    #[test]
    fn it_parses_out_instruction_string() {
        assert_eq!(
            Instruction::new(Operation::OUT, vec![Parameter::new(Mode::POSITION, 1)]),
            parse("4,1")
        );
    }

    #[test]
    fn it_parses_exit_instruction_string() {
        assert_eq!(Instruction::new(Operation::EXIT, vec![]), parse("99"));
    }

    #[test]
    fn it_parses_unknown_instruction_string() {
        assert_eq!(Instruction::new(Operation::NOOP, vec![]), parse("0,1,1"));
    }
}
