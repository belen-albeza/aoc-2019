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

struct RepeatingChunker {
    i: usize,
    digits: Vec<u64>,
}

impl Iterator for RepeatingChunker {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.digits.len() {
            return None;
        }

        let value = self.digits[self.i];

        let repeating: Vec<u64> = self.digits[self.i..]
            .iter()
            .take_while(|x| **x == value)
            .map(|x| *x)
            .collect();

        self.i += repeating.len();
        Some(repeating)
    }
}

fn repeating_chunks(digits: &[u64]) -> RepeatingChunker {
    RepeatingChunker {
        i: 0,
        digits: digits.to_owned(),
    }
}

fn is_valid_password(password: &[u64]) -> bool {
    let has_valid_length = password.len() == 6;
    let has_double = password.windows(2).any(|pair| pair[0] == pair[1]);
    let is_not_decreasing = password.windows(2).all(|pair| pair[0] <= pair[1]);

    has_double && is_not_decreasing && has_valid_length
}

fn is_valid_password_v2(password: &[u64]) -> bool {
    let has_valid_length = password.len() == 6;
    let is_not_decreasing = password.windows(2).all(|pair| pair[0] <= pair[1]);

    let has_isolated_double = repeating_chunks(password).any(|x| x.len() == 2);

    has_valid_length && is_not_decreasing && has_isolated_double
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(u64, u64)) -> u64 {
    let (start, end) = *input;

    password_gen(start, end)
        .filter(|x| is_valid_password(x))
        .count() as u64
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(u64, u64)) -> u64 {
    let (start, end) = *input;

    password_gen(start, end)
        .filter(|x| is_valid_password_v2(x))
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

    #[test]
    fn test_is_valid_password_v2() {
        assert_eq!(is_valid_password_v2(&[1, 1, 2, 2, 3, 3]), true);
        assert_eq!(is_valid_password_v2(&[1, 2, 3, 4, 4, 4]), false);
        assert_eq!(is_valid_password_v2(&[1, 1, 1, 1, 2, 2]), true);
        assert_eq!(is_valid_password_v2(&[1, 1, 1, 1, 1, 1]), false);
        assert_eq!(is_valid_password_v2(&[4, 4, 5, 6, 7, 8]), true);
        assert_eq!(is_valid_password_v2(&[4, 5, 6, 8, 8, 9]), true);
        assert_eq!(is_valid_password_v2(&[4, 4, 5, 6, 7, 8]), true);
        assert_eq!(is_valid_password_v2(&[4, 5, 5, 6, 7, 8]), true);
        assert_eq!(is_valid_password_v2(&[8, 8, 8, 9, 9, 9]), false);
    }
}
