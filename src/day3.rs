#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.to_string()).collect())
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    steps: u64,
}

impl PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Wire {
    pos: Position,
    pub positions: Vec<Position>,
}

impl Wire {
    fn new(directions: Vec<String>) -> Self {
        let mut n = Wire {
            pos: Position {
                x: 0,
                y: 0,
                steps: 0,
            },
            positions: vec![],
        };
        n.execute_directions(directions);
        n
    }

    fn execute_direction(&mut self, direction: &str) {
        let (d, steps) = (&direction[0..1], direction[1..].parse::<u32>().unwrap());
        let mut new_pos: Vec<Position> = match d {
            "R" => (0..=steps)
                .map(|s| Position {
                    x: self.pos.x + s as i64,
                    y: self.pos.y,
                    steps: self.pos.steps + u64::from(s),
                })
                .collect(),
            "L" => (0..=steps)
                .map(|s| Position {
                    x: self.pos.x - s as i64,
                    y: self.pos.y,
                    steps: self.pos.steps + u64::from(s),
                })
                .collect(),
            "U" => (0..=steps)
                .map(|s| Position {
                    x: self.pos.x,
                    y: self.pos.y + s as i64,
                    steps: self.pos.steps + u64::from(s),
                })
                .collect(),
            "D" => (0..=steps)
                .map(|s| Position {
                    x: self.pos.x,
                    y: self.pos.y - s as i64,
                    steps: self.pos.steps + u64::from(s),
                })
                .collect(),
            _ => panic!("Unknown direction value {}", d),
        };
        // println!("PRV POS {:?} DIR {} NEW POS {:?}", self.pos, direction, new_pos.last().unwrap().clone());
        self.pos = *new_pos.last().unwrap();
        self.positions.append(&mut new_pos)
    }

    fn execute_directions(&mut self, directions: Vec<String>) {
        for d in directions {
            self.execute_direction(&d);
        }
    }
}

fn get_same_coordinates(wire1: Wire, wire2: Wire) -> Vec<(Position, Position)> {
    let mut res = vec![];
    for pos1 in &wire1.positions[1..] {
        for pos2 in &wire2.positions[1..] {
            if pos1 == pos2 {
                res.push((*pos1, *pos2));
            }
        }
    }
    res
}

fn get_smallest_distances(coordinates: Vec<(Position, Position)>) -> u32 {
    println!("SMALLES FIND {:?}", coordinates);
    coordinates
        .iter()
        .map(|(p1, _p2)| p1.x.abs() + p1.y.abs())
        .min()
        .unwrap() as u32
}

fn get_smallest_steps(coordinates: Vec<(Position, Position)>) -> u32 {
    println!("SMALLES FIND {:?}", coordinates);
    coordinates
        .iter()
        .map(|(p1, p2)| p1.steps + p2.steps)
        .min()
        .unwrap() as u32
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<String>]) -> u32 {
    let wire1 = Wire::new(input[0].clone());
    let wire2 = Wire::new(input[1].clone());
    let same_coord = get_same_coordinates(wire1, wire2);
    get_smallest_distances(same_coord)
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<String>]) -> u32 {
    let wire1 = Wire::new(input[0].clone());
    let wire2 = Wire::new(input[1].clone());
    let same_coord = get_same_coordinates(wire1, wire2);
    get_smallest_steps(same_coord)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("R8,U5,L5,D3\nU7,R6,D4,L4")), 6);
        assert_eq!(part1(&input_generator("R8,D5,L5,U3\nD7,R6,U4,L4")), 6);
        assert_eq!(
            part1(&input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        );
        assert_eq!(
            part1(&input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator("R8,U5,L5,D3\nU7,R6,D4,L4")), 30);
        assert_eq!(part2(&input_generator("R8,D5,L5,U3\nD7,R6,U4,L4")), 30);
        assert_eq!(
            part2(&input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
        assert_eq!(
            part2(&input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );
    }
}
