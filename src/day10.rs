use num::rational::Ratio;
use std::collections::{BTreeMap, HashSet};

#[aoc_generator(day10)]
fn input_generator(input: &str) -> HashSet<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(j, s)| {
            s.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(i, _)| Position::new(i as i64, j as i64))
        })
        .collect()
}

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd)]
struct Angle {
    x_sign: i64,
    ratio: Ratio<i64>,
}

impl Angle {
    fn new(x: i64, y: i64) -> Self {
        let x_sign = x.signum();
        let y_sign = y.signum();
        let ratio = if x_sign == 0 {
            Ratio::new(y_sign, 1)
        } else {
            Ratio::new(y, x)
        };
        Angle { x_sign, ratio }
    }
}

impl std::cmp::Ord for Angle {
    fn cmp(&self, other: &Angle) -> std::cmp::Ordering {
        // println!("cmp {:?} {:?}", self, other);
        if self == other {
            return std::cmp::Ordering::Equal;
        }
        if self.x_sign == 0 && *self.ratio.numer() < 0 {
            return std::cmp::Ordering::Less;
        }
        if other.x_sign == 0 && *other.ratio.numer() < 0 {
            return std::cmp::Ordering::Greater;
        }
        if self.x_sign > 0 && other.x_sign <= 0 {
            return std::cmp::Ordering::Less;
        }
        if other.x_sign > 0 && self.x_sign <= 0 {
            return std::cmp::Ordering::Greater;
        }
        if self.x_sign > 0 && other.x_sign > 0 {
            return self.ratio.cmp(&other.ratio);
        }
        if self.x_sign == 0 {
            return std::cmp::Ordering::Less;
        }
        if other.x_sign == 0 {
            return std::cmp::Ordering::Greater;
        }
        self.ratio.cmp(&other.ratio)
    }
}

fn get_best_position(asteroids: &HashSet<Position>) -> (Position, usize) {
    asteroids
        .iter()
        .map(|pos| {
            let nb: HashSet<_> = asteroids
                .iter()
                .filter(|&p| pos != p)
                .map(|p| {
                    let x = pos.x - p.x;
                    let y = pos.y - p.y;
                    Angle::new(x, y)
                })
                .collect();
            (*pos, nb.len())
        })
        .max_by_key(|(_, nb)| *nb)
        .unwrap()
}

#[aoc(day10, part1)]
fn part1(asteroids: &HashSet<Position>) -> usize {
    get_best_position(asteroids).1
}

#[aoc(day10, part2)]
fn part2(asteroids: &HashSet<Position>) -> usize {
    let station = get_best_position(asteroids).0;
    let mut directions: BTreeMap<Angle, Vec<Position>> = BTreeMap::new();
    for (a, p) in asteroids
        .iter()
        .filter(|&&p| p != station)
        .map(|p| (Angle::new(p.x - station.x, p.y - station.y), p))
    {
        directions.entry(a).or_default().push(*p);
    }
    for line in directions.values_mut() {
        line.sort_by_key(|p| -(p.x - station.x).abs() - (p.y - station.y).abs());
    }
    let mut index = 0;
    loop {
        for line in directions.values_mut() {
            if let Some(pos) = line.pop() {
                index += 1;
                if index == 200 {
                    // println!("INDEX 200 IS {}", pos.x * 100 + pos.y);
                    return (pos.x * 100 + pos.y) as usize;
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#..#
.....
#####
....#
...##";
        let input = &input_generator(input);
        assert_eq!(get_best_position(input), (Position::new(3, 4), 8));
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let input = &input_generator(input);
        assert_eq!(get_best_position(input), (Position::new(5, 8), 33));
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let input = &input_generator(input);
        assert_eq!(get_best_position(input), (Position::new(1, 2), 35));
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let input = &input_generator(input);
        assert_eq!(get_best_position(input), (Position::new(6, 3), 41));
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let input = &input_generator(input);
        assert_eq!(get_best_position(input), (Position::new(11, 13), 210));
        assert_eq!(part2(input), 802);
    }
}
