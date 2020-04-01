use num::integer::lcm;
use std::cmp::Ordering;

#[aoc_generator(day12)]
fn input_generator(_: &str) -> Vec<Moon> {
    vec![
        Moon::new(-1, -4, 0),
        Moon::new(4, 7, -1),
        Moon::new(-14, -10, 9),
        Moon::new(1, 2, 17),
    ]
}

#[derive(Clone, Debug)]
struct Pos3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos3D {
    fn abs_sum(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, Debug)]
struct Moon {
    pos: Pos3D,
    velocity: Pos3D,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Moon {
            pos: Pos3D { x, y, z },
            velocity: Pos3D { x: 0, y: 0, z: 0 },
        }
    }

    fn get_energy(&self) -> i64 {
        let potential = self.pos.abs_sum();
        let kinetic = self.velocity.abs_sum();
        // dbg!(potential * kinetic);
        potential * kinetic
    }

    fn compare(&self, other: &Moon) -> Pos3D {
        let v = &self.pos;
        let ov = &other.pos;
        // dbg!("COMPARE", &v, &ov);
        let x = cmp_axis(v.x, ov.x);
        let y = cmp_axis(v.y, ov.y);
        let z = cmp_axis(v.z, ov.z);
        Pos3D { x, y, z }
    }

    fn add_velocity(&mut self, v: &Pos3D) {
        self.velocity.x += v.x;
        self.velocity.y += v.y;
        self.velocity.z += v.z;
    }

    fn remove_velocity(&mut self, v: &Pos3D) {
        self.velocity.x -= v.x;
        self.velocity.y -= v.y;
        self.velocity.z -= v.z;
    }

    fn update_gravity(&mut self) {
        // println!("Updating gravity {:?}", self.velocity);
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
        self.pos.z += self.velocity.z;
    }
}

fn cmp_axis(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn simulate(mut moons: Vec<Moon>, steps: u32) -> i64 {
    for _ in 1..=steps {
        // println!("Step {}", k);
        for i in 0..moons.len() {
            for j in (i + 1)..moons.len() {
                let d = moons[i].compare(&moons[j]);
                moons[i].add_velocity(&d);
                moons[j].remove_velocity(&d);
            }
            moons[i].update_gravity()
        }
    }

    moons.iter().map(|m| m.get_energy()).sum()
}

fn simulate_one_axis(values: &[i64]) -> usize {
    let mut values: Vec<(i64, i64)> = values.iter().map(|&v| (v, 0)).collect();
    let init = values.clone();
    for c in 1.. {
        for i in 0..values.len() {
            for j in (i + 1)..values.len() {
                let d = cmp_axis(values[i].0, values[j].0);
                values[i].1 += d;
                values[j].1 -= d;
            }
            values[i].0 += values[i].1
        }
        if values == init {
            return c;
        }
    }
    unreachable!()
}

#[aoc(day12, part1)]
fn part1(input: &[Moon]) -> i64 {
    simulate(input.to_vec(), 1000)
}

#[aoc(day12, part2)]
fn part2(input: &[Moon]) -> usize {
    let x = simulate_one_axis(&input.iter().map(|m| m.pos.x).collect::<Vec<_>>());
    let y = simulate_one_axis(&input.iter().map(|m| m.pos.y).collect::<Vec<_>>());
    let z = simulate_one_axis(&input.iter().map(|m| m.pos.z).collect::<Vec<_>>());
    lcm(x, lcm(y, z))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        assert_eq!(simulate(input, 10), 179);
        let input = vec![
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ];
        assert_eq!(simulate(input, 100), 1940);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        assert_eq!(part2(&input), 2772);
        let input = vec![
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ];
        assert_eq!(part2(&input), 4_686_774_924);
    }
}
