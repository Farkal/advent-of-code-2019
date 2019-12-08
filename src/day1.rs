#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn compute_mass(x: &u32) -> u32 {
    let res = i64::from(*x) / 3 - 2;
    if res > 0 {
        res as u32
    } else {
        0
    }
}

fn compute_total_mass(x: &u32) -> u32 {
    let m = compute_mass(x);
    if m == 0 {
        m
    } else {
        m + compute_total_mass(&m)
    }
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().map(compute_mass).sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    input.iter().map(compute_total_mass).sum()
}

#[cfg(test)]
pub mod tests {
    use super::{compute_mass, compute_total_mass};

    #[test]
    fn test_part1() {
        assert_eq!(compute_mass(&12), 2);
        assert_eq!(compute_mass(&14), 2);
        assert_eq!(compute_mass(&1969), 654);
        assert_eq!(compute_mass(&100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(compute_total_mass(&14), 2);
        assert_eq!(compute_total_mass(&1969), 966);
        assert_eq!(compute_total_mass(&100756), 50346);
    }
}
