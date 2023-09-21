mod models;
mod permute;
mod simulate;
extern crate bincode;
extern crate serde;
extern crate sled;

use bincode::serialize;
use serde::{Deserialize, Serialize};
use sled::Db;

fn main() {
    let tree = sled::open("/tmp/welcome-to-sled").expect("open");

    let all_recipes = permute::get_all_recipes(
        models::INGREDIENTS_VALUES.as_slice(),
        vec!["cut", "ferment", "infuse"],
        5,
    );

    dbg!(all_recipes.len());

    // let mut batch = sled::Batch::default();

    // for (i, recipe) in all_recipes.iter().enumerate() {
    //     let key = format!("recipe_{}", i);
    //     let serialized_recipe = serialize(recipe).unwrap();
    //     // dbg!(serialized_recipe.len());
    //     batch.insert(key.as_bytes(), serialized_recipe);
    // }

    // tree.apply_batch(batch).unwrap();
}
