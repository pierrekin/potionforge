use std::{fs::File, io::Read};

use crate::printer;
use potionforge::enumerate::EnumerateConfig;
use potionforge::simulate::SimulateConfig;
use potionforge::{enumerate, recommend, simulate};

use serde::Deserialize;

use potionforge::models::{Process, Recipe};
use potionforge::recommend::{
    AlchemistAttributes, BrandingCounts, IngredientCounts, MarketConditions,
};

#[derive(Debug, Deserialize)]
struct RecommendConfig {
    arcane_power: i64,
    utilisation: i32,
    processes: Vec<Process>,
    ingredients: IngredientCounts,
    alchemists: AlchemistAttributes,
    market: MarketConditions,
    branding: BrandingCounts,
}

/// Load configuration from the specified file matching the Config struct.
fn load_config(filename: String) -> Result<RecommendConfig, Box<dyn std::error::Error>> {
    let mut config_file = File::open(filename)?;
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents)?;
    Ok(serde_yaml::from_str(&config_contents)?)
}

/// Display some summary statistics and a table of recommendated recipes.
fn display_results(recommendations: &[Recipe]) {
    // Appeal of all the recipes linearly combined together.
    let total_appeal: i32 = recommendations
        .iter()
        .map(|recipe| recipe.overall_appeal)
        .sum();
    // Potency of all the recipes linearly combined together.
    let total_potency: i32 = recommendations
        .iter()
        .map(|recipe| recipe.overall_potency)
        .sum();

    println!("Total Appeal: {}", total_appeal);
    println!("Total Potency: {}", total_potency);
    printer::print_recipes_table(&recommendations);
}

/// Recommend potions that can be created using the provided configuration
///
/// 1. Read configuration file specified and extract available ingredients,
/// alchemy configuration and market conditions.
///
/// 2. Determine all possible recipes that can be crafted using the available
/// ingredients.
///
/// 3. Recommend a combination of recipes using the potionforge algorithm.
pub fn recommend(
    config_filename: String,
    solver_loglevel: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Recommend using config file: {}", config_filename);
    let config = load_config(config_filename)?;

    let ingredient_keys: Vec<_> = config.ingredients.keys().map(|key| key.clone()).collect();

    let enumerate_config = EnumerateConfig {
        ingredients: ingredient_keys,
        arcane_power: config.arcane_power,
        utilisation: config.utilisation,
        processes: config.processes,
    };

    let simulate_config = SimulateConfig {
        alchemists_attributes: config.alchemists,
        market_conditions: config.market,
        branding_counts: config.branding,
    };

    println!("Enumerating possible recipes...");
    let possible_recipes: Vec<Recipe> = enumerate::enumerate(&enumerate_config, &simulate_config);

    println!("Recommending optimal recipes...");
    let recommendations: Vec<Recipe> = recommend::recommend(
        possible_recipes,
        &config.ingredients,
        config.utilisation,
        solver_loglevel,
    );

    display_results(&recommendations);
    Ok(())
}
