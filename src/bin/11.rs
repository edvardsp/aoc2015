use std::fmt::Write;

use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Password([u8; 8]);

impl Password {
    fn illegal_pos(&self) -> Option<usize> {
        self.0.iter().enumerate().find_map(|(i, b)| {
            if matches!(b, b'i' | b'o' | b'l') {
                Some(i)
            } else {
                None
            }
        })
    }

    fn has_three_letter_seq(&self) -> bool {
        self.0
            .windows(3)
            .any(|seq| seq[0] + 1 == seq[1] && seq[1] + 1 == seq[2])
    }

    fn has_two_pairs(&self) -> bool {
        self.0
            .windows(2)
            .filter(|seq| seq[0] == seq[1])
            .unique()
            .count()
            > 1
    }

    fn is_valid(&self) -> bool {
        self.has_three_letter_seq() && self.illegal_pos().is_none() && self.has_two_pairs()
    }

    fn increment_with_skip(&mut self) {
        if let Some(n) = self.illegal_pos() {
            self.0[n] += 1;
            self.0[n + 1..].fill(b'a');
        } else {
            for c in self.0.iter_mut().rev() {
                if *c == b'z' {
                    *c = b'a';
                } else {
                    *c += 1;
                    break;
                }
            }
        }
    }
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), 8);
        let bytes = value.as_bytes();
        let inner = std::array::from_fn(|i| bytes[i]);
        Self(inner)
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            f.write_char(*b as char)?;
        }
        Ok(())
    }
}

fn next_password(input: &Password) -> Password {
    let mut pw = *input;
    loop {
        pw.increment_with_skip();
        if pw.is_valid() {
            break;
        }
    }
    pw
}

pub fn part_one(input: &str) -> Option<Password> {
    Some(next_password(&input.into()))
}

pub fn part_two(input: &str) -> Option<Password> {
    Some(next_password(&next_password(&input.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = next_password(&"abcdefgh".into());
        assert_eq!(result, Password::from("abcdffaa"));
    }
}
