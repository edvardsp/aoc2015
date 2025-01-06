use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(13);

struct Seating {
    map: HashMap<String, HashMap<String, isize>>,
}

impl Seating {
    fn include_myself(&mut self) {
        let myself = "Antagonist".to_string();
        let others: Vec<_> = self.map.keys().cloned().collect();
        for other in others {
            self.map.get_mut(&other).unwrap().insert(myself.clone(), 0);
            self.map.entry(myself.clone()).or_default().insert(other, 0);
        }
    }

    fn score(&self, order: &[&String]) -> isize {
        let n = order.len();
        (0..n)
            .map(|i| {
                let p0 = order[i];
                let p1 = order[(i + 1) % n];
                self.map[p0][p1] + self.map[p1][p0]
            })
            .sum()
    }

    fn optimal(&self) -> isize {
        self.map
            .keys()
            .permutations(self.map.keys().len())
            .par_bridge()
            .map(|order| self.score(&order))
            .max()
            .unwrap()
    }
}

impl From<&str> for Seating {
    fn from(value: &str) -> Self {
        let mut map: HashMap<String, HashMap<String, isize>> = HashMap::new();
        for line in value.lines() {
            let tokens: Vec<_> = line.split_whitespace().collect();
            let main = tokens[0].to_string();
            let modifier = match tokens[2] {
                "gain" => 1,
                "lose" => -1,
                tok => unreachable!("invalid modified: {}", tok),
            };
            let points: isize = tokens[3].parse().unwrap();
            let neighbor = tokens[10].strip_suffix(".").unwrap().to_string();
            map.entry(main)
                .or_default()
                .insert(neighbor, modifier * points);
        }
        Self { map }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let seating: Seating = input.into();
    Some(seating.optimal())
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut seating: Seating = input.into();
    seating.include_myself();
    Some(seating.optimal())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }
}
