mod interpreter;
mod program;

#[derive(Clone, PartialEq, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub params: Vec<Parameter>,
}

impl Instruction {
    pub fn new(operation: Operation, params: Vec<Parameter>) -> Instruction {
        Instruction {
            opcode: Opcode {
                code: operation,
                parameter_modes: params.iter().map(|it| it.mode).collect(),
            },
            params: params,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Opcode {
    pub code: Operation,
    pub parameter_modes: Vec<Mode>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Operation {
    NOOP = 0,
    ADD = 1,
    MULTIPLY = 2,
    STORE = 3,
    OUT = 4,
    EXIT = 99,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Parameter {
    mode: Mode,
    value: usize,
}

impl Parameter {
    pub fn new(mode: Mode, value: usize) -> Parameter {
        Parameter {
            mode: mode,
            value: value,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mode {
    POSITION = 0,
    IMMEDIATE = 1,
}
