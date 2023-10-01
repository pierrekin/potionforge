mod models;

use models::*;
use pyo3::prelude::*;

#[pyfunction]
fn get_ingredients() -> PyResult<Vec<Ingredient>> {
    let ingredients: Vec<_> = ::potionforge::models::INGREDIENTS
        .0
        .iter()
        .map(|(_, ingredient)| Ingredient::from(ingredient.clone()))
        .collect();

    Ok(ingredients)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn potionforge(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<IngredientKey>()?;
    m.add_function(wrap_pyfunction!(get_ingredients, m)?)?;
    Ok(())
}
