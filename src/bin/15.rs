advent_of_code::solution!(15);

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let tokens: Vec<_> = value.split_whitespace().collect();
        let capacity = tokens[2].strip_suffix(",").unwrap().parse().unwrap();
        let durability = tokens[4].strip_suffix(",").unwrap().parse().unwrap();
        let flavor = tokens[6].strip_suffix(",").unwrap().parse().unwrap();
        let texture = tokens[8].strip_suffix(",").unwrap().parse().unwrap();
        let calories = tokens[10].parse().unwrap();
        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[derive(Default)]
struct Recipe {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Recipe {
    fn add(&self, ingredient: &Ingredient, amount: usize) -> Self {
        let amount = amount as i64;
        Self {
            capacity: self.capacity + ingredient.capacity * amount,
            durability: self.durability + ingredient.durability * amount,
            flavor: self.flavor + ingredient.flavor * amount,
            texture: self.texture + ingredient.texture * amount,
            calories: self.calories + ingredient.calories * amount,
        }
    }

    fn score(&self) -> i64 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }

    fn is_diet(&self) -> bool {
        self.calories == 500
    }
}

fn combine(ingredients: &[Ingredient], recipe: &Recipe, spoons: usize) -> i64 {
    match ingredients.split_first() {
        Some((head, tail)) => (0..=spoons)
            .map(|n| {
                let modified_recipe = recipe.add(head, n);
                combine(tail, &modified_recipe, spoons - n)
            })
            .max()
            .unwrap(),
        None => recipe.score(),
    }
}

fn combine_diet(ingredients: &[Ingredient], recipe: &Recipe, spoons: usize) -> Option<i64> {
    match ingredients.split_first() {
        Some((head, tail)) => (0..=spoons)
            .filter_map(|n| {
                let modified_recipe = recipe.add(head, n);
                combine_diet(tail, &modified_recipe, spoons - n)
            })
            .max(),
        None if recipe.is_diet() => Some(recipe.score()),
        None => None,
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let ingredients: Vec<_> = input.lines().map(Ingredient::from).collect();
    Some(combine(&ingredients, &Recipe::default(), 100))
}

pub fn part_two(input: &str) -> Option<i64> {
    let ingredients: Vec<_> = input.lines().map(Ingredient::from).collect();
    combine_diet(&ingredients, &Recipe::default(), 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62842880));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57600000));
    }
}
