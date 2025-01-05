advent_of_code::solution!(10);

fn groups(input: &[u8], output: &mut Vec<u8>) {
    output.clear();
    let mut current = input[0];
    let mut total = 0;
    for n in input {
        if *n == current {
            total += 1;
        } else {
            output.push(total);
            output.push(current);
            current = *n;
            total = 1;
        }
    }
    output.push(total);
    output.push(current);
}

fn lookandsay(string: &str, depth: usize) -> usize {
    let mut input: Vec<_> = string.as_bytes().iter().map(|b| b - b'0').collect();
    let mut output = Vec::new();
    for _ in 0..=depth {
        groups(&input, &mut output);
        std::mem::swap(&mut input, &mut output);
    }
    output.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(lookandsay(input, 40))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(lookandsay(input, 50))
}
