use rayon::prelude::*;

advent_of_code::solution!(4);

fn hash(n: usize, seed: &str) -> String {
    let input = format!("{}{}", seed, n);
    let digest = md5::compute(input);
    format!("{:X}", digest)
}

fn mine(seed: &str, prefix: &str) -> Option<usize> {
    (1..usize::MAX)
        .into_par_iter()
        .by_uniform_blocks(1_000_000)
        .find_first(|n| hash(*n, seed).starts_with(prefix))
}

pub fn part_one(input: &str) -> Option<usize> {
    mine(input, "00000")
}

pub fn part_two(input: &str) -> Option<usize> {
    mine(input, "000000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("abcdef"), Some(609043));
        assert_eq!(part_one("pqrstuv"), Some(1048970));
    }
}
