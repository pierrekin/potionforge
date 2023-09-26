extern crate prettytable;

use std::cmp::Ordering;

use potionforge::models::{
    Department, Ingredient, OverallPurity, OverallTaste, OverallToxicity, Recipe, TasteEffect,
    ToxicityEffect,
};
use prettytable::{Cell, Row, Table};

pub fn _print_ingredients_table(ingredients: &Vec<Ingredient>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Index"),
        Cell::new("Kind"),
        Cell::new("Key"),
        Cell::new("Process"),
    ]));

    for (i, ingredient) in ingredients.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&(i + 1).to_string()),
            Cell::new(&format!("{:?}", ingredient.key)),
            Cell::new(&format!("{:?}", ingredient.kind)),
            Cell::new(&format!("{:?}", ingredient.process)),
        ]));
    }

    table.printstd();
}

pub fn print_recipes_table(recipes: &Vec<Recipe>) {
    let mut recipes = recipes.clone();
    recipes.sort_by(|a, b| {
        let dept_cmp = a.potion_kind.department.cmp(&b.potion_kind.department);
        if dept_cmp == Ordering::Equal {
            let main_effect_cmp = a.potion_kind.parts.0.cmp(&b.potion_kind.parts.0);
            if main_effect_cmp == Ordering::Equal {
                return a.potion_kind.parts.1.cmp(&b.potion_kind.parts.1);
            }
            main_effect_cmp
        } else {
            dept_cmp
        }
    });

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Index"),
        Cell::new("Department"),
        Cell::new("Potion"),
        Cell::new("Ingredients"),
        Cell::new("Purity"),
        Cell::new("Toxicity"),
        Cell::new("Taste"),
        Cell::new("Appeal"),
    ]));

    for (i, recipe) in recipes.iter().enumerate() {
        let dept = match recipe.potion_kind.department {
            Department::Health => "Health",
            Department::Sourcery => "Sourcery",
            Department::Provisions => "Provisions",
        };

        let ingredients: Vec<String> = recipe
            .ingredients
            .iter()
            .map(|ing| format!("{:?} ({})", ing.key, ing.process.to_human()))
            .collect();

        let toxicity_tag = match recipe.potion_kind.toxicity_effect {
            ToxicityEffect::ToxicPositive => {
                if matches!(recipe.overall_toxicity, OverallToxicity::Neutral) {
                    "".to_string()
                } else if matches!(
                    recipe.overall_toxicity,
                    OverallToxicity::Toxic | OverallToxicity::VeryToxic
                ) {
                    format!("+{:?}", recipe.overall_toxicity)
                } else {
                    format!("-{:?}", recipe.overall_toxicity)
                }
            }
            ToxicityEffect::ToxicNegative => {
                if matches!(recipe.overall_toxicity, OverallToxicity::Neutral) {
                    "".to_string()
                } else if matches!(
                    recipe.overall_toxicity,
                    OverallToxicity::Toxic | OverallToxicity::VeryToxic
                ) {
                    format!("-{:?}", recipe.overall_toxicity)
                } else {
                    format!("+{:?}", recipe.overall_toxicity)
                }
            }
        };

        let taste_tag = match recipe.potion_kind.taste_effect {
            TasteEffect::TastyPositive => {
                if matches!(recipe.overall_taste, OverallTaste::Bland) {
                    "".to_string()
                } else if matches!(
                    recipe.overall_taste,
                    OverallTaste::Tasty
                        | OverallTaste::Flavorful
                        | OverallTaste::Bitter
                        | OverallTaste::Sweet
                        | OverallTaste::Delicious
                ) {
                    format!("+{:?}", recipe.overall_taste)
                } else {
                    format!("-{:?}", recipe.overall_taste)
                }
            }
            TasteEffect::TastyNegative => {
                if matches!(recipe.overall_taste, OverallTaste::Bland) {
                    "".to_string()
                } else if matches!(
                    recipe.overall_taste,
                    OverallTaste::Tasty
                        | OverallTaste::Bitter
                        | OverallTaste::Foul
                        | OverallTaste::Unsavory
                        | OverallTaste::Icky
                ) {
                    format!("+{:?}", recipe.overall_taste)
                } else {
                    format!("-{:?}", recipe.overall_taste)
                }
            }
            TasteEffect::TastyNeutral => "".to_string(),
        };

        let purity_tag = match recipe.overall_purity {
            OverallPurity::Neutral => "",
            OverallPurity::Impure => "-Impure",
        };

        // Normalise the ingredients list.
        let mut local_ingredients = ingredients.clone();
        local_ingredients.sort();

        table.add_row(Row::new(vec![
            Cell::new(&(i + 1).to_string()),
            Cell::new(dept),
            Cell::new(&format!("{:?}", recipe.potion_kind.key)),
            Cell::new(&local_ingredients.join(", ")),
            Cell::new(&format!("{}", purity_tag)),
            Cell::new(&format!("{}", toxicity_tag)),
            Cell::new(&format!("{}", taste_tag)),
            Cell::new(&recipe.overall_appeal.to_string()),
        ]));
    }

    table.printstd();
}

pub fn _print_combinations_table(combinations: &Vec<Vec<Ingredient>>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Index"), Cell::new("Combination")]));

    for (i, combination) in combinations.iter().enumerate() {
        let ingredients: Vec<String> = combination
            .iter()
            .map(|ingredient| format!("{:?} ({:?})", ingredient.key, ingredient.process))
            .collect();

        table.add_row(Row::new(vec![
            Cell::new(&(i + 1).to_string()),
            Cell::new(&ingredients.join(", ")),
        ]));
    }

    table.printstd();
}

pub fn _print_combination(combination: &Vec<Ingredient>) {
    let ingredients: Vec<String> = combination
        .iter()
        .map(|ing| format!("{:?} ({:?})", ing.key, ing.process))
        .collect();

    println!("{}", &ingredients.join(", "));
}