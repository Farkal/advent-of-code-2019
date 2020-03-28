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

    // #[test]
    // fn test_part1() {
    //     let input = input_generator("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    //     assert_eq!(find_max_sequence(&input), 43210);
    //     let input = input_generator(
    //         "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
    //     );
    //     assert_eq!(find_max_sequence(&input), 54321);
    //     let input = input_generator("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    //     assert_eq!(find_max_sequence(&input), 65210);
    // }

    // #[test]
    // fn test_part2() {
    //     let input = input_generator(
    //         "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    //     );
    //     assert_eq!(find_max_sequence_part2(&input), 139_629_729);
    //     let input = input_generator(
    //         "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
    //     );
    //     assert_eq!(find_max_sequence_part2(&input), 18216);
    //     // assert_eq!(compute_total_mass(1969), 966);
    //     // assert_eq!(compute_total_mass(100756), 50346);
    // }

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
