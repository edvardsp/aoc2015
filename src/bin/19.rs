use std::collections::{HashMap, HashSet};

use regex::Regex;

advent_of_code::solution!(19);

struct Replacements {
    lut: HashMap<String, Vec<String>>,
}

impl From<&str> for Replacements {
    fn from(value: &str) -> Self {
        let mut lut: HashMap<String, Vec<String>> = HashMap::new();
        for line in value.lines() {
            let (from, to) = line.split_once(" => ").unwrap();
            lut.entry(from.to_string())
                .or_default()
                .push(to.to_string());
        }
        Self { lut }
    }
}

struct Input {
    replacements: Replacements,
    target: String,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (replacements_str, target) = value.split_once("\n\n").unwrap();
        let replacements: Replacements = replacements_str.into();
        let target = target.to_string();
        Self {
            replacements,
            target,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let Input {
        replacements,
        target,
    } = input.into();

    let mut molecules = HashSet::new();
    for (from, to) in &replacements.lut {
        for (pos, m) in target.match_indices(from) {
            for token in to {
                let mut molecule = target.clone();
                molecule.replace_range(pos..pos + m.len(), token);
                molecules.insert(molecule);
            }
        }
    }

    Some(molecules.len())
}

fn sequence(input: &str) -> Vec<String> {
    let boundary = Regex::new(r"([a-z]|[A-Z])([A-Z])").unwrap();
    let mut current = input.to_string();
    loop {
        let next = boundary.replace(&current, "$1 $2");
        if current.len() == next.len() {
            break;
        }
        current = next.to_string();
    }
    current.split_whitespace().map(|s| s.to_owned()).collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, target) = input.split_once("\n\n").unwrap();

    // From https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    let molecules = sequence(target);
    let rn_or_ar = molecules
        .iter()
        .filter(|m| matches!(m.as_str(), "Rn" | "Ar"))
        .count();
    let y = molecules.iter().filter(|&m| m == "Y").count();
    let steps = molecules.len() - rn_or_ar - 2 * y - 1;
    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
