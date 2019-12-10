fn number_to_vec(n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn check_digit_increase(digits: &Vec<u32>) -> bool {
    for i in 0..digits.len() - 1 {
        if digits[i] > digits[i + 1] {
            return false;
        }
    }
    true
}

fn check_digit_double(digits: &Vec<u32>) -> bool {
    for i in 0..digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            return true;
        }
    }
    false
}

fn check_digit_double_strict(digits: &Vec<u32>) -> bool {
    let mut last_value = digits[0];
    let mut count_value = 1;
    for i in 1..digits.len() {
        if last_value == digits[i] {
            count_value += 1
        } else if count_value == 2 {
            return true;
        } else {
            count_value = 1;
            last_value = digits[i]
        }
    }
    count_value == 2
}

fn check_password(n: u32) -> bool {
    let vec_n = number_to_vec(n);
    check_digit_increase(&vec_n) && check_digit_double(&vec_n)
}

fn check_password_strict(n: u32) -> bool {
    let vec_n = number_to_vec(n);
    check_digit_increase(&vec_n) && check_digit_double_strict(&vec_n)
}

#[aoc(day4, part1)]
pub fn part1(range: &str) -> usize {
    let range: Vec<u32> = range
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut passwords = vec![];
    for n in range[0]..range[1] + 1 {
        if check_password(n) {
            passwords.push(n)
        }
    }
    passwords.len()
}

#[aoc(day4, part2)]
pub fn part2(range: &str) -> usize {
    let range: Vec<u32> = range
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut passwords = vec![];
    for n in range[0]..range[1] + 1 {
        if check_password_strict(n) {
            passwords.push(n)
        }
    }
    passwords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(check_password(111111), true);
        assert_eq!(check_password(223450), false);
        assert_eq!(check_password(123789), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(check_password_strict(112233), true);
        assert_eq!(check_password_strict(123444), false);
        assert_eq!(check_password_strict(111122), true);
    }
}
