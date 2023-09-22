use crate::models::{Ingredient, Recipe};
use crate::simulate;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::ops::Range;
use std::sync::Mutex;

const SIMULATE_SUBRANGE_SIZE: usize = 5_000;

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

fn binomial_coefficient(n: usize, k: usize) -> usize {
    let mut coeff = 1;
    for i in 0..k {
        coeff *= n - i;
        coeff /= i + 1;
    }
    coeff
}

fn generate_combination(ingredients: &[Ingredient], k: usize, index: usize) -> Vec<Ingredient> {
    let n = ingredients.len();
    let mut combination = Vec::with_capacity(k);
    let mut elements = ingredients.to_vec();

    let mut remaining_index = index;

    for i in 0..k {
        let binom = binomial_coefficient(n - i - 1, k - i - 1);
        let chosen = remaining_index / binom;
        combination.push(elements.remove(chosen));
        remaining_index %= binom;
    }

    combination
}

fn report_progress(step: usize, total: usize, msg: &str) {
    println!(
        "{} {}",
        style(format!("[{}/{}]", step, total)).bold().dim(),
        msg
    );
}

fn setup_progress_bar(total_combinations: usize) -> Mutex<ProgressBar> {
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

fn solve_subrange(range: Range<usize>, all_ingredients: &[Ingredient], k: usize) -> Vec<Recipe> {
    range
        .filter_map(|index| {
            let combination = generate_combination(&all_ingredients, k, index);
            simulate::simulate(combination.as_slice())
        })
        .collect()
}

pub fn get_all_recipes(
    raw_ingredients: &[&Ingredient],
    processes: Vec<&str>,
    r: usize,
) -> Vec<Recipe> {
    report_progress(1, r, "Permuting ingredients...");
    let all_ingredients = permute_ingredients(raw_ingredients, processes);

    let all_recipes: Vec<_> = (2..=r)
        .flat_map(|k| {
            report_progress(k - 1, r, &format!("Simulating recipes k={}...", k));

            let total_combinations = binomial_coefficient(all_ingredients.len(), k);
            let progress = setup_progress_bar(total_combinations);

            let num_subranges = total_combinations / SIMULATE_SUBRANGE_SIZE;
            let subranges: Vec<_> = (0..num_subranges)
                .map(|i| i * SIMULATE_SUBRANGE_SIZE..(i + 1) * SIMULATE_SUBRANGE_SIZE)
                .collect();

            let k_recipes: Vec<_> = subranges
                .into_par_iter()
                .map(|range| {
                    progress.lock().unwrap().inc(range.len() as u64);
                    solve_subrange(range, &all_ingredients, k)
                })
                .flatten()
                .collect();

            progress.lock().unwrap().finish_and_clear();

            k_recipes
        })
        .collect();

    all_recipes
}
