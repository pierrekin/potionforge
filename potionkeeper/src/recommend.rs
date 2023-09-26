use std::{fs::File, io::Read};

use crate::{printer, Config};
use potionforge::{enumerate, recommend};

pub fn recommend(config_filename: String, cbc_loglevel: String) {
    println!("Recommend using config file: {}", config_filename);

    let mut config_file = File::open("./config.yml").unwrap();
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents).unwrap();

    let config: Config = serde_yaml::from_str(&config_contents).unwrap();
    println!("{:?}", config);

    let mut available_ingredient_keys: Vec<_> = config.ingredients.keys().collect();

    // TODO: Troubleshoot non-determinism on key order.
    available_ingredient_keys.sort();

    println!("Enumerating possible recipes...");
    let possible_recipes = enumerate::get_all_recipes(
        &available_ingredient_keys,
        &config.processes,
        config.arcane_power,
    );

    println!("Recommending optimal recipes...");
    let recommendations = recommend::recommend(
        possible_recipes,
        &config.ingredients,
        config.utilisation,
        cbc_loglevel,
    );
    let total_appeal: i32 = recommendations
        .iter()
        .map(|recipe| recipe.overall_appeal)
        .sum();

    println!("Total Appeal: {}", total_appeal);
    printer::print_recipes_table(&recommendations);
}
