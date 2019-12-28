use crate::computer::*;

pub fn parse_opcode(input: usize) -> Opcode {
    let digits: Vec<u32> = input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let mut corrected_input = vec![0; 5];
    corrected_input.splice(5 - digits.len()..5, digits);

    match corrected_input[4] {
        1 => Opcode {
            code: Operation::ADD,
            parameter_modes: parse_parameter_modes(&corrected_input[0..3]),
        },
        2 => Opcode {
            code: Operation::MULTIPLY,
            parameter_modes: parse_parameter_modes(&corrected_input[0..3]),
        },
        3 => Opcode {
            code: Operation::STORE,
            parameter_modes: parse_parameter_modes(&[corrected_input[1]]),
        },
        4 => Opcode {
            code: Operation::OUT,
            parameter_modes: parse_parameter_modes(&[corrected_input[1]]),
        },
        9 => Opcode {
            code: Operation::EXIT,
            parameter_modes: vec![],
        },
        _ => Opcode {
            code: Operation::NOOP,
            parameter_modes: vec![],
        },
    }
}

fn parse_parameter_modes(input: &[u32]) -> Vec<Mode> {
    input
        .iter()
        .map(|it| match it == &0 {
            true => Mode::POSITION,
            false => Mode::IMMEDIATE,
        })
        .collect()
}
