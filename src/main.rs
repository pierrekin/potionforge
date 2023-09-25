use std::fs::File;
use std::io::Read;

use models::{IngredientCounts, IngredientKey};
use serde::Deserialize;

mod models;
mod permute;
mod printer;
mod recommend;
mod simulate;

#[derive(Debug, Deserialize)]
struct Config {
    arcane_power: i64,
    ingredients: IngredientCounts,
}

fn main() {
    let mut config_file = File::open("./config.yml").unwrap();
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents).unwrap();

    let config: Config = serde_yaml::from_str(&config_contents).unwrap();
    println!("{:?}", config);

    let available_ingredient_keys: Vec<_> = config.ingredients.keys().collect();

    let possible_recipes = permute::get_all_recipes(
        available_ingredient_keys,
        vec!["cut", "ferment", "infuse"],
        config.arcane_power,
    );

    let recommendations = recommend::recommend(possible_recipes, &config.ingredients);
    printer::print_recipes_semi_compact(recommendations);
}
