use crate::models::{Ingredient, IngredientKey, Process};
use crate::process;

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
