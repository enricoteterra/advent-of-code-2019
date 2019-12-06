mod password;
mod complex_password;

use std::time::{Instant};

fn main() {
    // part 1
    let now = Instant::now();
    
    let input1 = 124075..580769;
    let number_of_possible_passwords: usize = input1
        .map(|it| password::create(it.to_string()))
        .filter(|it| it != password::NO_PASSWORD)
        .count();
    println!("number of possible passwords: {}", number_of_possible_passwords);

    println!("time taken: {}ms", now.elapsed().as_millis());

    // part 2
    let now2 = Instant::now();

    let input2 = 124075..580769;
    let number_of_possible_complex_passwords: usize = input2
        .map(|it| complex_password::create(it.to_string()))
        .filter(|it| it != complex_password::NO_COMPLEX_PASSWORD)
        .count();
    println!("number of possible complex passwords: {}", number_of_possible_complex_passwords);

    println!("time taken: {}ms", now2.elapsed().as_millis());
}
