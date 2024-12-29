advent_of_code::solution!(5);

fn is_vowel(ch: &u8) -> bool {
    matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u')
}

fn is_naughy_pair(token: &[u8]) -> bool {
    matches!(token, b"ab" | b"cd" | b"pq" | b"xy")
}

fn is_nice_v0(string: &str) -> bool {
    let bytes = string.as_bytes();

    let enough_vowels = bytes.iter().copied().filter(is_vowel).count() >= 3;
    let twice_in_row = bytes.windows(2).any(|window| window[1] == window[0]);
    let naughty_pair = bytes.windows(2).any(is_naughy_pair);

    enough_vowels && twice_in_row && !naughty_pair
}

fn is_nice_v1(string: &str) -> bool {
    let bytes = string.as_bytes();

    let repeated_pair = bytes
        .windows(2)
        .enumerate()
        .any(|(offset, lhs)| bytes[offset + 2..].windows(2).any(|rhs| lhs == rhs));
    let repeated_letter = bytes.windows(3).any(|window| window[2] == window[0]);

    repeated_pair && repeated_letter
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter(|word| is_nice_v0(word)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter(|word| is_nice_v1(word)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(is_nice_v0("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_v0("aaa"), true);
        assert_eq!(is_nice_v0("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_v0("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_v0("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(is_nice_v1("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_v1("xxyxx"), true);
        assert_eq!(is_nice_v1("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_v1("ieodomkazucvgmuy"), false);
    }
}
