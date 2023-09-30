use std::collections::HashMap;

use highs::{Col, HighsModelStatus, RowProblem, Sense};
use itertools::Itertools;

use crate::models::{
    traits::GetByKey, AlchemistAttribute, BrandingCategory, Department, IngredientKey,
    MarketCondition, PotionKindKey, Recipe, POTION_KINDS,
};

pub type IngredientCounts = HashMap<IngredientKey, i32>;
pub type AlchemistAttributes = HashMap<AlchemistAttribute, i32>;
pub type MarketConditions = HashMap<PotionKindKey, Vec<MarketCondition>>;
pub type BrandingCounts = HashMap<BrandingCategory, i32>;

#[derive(Debug)]
pub struct RecommendConfig {
    pub available_ingredients: IngredientCounts,
    pub utilisation: i32,
    pub potions: Vec<PotionKindKey>,
}

/// Check whether two floats a and b are within epsilon of each other.
fn nearly_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

fn create_binary_columns(pb: &mut RowProblem, objectives: &Vec<f64>) -> Vec<Col> {
    let num_columns = objectives.len();
    let mut columns = Vec::with_capacity(num_columns);
    for (_, objective) in (0..num_columns).zip(objectives) {
        let column = pb.add_integer_column(objective.clone(), 0..1);
        columns.push(column);
    }
    columns
}

fn create_ingredient_constraints(
    pb: &mut RowProblem,
    columns: &[Col],
    recipes: &[Recipe],
    available_ingredients: &IngredientCounts,
    utilisation: i32,
) {
    // No more than the available amount of each ingredient.
    // For each available ingredient.
    for (ingredient_key, ingredient_count) in available_ingredients.iter() {
        // Only allow up to the available quantity of the ingredient to be used.
        let upper_occurrances = ingredient_count * utilisation;

        // For each recipe that might use the ingredient.
        // If the recipe uses the ingredient
        // Add a coefficient indicating the reipe uses the ingredient.

        let factors: Vec<(_, f64)> = columns
            .iter()
            .zip(recipes.iter())
            .map(|(column, recipe)| {
                let contains_key = recipe
                    .ingredients
                    .iter()
                    .any(|ingredient| ingredient.key == *ingredient_key);

                (*column, if contains_key { 1. } else { 0. })
            })
            .collect();

        // Create a constraint for the ingredient.
        pb.add_row(0..upper_occurrances, factors);
    }
}

fn create_potion_kind_constraints(
    pb: &mut RowProblem,
    columns: &[Col],
    recipes: &[Recipe],
    potions: &[PotionKindKey],
) {
    // No more than one of each potion kind.
    for (potion_kind_key, _) in POTION_KINDS.iter() {
        let upper_bound = 1.;
        let lower_bound = if potions.contains(potion_kind_key) {
            1.
        } else {
            0.
        };

        let factors: Vec<(_, f64)> = columns
            .iter()
            .zip(recipes.iter())
            .map(|(column, recipe)| {
                (
                    *column,
                    if recipe.potion_kind_key == *potion_kind_key {
                        1.
                    } else {
                        0.
                    },
                )
            })
            .collect();

        // Create a constraint for the potion kind.
        pb.add_row(lower_bound..=upper_bound, factors);
    }
}

fn create_department_constraints(pb: &mut RowProblem, columns: &[Col], recipes: &[Recipe]) {
    let departments = [
        Department::Health,
        Department::Sourcery,
        Department::Provisions,
    ];

    for &department in departments.iter() {
        let upper_bound = 5.;
        let lower_bound = 1.;

        let factors: Vec<(_, f64)> = columns
            .iter()
            .zip(recipes.iter())
            .map(|(column, recipe)| {
                let potion_kind = POTION_KINDS.get_by_key(&recipe.potion_kind_key);
                (
                    *column,
                    if potion_kind.department == department {
                        1.
                    } else {
                        0.
                    },
                )
            })
            .collect();

        // Create a constraint for each department.
        pb.add_row(lower_bound..=upper_bound, factors);
    }
}

fn create_number_constraints(pb: &mut RowProblem, columns: &[Col], min_recipes: i32) {
    let factors: Vec<(Col, f64)> = columns.iter().map(|&column| (column, 1.)).collect();
    pb.add_row((min_recipes as f64).., factors);
}

fn create_appeal_constraints(
    pb: &mut RowProblem,
    columns: &[Col],
    possible_recipes: &[Recipe],
    min_appeal: i32,
) {
    let factors: Vec<(Col, f64)> = columns
        .iter()
        .zip(possible_recipes.iter())
        .map(|(&column, recipe)| (column, recipe.overall_appeal as f64))
        .collect();

    pb.add_row(min_appeal as f64.., factors);
}

fn create_appeal_objectives(possible_recipes: &[Recipe]) -> Vec<f64> {
    possible_recipes
        .iter()
        .map(|recipe| recipe.overall_appeal as f64)
        .collect_vec()
}

fn create_potency_objectives(possible_recipes: &[Recipe]) -> Vec<f64> {
    possible_recipes
        .iter()
        .map(|recipe| recipe.overall_potency as f64)
        .collect_vec()
}

fn maximise_recipes(
    possible_recipes: &Vec<Recipe>,
    available_ingredients: &IngredientCounts,
    utilisation: i32,
    potions: &[PotionKindKey],
) -> i32 {
    // TODO: Signal progress to the calling process.
    // println!("Maximising recipes.");

    // Create the problem.
    let mut pb = RowProblem::default();

    // Objective function: maximize the number of selected recipes
    let objectives = vec![1.0; possible_recipes.len()];

    // The columns: a binary variable for each recipe with coeffecient 1.0.
    let columns = create_binary_columns(&mut pb, &objectives);

    // The rows: constraints.
    create_ingredient_constraints(
        &mut pb,
        &columns,
        &possible_recipes,
        &available_ingredients,
        utilisation,
    );
    create_potion_kind_constraints(&mut pb, &columns, &possible_recipes, &potions);
    create_department_constraints(&mut pb, &columns, &possible_recipes);

    // Create a Highs model to be optimised.
    let model = pb.optimise(Sense::Maximise);

    // Solve the problem. Returns the solution
    let solved = model.solve();

    // Check the solver finished and solution is proven optimal.
    assert_eq!(solved.status(), HighsModelStatus::Optimal);

    let solution = solved.get_solution();

    let count = solution
        .columns()
        .iter()
        .filter(|&&value| nearly_equal(value, 1., 1e-6))
        .count();

    count as i32
}

fn maximise_appeal(
    possible_recipes: &Vec<Recipe>,
    available_ingredients: &IngredientCounts,
    utilisation: i32,
    potions: &[PotionKindKey],
    min_recipes: i32,
) -> i32 {
    // TODO: Signal progress to the calling process.
    // println!("Maximising appeal.");

    // Create the problem.
    let mut pb = RowProblem::default();

    // Objective function: maximize the combined appeal of all recipes
    let objectives = create_appeal_objectives(possible_recipes);

    // The columns: a binary variable for each recipe with coeffecient 1.0.
    let columns = create_binary_columns(&mut pb, &objectives);

    // The rows: constraints.
    create_ingredient_constraints(
        &mut pb,
        &columns,
        &possible_recipes,
        &available_ingredients,
        utilisation,
    );
    create_potion_kind_constraints(&mut pb, &columns, &possible_recipes, &potions);
    create_department_constraints(&mut pb, &columns, &possible_recipes);
    create_number_constraints(&mut pb, &columns, min_recipes);

    // Create a Highs model to be optimised.
    let model = pb.optimise(Sense::Maximise);

    // Solve the problem. Returns the solution
    let solved = model.solve();

    // Check the solver finished and solution is proven optimal.
    assert_eq!(solved.status(), HighsModelStatus::Optimal);

    let solution = solved.get_solution();

    let total_appeal: f64 = solution
        .columns()
        .iter()
        .zip(objectives)
        .filter(|(value, _)| nearly_equal(**value, 1., 1e-6))
        .map(|(_, appeal)| appeal)
        .sum();

    total_appeal as i32
}

fn maximise_potency(
    possible_recipes: &[Recipe],
    available_ingredients: &HashMap<IngredientKey, i32>,
    utilisation: i32,
    potions: &[PotionKindKey],
    min_recipes: i32,
    min_appeal: i32,
) -> Vec<Recipe> {
    // TODO: Signal progress to the calling process.
    // println!("Maximising appeal.");

    // Create the problem.
    let mut pb = RowProblem::default();

    // Objective function: maximize the combined potency of all recipes
    let objectives = create_potency_objectives(possible_recipes);

    // The columns: a binary variable for each recipe with coeffecient 1.0.
    let columns = create_binary_columns(&mut pb, &objectives);

    // The rows: constraints.
    create_ingredient_constraints(
        &mut pb,
        &columns,
        &possible_recipes,
        &available_ingredients,
        utilisation,
    );
    create_potion_kind_constraints(&mut pb, &columns, &possible_recipes, &potions);
    create_department_constraints(&mut pb, &columns, &possible_recipes);
    create_number_constraints(&mut pb, &columns, min_recipes);
    create_appeal_constraints(&mut pb, &columns, &possible_recipes, min_appeal);

    // Create a Highs model to be optimised.
    let model = pb.optimise(Sense::Maximise);

    // Solve the problem. Returns the solution
    let solved = model.solve();

    // Check the solver finished and solution is proven optimal.
    assert_eq!(solved.status(), HighsModelStatus::Optimal);

    let solution = solved.get_solution();

    let recipes: Vec<_> = solution
        .columns()
        .iter()
        .zip(possible_recipes)
        .filter(|(value, _)| nearly_equal(**value, 1., 1e-6))
        .map(|(_, recipe)| recipe.clone())
        .collect();

    recipes
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
