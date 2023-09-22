mod models;
mod permute;
mod recommend;
mod simulate;

fn main() {
    let all_recipes = permute::get_all_recipes(
        models::INGREDIENTS_VALUES.as_slice(),
        vec!["cut", "ferment", "infuse"],
        4,
    );

    let _all_recommendations = recommend::recommend(all_recipes);
}
