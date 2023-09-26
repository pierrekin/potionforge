extern crate coin_cbc;

use std::collections::HashMap;

use coin_cbc::{raw::Status, Col, Model, Sense};
use itertools::Itertools;

use crate::models::{traits::GetByKey, Department, IngredientKey, Recipe, POTION_KINDS};

pub type IngredientCounts = HashMap<IngredientKey, i32>;

fn create_binary_columns(model: &mut Model, num_columns: usize, objectives: Vec<f64>) -> Vec<Col> {
    let mut columns = Vec::with_capacity(num_columns);
    for (_, objective) in (0..num_columns).zip(objectives) {
        let column = model.add_binary();
        columns.push(column);
        model.set_obj_coeff(column, objective);
    }
    columns
}

fn create_ingredient_constraints(
    model: &mut Model,
    columns: &[Col],
    recipes: &[Recipe],
    available_ingredients: &IngredientCounts,
    utilisation: i32,
) {
    // No more than the available amount of each ingredient.
    // For each available ingredient.
    for (ingredient_key, ingredient_count) in available_ingredients.iter() {
        // Create a constraint for the ingredient.
        let ingredient_row = model.add_row();
        // Only allow up to the available quantity of the ingredient to be used.
        let upper_occurrances = ingredient_count * utilisation;
        model.set_row_upper(ingredient_row, upper_occurrances as f64);

        // For each recipe that might use the ingredient.
        for (column, recipe) in columns.iter().zip(recipes.iter()) {
            // If the recipe uses the ingredient
            if recipe
                .ingredients
                .iter()
                .map(|ingredient| ingredient.key)
                .contains(ingredient_key)
            {
                // Add a coefficient indicating the reipe uses the ingredient.
                model.set_weight(ingredient_row, *column, 1.);
            }
        }
    }
}

fn create_potion_kind_constraints(model: &mut Model, columns: &[Col], recipes: &[Recipe]) {
    // No more than one of each potion kind.
    for (potion_kind_key, _) in POTION_KINDS.iter() {
        let potion_kind_row = model.add_row();
        model.set_row_upper(potion_kind_row, 1.);

        for (column, recipe) in columns.iter().zip(recipes.iter()) {
            if recipe.potion_kind_key == *potion_kind_key {
                model.set_weight(potion_kind_row, *column, 1.);
            }
        }
    }
}

fn create_department_constraints(model: &mut Model, columns: &[Col], recipes: &[Recipe]) {
    // Constaints for each department.
    let health_row = model.add_row();
    let sourcery_row = model.add_row();
    let provisions_row = model.add_row();

    // No more than specified recipes per department.
    model.set_row_upper(health_row, 5.);
    model.set_row_upper(sourcery_row, 5.);
    model.set_row_upper(provisions_row, 5.);

    // No fewer than specified recipes per department.
    model.set_row_lower(health_row, 1.);
    model.set_row_lower(sourcery_row, 1.);
    model.set_row_lower(provisions_row, 1.);

    for (column, recipe) in columns.iter().zip(recipes.iter()) {
        let potion_kind = POTION_KINDS.get_by_key(&recipe.potion_kind_key);

        if potion_kind.department == Department::Health {
            model.set_weight(health_row, *column, 1.);
        } else if potion_kind.department == Department::Sourcery {
            model.set_weight(sourcery_row, *column, 1.);
        } else if potion_kind.department == Department::Provisions {
            model.set_weight(provisions_row, *column, 1.);
        } else {
            unreachable!();
        }
    }
}

fn maximise_recipes(
    possible_recipes: &Vec<Recipe>,
    available_ingredients: &IngredientCounts,
    utilisation: i32,
    cbc_loglevel: &str,
) -> i32 {
    // TODO: Signal progress to the calling process.
    // println!("Maximising recipes.");

    // Create the problem.
    let mut model = Model::default();
    model.set_parameter("logLevel", &cbc_loglevel);

    // Set objective sense.
    model.set_obj_sense(Sense::Maximize);

    // Objective function: maximize the number of selected recipes
    let objectives = vec![1.0; possible_recipes.len()];

    // The columns: a binary variable for each recipe with coeffecient 1.0.
    let columns = create_binary_columns(&mut model, possible_recipes.len(), objectives);

    // The rows: constraints.
    create_ingredient_constraints(
        &mut model,
        &columns,
        &possible_recipes,
        &available_ingredients,
        utilisation,
    );
    create_potion_kind_constraints(&mut model, &columns, &possible_recipes);
    create_department_constraints(&mut model, &columns, &possible_recipes);

    // Solve the problem. Returns the solution
    let solution = model.solve();
    let raw_solution = solution.raw().to_owned();

    // Check the solver finished and solution is proven optimal.
    assert_eq!(Status::Finished, raw_solution.status());
    assert!(raw_solution.is_proven_optimal());

    solution.raw().obj_value() as i32
}

fn create_appeal_objectives(possible_recipes: &[Recipe]) -> Vec<f64> {
    possible_recipes
        .iter()
        .map(|recipe| recipe.overall_appeal as f64)
        .collect_vec()
}

fn create_number_constraints(model: &mut Model, columns: &[Col], min_recipes: i32) {
    // At least min_recipes number of recipes
    let recipes_row = model.add_row();
    model.set_row_lower(recipes_row, min_recipes as f64);

    for column in columns.iter() {
        model.set_weight(recipes_row, *column, 1.);
    }
}

fn maximise_appeal(
    possible_recipes: &Vec<Recipe>,
    available_ingredients: &IngredientCounts,
    min_recipes: i32,
    utilisation: i32,
    cbc_loglevel: &str,
) -> Vec<Recipe> {
    // TODO: Signal progress to the calling process.
    // println!("Maximising appeal.");

    // Create the problem.
    let mut model = Model::default();
    model.set_parameter("logLevel", &cbc_loglevel);

    // Set objective sense.
    model.set_obj_sense(Sense::Maximize);

    // Objective function: maximize the combined appeal of all recipes
    let objectives = create_appeal_objectives(possible_recipes);

    // The columns: a binary variable for each recipe with coeffecient 1.0.
    let columns = create_binary_columns(&mut model, possible_recipes.len(), objectives);

    // The rows: constraints.
    create_ingredient_constraints(
        &mut model,
        &columns,
        &possible_recipes,
        &available_ingredients,
        utilisation,
    );
    create_potion_kind_constraints(&mut model, &columns, &possible_recipes);
    create_department_constraints(&mut model, &columns, &possible_recipes);
    create_number_constraints(&mut model, &columns, min_recipes);

    // Solve the problem. Returns the solution
    let solution = model.solve();
    let raw_solution = solution.raw().to_owned();

    // Check the solver finished and solution is proven optimal.
    assert_eq!(Status::Finished, raw_solution.status());
    assert!(raw_solution.is_proven_optimal());

    columns
        .iter()
        .zip(possible_recipes.iter())
        .filter(|(column, _)| solution.col(**column) == 1.0)
        .map(|(_, recipe)| recipe.clone())
        .collect()
}

pub fn recommend(
    possible_recipes: Vec<Recipe>,
    available_ingredients: &IngredientCounts,
    utilisation: i32,
    cbc_loglevel: String,
) -> Vec<Recipe> {
    let num_recipes = maximise_recipes(
        &possible_recipes,
        available_ingredients,
        utilisation,
        &cbc_loglevel,
    );
    maximise_appeal(
        &possible_recipes,
        available_ingredients,
        num_recipes,
        utilisation,
        &cbc_loglevel,
    )
}
