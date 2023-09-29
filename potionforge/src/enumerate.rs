use crate::models::traits::GetByKey;
use crate::models::{
    Ingredient, IngredientKey, IngredientPart, OverallToxicity, Process, Recipe, ToxicityEffect,
    INGREDIENTS, POTION_KINDS,
};
use crate::process;
use crate::simulate::{self, collect_parts, SimulateConfig};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
pub struct EnumerateConfig {
    pub ingredients: Vec<IngredientKey>,
    pub arcane_power: i64,
    pub utilisation: i32,
    pub processes: Vec<Process>,
}

pub fn permute_ingredient(ingredient: &Ingredient, processes: &Vec<Process>) -> Vec<Ingredient> {
    let mut result = vec![ingredient.clone()];

    if processes.contains(&Process::Crush) {
        if let Some(crushed_ingredient) = process::process_crush(ingredient) {
            result.push(crushed_ingredient);
        };
    }

    if processes.contains(&Process::Blanch) {
        if let Some(blanched_ingredient) = process::process_blanch(ingredient) {
            result.push(blanched_ingredient);
        }
    }

    if processes.contains(&Process::Dry) {
        if let Some(dried_ingredient) = process::process_dry(ingredient) {
            result.push(dried_ingredient);
        }
    }

    if processes.contains(&Process::Pickle) {
        if let Some(pickled_ingredient) = process::process_pickle(ingredient) {
            result.push(pickled_ingredient);
        }
    }

    if processes.contains(&Process::Ferment) {
        for ingredient in result.clone() {
            if let Some(ferment_result) = process::process_ferment(&ingredient) {
                result.push(ferment_result);
            }
        }
    }

    if processes.contains(&Process::Infuse) {
        for ingredient in result.clone() {
            if let Some(infuse_result) = process::process_infuse(&ingredient) {
                result.push(infuse_result);
            }
        }
    }

    result
}

pub fn permute_ingredients(
    ingredients: &[&Ingredient],
    processes: &Vec<Process>,
) -> Vec<Ingredient> {
    ingredients
        .iter()
        .flat_map(|ing| permute_ingredient(ing, &processes))
        .collect()
}

/// Validate a combination of ingredienst is a possible recipe.
fn is_combination_valid(combination: &Vec<Ingredient>) -> bool {
    // TODO: This count shouldn't be hard coded here.
    let mut keys = [false; 16];

    // If any single ingredient appears more than once
    for ingredient in combination {
        let index = ingredient.key as usize;
        if keys[index] {
            return false;
        }
        keys[index] = true;
    }

    true
}

fn is_combination_reasonable(combination: &Vec<Ingredient>) -> bool {
    // If any ingredient contains an impurity.
    let parts = collect_parts(&combination);
    return !parts.contains(&IngredientPart::Impurity);
}

fn is_recipe_reasonable(recipe: &Recipe) -> bool {
    // If the recipe overall appeal is negative.
    if recipe.overall_appeal < 0 {
        return false;
    }

    // If the recipe toxicity and toxicity appeal do not match.
    let potion_kind = POTION_KINDS.get_by_key(&recipe.potion_kind_key);
    match potion_kind.toxicity_effect {
        ToxicityEffect::ToxicPositive => {
            if matches!(recipe.overall_toxicity, OverallToxicity::Antitoxic)
                || matches!(recipe.overall_toxicity, OverallToxicity::Veryantitoxic)
            {
                return false;
            }
        }
        ToxicityEffect::ToxicNegative => {
            if matches!(recipe.overall_toxicity, OverallToxicity::Toxic)
                || matches!(recipe.overall_toxicity, OverallToxicity::VeryToxic)
            {
                return false;
            }
        }
    }

    return true;
}

pub fn enumerate_and_simulate(
    enumerate_config: &EnumerateConfig,
    simulate_config: &SimulateConfig,
    // raw_ingredients: &Vec<&IngredientKey>,
    // processes: &Vec<Process>,
    // r: i64,
    // alchemist_attributes: &AlchemistAttributes,
    // market_conditions: &MarketConditions,
    // branding_counts: &BrandingCounts,
) -> Vec<Recipe> {
    let raw_ingredients: Vec<_> = enumerate_config
        .ingredients
        .iter()
        .map(|key| INGREDIENTS.get_by_key(key))
        .collect();
    let all_ingredients =
        permute_ingredients(raw_ingredients.as_slice(), &enumerate_config.processes);

    (2..=enumerate_config.arcane_power)
        .into_par_iter()
        .flat_map(|k| {
            all_ingredients
                .iter()
                .combinations(k as usize)
                .collect::<Vec<_>>()
                .into_par_iter()
                .filter_map(|combination| {
                    let local_combination = combination.into_iter().cloned().collect();
                    if !is_combination_valid(&local_combination) {
                        return None;
                    }
                    if !is_combination_reasonable(&local_combination) {
                        return None;
                    }
                    let recipe = simulate::simulate(local_combination.as_slice(), &simulate_config);
                    if recipe.is_none() {
                        return None;
                    }

                    let recipe = recipe.unwrap();

                    if !is_recipe_reasonable(&recipe) {
                        return None;
                    }

                    Some(recipe)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
