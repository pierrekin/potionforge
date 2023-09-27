use models::{IngredientKey, Process};
use pyo3::{prelude::*, types::PyDict};
use recommend::IngredientCounts;

pub mod enumerate;
pub mod models;
pub mod process;
pub mod recommend;
pub mod simulate;

impl From<&str> for IngredientKey {
    fn from(ingredient: &str) -> IngredientKey {
        match ingredient {
            "catnip" => IngredientKey::Catnip,
            "lupine" => IngredientKey::Lupine,
            "mandrake" => IngredientKey::Mandrake,
            "nightshade" => IngredientKey::Nightshade,
            "sage" => IngredientKey::Sage,
            "thyme" => IngredientKey::Thyme,
            "wormwood" => IngredientKey::Wormwood,
            "anise" => IngredientKey::Anise,
            "deadmans" => IngredientKey::Deadmans,
            "deathcap" => IngredientKey::Deathcap,
            "elven" => IngredientKey::Elven,
            "flyagaric" => IngredientKey::Flyagaric,
            "pluteus" => IngredientKey::Pluteus,
            "wizards" => IngredientKey::Wizards,
            "asporeus" => IngredientKey::Asporeus,
            "stinkhorn" => IngredientKey::Stinkhorn,
            _ => unreachable!(),
        }
    }
}

// / Recommends potions that can be crafted with the provided configuration
// #[pyfunction]
// fn my_recommend(available_ingredients: &PyDict, arcane_power: i64) {
//     let mut rust_available_ingredients = IngredientCounts::new();

//     for (key, value) in available_ingredients.iter() {
//         let key: String = key.extract().unwrap();
//         let rust_key: IngredientKey = key.as_str().into();
//         let value: i32 = value.extract().unwrap();
//         rust_available_ingredients.insert(rust_key, value);
//     }

//     let processes = vec![Process::Blanch, Process::Crush];

//     let utilisation = 5;

//     let possible_recipes = enumerate::get_all_recipes(
//         &rust_available_ingredients
//             .iter()
//             .map(|(key, _)| key)
//             .collect(),
//         &processes,
//         arcane_power,
//     );

//     let _recommendations = recommend::recommend(
//         possible_recipes,
//         &rust_available_ingredients,
//         utilisation,
//         "0".to_string(),
//     );
// }

// /// A Python module implemented in Rust. The name of this function must match
// /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
// /// import the module.
// #[pymodule]
// fn potionforge(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pyfunction!(my_recommend))?;
//     Ok(())
// }
