use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    input.into_iter().fold(0, |total, mass| total + fuel(*mass))
}

fn fuel2(mass: u64) -> u64 {
    let res = fuel(mass);
    if res <= 0 {
        res
    } else {
        res + fuel2(res)
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    input
        .into_iter()
        .fold(0, |total, mass| total + fuel2(*mass))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_simple() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
    }

    #[test]
    fn test_fuel_bigger_amounts() {
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn test_fuel2() {
        assert_eq!(fuel2(1969), 966);
        assert_eq!(fuel2(100756), 50346);
    }
}
