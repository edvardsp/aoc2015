use ndarray::{s, Array2};
use regex::Regex;
use std::ops::RangeInclusive;

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug)]
enum Op {
    Off,
    On,
    Toggle,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "turn off" => Op::Off,
            "turn on" => Op::On,
            "toggle" => Op::Toggle,
            _ => panic!("Invalid op: {}", value),
        }
    }
}

struct Instr {
    op: Op,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(turn off|turn on|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        let c = re.captures(value).unwrap();
        let op = c[1].into();
        let x_start = c[2].parse().unwrap();
        let y_start = c[3].parse().unwrap();
        let x_stop = c[4].parse().unwrap();
        let y_stop = c[5].parse().unwrap();
        assert!(x_start <= x_stop);
        assert!(y_start <= y_stop);
        let x_range = x_start..=x_stop;
        let y_range = y_start..=y_stop;
        Self {
            op,
            x_range,
            y_range,
        }
    }
}

struct Lights {
    map: Array2<usize>,
}

impl Lights {
    fn new() -> Self {
        let map = Array2::from_elem((1000, 1000), 0);
        Self { map }
    }

    fn apply_v0(&mut self, instr: &Instr) {
        let mut slice = self
            .map
            .slice_mut(s![instr.y_range.clone(), instr.x_range.clone()]);
        match instr.op {
            Op::Off => slice.fill(0),
            Op::On => slice.fill(1),
            Op::Toggle => slice.map_inplace(|b| *b ^= 1),
        }
    }

    fn apply_v1(&mut self, instr: &Instr) {
        let mut slice = self
            .map
            .slice_mut(s![instr.y_range.clone(), instr.x_range.clone()]);
        match instr.op {
            Op::Off => slice.map_inplace(|b| *b = b.saturating_sub(1)),
            Op::On => slice.map_inplace(|b| *b += 1),
            Op::Toggle => slice.map_inplace(|b| *b += 2),
        }
    }

    fn len(&self) -> usize {
        self.map.iter().filter(|val| **val > 0).count()
    }

    fn brightness(&self) -> usize {
        self.map.iter().sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lights = Lights::new();
    for instr in input.lines().map(Instr::from) {
        lights.apply_v0(&instr);
    }
    Some(lights.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lights = Lights::new();
    for instr in input.lines().map(Instr::from) {
        lights.apply_v1(&instr);
    }
    Some(lights.brightness())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(998996));
        assert_eq!(part_one("turn on 0,0 through 999,999"), Some(1000 * 1000));
        assert_eq!(part_one("toggle 0,0 through 999,0"), Some(1000));
        assert_eq!(part_one("turn off 499,499 through 500,500"), Some(0));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("turn on 0,0 through 0,0"), Some(1));
        assert_eq!(part_two("toggle 0,0 through 999,999"), Some(2000000));
    }
}
