use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Opcode {
    Add,
    Mul,
    Halt,
}

impl TryFrom<i64> for Opcode {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::Halt),
            _ => Err(format!("unsupported opcode {}", value)),
        }
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
        while let Some(raw_opcode) = self.memory.get(self.ip) {
            let opcode = Opcode::try_from(*raw_opcode)?;
            match opcode {
                Opcode::Add => self.exec_add()?,
                Opcode::Mul => self.exec_mul()?,
                Opcode::Halt => break,
                // _ => todo!("unimplemented"),
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

    fn read_params3(&mut self) -> Result<(i64, i64, i64), String> {
        let x = self.read_mem(self.ip + 1)?;
        let y = self.read_mem(self.ip + 2)?;
        let z = self.read_mem(self.ip + 3)?;

        Ok((x, y, z))
    }

    fn exec_add(&mut self) -> Result<(), String> {
        let (ix, iy, iz) = self.read_params3()?;

        let sum = self.read_mem(ix as usize)? + self.read_mem(iy as usize)?;
        self.write_mem(iz as usize, sum)?;

        self.ip += 4;
        Ok(())
    }

    fn exec_mul(&mut self) -> Result<(), String> {
        let (ix, iy, iz) = self.read_params3()?;

        let mul = self.read_mem(ix as usize)? * self.read_mem(iy as usize)?;
        self.write_mem(iz as usize, mul)?;

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
    fn test_run_simple() {
        assert_eq!(run(&vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(run(&vec![2, 3, 0, 3, 99]), 2);
        assert_eq!(run(&vec![2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(run(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }
}
