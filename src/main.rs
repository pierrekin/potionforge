use models::{IngredientCounts, IngredientKey};

mod models;
mod permute;
mod printer;
mod recommend;
mod simulate;

fn main() {
    let mut available_ingredients = IngredientCounts::new();
    available_ingredients.insert(IngredientKey::Asporeus, 1);
    available_ingredients.insert(IngredientKey::Catnip, 1);
    available_ingredients.insert(IngredientKey::Elven, 1);
    available_ingredients.insert(IngredientKey::Flyagaric, 1);
    available_ingredients.insert(IngredientKey::Sage, 1);
    available_ingredients.insert(IngredientKey::Wizards, 1);
    available_ingredients.insert(IngredientKey::Nightshade, 1);
    available_ingredients.insert(IngredientKey::Pluteus, 1);
    available_ingredients.insert(IngredientKey::Thyme, 1);
    available_ingredients.insert(IngredientKey::Wormwood, 1);

    let available_ingredient_keys: Vec<_> = available_ingredients.keys().collect();

    let possible_recipes = permute::get_all_recipes(
        available_ingredient_keys,
        vec!["cut", "ferment", "infuse"],
        6,
    );

    let recommendations = recommend::recommend(possible_recipes, &available_ingredients);
    printer::print_recipes_semi_compact(recommendations);
}
