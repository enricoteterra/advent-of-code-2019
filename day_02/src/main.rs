use std::fs;

mod ship_computer;
use ship_computer::ShipComputer;

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading file failed");
    let instructions: Vec<usize> = input.split(",").map(|it| it.parse().unwrap()).collect();
    
    let mut input = instructions.to_vec();
    for noun in 0..99 {
        for verb in 0..99 {
            input[1] = noun;
            input[2] = verb;

            let mut computer = ShipComputer::new(Some(input.to_vec()));
            computer.run_program();

            if computer.memory[0] == 19690720 {
                println!("noun: {}, verb: {}, output: {}", noun, verb, 100 * noun + verb);
                break;
            }
        }
    }
    
    // instructions[1] = 12;
    // instructions[2] = 2;
    
    // let mut computer = ShipComputer::new(Some(instructions.to_vec()));
    // computer.run_program();

    // println!("{}", computer.memory[0]);
}