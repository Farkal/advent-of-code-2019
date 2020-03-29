use super::intcode::{ExitCode, IntCode};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[i64]) -> i64 {
    let mut i = IntCode::new(input.to_vec(), vec![1]);
    loop {
        if i.execute() == ExitCode::Stop {
            break;
        }
    }
    i.output[0]
}

#[aoc(day9, part2)]
fn part2(input: &[i64]) -> i64 {
    let mut i = IntCode::new(input.to_vec(), vec![2]);
    loop {
        if i.execute() == ExitCode::Stop {
            break;
        }
    }
    i.output[0]
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn prepare_test(input: &str, manual_input: Vec<i64>) -> Vec<i64> {
        let input = &input_generator(input);
        let mut i = IntCode::new(input.to_vec(), manual_input);
        loop {
            if i.execute() == ExitCode::Stop {
                return i.output;
            }
        }
    }

    #[test]
    fn test_relative() {
        let r = prepare_test("1102,34915192,34915192,7,4,7,99,0", vec![]);
        assert_eq!(r[0], 1_219_070_632_396_864);
        let r = prepare_test("104,1125899906842624,99", vec![]);
        assert_eq!(r[0], 1_125_899_906_842_624);
        let r = prepare_test(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
            vec![],
        );
        assert_eq!(
            r,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }
}
