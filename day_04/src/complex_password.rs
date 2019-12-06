use std::collections::HashMap;
use crate::password;

type ComplexPassword = String;


pub fn create(input: String) -> ComplexPassword {
    if !has_at_least_one_group_of_2_matching_digits(&input) { return NO_COMPLEX_PASSWORD.to_string(); }
    return password::create(input);
}

fn has_at_least_one_group_of_2_matching_digits(input: &str) -> bool {
    let digits = input.chars().collect::<Vec<char>>();
    let groups = find_adjacent_groups(&digits);
    return !groups.iter()
        .map(|(_, val)| val)
        .filter(|val| **val == 2)
        .collect::<Vec<&i32>>()
        .is_empty();
}

fn find_adjacent_groups(input: &Vec<char>) -> HashMap<char, i32> {
    let mut groups: HashMap<char, i32> = HashMap::new();
    let mut previous_digit = NO_DIGIT;

    for digit in input.iter() {
        if previous_digit != NO_DIGIT && digit == previous_digit {
            groups.insert(*digit, groups.get(digit).unwrap_or(&1) +1);
        }
        previous_digit = digit;
    }
    return groups;
}

pub static NO_COMPLEX_PASSWORD: &str = password::NO_PASSWORD;
static NO_DIGIT: &char = &'_';

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_must_have_at_least_one_group_of_2_matching_digits() {
        assert_eq!(NO_COMPLEX_PASSWORD, create("123444".to_string()));
        assert_eq!("112233", create("112233".to_string()));
        assert_eq!("111122", create("111122".to_string()));
    }
}