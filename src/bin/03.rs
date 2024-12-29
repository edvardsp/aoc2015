use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl From<u8> for Dir {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Dir::North,
            b'>' => Dir::East,
            b'v' => Dir::South,
            b'<' => Dir::West,
            _ => unreachable!("Invalid character: {}", value as char),
        }
    }
}

fn dir_iter(value: &str) -> impl Iterator<Item = Dir> + '_ {
    value.as_bytes().iter().copied().map(Dir::from)
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

impl Coord {
    fn step(&self, dir: Dir) -> Self {
        let Self(x, y) = *self;
        match dir {
            Dir::North => Self(x, y - 1),
            Dir::East => Self(x + 1, y),
            Dir::South => Self(x, y + 1),
            Dir::West => Self(x - 1, y),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut visited = HashSet::new();
    visited.insert(Coord::default());
    let mut santa = Coord::default();
    for dir in dir_iter(input) {
        santa = santa.step(dir);
        visited.insert(santa);
    }
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut visited = HashSet::new();
    visited.insert(Coord::default());
    let mut santa = Coord::default();
    let mut robosanta = Coord::default();
    for (i, dir) in dir_iter(input).enumerate() {
        if i % 2 == 0 {
            santa = santa.step(dir);
            visited.insert(santa);
        } else {
            robosanta = robosanta.step(dir);
            visited.insert(robosanta);
        }
    }
    Some(visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(">"), Some(2));
        assert_eq!(part_one("^>v<"), Some(4));
        assert_eq!(part_one("^v^v^v^v^v"), Some(2));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), Some(3));
        assert_eq!(part_two("^>v<"), Some(3));
        assert_eq!(part_two("^v^v^v^v^v"), Some(11));
    }
}
