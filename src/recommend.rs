extern crate coin_cbc;

use coin_cbc::{raw::Status, Col, Model, Row, Sense};

use crate::models::{Department, Recipe};

fn create_binary_columns(model: &mut Model, num_columns: usize, coefficient: f64) -> Vec<Col> {
    let mut columns = Vec::with_capacity(num_columns);
    for _ in 0..num_columns {
        let column = model.add_binary();
        model.set_obj_coeff(column, coefficient);
        columns.push(column);
    }
    columns
}

fn create_recipe_rows(model: &mut Model, columns: &[Col], all_recipes: &[Recipe]) -> [Row; 3] {
    // No more than 5 Health recipes.
    let mut health_row = model.add_row();
    model.set_row_upper(health_row, 5.);
    for (column, recipe) in columns.iter().zip(all_recipes.iter()) {
        if (recipe.potion_kind.department == Department::Health) {
            model.set_weight(health_row, *column, 1.);
        }
    }

    // No more than 5 Sourcery recipes.
    let mut sourcery_row = model.add_row();
    model.set_row_upper(sourcery_row, 5.);
    for (column, recipe) in columns.iter().zip(all_recipes.iter()) {
        if (recipe.potion_kind.department == Department::Sourcery) {
            model.set_weight(sourcery_row, *column, 1.);
        }
    }

    // No more than 5 Provisions recipes.
    let mut provisions_row = model.add_row();
    model.set_row_upper(provisions_row, 5.);
    for (column, recipe) in columns.iter().zip(all_recipes.iter()) {
        if (recipe.potion_kind.department == Department::Provisions) {
            model.set_weight(provisions_row, *column, 1.);
        }
    }
    [health_row, sourcery_row, provisions_row]
}

pub fn recommend(all_recipes: Vec<Recipe>) -> () {
    // Create the problem.
    let mut model = Model::default();

    // Set objective sense.
    model.set_obj_sense(Sense::Maximize);

    // Objective function: maximize the number of selected recipes
    let mut obj_coeffs = vec![1.0; all_recipes.len()];

    // The columns: a binary variable for each recipe with coeffecient 1.0.
    let mut cols = create_binary_columns(&mut model, all_recipes.len(), 1.);

    // The rows: constraints.
    let mut rows = create_recipe_rows(&mut model, &cols, &all_recipes);
    // Solve the problem. Returns the solution
    let sol = model.solve();

    // Check the result
    assert_eq!(Status::Finished, sol.raw().status());

    let num_recommendations = sol.raw().obj_value();
    dbg!(num_recommendations);
}
