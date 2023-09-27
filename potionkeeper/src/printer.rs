extern crate prettytable;

use std::cmp::Ordering;

use potionforge::models::{
    traits::{GetByKey, GetName, ToHumanReadable},
    Ingredient, OverallPurity, OverallTaste, OverallToxicity, Recipe, TasteEffect, ToxicityEffect,
    POTION_KINDS,
};
use prettytable::{Cell, Row, Table};

fn get_ingredients_string(ingredients: &[Ingredient]) -> String {
    let mut local_ingredients: Vec<String> = ingredients
        .iter()
        .map(|ing| format!("{} ({})", ing.name(), ing.process.to_human()))
        .collect();
    local_ingredients.sort();
    local_ingredients.join(", ")
}

fn get_toxicity_tag(
    toxicity_effect: &ToxicityEffect,
    overall_toxicity: &OverallToxicity,
) -> String {
    match toxicity_effect {
        ToxicityEffect::ToxicPositive => {
            if matches!(overall_toxicity, OverallToxicity::Neutral) {
                "".to_string()
            } else if matches!(
                overall_toxicity,
                OverallToxicity::Toxic | OverallToxicity::VeryToxic
            ) {
                format!("+{:?}", overall_toxicity)
            } else {
                format!("-{:?}", overall_toxicity)
            }
        }
        ToxicityEffect::ToxicNegative => {
            if matches!(overall_toxicity, OverallToxicity::Neutral) {
                "".to_string()
            } else if matches!(
                overall_toxicity,
                OverallToxicity::Toxic | OverallToxicity::VeryToxic
            ) {
                format!("-{:?}", overall_toxicity)
            } else {
                format!("+{:?}", overall_toxicity)
            }
        }
    }
}

fn get_taste_tag(taste_effect: &TasteEffect, overall_taste: &OverallTaste) -> String {
    match taste_effect {
        TasteEffect::TastyPositive => {
            if matches!(overall_taste, OverallTaste::Bland) {
                "".to_string()
            } else if matches!(
                overall_taste,
                OverallTaste::Tasty
                    | OverallTaste::Flavorful
                    | OverallTaste::Bitter
                    | OverallTaste::Sweet
                    | OverallTaste::Delicious
            ) {
                format!("+{:?}", overall_taste)
            } else {
                format!("-{:?}", overall_taste)
            }
        }
        TasteEffect::TastyNegative => {
            if matches!(overall_taste, OverallTaste::Bland) {
                "".to_string()
            } else if matches!(
                overall_taste,
                OverallTaste::Tasty
                    | OverallTaste::Bitter
                    | OverallTaste::Foul
                    | OverallTaste::Unsavory
                    | OverallTaste::Icky
            ) {
                format!("+{:?}", overall_taste)
            } else {
                format!("-{:?}", overall_taste)
            }
        }
        TasteEffect::TastyNeutral => "".to_string(),
    }
}

fn get_purity_tag(overall_purity: &OverallPurity) -> &'static str {
    match overall_purity {
        OverallPurity::Neutral => "Pure",
        OverallPurity::Impure => "-Impure",
    }
}

fn add_recipe_row(table: &mut Table, index: usize, recipe: &Recipe) {
    let ingredients = get_ingredients_string(&recipe.ingredients);
    let potion_kind = POTION_KINDS.get_by_key(&recipe.potion_kind_key);
    let toxicity_tag = get_toxicity_tag(&potion_kind.toxicity_effect, &recipe.overall_toxicity);
    let taste_tag = get_taste_tag(&potion_kind.taste_effect, &recipe.overall_taste);
    let purity_tag = get_purity_tag(&recipe.overall_purity);

    table.add_row(Row::new(vec![
        Cell::new(&(index + 1).to_string()),
        Cell::new(&potion_kind.department.name()),
        Cell::new(&potion_kind.name()),
        Cell::new(&ingredients),
        Cell::new(&purity_tag),
        Cell::new(&toxicity_tag),
        Cell::new(&taste_tag),
        Cell::new(format!("{}%", recipe.overall_appeal).as_str()),
        Cell::new(format!("{:.1}", recipe.overall_potency as f64 / 100.).as_str()),
    ]));
}

fn create_table(headers: Vec<&str>) -> Table {
    let row: Vec<Cell> = headers.into_iter().map(|h| Cell::new(h)).collect();
    let mut table = Table::new();
    table.add_row(Row::new(row));
    table
}

fn sort_recipes(recipes: &mut Vec<Recipe>) {
    recipes.sort_by(|a, b| {
        let a_potion_kind = POTION_KINDS.get_by_key(&a.potion_kind_key);
        let b_potion_kind = POTION_KINDS.get_by_key(&b.potion_kind_key);

        let dept_cmp = a_potion_kind.department.cmp(&b_potion_kind.department);
        if dept_cmp == Ordering::Equal {
            let main_effect_cmp = a_potion_kind.parts.0.cmp(&b_potion_kind.parts.0);
            if main_effect_cmp == Ordering::Equal {
                return a_potion_kind.parts.1.cmp(&b_potion_kind.parts.1);
            }
            main_effect_cmp
        } else {
            dept_cmp
        }
    });
}

pub fn print_recipes_table(recipes: &[Recipe]) {
    let mut sorted_recipes = recipes.to_vec();
    sort_recipes(&mut sorted_recipes);

    let mut table = create_table(vec![
        "Index",
        "Department",
        "Potion",
        "Ingredients",
        "Purity",
        "Toxicity",
        "Taste",
        "Appeal",
        "Potency",
    ]);

    for (i, recipe) in sorted_recipes.iter().enumerate() {
        add_recipe_row(&mut table, i, recipe);
    }
    table.printstd();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let headers = vec!["Index", "Name", "Age"];
        let table = create_table(headers.clone());
        let expected_row = Row::new(headers.into_iter().map(|h| Cell::new(h)).collect());

        // Check if the first row matches the headers
        assert_eq!(table.get_row(0).unwrap(), &expected_row);

        // Check if the table has only one row (the header)
        assert_eq!(table.len(), 1);
    }
}
