use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    enumerate::{permute_ingredients, EnumerateConfig},
    models::{
        traits::GetByKey, Ingredient, IngredientPart, OverallToxicity, Recipe, ToxicityEffect,
        INGREDIENTS, POTION_KINDS,
    },
    recommend::{maximise_appeal, maximise_potency, maximise_recipes, RecommendConfig},
    simulate::{self, collect_parts, SimulateConfig},
};

pub fn enumerate_and_simulate(
    enumerate_config: &EnumerateConfig,
    simulate_config: &SimulateConfig,
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

pub fn recommend(possible_recipes: Vec<Recipe>, config: &RecommendConfig) -> Vec<Recipe> {
    let recipe_count = maximise_recipes(
        &possible_recipes,
        &config.available_ingredients,
        config.utilisation,
        &config.potions,
    );

    let appeal = maximise_appeal(
        &possible_recipes,
        &config.available_ingredients,
        config.utilisation,
        &config.potions,
        recipe_count,
    );

    maximise_potency(
        &possible_recipes,
        &config.available_ingredients,
        config.utilisation,
        &config.potions,
        recipe_count,
        appeal,
    )
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
