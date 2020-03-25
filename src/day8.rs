use itertools::Itertools;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

fn count_digit(x: &[u32], digit: u32) -> usize {
    x.iter().filter(|&&x| x == digit).count()
}

#[aoc(day8, part1)]
fn part1(input: &[u32]) -> usize {
    let min_zero_chunk = input
        .chunks(25 * 6)
        .min_by_key(|x| count_digit(x, 0))
        .unwrap();
    let ones = count_digit(min_zero_chunk, 1);
    let twos = count_digit(min_zero_chunk, 2);
    ones * twos
}

#[aoc(day8, part2)]
fn part2(input: &[u32]) -> u32 {
    let mut res = vec![2; 150];
    for c in input.chunks(25 * 6) {
        res = c
            .iter()
            .zip(res.iter())
            .map(|(&p_x, &p_res)| if p_res == 2 { p_x } else { p_res })
            .collect();
    }
    let message = res
        .iter()
        .map(|&x| if x == 0 { ' ' } else { '█' })
        .chunks(25);

    for l in &message {
        println!("{}", l.collect::<String>())
    }

    0
}
