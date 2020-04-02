use super::intcode::{ExitCode, IntCode};
use std::cmp::Ordering;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub fn cmp_pb(b: i64, p: i64) -> i64 {
    match b.cmp(&p) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &[i64]) -> u32 {
    let mut i = IntCode::new(input.to_vec(), vec![]);
    let mut block_tile = 0;
    let mut j = 0;
    loop {
        match i.execute() {
            ExitCode::Output(o) => {
                if o == 2 && j == 2 {
                    block_tile += 1
                };
                j = (j + 1) % 3
            }
            ExitCode::Stop => break,
            _ => unreachable!(),
        }
    }
    block_tile
}

#[aoc(day13, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut input = input.to_vec();
    input[0] = 2;
    let mut i = IntCode::new(input, vec![]);
    let mut param = [0; 3];
    let (mut score, mut px, mut bx, mut j) = (0, 0, 0, 0);
    loop {
        match i.execute() {
            ExitCode::Output(o) => {
                param[j] = o;
                if j == 2 {
                    match param {
                        [-1, 0, s] => score = s,
                        [x, _, 3] => px = x,
                        [x, _, 4] => bx = x,
                        _ => {}
                    }
                }
                j = (j + 1) % 3
            }
            ExitCode::AwaitInput => i.push_input(cmp_pb(bx, px)),
            ExitCode::Stop => break,
            _ => unreachable!(),
        }
    }
    score
}
