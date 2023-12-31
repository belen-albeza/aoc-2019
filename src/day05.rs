use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::io;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ParamMode {
    Position,
    Immediate,
}

impl TryFrom<i64> for ParamMode {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(format!("unrecognized param mode: {}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Opcode {
    Add,
    Mul,
    Halt,
    Input,
    Output,
    JumpNotZero,
    JumpZero,
    Less,
    Equal,
}

impl TryFrom<i64> for Opcode {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpNotZero),
            6 => Ok(Self::JumpZero),
            7 => Ok(Self::Less),
            8 => Ok(Self::Equal),
            99 => Ok(Self::Halt),
            _ => Err(format!("unsupported opcode {}", value)),
        }
    }
}

impl Opcode {
    pub fn modes_mask(&self) -> [Option<ParamMode>; 3] {
        match self {
            Self::Add => [None, None, Some(ParamMode::Immediate)],
            Self::Mul => [None, None, Some(ParamMode::Immediate)],
            Self::Input => [Some(ParamMode::Immediate), None, None],
            Self::Less => [None, None, Some(ParamMode::Immediate)],
            Self::Equal => [None, None, Some(ParamMode::Immediate)],
            _ => [None, None, None],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Instruction {
    opcode: Opcode,
    modes: [ParamMode; 3],
}

impl TryFrom<i64> for Instruction {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(value % 100)?;
        let mut modes = [ParamMode::Position; 3];

        for i in 0..modes.len() {
            let raw_mode = (value % 10_i64.pow(3 + i as u32)) / 10_i64.pow(2 + i as u32);
            let unmasked_mode = ParamMode::try_from(raw_mode)?;
            modes[i] = opcode.modes_mask()[i].unwrap_or(unmasked_mode);
        }

        Ok(Self { opcode, modes })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct VM {
    ip: usize,
    memory: Vec<i64>,
}

impl VM {
    pub fn new(src: &[i64]) -> Self {
        Self {
            ip: 0,
            memory: src.to_owned(),
        }
    }

    pub fn run(
        &mut self,
        input: &mut impl io::BufRead,
        output: &mut impl io::Write,
    ) -> Result<i64, String> {
        while let Some(raw_instruction) = self.memory.get(self.ip) {
            let instruction = Instruction::try_from(*raw_instruction)?;
            match instruction.opcode {
                Opcode::Add => self.exec_add(instruction.modes)?,
                Opcode::Mul => self.exec_mul(instruction.modes)?,
                Opcode::Input => self.exec_input(input, instruction.modes)?,
                Opcode::Output => self.exec_output(output, instruction.modes)?,
                Opcode::JumpNotZero => self.exec_jump_not_zero(instruction.modes)?,
                Opcode::JumpZero => self.exec_jump_zero(instruction.modes)?,
                Opcode::Less => self.exec_less(instruction.modes)?,
                Opcode::Equal => self.exec_equal(instruction.modes)?,
                Opcode::Halt => break,
            }
        }

        Ok(*self.memory.get(0).unwrap_or(&0))
    }

    fn read_mem(&self, addr: usize) -> Result<i64, String> {
        self.memory
            .get(addr)
            .map(|x| *x)
            .ok_or(format!("invalid address: {}", addr))
    }

    fn write_mem(&mut self, addr: usize, value: i64) -> Result<(), String> {
        let i = self
            .memory
            .get_mut(addr)
            .ok_or(format!("invalid address: {}", addr))?;
        *i = value;

        Ok(())
    }

    fn read_params3(&mut self, modes: [ParamMode; 3]) -> Result<(i64, i64, i64), String> {
        let mut params = [
            self.read_mem(self.ip + 1)?,
            self.read_mem(self.ip + 2)?,
            self.read_mem(self.ip + 3)?,
        ];

        for i in 0..params.len() {
            if modes[i] == ParamMode::Position {
                params[i] = self.read_mem(params[i] as usize)?;
            }
        }

        Ok((params[0], params[1], params[2]))
    }

    fn read_params2(&mut self, modes: [ParamMode; 3]) -> Result<(i64, i64), String> {
        let mut params = [self.read_mem(self.ip + 1)?, self.read_mem(self.ip + 2)?];

        for i in 0..params.len() {
            if modes[i] == ParamMode::Position {
                params[i] = self.read_mem(params[i] as usize)?;
            }
        }

        Ok((params[0], params[1]))
    }

    fn read_params1(&mut self, modes: [ParamMode; 3]) -> Result<i64, String> {
        let mut param = self.read_mem(self.ip + 1)?;

        if modes[0] == ParamMode::Position {
            param = self.read_mem(param as usize)?;
        }

        Ok(param)
    }

    fn exec_add(&mut self, modes: [ParamMode; 3]) -> Result<(), String> {
        let (x, y, z) = self.read_params3(modes)?;

        let sum = x + y;
        self.write_mem(z as usize, sum)?;

        self.ip += 4;
        Ok(())
    }

    fn exec_mul(&mut self, modes: [ParamMode; 3]) -> Result<(), String> {
        let (x, y, z) = self.read_params3(modes)?;

        let mul = x * y;
        self.write_mem(z as usize, mul)?;

        self.ip += 4;
        Ok(())
    }

    fn exec_output(
        &mut self,
        output: &mut impl io::Write,
        modes: [ParamMode; 3],
    ) -> Result<(), String> {
        let x = self.read_params1(modes)?;
        writeln!(output, "{}", x).map_err(|x| format!("{}", x))?;

        self.ip += 2;
        Ok(())
    }

    fn exec_input(
        &mut self,
        input: &mut impl io::BufRead,
        modes: [ParamMode; 3],
    ) -> Result<(), String> {
        let mut buffer = String::new();
        input
            .read_to_string(&mut buffer)
            .map_err(|x| format!("{}", x))?;

        let value: i64 = buffer
            .parse()
            .map_err(|_| format!("invalid input: {}", buffer))?;
        let x = self.read_params1(modes)?;

        self.write_mem(x as usize, value)?;

        self.ip += 2;
        Ok(())
    }

    fn exec_jump_not_zero(&mut self, modes: [ParamMode; 3]) -> Result<(), String> {
        let (x, addr) = self.read_params2(modes)?;
        self.ip += 3;

        if x != 0 {
            self.ip = addr as usize;
        }

        Ok(())
    }

    fn exec_jump_zero(&mut self, modes: [ParamMode; 3]) -> Result<(), String> {
        let (x, addr) = self.read_params2(modes)?;
        self.ip += 3;

        if x == 0 {
            self.ip = addr as usize;
        }

        Ok(())
    }

    fn exec_less(&mut self, modes: [ParamMode; 3]) -> Result<(), String> {
        let (x, y, addr) = self.read_params3(modes)?;
        self.write_mem(addr as usize, (x < y) as i64)?;

        self.ip += 4;
        Ok(())
    }

    fn exec_equal(&mut self, modes: [ParamMode; 3]) -> Result<(), String> {
        let (x, y, addr) = self.read_params3(modes)?;
        self.write_mem(addr as usize, (x == y) as i64)?;

        self.ip += 4;
        Ok(())
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(src: &[i64]) -> Result<String, String> {
    let mut output = vec![];
    let mut vm = VM::new(src);
    vm.run(&mut "1".as_bytes(), &mut output)?;

    Ok(String::from_utf8(output).unwrap())
}

#[aoc(day5, part2)]
pub fn solve_part2(src: &[i64]) -> Result<String, String> {
    let mut output = vec![];
    let mut vm = VM::new(src);
    vm.run(&mut "5".as_bytes(), &mut output)?;

    Ok(String::from_utf8(output).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(src: &[i64] /* input: &mut impl io::BufRe, output: &mut impl io::Write */) -> i64 {
        // io::stdin.lock()
        // io::stdout()
        run_with_buffers(src, "", &mut vec![])
    }

    fn run_with_buffers(src: &[i64], input: &str, output: &mut impl io::Write) -> i64 {
        let mut vm = VM::new(src);
        vm.run(&mut input.as_bytes(), output).unwrap()
    }

    #[test]
    fn test_parse_input() {
        let input = "1101,100,-1,4,0";
        assert_eq!(parse_input(input), vec![1101, 100, -1, 4, 0])
    }

    #[test]
    fn test_parse_opcode() {
        assert_eq!(Opcode::try_from(99), Ok(Opcode::Halt));
        assert!(Opcode::try_from(0).is_err());
    }

    #[test]
    fn test_parse_instruction_unmasked() {
        assert_eq!(
            Instruction::try_from(1004),
            Ok(Instruction {
                opcode: Opcode::Output,
                modes: [
                    ParamMode::Position,
                    ParamMode::Immediate,
                    ParamMode::Position
                ]
            })
        );
    }

    #[test]
    fn test_parse_instruction_masked() {
        assert_eq!(
            Instruction::try_from(1001),
            Ok(Instruction {
                opcode: Opcode::Add,
                modes: [
                    ParamMode::Position,
                    ParamMode::Immediate,
                    ParamMode::Immediate,
                ]
            })
        );
    }

    #[test]
    fn test_run_day02() {
        assert_eq!(run(&vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(run(&vec![2, 3, 0, 3, 99]), 2);
        assert_eq!(run(&vec![2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(run(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }

    #[test]
    fn test_run_with_modes() {
        assert_eq!(run(&vec![1101, 100, -1, 0]), 99);
    }

    #[test]
    fn test_output_in_buffer() {
        let mut buffer = vec![];
        run_with_buffers(&vec![4, 0], "", &mut buffer);
        assert_eq!(String::from_utf8(buffer).unwrap(), "4\n");
    }

    #[test]
    fn test_input_in_buffer() {
        let mut buffer = vec![];
        assert_eq!(
            run_with_buffers(&vec![3, 2, 0, 1, 1, 0], "1101", &mut buffer),
            2
        );
    }

    #[test]
    fn test_jumps_with_position_mode() {
        // 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9
        let mut buffer = vec![];
        run_with_buffers(
            &vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            "0",
            &mut buffer,
        );
        assert_eq!(String::from_utf8(buffer).unwrap(), "0\n");

        let mut buffer2 = vec![];
        run_with_buffers(
            &vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            "8",
            &mut buffer2,
        );
        assert_eq!(String::from_utf8(buffer2).unwrap(), "1\n");
    }

    #[test]
    fn test_jumps_with_immediate_mode() {
        let mut buffer = vec![];
        run_with_buffers(
            &vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            "0",
            &mut buffer,
        );
        assert_eq!(String::from_utf8(buffer).unwrap(), "0\n");

        let mut buffer2 = vec![];
        run_with_buffers(
            &vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            "8",
            &mut buffer2,
        );
        assert_eq!(String::from_utf8(buffer2).unwrap(), "1\n");
    }

    #[test]
    fn test_equals_with_position_mode() {
        let mut buffer = vec![];
        run_with_buffers(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], "8", &mut buffer);
        assert_eq!(String::from_utf8(buffer).unwrap(), "1\n");

        let mut buffer2 = vec![];
        run_with_buffers(
            &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            "-8",
            &mut buffer2,
        );
        assert_eq!(String::from_utf8(buffer2).unwrap(), "0\n");
    }

    #[test]
    fn test_less_with_immediate_mode() {
        let mut buffer1 = vec![];
        run_with_buffers(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], "7", &mut buffer1);
        assert_eq!(String::from_utf8(buffer1).unwrap(), "1\n");

        let mut buffer2 = vec![];
        run_with_buffers(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], "8", &mut buffer2);
        assert_eq!(String::from_utf8(buffer2).unwrap(), "0\n");
    }
}
