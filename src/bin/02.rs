advent_of_code::solution!(2);

struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    fn wrapping_paper(&self) -> usize {
        let slack = (self.l * self.w).min(self.l * self.h).min(self.w * self.h);
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l + slack
    }

    fn ribbon(&self) -> usize {
        let max_side = self.l.max(self.w).max(self.h);
        let bow = self.l * self.w * self.h;
        2 * self.l + 2 * self.w + 2 * self.h - 2 * max_side + bow
    }
}

impl From<&str> for Present {
    fn from(value: &str) -> Self {
        let mut dim_iter = value.splitn(3, "x").map(|n| n.parse().unwrap());
        let l = dim_iter.next().unwrap();
        let w = dim_iter.next().unwrap();
        let h = dim_iter.next().unwrap();
        Self { l, w, h }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let ans = input
        .lines()
        .map(Present::from)
        .map(|p| p.wrapping_paper())
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ans = input.lines().map(Present::from).map(|p| p.ribbon()).sum();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2x3x4"), Some(58));
        assert_eq!(part_one("1x1x10"), Some(43));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2x3x4"), Some(34));
        assert_eq!(part_two("1x1x10"), Some(14));
    }
}
