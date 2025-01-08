#![allow(clippy::reversed_empty_ranges)]

use ndarray::{s, Array2};

advent_of_code::solution!(18);

const ON: char = '#';
const OFF: char = '.';
const PAD: char = ' ';

struct Map {
    tiles: Array2<char>,
    fixed_corners: bool,
}

impl Map {
    fn corners(&self) -> [(usize, usize); 4] {
        let (rows, cols) = self.tiles.dim();
        [(1, 1), (1, cols - 2), (rows - 2, 1), (rows - 2, cols - 2)]
    }

    fn set_corners(&mut self) {
        for corner in self.corners() {
            self.tiles[corner] = ON;
        }
        self.fixed_corners = true;
    }

    fn tick(&mut self) {
        let prev = self.tiles.clone();
        let corners = self.corners();
        for ((y, x), ch) in self.tiles.indexed_iter_mut() {
            if *ch == PAD {
                continue;
            }
            if self.fixed_corners && corners.contains(&(y, x)) {
                continue;
            }

            let counted_on = prev
                .slice(s![y - 1..=y + 1, x - 1..=x + 1])
                .iter()
                .filter(|ch| **ch == ON)
                .count();

            match (*ch, counted_on) {
                (ON, 3 | 4) => {}
                (ON, _) => *ch = OFF,
                (OFF, 3) => *ch = ON,
                (OFF, _) => {}
                (_, _) => {}
            }
        }
    }

    fn lights(&self) -> usize {
        self.tiles.iter().filter(|c| **c == ON).count()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let chars: Vec<_> = value.chars().filter(|c| c.is_ascii_graphic()).collect();
        let height = value.lines().count();
        let width = chars.len() / height;
        let shape = (height, width);
        let inner = Array2::from_shape_vec(shape, chars).unwrap();
        let mut tiles = Array2::from_elem((height + 2, width + 2), PAD);
        tiles.slice_mut(s![1..-1, 1..-1]).assign(&inner);
        Self {
            tiles,
            fixed_corners: false,
        }
    }
}

fn simulate(input: &str, steps: usize, corners: bool) -> usize {
    let mut map: Map = input.into();
    if corners {
        map.set_corners();
    }
    for _ in 0..steps {
        map.tick();
    }
    map.lights()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(simulate(input, 100, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(simulate(input, 100, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = simulate(
            &advent_of_code::template::read_file("examples", DAY),
            4,
            false,
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two() {
        let result = simulate(
            &advent_of_code::template::read_file("examples", DAY),
            5,
            true,
        );
        assert_eq!(result, 17);
    }
}
