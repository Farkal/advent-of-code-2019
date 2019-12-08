#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn intcode_compute(mut input: Vec<u32>) -> Vec<u32> {
    let mut index = 0;
    while input[index] != 99 {
        let (a, b, r) = (
            input[index + 1] as usize,
            input[index + 2] as usize,
            input[index + 3] as usize,
        );
        match input[index] {
            1 => input[r] = input[a] + input[b],
            2 => input[r] = input[a] * input[b],
            _ => panic!("Unexpected input value {}", input[0]),
        }
        index += 4;
    }
    input
}

#[aoc(day2, part1)]
pub fn part1(input: &[u32]) -> String {
    let res = intcode_compute(input.to_vec());
    let res: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    let res = res.join(",");
    res
}

#[aoc(day2, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let input = input.to_vec();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut fresh_input = input.clone();
            fresh_input[1] = noun;
            fresh_input[2] = verb;
            let res = intcode_compute(fresh_input);
            if res[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    return 0;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("1,0,0,0,99")), "2,0,0,0,99");
        assert_eq!(part1(&input_generator("2,3,0,3,99")), "2,3,0,6,99");
        assert_eq!(part1(&input_generator("2,4,4,5,99,0")), "2,4,4,5,99,9801");
        assert_eq!(
            part1(&input_generator("1,1,1,4,99,5,6,0,99")),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}
