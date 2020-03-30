use super::intcode::{ExitCode, IntCode};
use std::{
    cmp::{max, min},
    collections::HashMap,
};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn get_dir(current_dir: char, indice: i64) -> char {
    if indice == 0 {
        match current_dir {
            'U' => 'L',
            'L' => 'D',
            'D' => 'R',
            'R' => 'U',
            _ => unreachable!(),
        }
    } else {
        match current_dir {
            'U' => 'R',
            'R' => 'D',
            'D' => 'L',
            'L' => 'U',
            _ => unreachable!(),
        }
    }
}

fn paint(input: &[i64], start_value: i64) -> HashMap<(i32, i32), i64> {
    let mut map = HashMap::new();
    let mut robot = IntCode::new(input.to_vec(), vec![]);
    map.insert((0, 0), start_value);

    let (mut x, mut y, mut dir) = (0, 0, 'U');

    loop {
        match robot.execute() {
            ExitCode::Stop => break,
            ExitCode::Output(c) => {
                if let ExitCode::Output(d) = robot.execute() {
                    map.insert((x, y), c);
                    dir = get_dir(dir, d);
                    match dir {
                        'U' => y += 1,
                        'L' => x += 1,
                        'D' => y -= 1,
                        'R' => x -= 1,
                        _ => unreachable!(),
                    }
                } else {
                    unreachable!()
                }
            }
            ExitCode::AwaitInput => robot.push_input(*map.get(&(x, y)).unwrap_or(&0)),
            _ => unreachable!(),
        }
    }
    map
}

fn display_map(map: &HashMap<(i32, i32), i64>) {
    let (x_min, x_max, y_min, y_max) =
        map.keys()
            .fold((0, 0, 0, 0), |(x_min, x_max, y_min, y_max), &(x, y)| {
                let x_min = min(x, x_min);
                let x_max = max(x, x_max);
                let y_min = min(y, y_min);
                let y_max = max(y, y_max);
                (x_min, x_max, y_min, y_max)
            });
    println!("{}-{} {}-{}", x_min, x_max, y_min, y_max);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let v = map.get(&(x, y)).unwrap_or(&0);
            print!("{}", if v == &1 { '█' } else { ' ' })
        }
        println!()
    }
}

#[aoc(day11, part1)]
fn part1(input: &[i64]) -> usize {
    let map = paint(input, 0);
    display_map(&map);
    map.len()
}

#[aoc(day11, part2)]
fn part2(input: &[i64]) -> usize {
    let map = paint(input, 1);
    display_map(&map);
    map.len()
}
