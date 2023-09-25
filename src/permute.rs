use crate::models::{GetByKey, Ingredient, IngredientKey, Recipe, INGREDIENTS};
use crate::simulate;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::ops::Range;
use std::sync::Mutex;

const SIMULATE_SUBRANGE_SIZE: i64 = 5_000;

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
    let mut elements = ingredients.to_vec();

    let mut remaining_index = index;

    for i in 0..k {
        let binom = binomial_coefficient(n - i - 1, k - i - 1);
        let chosen = remaining_index / binom;
        combination.push(elements.remove(chosen as usize));
        remaining_index %= binom;
    }

    combination
}

fn report_progress(step: i64, total: i64, msg: &str) {
    println!(
        "{} {}",
        style(format!("[{}/{}]", step, total)).bold().dim(),
        msg
    );
}

fn setup_progress_bar(total_combinations: i64) -> Mutex<ProgressBar> {
    let progress = Mutex::new(ProgressBar::new(total_combinations as u64));
    progress.lock().unwrap().set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta_precise}) {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    progress
}

fn check_combination(combination: &Vec<Ingredient>) -> bool {
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

fn solve_subrange(range: Range<i64>, all_ingredients: &[Ingredient], k: i64) -> Vec<Recipe> {
    range
        .filter_map(|index| {
            let combination = generate_combination(&all_ingredients, k, index as i64);
            if check_combination(&combination) {
                simulate::simulate(&combination.as_slice())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_recipes(
    raw_ingredients: Vec<&IngredientKey>,
    processes: Vec<&str>,
    r: i64,
) -> Vec<Recipe> {
    report_progress(1, r, "Permuting ingredients...");
    let raw_ingredients: Vec<_> = raw_ingredients
        .iter()
        .map(|key| INGREDIENTS.get_by_key(key))
        .collect();

    let all_ingredients = permute_ingredients(raw_ingredients.as_slice(), processes);

    let mut result = Vec::new();

    for k in 2..=r {
        let total_combinations = binomial_coefficient(all_ingredients.len() as i64, k);
        for index in 0..total_combinations {
            let combination = generate_combination(&all_ingredients, k, index as i64);
            if check_combination(&combination) {
                let simulated = simulate::simulate(combination.as_slice());
                result.extend(simulated);
            }
        }
    }

    result
}
