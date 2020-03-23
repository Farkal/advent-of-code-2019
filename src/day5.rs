use super::intcode::IntCode;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[i32]) -> String {
    let mut i = IntCode::new(input.to_vec(), None);
    let res = i.execute();
    let res: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    res.join(",")
}

#[aoc(day5, part2)]
pub fn part2(input: &[i32]) -> String {
    let mut i = IntCode::new(input.to_vec(), None);
    let res = i.execute();
    let res: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    res.join(",")
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

    #[test]
    fn test_part1_parameters() {
        assert_eq!(part1(&input_generator("1002,4,3,4,33")), "1002,4,3,4,99");
        assert_eq!(
            part1(&input_generator("1101,100,-1,4,0")),
            "1101,100,-1,4,99"
        );
    }

    fn prepare_test(input: &str, manual_input: Option<Vec<i32>>) -> Vec<i32> {
        let input = &input_generator(input);
        let mut i = IntCode::new(input.to_vec(), manual_input);
        i.execute();
        i.output
    }

    #[test]
    fn test_equals() {
        // If input equals 8 output 1 else 0
        let r = prepare_test("3,9,8,9,10,9,4,9,99,-1,8", Some(vec![0]));
        assert_eq!(r[0], 0);
        let r = prepare_test("3,9,8,9,10,9,4,9,99,-1,8", Some(vec![8]));
        assert_eq!(r[0], 1);
        let r = prepare_test("3,3,1108,-1,8,3,4,3,99", Some(vec![0]));
        assert_eq!(r[0], 0);
        let r = prepare_test("3,3,1108,-1,8,3,4,3,99", Some(vec![8]));
        assert_eq!(r[0], 1);
    }

    #[test]
    fn test_less_than() {
        // If input less than 8 output 1 else 0
        let r = prepare_test("3,9,7,9,10,9,4,9,99,-1,8", Some(vec![9]));
        assert_eq!(r[0], 0);
        let r = prepare_test("3,9,7,9,10,9,4,9,99,-1,8", Some(vec![7]));
        assert_eq!(r[0], 1);
        let r = prepare_test("3,3,1107,-1,8,3,4,3,99", Some(vec![9]));
        assert_eq!(r[0], 0);
        let r = prepare_test("3,3,1107,-1,8,3,4,3,99", Some(vec![7]));
        assert_eq!(r[0], 1);
    }

    #[test]
    fn test_jump() {
        let r = prepare_test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", Some(vec![0]));
        assert_eq!(r[0], 0);
        let r = prepare_test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", Some(vec![1]));
        assert_eq!(r[0], 1);
        let r = prepare_test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", Some(vec![0]));
        assert_eq!(r[0], 0);
        let r = prepare_test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", Some(vec![1]));
        assert_eq!(r[0], 1);
    }

    #[test]
    fn day5_test_part2() {
        let r = prepare_test("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", Some(vec![7]));
        assert_eq!(r[0], 999);
        let r = prepare_test("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", Some(vec![8]));
        assert_eq!(r[0], 1000);
        let r = prepare_test("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", Some(vec![9]));
        assert_eq!(r[0], 1001);
    }
}
