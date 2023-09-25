extern crate coin_cbc;

use coin_cbc::{raw::Status, Col, Model, Sense};
use itertools::Itertools;

use crate::models::{Department, GetByKey, IngredientCounts, Recipe, INGREDIENTS};

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
) {
    // No more than the available amount of each ingredient.
    for (ingredient_key, ingredient_count) in available_ingredients.iter() {
        let ingredient_row = model.add_row();
        model.set_row_upper(ingredient_row, *ingredient_count as f64);
        for (column, recipe) in columns.iter().zip(recipes.iter()) {
            let ingredient = INGREDIENTS.get_by_key(ingredient_key);
            if recipe.ingredients.contains(ingredient) {
                model.set_weight(ingredient_row, *column, 1.);
            }
        }
    }
}

fn create_department_constraints(model: &mut Model, columns: &[Col], recipes: &[Recipe]) {
    // No more than 5 Health recipes.
    let health_row = model.add_row();
    model.set_row_upper(health_row, 5.);
    for (column, recipe) in columns.iter().zip(recipes.iter()) {
        if recipe.potion_kind.department == Department::Health {
            model.set_weight(health_row, *column, 1.);
        }
    }

    // No more than 5 Sourcery recipes.
    let sourcery_row = model.add_row();
    model.set_row_upper(sourcery_row, 5.);
    for (column, recipe) in columns.iter().zip(recipes.iter()) {
        if recipe.potion_kind.department == Department::Sourcery {
            model.set_weight(sourcery_row, *column, 1.);
        }
    }

    // No more than 5 Provisions recipes.
    let provisions_row = model.add_row();
    model.set_row_upper(provisions_row, 5.);
    for (column, recipe) in columns.iter().zip(recipes.iter()) {
        if recipe.potion_kind.department == Department::Provisions {
            model.set_weight(provisions_row, *column, 1.);
        }
    }
}

fn maximise_recipes(
    possible_recipes: &Vec<Recipe>,
    available_ingredients: &IngredientCounts,
) -> i32 {
    // Create the problem.
    let mut model = Model::default();
    model.set_parameter("logLevel", "0");

    // Set objective sense.
    model.set_obj_sense(Sense::Maximize);

    // Objective function: maximize the number of selected recipes
    let objectives = vec![1.0; available_ingredients.len()];

    // The columns: a binary variable for each recipe with coeffecient 1.0.
    let columns = create_binary_columns(&mut model, possible_recipes.len(), objectives);

    // The rows: constraints.
    create_ingredient_constraints(
        &mut model,
        &columns,
        &possible_recipes,
        &available_ingredients,
    );
    create_department_constraints(&mut model, &columns, &possible_recipes);

    // Solve the problem. Returns the solution
    let solution = model.solve();

    // Check the solver finished.
    assert_eq!(Status::Finished, solution.raw().status());

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
) -> Vec<Recipe> {
    // Create the problem.
    let mut model = Model::default();
    model.set_parameter("logLevel", "0");

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
    );
    create_department_constraints(&mut model, &columns, &possible_recipes);
    create_number_constraints(&mut model, &columns, min_recipes);

    // Solve the problem. Returns the solution
    let solution = model.solve();

    // Check the solver finished.
    assert_eq!(Status::Finished, solution.raw().status());

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
) -> Vec<Recipe> {
    // let num_recipes = maximise_recipes(&possible_recipes, available_ingredients);
    let num_recipes = 0;
    maximise_appeal(&possible_recipes, available_ingredients, num_recipes)
}
