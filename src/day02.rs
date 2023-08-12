use std::u64;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn run(src: &[u64]) -> Vec<u64> {
    let mut res = src.to_owned();
    let mut pc: usize = 0;

    while pc < res.len() {
        match res[pc] {
            99 => break,
            1 => {
                let ix = res[pc + 1] as usize;
                let iy = res[pc + 2] as usize;
                let target = res[pc + 3] as usize;
                res[target] = res[ix] + res[iy];
                pc += 4;
            }
            2 => {
                let ix = res[pc + 1] as usize;
                let iy = res[pc + 2] as usize;
                let target = res[pc + 3] as usize;
                res[target] = res[ix] * res[iy];
                pc += 4;
            }
            _ => unreachable!("unknown opcode"),
        }
    }

    res
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    let mut src = input.to_owned();
    src[1] = 12;
    src[2] = 2;

    let final_src = run(&src);
    final_src[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_example() {
        assert_eq!(
            run(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        )
    }

    #[test]
    fn test_run_simple() {
        assert_eq!(run(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(run(&vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            run(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
