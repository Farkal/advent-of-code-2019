// use std::io::{stdin, stdout, Write};

#[derive(Debug, PartialEq)]
pub enum ExitCode {
    Output(i64),
    Stop,
    AwaitInput,
    UnknowCode,
}

#[derive(Debug, Clone)]
pub struct IntCode {
    pub index: usize,
    pub content: Vec<i64>,
    pub manual_input: Vec<i64>,
    pub manual_input_index: usize,
    pub output: Vec<i64>,
    pub relative_base: i64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn from_int(m: i64) -> ParamMode {
        match m {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
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
    AddToRelative,
    Stop,
    Unknown,
}

impl OperationType {
    fn from_int(o: i64) -> OperationType {
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
            9 => OperationType::AddToRelative,
            _ => OperationType::Unknown,
        }
    }
}

#[derive(Debug)]
struct Operation {
    mode: OperationType,
    params_mode: Vec<ParamMode>,
}

fn parse_parameter(p: i64) -> Operation {
    let p = &format!("{:05}", p)[..];
    let p3type = p[..1].parse::<i64>().unwrap();
    let p2type = p[1..2].parse::<i64>().unwrap();
    let p1type = p[2..3].parse::<i64>().unwrap();
    let o = p[3..5].parse::<i64>().unwrap();
    // println!("{} {} {} {}", o, p1type, p2type, p3type);
    // println!("Opération is {:?}", OperationType::from_int(o));
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
    pub fn new(input: Vec<i64>, manual_input: Vec<i64>) -> Self {
        IntCode {
            index: 0,
            content: input,
            manual_input,
            manual_input_index: 0,
            output: vec![],
            relative_base: 0,
        }
    }
    pub fn execute(&mut self) -> ExitCode {
        loop {
            if let Err(e) = self.execute_operation() {
                return e;
            }
        }
    }

    fn execute_operation(&mut self) -> Result<bool, ExitCode> {
        // println!(
        //     "{} - OP {} {:?} {}",
        //     self.index,
        //     self.content[self.index],
        //     self.manual_input,
        //     self.manual_input_index
        //     // &self.content[..]
        // );
        let o_mode = OperationType::from_int(self.content[self.index]);
        let o = if OperationType::Unknown == o_mode {
            parse_parameter(self.content[self.index])
        } else {
            Operation {
                mode: o_mode,
                params_mode: vec![ParamMode::Position; 3],
            }
        };
        // println!("EXECUTING OP {:?}", o);
        self.run_operation(o)
    }

    fn run_operation(&mut self, o: Operation) -> Result<bool, ExitCode> {
        // println!("{:?}", o);
        match o.mode {
            OperationType::Add => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                // println!("{} + {}", val1, val2);
                let i = self.get_addr(3, o.params_mode[2]);
                self.write_result_to_addr(i as usize, val1 + val2);
                self.index += 4;
                Ok(true)
            }
            OperationType::Mult => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                // println!("{} * {}", val1, val2);
                let i = self.get_addr(3, o.params_mode[2]);
                self.write_result_to_addr(i as usize, val1 * val2);
                self.index += 4;
                Ok(true)
            }
            OperationType::Input => {
                // println!("INPUT {:?} {}", o, self.content[self.index + 1]);
                let a = self.get_addr(1, o.params_mode[0]);
                self.get_input(a as usize)?;
                self.index += 2;
                Ok(true)
            }
            OperationType::Output => {
                let val1 = self.get_param(1, o.params_mode[0]);
                self.display_value(val1);
                self.index += 2;
                Err(ExitCode::Output(val1))
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
                Ok(true)
            }
            OperationType::JumpFalse => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                if val1 == 0 {
                    self.index = val2 as usize;
                } else {
                    self.index += 3;
                }
                Ok(true)
            }
            OperationType::LessThan => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                let addr = self.get_addr(3, o.params_mode[2]);
                self.write_result_to_addr(addr as usize, (val1 < val2) as i64);
                self.index += 4;
                Ok(true)
            }
            OperationType::Equals => {
                let val1 = self.get_param(1, o.params_mode[0]);
                let val2 = self.get_param(2, o.params_mode[1]);
                let addr = self.get_addr(3, o.params_mode[2]);
                // println!("{} == {} -> {} {} {:?}", val1, val2, addr, self.content[self.index + 3], o.params_mode[2]);
                self.write_result_to_addr(addr as usize, (val1 == val2) as i64);
                self.index += 4;
                Ok(true)
            }
            OperationType::AddToRelative => {
                let val1 = self.get_param(1, o.params_mode[0]);
                self.relative_base += val1;
                self.index += 2;
                Ok(true)
            }
            OperationType::Stop => Err(ExitCode::Stop),
            _ => {
                println!("Unknown op code");
                Err(ExitCode::UnknowCode)
            }
        }
    }

    fn get_param(&self, i: usize, mode: ParamMode) -> i64 {
        // println!("Get param {} by {:?}", self.index + i, mode);
        let mut val = self.content[self.index + i];
        // println!("VALUE {}", val);
        if mode == ParamMode::Position {
            val = if val >= self.content.len() as i64 {
                0
            } else {
                self.content[val as usize]
            }
        } else if mode == ParamMode::Relative {
            let index = self.relative_base + val;
            val = if index >= self.content.len() as i64 {
                0
            } else {
                self.content[index as usize]
            }
        }
        val
    }

    fn get_addr(&self, i: usize, mode: ParamMode) -> i64 {
        // println!("Get addr {} by {:?}", self.index + i, mode);
        match mode {
            ParamMode::Position | ParamMode::Immediate => self.content[self.index + i],
            ParamMode::Relative => self.content[self.index + i] + self.relative_base,
        }
    }

    fn write_result_to_addr(&mut self, i: usize, val: i64) {
        if i >= self.content.len() {
            self.content.resize(i + 1, 0);
        }
        self.content[i] = val;
    }

    pub fn push_input(&mut self, input: i64) {
        self.manual_input.push(input)
    }

    fn get_input(&mut self, address: usize) -> Result<(), ExitCode> {
        // println!("Get inputs");
        if self.manual_input_index >= self.manual_input.len() {
            return Err(ExitCode::AwaitInput);
        }
        let input = self.manual_input[self.manual_input_index];
        // println!("INPUT = {}", input);
        self.write_result_to_addr(address, input);
        self.manual_input_index += 1;
        // } else {
        //     let mut s = String::new();
        //     print!("Please enter some text: ");
        //     let _ = stdout().flush();
        //     stdin()
        //         .read_line(&mut s)
        //         .expect("Did not enter a correct string");
        //     if let Some('\n') = s.chars().next_back() {
        //         s.pop();
        //     }
        //     if let Some('\r') = s.chars().next_back() {
        //         s.pop();
        //     }
        //     println!("You typed: {}", s);
        //     self.write_result_to_addr(address, s.parse::<i32>().unwrap())
        // }
        Ok(())
    }

    fn display_value(&mut self, val: i64) {
        // Display content at the address of index + 1
        // println!("DISPLAYING {}", val);
        self.output.push(val);
    }
}
