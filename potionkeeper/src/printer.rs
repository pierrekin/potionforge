extern crate prettytable;

use std::cmp::Ordering;

use potionforge::models::{
    traits::{GetName, ToHumanReadable},
    Ingredient, OverallPurity, OverallTaste, OverallToxicity, Recipe, TasteEffect, ToxicityEffect,
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
        OverallPurity::Neutral => "",
        OverallPurity::Impure => "-Impure",
    }
}

fn add_recipe_row(table: &mut Table, index: usize, recipe: &Recipe) {
    let ingredients = get_ingredients_string(&recipe.ingredients);
    let toxicity_tag = get_toxicity_tag(
        &recipe.potion_kind.toxicity_effect,
        &recipe.overall_toxicity,
    );
    let taste_tag = get_taste_tag(&recipe.potion_kind.taste_effect, &recipe.overall_taste);
    let purity_tag = get_purity_tag(&recipe.overall_purity);

    table.add_row(Row::new(vec![
        Cell::new(&(index + 1).to_string()),
        Cell::new(&recipe.potion_kind.department.name()),
        Cell::new(&recipe.potion_kind.name()),
        Cell::new(&ingredients),
        Cell::new(&purity_tag),
        Cell::new(&toxicity_tag),
        Cell::new(&taste_tag),
        Cell::new(&recipe.overall_appeal.to_string()),
    ]));
}

fn create_table(headers: Vec<&str>) -> Table {
    let row: Vec<Cell> = headers.into_iter().map(|h| Cell::new(h)).collect();
    let mut table = Table::new();
    table.add_row(Row::new(row));
    table
}

fn add_ingredient_row(table: &mut Table, index: usize, ingredient: &Ingredient) {
    table.add_row(Row::new(vec![
        Cell::new(&(index + 1).to_string()),
        Cell::new(&format!("{:?}", ingredient.key)),
        Cell::new(&format!("{:?}", ingredient.kind)),
        Cell::new(&format!("{:?}", ingredient.process)),
    ]));
}

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

fn sort_recipes(recipes: &mut Vec<Recipe>) {
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
    ]);

    for (i, recipe) in sorted_recipes.iter().enumerate() {
        add_recipe_row(&mut table, i, recipe);
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

#[cfg(test)]
mod tests {
    use super::*;
    use potionforge::models::{traits::GetByKey, IngredientKey, INGREDIENTS};

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

    #[test]
    fn test_add_ingredient_row() {
        let mut table = Table::new();
        let ingredient = INGREDIENTS.get_by_key(&IngredientKey::Anise);
        add_ingredient_row(&mut table, 0, &ingredient);

        let expected_row = Row::new(vec![
            Cell::new("1"),
            Cell::new("Anise"),
            Cell::new("Herb"),
            Cell::new("Raw"),
        ]);

        // Check if the first row matches the expected row
        assert_eq!(table.get_row(0).unwrap(), &expected_row);

        // Check if the table has only one row
        assert_eq!(table.len(), 1);
    }
}
