use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(17);

fn combine(leftovers: &[i32], target: i32, count: usize, tally: &mut HashMap<usize, usize>) {
    match target.cmp(&0) {
        Ordering::Equal => {
            tally.entry(count).and_modify(|e| *e += 1).or_insert(1);
            return;
        }
        Ordering::Less => return,
        Ordering::Greater => {}
    }

    if let Some((head, tail)) = leftovers.split_first() {
        combine(tail, target, count, tally);
        combine(tail, target - *head, count + 1, tally);
    }
}

fn all_combinations(input: &str, target: i32) -> HashMap<usize, usize> {
    let containers: Vec<_> = input.lines().map(|n| n.parse().unwrap()).collect();
    let mut tally = HashMap::new();
    combine(&containers, target, 0, &mut tally);
    tally
}

fn count_combinations(input: &str, target: i32) -> usize {
    let tally = all_combinations(input, target);
    tally.values().sum()
}

fn count_min_combinations(input: &str, target: i32) -> usize {
    let tally = all_combinations(input, target);
    let min_combination = tally.keys().min().unwrap();
    tally[min_combination]
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(count_combinations(input, 150))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(count_min_combinations(input, 150))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = count_combinations(&advent_of_code::template::read_file("examples", DAY), 25);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two() {
        let result =
            count_min_combinations(&advent_of_code::template::read_file("examples", DAY), 25);
        assert_eq!(result, 3);
    }
}
