use std::fs;

mod ship_computer;
use ship_computer::ShipComputer;

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading file failed");
    let mut instructions: Vec<usize> = input.split(",").map(|it| it.parse().unwrap()).collect();
    
    instructions[1] = 12;
    instructions[2] = 2;
    
    let mut computer = ShipComputer::new(Some(instructions.to_vec()));
    computer.run_program();

    println!("{}", computer.memory[0]);
}