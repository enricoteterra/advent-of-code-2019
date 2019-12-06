type Password = String;

pub fn create(input: String) -> Password {
    if input.len() != 6 { return NO_PASSWORD.to_string(); }
    if !has_at_least_one_pair_of_repeated_adjacent_chars(&input) { return NO_PASSWORD.to_string(); }
    if !has_all_digits_in_increasing_order(&input) { return NO_PASSWORD.to_string(); }
    return input;
}

fn has_at_least_one_pair_of_repeated_adjacent_chars(input: &str) -> bool {
    let digits = input.chars().collect::<Vec<char>>();
    let mut previous_digit: &char = NO_DIGIT;
    let mut has_repeated_adjacent_chars = false;

    for digit in digits.iter() {
        if previous_digit != NO_DIGIT && previous_digit == digit {
            has_repeated_adjacent_chars = true;
            break;
        }
        previous_digit = digit;
    }
    return has_repeated_adjacent_chars;
}

fn has_all_digits_in_increasing_order(input: &str) -> bool {
    let digits = input.chars().collect::<Vec<char>>();
    let mut previous_digit: &char = &'0';
    let mut has_digits_in_increasing_order = true;

    for digit in digits.iter() {
        if previous_digit != NO_DIGIT && previous_digit.to_digit(10) > digit.to_digit(10) {
            has_digits_in_increasing_order = false;
            break;
        }
        previous_digit = digit;
    }
    return has_digits_in_increasing_order;
}

pub static NO_PASSWORD: &str = "NO_PASSWORD";
static NO_DIGIT: &char = &'_';


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_must_have_six_digits() {
        assert_eq!(NO_PASSWORD, create("".to_string()));
        assert_eq!(NO_PASSWORD, create("13556".to_string()));
        assert_eq!(NO_PASSWORD, create("1355689".to_string()));
        assert_eq!("124557", create("124557".to_string()));
    }

    #[test]
    fn password_must_have_two_adjacent_digits_that_are_the_same() {
        assert_eq!(NO_PASSWORD, create("123456".to_string()));
        assert_eq!("122446", create("122446".to_string()));
        assert_eq!("122456", create("122456".to_string()));
    }

    #[test]
    fn password_digits_must_be_in_increasing_order() {
        assert_eq!(NO_PASSWORD, create("123450".to_string()));
        assert_eq!(NO_PASSWORD, create("122454".to_string()));
        assert_eq!(NO_PASSWORD, create("101111".to_string()));
        assert_eq!("111111", create("111111".to_string()));
        assert_eq!("123378", create("123378".to_string()));
    }
}