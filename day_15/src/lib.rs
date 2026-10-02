use std::cmp;
use std::str::FromStr;

pub type IngredientAmount = (i64, Ingredient);

#[derive(Debug, Copy, Clone)]
pub struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl FromStr for Ingredient {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();
        let scores = [2, 4, 6, 8, 10]
            .iter()
            .map(|&i| split[i].trim_end_matches(",").parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            capacity: scores[0],
            durability: scores[1],
            flavor: scores[2],
            texture: scores[3],
            calories: scores[4],
        })
    }
}

pub fn calculate_best_recipe(ingredients: &[Ingredient], amount: i64) -> i64 {
    let mut amounts = Vec::new();
    calculate_best_recipe_internal(ingredients, &mut amounts, amount)
}

fn calculate_best_recipe_internal(
    ingredients: &[Ingredient],
    amounts: &mut Vec<i64>,
    remaining: i64,
) -> i64 {
    if amounts.len() + 1 == ingredients.len() {
        amounts.push(remaining);
        let recipe = amounts
            .iter()
            .cloned()
            .zip(ingredients.iter().cloned())
            .collect::<Vec<_>>();
        amounts.pop();

        return get_recipe_score(&recipe[..]);
    }

    let mut best_found = 0;
    for n in 0..=remaining {
        amounts.push(n);
        let remaining_for_rest = remaining - n;
        let result = calculate_best_recipe_internal(
            ingredients,
            amounts,
            remaining_for_rest,
        );
        best_found = cmp::max(best_found, result);
        amounts.pop();
    }

    best_found
}

fn get_recipe_score(recipe: &[IngredientAmount]) -> i64 {
    let capacity = get_property_score(recipe, |i| i.capacity);
    let durability = get_property_score(recipe, |i| i.durability);
    let flavor = get_property_score(recipe, |i| i.flavor);
    let texture = get_property_score(recipe, |i| i.texture);
    capacity * durability * flavor * texture
}

fn get_property_score(recipe: &[IngredientAmount], property_get: impl Fn(&Ingredient) -> i64) -> i64 {
    let result = recipe.iter()
        .map(|(amount, ingredient)| {
            let value = property_get(ingredient);
            value * *amount
        })
        .sum();
    cmp::max(result, 0)
}

mod tests {
    use super::*;

    const BUTTERSCOTCH: Ingredient = Ingredient {
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        calories: 8,
    };

    const CINNAMON: Ingredient = Ingredient {
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        calories: 3,
    };

    #[test]
    pub fn total_score_sums_correctly() {
        assert_eq!(
            get_recipe_score(&[(44, BUTTERSCOTCH), (56, CINNAMON)]),
            62842880
        );
    }
}
