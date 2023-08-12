use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> (u64, u64) {
    let nums: Vec<u64> = input.split("-").map(|x| x.parse().unwrap()).collect();
    (nums[0], nums[1])
}

struct PasswordGen {
    curr: u64,
    end: u64,
}

impl Iterator for PasswordGen {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.curr <= self.end {
            Some(to_digits(self.curr))
        } else {
            None
        };

        self.curr += 1;

        res
    }
}

fn to_digits(x: u64) -> Vec<u64> {
    x.to_string()
        .chars()
        .map(|d| d.to_string().parse().unwrap())
        .collect()
}

fn password_gen(start: u64, end: u64) -> PasswordGen {
    PasswordGen { curr: start, end }
}

fn is_valid_password(password: &[u64]) -> bool {
    let has_valid_length = password.len() == 6;
    let has_double = password.windows(2).any(|pair| pair[0] == pair[1]);
    let is_not_decreasing = password.windows(2).all(|pair| pair[0] <= pair[1]);

    has_double && is_not_decreasing && has_valid_length
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(u64, u64)) -> u64 {
    let (start, end) = *input;

    password_gen(start, end)
        .filter(|x| is_valid_password(x))
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password() {
        assert_eq!(is_valid_password(&[1, 1, 1, 1, 1, 1]), true);
        assert_eq!(is_valid_password(&[2, 2, 3, 4, 5, 0]), false);
        assert_eq!(is_valid_password(&[1, 2, 3, 7, 8, 9]), false);
    }
}
