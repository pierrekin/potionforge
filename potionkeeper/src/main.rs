use std::fs::File;
use std::io::Read;

extern crate clap;
use clap::{arg, command, Command};
use models::{IngredientCounts, Process};
use serde::Deserialize;

mod enumerate;
mod models;
mod printer;
mod recommend;
mod simulate;

#[derive(Debug, Deserialize)]
struct Config {
    arcane_power: i64,
    ingredients: IngredientCounts,
    processes: Vec<Process>,
    utilisation: i32,
}

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("recommend")
                .about("Recommend a potion")
                .arg(
                    arg!(-l --"cbc-loglevel" <STRING> "CBC log level")
                        .default_value("0")
                        .required(false),
                )
                .arg(
                    arg!(-c --config <PATH> "Config file")
                        .default_value("config.yml")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("debug").about("Debug a potion").arg(
                arg!(-c --config <PATH> "Config file")
                    .default_value("config.yml")
                    .required(false),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("recommend") {
        let config_file = matches.get_one::<String>("config").unwrap();
        let cbc_loglevel = matches.get_one::<String>("cbc-loglevel").unwrap();
        recommend(config_file.clone(), cbc_loglevel.clone());
    }

    if let Some(matches) = matches.subcommand_matches("debug") {
        let config_file = matches.get_one::<String>("config").unwrap();
        debug(config_file.clone());
    }
}

fn recommend(config_filename: String, cbc_loglevel: String) {
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

fn debug(config_filename: String) {
    println!("Debug using config file: {}", config_filename);
}
