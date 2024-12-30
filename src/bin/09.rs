use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(9);

struct Map<'i> {
    edges: HashMap<&'i str, HashMap<&'i str, usize>>,
}

impl<'i> Map<'i> {
    fn score(&self, order: &[&&'i str]) -> usize {
        order.windows(2).fold(0, |acc, edge| {
            let from = edge[0];
            let to = edge[1];
            let value = self.edges[from][to];
            acc + value
        })
    }

    fn all_paths(&self) -> impl ParallelIterator<Item = usize> + '_ {
        self.edges
            .keys()
            .permutations(self.edges.len())
            .par_bridge()
            .map(|order| self.score(&order))
    }

    fn shortest_path(&self) -> usize {
        self.all_paths().min().unwrap()
    }

    fn longest_path(&self) -> usize {
        self.all_paths().max().unwrap()
    }
}

impl<'i> From<&'i str> for Map<'i> {
    fn from(value: &'i str) -> Self {
        let mut edges: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
        for line in value.lines() {
            let (from, tail) = line.split_once(" to ").unwrap();
            let (to, value_str) = tail.split_once(" = ").unwrap();
            let value: usize = value_str.parse().unwrap();
            edges.entry(from).or_default().insert(to, value);
            edges.entry(to).or_default().insert(from, value);
        }
        Self { edges }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.shortest_path())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input.into();
    Some(map.longest_path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
