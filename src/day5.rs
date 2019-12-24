use std::io::{stdin, stdout, Write};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

struct IntCode {
    index: usize,
    content: Vec<i32>,
    manual_input: Option<Vec<i32>>,
    output: Vec<i32>
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn from_int(m: i32) -> ParamMode {
        match m {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!("Unknow param mode"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum OperationType {
    Add,
    Mult,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    Stop,
    Unknown,
}

impl OperationType {
    fn from_int(o: i32) -> OperationType {
        match o {
            99 => OperationType::Stop,
            1 => OperationType::Add,
            2 => OperationType::Mult,
            3 => OperationType::Input,
            4 => OperationType::Output,
            5 => OperationType::JumpTrue,
            6 => OperationType::JumpFalse,
            7 => OperationType::LessThan,
            8 => OperationType::Equals,
            _ => OperationType::Unknown,
        }
    }
}

#[derive(Debug)]
struct Operation {
    mode: OperationType,
    params_mode: Vec<ParamMode>,
}

fn parse_parameter(p: i32) -> Operation {
    let p = &format!("{:05}", p)[..];
    let p3type = p[..1].parse::<i32>().unwrap();
    let p2type = p[1..2].parse::<i32>().unwrap();
    let p1type = p[2..3].parse::<i32>().unwrap();
    let o = p[3..5].parse::<i32>().unwrap();
    // println!("{} {} {} {}", o, p1type, p2type, p3type);
    Operation {
        mode: OperationType::from_int(o),
        params_mode: vec![
            ParamMode::from_int(p1type),
            ParamMode::from_int(p2type),
            ParamMode::from_int(p3type),
        ],
    }
}

impl IntCode {
    fn execute(&mut self) -> Vec<i32> {
        while self.execute_operation() {}
        self.content.clone()
    }

    fn execute_operation(&mut self) -> bool {
        // println!("{} - OP {} {:?}", self.index, self.content[self.index], &self.content[..]);
        let o_mode = OperationType::from_int(self.content[self.index]);
        let o = if OperationType::Unknown == o_mode {
            parse_parameter(self.content[self.index])
        } else {
            Operation {
                mode: o_mode,
                params_mode: vec![ParamMode::Position; 3],
            }
        };
        self.run_operation(o)
    }

    fn run_operation(&mut self, o: Operation) -> bool {
        // println!("{:?}", o);
        match o.mode {
            OperationType::Add => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                println!("{} + {}", val1, val2);
                let i = self.content[self.index + 3] as usize;
                self.write_result_to_addr(i, val1 + val2);
                self.index += 4;
                true
            }
            OperationType::Mult => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                // println!("{} * {}", val1, val2);
                let i = self.content[self.index + 3] as usize;
                self.write_result_to_addr(i, val1 * val2);
                self.index += 4;
                true
            }
            OperationType::Input => {
                let a = self.content[self.index + 1] as usize;
                self.get_input(a);
                self.index += 2;
                true
            }
            OperationType::Output => {
                let val1 = self.get_param(1, o.params_mode[0]);
                self.display_value(val1);
                self.index += 2;
                true
            }
            OperationType::JumpTrue => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                // println!("{} {}", val1, val2);
                if val1 != 0 {
                    self.index = val2 as usize;
                } else {
                    self.index += 3;
                }
                true
            }
            OperationType::JumpFalse => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                if val1 == 0 {
                    self.index = val2 as usize;
                } else {
                    self.index += 3;
                }
                true
            }
            OperationType::LessThan => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                let val3 = self.get_param(3, ParamMode::Immediate);
                self.content[val3 as usize] = (val1 < val2) as i32;
                self.index += 4;
                true
            }
            OperationType::Equals => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                let val3 = self.get_param(3, ParamMode::Immediate);
                // println!("{} == {} -> {} {} {:?}", val1, val2, val3, self.content[self.index + 3], o.params_mode[2]);
                self.content[val3 as usize] = (val1 == val2) as i32;
                self.index += 4;
                true
            }
            OperationType::Stop => false,
            _ => {
                println!("Unknown op code");
                false
            }
        }
    }

    fn get_param(&self, i: usize, mode: ParamMode) -> i32 {
        let mut val = self.content[self.index + i];
        if mode == ParamMode::Position {
            val = self.content[val as usize]
        }
        val
    }

    fn write_result_to_addr(&mut self, i: usize, val: i32) {
        self.content[i] = val;
    }

    fn get_input(&mut self, address: usize) {
        if let Some(m_i) = self.manual_input.clone() {
            println!("INPUT = {}", m_i[0]);
            self.write_result_to_addr(address, m_i[0])
        } else {
            let mut s = String::new();
            print!("Please enter some text: ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            println!("You typed: {}", s);
            self.write_result_to_addr(address, s.parse::<i32>().unwrap())
        }
    }

    fn display_value(&mut self, val: i32) {
        // Display content at the address of index + 1
        println!("DISPLAYING {}", val);
        self.output.push(val);
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &[i32]) -> String {
    let mut i = IntCode {
        index: 0,
        content: input.to_vec(),
        manual_input: None,
        output: vec![]
    };
    let res = i.execute();
    let res: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    res.join(",")
}

#[aoc(day5, part2)]
pub fn part2(input: &[i32]) -> String {
    let mut i = IntCode {
        index: 0,
        content: input.to_vec(),
        manual_input: None,
        output: vec![]
    };
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
        let mut i = IntCode {
            index: 0,
            content: input.to_vec(),
            manual_input,
            output: vec![]
        };
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
