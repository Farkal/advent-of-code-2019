use std::io::{stdin, stdout, Write};

pub struct IntCode {
    pub index: usize,
    pub content: Vec<i32>,
    pub manual_input: Option<Vec<i32>>,
    pub manual_input_index: usize,
    pub output: Vec<i32>,
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
    pub fn new(input: Vec<i32>, manual_input: Option<Vec<i32>>) -> Self {
        IntCode {
            index: 0,
            content: input,
            manual_input,
            manual_input_index: 0,
            output: vec![],
        }
    }
    pub fn execute(&mut self) -> Vec<i32> {
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
                // println!("{} + {}", val1, val2);
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
            let input = m_i[self.manual_input_index];
            // println!("INPUT = {}", input);
            self.write_result_to_addr(address, input);
            self.manual_input_index += 1;
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
        // println!("DISPLAYING {}", val);
        self.output.push(val);
    }
}
