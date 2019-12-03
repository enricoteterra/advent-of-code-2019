use std::fs;

mod program;
use program::Program;

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading file failed");
    let instructions: Vec<usize> = input.split(",").map(|it| it.parse().unwrap()).collect();
    let mut program = Program::new(&instructions);
    program.run();
}
