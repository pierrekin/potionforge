extern crate prettytable;

use std::cmp::Ordering;

use crate::models::{Department, OverallToxicity, Recipe, ToxicityEffect};
use prettytable::{Cell, Row, Table};

pub fn print_recipes_table(recipes: Vec<Recipe>) {
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
        // Cell::new("Main Effect"),
        // Cell::new("Element"),
        Cell::new("Ingredients"),
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

        let mut tags = String::new();

        match recipe.potion_kind.toxicity_effect {
            ToxicityEffect::ToxicPositive => {
                if matches!(
                    recipe.overall_toxicity,
                    OverallToxicity::Toxic | OverallToxicity::VeryToxic
                ) {
                    tags.push_str("+");
                } else {
                    tags.push_str("-")
                }
            }
            ToxicityEffect::ToxicNegative => {
                if matches!(
                    recipe.overall_toxicity,
                    OverallToxicity::Toxic | OverallToxicity::VeryToxic
                ) {
                    tags.push_str("-");
                } else {
                    tags.push_str("+")
                }
            }
        }
        tags.push_str(format!("{:?}", recipe.overall_toxicity).as_str());

        table.add_row(Row::new(vec![
            Cell::new(&(i + 1).to_string()),
            Cell::new(dept),
            Cell::new(&format!("{:?}", recipe.potion_kind.key)),
            // Cell::new(&format!("{:?}", recipe.potion_kind.parts.0)),
            // Cell::new(&format!("{:?}", recipe.potion_kind.parts.1)),
            Cell::new(&ingredients.join(", ")),
            Cell::new(&format!("{:}", tags)),
            Cell::new(&format!("{:?}", recipe.overall_taste)),
            Cell::new(&recipe.overall_appeal.to_string()),
        ]));
    }

    table.printstd();
}
