use super::intcode::{ExitCode, IntCode};
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn try_sequence(sequence: Vec<&i64>, input: &[i64]) -> i64 {
    let mut last_input = 0;
    for i in sequence {
        let mut amp = IntCode::new(input.to_vec(), vec![*i, last_input]);

        amp.execute();
        last_input = amp.output[0];
    }
    // println!("RES IS {}", last_input);
    last_input
}
fn try_sequence_until_halt(sequence: Vec<&i64>, input: &[i64]) -> i64 {
    let mut last_input = 0;
    let mut amps = vec![IntCode::new(input.to_vec(), vec![]); 5];
    for i in 0..5 {
        amps[i].push_input(*sequence[i]);
    }
    for i in (0..5).cycle() {
        amps[i].push_input(last_input);
        match amps[i].execute() {
            ExitCode::Output(o) => last_input = o,
            ExitCode::Stop => return last_input,
            _ => unreachable!(),
        }
    }
    unreachable!()
}

fn find_max_sequence(input: &[i64]) -> i64 {
    let sequences = [0, 1, 2, 3, 4].iter().permutations(5);
    sequences.map(|s| try_sequence(s, input)).max().unwrap()
}

fn find_max_sequence_part2(input: &[i64]) -> i64 {
    let sequences = [5, 6, 7, 8, 9].iter().permutations(5);
    sequences
        .map(|s| try_sequence_until_halt(s, input))
        .max()
        .unwrap()
}

#[aoc(day7, part1)]
fn part1(input: &[i64]) -> i64 {
    find_max_sequence(input)
}

#[aoc(day7, part2)]
fn part2(input: &[i64]) -> i64 {
    find_max_sequence_part2(input)
}

#[cfg(test)]
pub mod tests {
    use super::{find_max_sequence, find_max_sequence_part2, input_generator};

    #[test]
    fn test_part1() {
        let input = input_generator("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(find_max_sequence(&input), 43210);
        let input = input_generator(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(find_max_sequence(&input), 54321);
        let input = input_generator("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        assert_eq!(find_max_sequence(&input), 65210);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(find_max_sequence_part2(&input), 139_629_729);
        let input = input_generator(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(find_max_sequence_part2(&input), 18216);
        // assert_eq!(compute_total_mass(1969), 966);
        // assert_eq!(compute_total_mass(100756), 50346);
    }
}
