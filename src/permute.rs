
use std::collections::HashSet;



use crate::models::{GetByKey, Ingredient, IngredientKey, Recipe, INGREDIENTS};
use crate::simulate;

use itertools::Itertools;
use rayon::prelude::*;

pub fn permute_ingredient(
    ingredient: &Ingredient,
    processes: Option<Vec<&str>>,
) -> Vec<Ingredient> {
    let mut result = vec![ingredient.clone()];
    let processes = processes.unwrap_or(vec!["cut"]);

    if processes.contains(&"cut") {
        if let Some(mut cut_results) = simulate::process_cut(ingredient) {
            result.append(&mut cut_results);
        }
    }

    if processes.contains(&"ferment") {
        if let Some(ferment_result) = simulate::process_ferment(ingredient) {
            result.push(ferment_result);
        }
    }

    if processes.contains(&"infuse") {
        if let Some(infuse_result) = simulate::process_infuse(ingredient) {
            result.push(infuse_result);
        }
    }

    result
}

pub fn permute_ingredients(ingredients: &[&Ingredient], processes: Vec<&str>) -> Vec<Ingredient> {
    ingredients
        .iter()
        .flat_map(|ing| permute_ingredient(ing, Some(processes.clone())))
        .collect()
}

fn binomial_coefficient(n: i64, k: i64) -> i64 {
    let mut coeff = 1;
    for i in 0..k {
        coeff *= n - i;
        coeff /= i + 1;
    }
    coeff
}

fn generate_combination(ingredients: &[Ingredient], k: i64, index: i64) -> Vec<Ingredient> {
    let n = ingredients.len() as i64;
    let mut combination = Vec::with_capacity(k as usize);
    let mut remaining_index = index;
    let mut used = vec![false; n as usize];

    for i in 0..k {
        let binom = binomial_coefficient(n - i - 1, k - i - 1);
        let mut chosen = remaining_index / binom;
        remaining_index %= binom;

        let mut actual_index = 0;
        while chosen >= 0 {
            if !used[actual_index] {
                if chosen == 0 {
                    break;
                }
                chosen -= 1;
            }
            actual_index += 1;
        }

        used[actual_index] = true;
        combination.push(ingredients[actual_index].clone());
    }

    combination
}

fn validate_combination(combination: &Vec<Ingredient>) -> bool {
    // TODO: This shouldn't be hard coded here.
    let mut keys = [false; 16];

    for ingredient in combination {
        let index = ingredient.key as usize;
        if keys[index] {
            return false;
        }
        keys[index] = true;
    }

    true
}

pub fn get_all_recipes(
    raw_ingredients: Vec<&IngredientKey>,
    processes: Vec<&str>,
    r: i64,
) -> Vec<Recipe> {
    let raw_ingredients: Vec<_> = raw_ingredients
        .iter()
        .map(|key| INGREDIENTS.get_by_key(key))
        .collect();

    let all_ingredients = permute_ingredients(raw_ingredients.as_slice(), processes);

    let mut i = 0;

    let mut debug: HashSet<Vec<Ingredient>> = HashSet::new();

    let result = (2..=r)
        .into_iter()
        .flat_map(|k| {
            let total_combinations = binomial_coefficient(all_ingredients.len() as i64, k);
            (0..total_combinations)
                // .into_par_iter()
                .into_iter()
                .filter_map(|index| {
                    i += 1;
                    let combination = generate_combination(&all_ingredients, k, index as i64);
                    debug.insert(combination.clone());
                    if !validate_combination(&combination) {
                        return None;
                    }
                    match simulate::simulate(combination.as_slice()) {
                        Some(inner_value) => Some(inner_value),
                        None => None,
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect();

    result
}
