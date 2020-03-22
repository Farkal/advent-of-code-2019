use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct Orbit {
    parent: String,
    child: String,
}

impl FromStr for Orbit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.trim().split(')').collect();
        Ok(Orbit {
            parent: data[0].into(),
            child: data[1].into(),
        })
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Orbit> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn steps(g: &HashMap<String, Vec<String>>, node: &str, rec: usize) -> usize {
    let res = match g.get(node) {
        Some(v) => v.len() * rec + v.iter().map(|k| steps(g, k, rec + 1)).sum::<usize>(),
        None => 0,
    };
    res
}

fn find_neighbors<'a>(node: &str, input: &'a [Orbit]) -> HashSet<&'a String> {
    input
        .iter()
        .filter(|o| o.parent == node || o.child == node)
        .map(|o| {
            if o.parent == node {
                &o.child
            } else {
                &o.parent
            }
        })
        .collect()
}

fn find_path_len<'a>(
    input: &'a [Orbit],
    current: &'a str,
    goal: &str,
    visited: &mut HashSet<&'a str>,
) -> Option<usize> {
    if visited.contains(current) {
        return None;
    }
    visited.insert(current);

    if current == goal {
        return Some(0);
    }

    find_neighbors(current, input)
        .iter()
        .map(|p| find_path_len(input, p, goal, visited))
        .find(|o| o.is_some())
        .unwrap_or(None)
        .map(|i| i + 1)
}

#[aoc(day6, part1)]
pub fn part1(input: &[Orbit]) -> usize {
    let g: HashMap<String, Vec<String>> = input
        .iter()
        .map(|p1| {
            let v = input
                .iter()
                .filter(|p2| p2.parent == p1.parent)
                .map(|p| p.child.clone())
                .collect();
            (p1.parent.clone(), v)
        })
        .collect();


    let r = steps(&g, "COM", 1);
    // println!("Final result {}", r);
    r
}

#[aoc(day6, part2)]
pub fn part2(input: &[Orbit]) -> usize {
    find_path_len(input, "YOU", "SAN", &mut HashSet::new()).unwrap() - 2
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&input_generator(
                "B)G
G)H
D)I
E)J
J)K
COM)B
B)C
C)D
D)E
E)F
K)L"
            )),
            42
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&input_generator(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
            )),
            4
        );
    }
}
