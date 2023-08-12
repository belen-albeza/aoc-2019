use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn run(src: &[u64]) -> u64 {
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

    res[0]
}

fn run_with_input(input: &[u64], noun: u64, verb: u64) -> u64 {
    let mut src = input.to_owned();
    src[1] = noun;
    src[2] = verb;

    run(&src)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    run_with_input(input, 12, 2)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let output = run_with_input(input, noun, verb);
            if output == 19690720 {
                return noun * 100 + verb;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_example() {
        assert_eq!(run(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]), 3500)
    }

    #[test]
    fn test_run_simple() {
        assert_eq!(run(&vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(run(&vec![2, 3, 0, 3, 99]), 2);
        assert_eq!(run(&vec![2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(run(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }
}
