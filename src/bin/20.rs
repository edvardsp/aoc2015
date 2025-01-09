advent_of_code::solution!(20);

fn find_house(lower_bound: usize) -> Option<usize> {
    let max_house = lower_bound / 10;
    let mut houses = vec![0; max_house + 1];
    for elf in 1..=max_house {
        for house in (elf..=max_house).step_by(elf) {
            houses[house] += (elf * 10) as u32;
        }
        if houses[elf] as usize >= lower_bound {
            return Some(elf);
        }
    }
    None
}

fn find_house_lazy(lower_bound: usize) -> Option<usize> {
    let max_house = lower_bound / 10;
    let mut houses = vec![0; max_house + 1];
    for elf in 1..=max_house {
        for house in (elf..=max_house).step_by(elf).take(50) {
            houses[house] += (elf * 11) as u32;
        }
        if houses[elf] as usize >= lower_bound {
            return Some(elf);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let lower_bound = input.parse().unwrap();
    find_house(lower_bound)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lower_bound = input.parse().unwrap();
    find_house_lazy(lower_bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(Some(1), find_house(10));
        assert_eq!(Some(2), find_house(30));
        assert_eq!(Some(3), find_house(40));
        assert_eq!(Some(4), find_house(60));
        assert_eq!(Some(4), find_house(70));
        assert_eq!(Some(6), find_house(80));
        assert_eq!(Some(6), find_house(120));
        assert_eq!(Some(8), find_house(130));
        assert_eq!(Some(8), find_house(150));
    }
}
