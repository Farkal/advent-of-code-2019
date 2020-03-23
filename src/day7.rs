use super::intcode::IntCode;
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn try_sequence(sequence: Vec<&i32>, input: &[i32]) -> i32 {
    let mut last_input = 0;
    for i in sequence {
        let mut amp = IntCode::new(input.to_vec(), Some(vec![*i, last_input]));

        amp.execute();
        last_input = amp.output[0];
    }
    // println!("RES IS {}", last_input);
    last_input
}

fn find_max_sequence(input: &[i32]) -> i32 {
    let sequences = [0, 1, 2, 3, 4].iter().permutations(5);
    sequences.map(|s| try_sequence(s, input)).max().unwrap()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> i32 {
    find_max_sequence(input)
}

// #[aoc(day7, part2)]
// fn part2(input: &[u32]) -> u32 {
//     input.iter().map(|x| compute_total_mass(*x)).sum()
// }

#[cfg(test)]
pub mod tests {
    use super::{find_max_sequence, input_generator};

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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(compute_total_mass(14), 2);
    //     assert_eq!(compute_total_mass(1969), 966);
    //     assert_eq!(compute_total_mass(100756), 50346);
    // }
}
