use std::{collections::HashMap, fs::File, io::Read};

use potionforge::{
    enumerate::process_ingredient,
    models::{traits::GetByKey, Ingredient, IngredientKey, Process, Recipe, INGREDIENTS},
    recommend::{AlchemistAttributes, MarketConditions},
    simulate::simulate,
};
use serde::Deserialize;

use crate::printer;

#[derive(Debug, Deserialize)]
struct Config {
    recipes: Vec<HashMap<IngredientKey, Vec<Process>>>,
}

pub fn debug(config_filename: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Debug using config file: {}", config_filename);

    let mut config_file = File::open(config_filename).unwrap();
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents).unwrap();

    let config: Config = serde_yaml::from_str(&config_contents).unwrap();
    println!("{:?}", config);

    let recipes: Vec<Recipe> = config
        .recipes
        .iter()
        .filter_map(|recipe| {
            let ingredients: Vec<Ingredient> = recipe
                .iter()
                .map(|(ingredient_key, ingredient_process)| {
                    let raw_ingredient = INGREDIENTS.get_by_key(ingredient_key);
                    process_ingredient(raw_ingredient, ingredient_process)
                })
                .collect();
            simulate(
                ingredients.as_slice(),
                &AlchemistAttributes::new(),
                &MarketConditions::new(),
            )
        })
        .collect();

    printer::print_recipes_table(&recipes);
    Ok(())
}
