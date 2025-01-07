use itertools::Itertools;

advent_of_code::solution!(16);

fn maybe_eq<T: PartialEq>(lhs: &Option<T>, rhs: &Option<T>) -> bool {
    match (lhs, rhs) {
        (Some(x), Some(y)) => x == y,
        (_, _) => true,
    }
}

fn maybe_lt<T: PartialOrd>(lhs: &Option<T>, rhs: &Option<T>) -> bool {
    match (lhs, rhs) {
        (Some(x), Some(y)) => x < y,
        (_, _) => true,
    }
}

#[derive(Default)]
struct AuntSue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl AuntSue {
    fn eq(&self, other: &Self) -> bool {
        maybe_eq(&self.children, &other.children)
            && maybe_eq(&self.cats, &other.cats)
            && maybe_eq(&self.samoyeds, &other.samoyeds)
            && maybe_eq(&self.pomeranians, &other.pomeranians)
            && maybe_eq(&self.akitas, &other.akitas)
            && maybe_eq(&self.vizslas, &other.vizslas)
            && maybe_eq(&self.goldfish, &other.goldfish)
            && maybe_eq(&self.trees, &other.trees)
            && maybe_eq(&self.cars, &other.cars)
            && maybe_eq(&self.perfumes, &other.perfumes)
    }

    fn kinda_eq(&self, other: &Self) -> bool {
        maybe_eq(&self.children, &other.children)
            && maybe_lt(&other.cats, &self.cats)
            && maybe_eq(&self.samoyeds, &other.samoyeds)
            && maybe_lt(&self.pomeranians, &other.pomeranians)
            && maybe_eq(&self.akitas, &other.akitas)
            && maybe_eq(&self.vizslas, &other.vizslas)
            && maybe_lt(&self.goldfish, &other.goldfish)
            && maybe_lt(&other.trees, &self.trees)
            && maybe_eq(&self.cars, &other.cars)
            && maybe_eq(&self.perfumes, &other.perfumes)
    }
}

impl From<&str> for AuntSue {
    fn from(value: &str) -> Self {
        let mut sue = Self::default();
        let (_, suffix) = value.split_once(": ").unwrap();
        for token in suffix.split(", ") {
            let (compound, value_str) = token.split_once(": ").unwrap();
            let value = value_str.parse().unwrap();
            match compound {
                "children" => sue.children = Some(value),
                "cats" => sue.cats = Some(value),
                "samoyeds" => sue.samoyeds = Some(value),
                "pomeranians" => sue.pomeranians = Some(value),
                "akitas" => sue.akitas = Some(value),
                "vizslas" => sue.vizslas = Some(value),
                "goldfish" => sue.goldfish = Some(value),
                "trees" => sue.trees = Some(value),
                "cars" => sue.cars = Some(value),
                "perfumes" => sue.perfumes = Some(value),
                _ => panic!("Invalid compound: {}", compound),
            }
        }
        sue
    }
}

const NEEDLE: AuntSue = AuntSue {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(AuntSue::from)
        .find_position(|sue| sue.eq(&NEEDLE))
        .map(|(pos, _)| pos)
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .lines()
        .map(AuntSue::from)
        .find_position(|sue| sue.kinda_eq(&NEEDLE))
        .map(|(pos, _)| pos)
}
