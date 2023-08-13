use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

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
}

impl TryFrom<i64> for Opcode {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            99 => Ok(Self::Halt),
            _ => Err(format!("unsupported opcode {}", value)),
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
            modes[i] = ParamMode::try_from(raw_mode)?;
        }

        Ok(Self { opcode, modes })
    }
}

impl Instruction {
    pub fn modes2(&self) -> [ParamMode; 3] {
        [self.modes[0], self.modes[1], ParamMode::Immediate]
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

    pub fn run(&mut self) -> Result<i64, String> {
        while let Some(raw_instruction) = self.memory.get(self.ip) {
            let instruction = Instruction::try_from(*raw_instruction)?;
            match instruction.opcode {
                Opcode::Add => self.exec_add(instruction.modes2())?,
                Opcode::Mul => self.exec_mul(instruction.modes2())?,
                Opcode::Halt => break,
                _ => todo!("unimplemented"),
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

        for i in 0..modes.len() {
            if modes[i] == ParamMode::Position {
                params[i] = self.read_mem(params[i] as usize)?;
            }
        }

        Ok((params[0], params[1], params[2]))
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
}

fn run(src: &[i64]) -> i64 {
    let mut vm = VM::new(src);
    vm.run().unwrap()
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_parse_instruction() {
        assert_eq!(
            Instruction::try_from(1002),
            Ok(Instruction {
                opcode: Opcode::Mul,
                modes: [
                    ParamMode::Position,
                    ParamMode::Immediate,
                    ParamMode::Position
                ]
            })
        );
    }

    #[test]
    fn test_run_backward_compatibility() {
        assert_eq!(run(&vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(run(&vec![2, 3, 0, 3, 99]), 2);
        assert_eq!(run(&vec![2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(run(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }

    #[test]
    fn test_run_with_modes() {
        assert_eq!(run(&vec![1101, 100, -1, 0]), 99);
    }
}
