use crate::app::opcode_parser::parse_opcode;
use crate::computer::*;

pub fn parse(input: &str) -> Vec<Instruction> {
    let mut codes: Vec<usize> = input.split(",").map(|it| it.parse().unwrap()).collect();
    codes.reverse();

    let mut instructions: Vec<Instruction> = vec![];
    while !codes.is_empty() {
        let opcode = parse_opcode(codes.pop().unwrap());
        if opcode.code != Operation::NOOP {
            let mut params: Vec<Parameter> = vec![];
            for mode in opcode.parameter_modes {
                params.push(Parameter::new(mode, codes.pop().unwrap()));
            }
            instructions.push(Instruction::new(opcode.code, params));
        }
    }
    return instructions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_list_of_instructions() {
        assert_eq!(
            vec![
                Instruction::new(
                    Operation::ADD,
                    vec![
                        Parameter::new(Mode::POSITION, 9),
                        Parameter::new(Mode::POSITION, 10),
                        Parameter::new(Mode::POSITION, 3),
                    ]
                ),
                Instruction::new(
                    Operation::MULTIPLY,
                    vec![
                        Parameter::new(Mode::POSITION, 3),
                        Parameter::new(Mode::POSITION, 11),
                        Parameter::new(Mode::POSITION, 0),
                    ]
                ),
                Instruction::new(Operation::EXIT, vec![])
            ],
            parse("1,9,10,3,2,3,11,0,99,30,40,50")
        )
    }
}
