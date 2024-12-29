use std::ops::ControlFlow;

advent_of_code::solution!(1);

fn floor_iter(input: &str) -> impl Iterator<Item = isize> + '_ {
    input.as_bytes().iter().map(|ch| match ch {
        b'(' => 1,
        b')' => -1,
        _ => unreachable!("Invalid character: {}", ch),
    })
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(floor_iter(input).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    floor_iter(input)
        .enumerate()
        .try_fold(0, |floor, (pos, dir)| match floor + dir {
            -1 => ControlFlow::Break(pos + 1),
            next => ControlFlow::Continue(next),
        })
        .break_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), Some(0));
        assert_eq!(part_one("()()"), Some(0));
        assert_eq!(part_one("((("), Some(3));
        assert_eq!(part_one("(()(()("), Some(3));
        assert_eq!(part_one("))((((("), Some(3));
        assert_eq!(part_one("())"), Some(-1));
        assert_eq!(part_one("))("), Some(-1));
        assert_eq!(part_one(")))"), Some(-3));
        assert_eq!(part_one(")())())"), Some(-3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), Some(1));
        assert_eq!(part_two("()())"), Some(5));
    }
}
