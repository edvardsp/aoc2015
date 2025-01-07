advent_of_code::solution!(14);

struct Reindeer {
    speed_at: u32,
    speed_for: u32,
    rest_for: u32,
}

impl Reindeer {
    fn travel(&self, duration: u32) -> u32 {
        let period = self.speed_for + self.rest_for;
        let interval = self.speed_at * self.speed_for;
        (duration / period) * interval + (duration % period).min(self.speed_for) * self.speed_at
    }
}

impl From<&str> for Reindeer {
    fn from(value: &str) -> Self {
        let tokens: Vec<_> = value.split_whitespace().collect();
        let speed_at = tokens[3].parse().unwrap();
        let speed_for = tokens[6].parse().unwrap();
        let rest_for = tokens[13].parse().unwrap();
        Self {
            speed_at,
            speed_for,
            rest_for,
        }
    }
}

fn race(input: &str, duration: u32) -> Option<u32> {
    input
        .lines()
        .map(Reindeer::from)
        .map(|r| r.travel(duration))
        .max()
}

fn points(input: &str, duration: u32) -> Option<u32> {
    let reindeer: Vec<_> = input.lines().map(Reindeer::from).collect();
    let mut tally = vec![0; reindeer.len()];
    for tick in 1..=duration {
        let distances: Vec<_> = reindeer.iter().map(|r| r.travel(tick)).collect();
        let leader = *distances.iter().max().unwrap();
        for (i, dist) in distances.into_iter().enumerate() {
            if dist == leader {
                tally[i] += 1;
            }
        }
    }
    tally.into_iter().max()
}

pub fn part_one(input: &str) -> Option<u32> {
    race(input, 2503)
}

pub fn part_two(input: &str) -> Option<u32> {
    points(input, 2503)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = race(&advent_of_code::template::read_file("examples", DAY), 1000);
        assert_eq!(result, Some(1120));
    }

    #[test]
    fn test_part_two() {
        let result = points(&advent_of_code::template::read_file("examples", DAY), 1000);
        assert_eq!(result, Some(689));
    }
}
