advent_of_code::solution!(8);

fn from_bytes_radix(bytes: &[u8]) -> u8 {
    u8::from_str_radix(std::str::from_utf8(bytes).unwrap(), 16).unwrap()
}

fn unescape(string: &[u8]) -> Vec<u8> {
    // Remove " prefix and suffix
    let mut string = &string[1..string.len() - 1];

    let mut output = Vec::new();
    while let Some(pos) = string.iter().position(|b| *b == b'\\') {
        let (head, tail) = string.split_at(pos);
        output.extend_from_slice(head);
        string = match &tail[0..2] {
            b"\\\\" => {
                output.push(b'\\');
                &tail[2..]
            }
            b"\\\"" => {
                output.push(b'\"');
                &tail[2..]
            }
            b"\\x" => {
                output.push(from_bytes_radix(&tail[2..4]));
                &tail[4..]
            }
            esc => panic!("Invalid escape character: {:?}", esc),
        };
    }

    output.extend_from_slice(string);
    output
}

fn calculate_decode(input: &str) -> usize {
    let bytes = input.as_bytes();
    bytes.len() - unescape(bytes).len()
}

fn calculate_encode(input: &str) -> usize {
    // +2 for the " prefix and suffix
    input.escape_default().count() - input.len() + 2
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(calculate_decode).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().map(calculate_encode).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
