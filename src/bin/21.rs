use itertools::{iproduct, Itertools};

advent_of_code::solution!(21);

const PLAYER_HITPOINTS: isize = 100;

const SHOP_STR: &str = r#"
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage+1     25     1       0
Damage+2     50     2       0
Damage+3    100     3       0
Defense+1    20     0       1
Defense+2    40     0       2
Defense+3    80     0       3
"#;

struct Stats {
    hitpoints: isize,
    damage: isize,
    armor: isize,
}

impl Stats {
    fn new(hitpoints: isize, items: &[&Item]) -> Self {
        let damage = items.iter().map(|i| i.damage).sum();
        let armor = items.iter().map(|i| i.armor).sum();
        Self {
            hitpoints,
            damage,
            armor,
        }
    }

    fn fight(&self, other: &Stats) -> bool {
        let self_damage = (self.damage - other.armor).max(1);
        let other_damage = (other.damage - self.armor).max(1);
        let self_turns = other.hitpoints / self_damage + (other.hitpoints % self_damage).min(1);
        let other_turns = self.hitpoints / other_damage + (self.hitpoints % other_damage).min(1);
        self_turns <= other_turns
    }
}

impl From<&str> for Stats {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let hitpoints = lines
            .next()
            .unwrap()
            .strip_prefix("Hit Points: ")
            .unwrap()
            .parse()
            .unwrap();
        let damage = lines
            .next()
            .unwrap()
            .strip_prefix("Damage: ")
            .unwrap()
            .parse()
            .unwrap();
        let armor = lines
            .next()
            .unwrap()
            .strip_prefix("Armor: ")
            .unwrap()
            .parse()
            .unwrap();
        Self {
            hitpoints,
            damage,
            armor,
        }
    }
}

#[derive(Debug)]
struct Item {
    #[allow(dead_code)]
    name: String,
    cost: usize,
    damage: isize,
    armor: isize,
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        let mut tokens = value.split_whitespace();
        let name = tokens.next().unwrap().to_string();
        let cost = tokens.next().unwrap().parse().unwrap();
        let damage = tokens.next().unwrap().parse().unwrap();
        let armor = tokens.next().unwrap().parse().unwrap();
        Self {
            name,
            cost,
            damage,
            armor,
        }
    }
}

#[derive(Debug)]
struct Shop {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl Shop {
    fn generate(
        &self,
        weapons: usize,
        armor: usize,
        rings: usize,
    ) -> impl Iterator<Item = Vec<&Item>> {
        iproduct!(
            self.weapons.iter().combinations(weapons),
            self.armor.iter().combinations(armor),
            self.rings.iter().combinations(rings)
        )
        .map(|(mut w, mut a, mut r)| {
            w.append(&mut a);
            w.append(&mut r);
            w
        })
    }

    fn combos(&self) -> impl Iterator<Item = Vec<&Item>> {
        let weapons = 1..=1; // 1 weapon required
        let armor = 0..=1; // 1 armor is optional
        let rings = 0..=2; // 0-2 rings
        iproduct!(weapons, armor, rings).flat_map(|(w, a, r)| self.generate(w, a, r))
    }
}

impl From<&str> for Shop {
    fn from(value: &str) -> Self {
        let mut shop_str = value.split("\n\n");
        let weapons_str = shop_str.next().unwrap();
        let armor_str = shop_str.next().unwrap();
        let rings_str = shop_str.next().unwrap();

        let weapons: Vec<_> = weapons_str.lines().skip(1).map(Item::from).collect();
        let armor: Vec<_> = armor_str.lines().skip(1).map(Item::from).collect();
        let rings: Vec<_> = rings_str.lines().skip(1).map(Item::from).collect();

        Self {
            weapons,
            armor,
            rings,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let boss: Stats = input.into();
    let shop: Shop = SHOP_STR.trim().into();
    shop.combos()
        .filter(|combo| {
            let player = Stats::new(PLAYER_HITPOINTS, combo);
            player.fight(&boss)
        })
        .map(|combo| combo.iter().map(|i| i.cost).sum())
        .min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let boss: Stats = input.into();
    let shop: Shop = SHOP_STR.trim().into();
    shop.combos()
        .filter(|combo| {
            let player = Stats::new(PLAYER_HITPOINTS, combo);
            !player.fight(&boss)
        })
        .map(|combo| combo.iter().map(|i| i.cost).sum())
        .max()
}
