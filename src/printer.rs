use crate::models::{
    Ingredient, PotionKind, Recipe, DEPARTMENT_NAMES_MAP, RAW_INGREDIENT_NAMES_MAP,
};

pub fn print_cheatsheet(title: &str, mut recipes: Vec<Recipe>) {
    println!("--- Potions ({}) ---", title);

    fn ingredient_summary(ingredient: &Ingredient) -> String {
        let process_summary = ingredient.process.join(", ");
        format!(
            "{} ({})",
            RAW_INGREDIENT_NAMES_MAP
                .get(&ingredient.key)
                .unwrap_or(&"".to_string()),
            process_summary
        )
    }

    fn get_key(recipe: &Recipe) -> String {
        format!(
            "{}.{}.{}.{}",
            recipe.potion_kind.department, recipe.appeal, recipe.potency, recipe.potion_kind.name
        )
    }

    recipes.sort_by(|a, b| get_key(b).cmp(&get_key(a)));

    for recipe in recipes {
        let ingredients_summary: Vec<String> = recipe
            .ingredients
            .iter()
            .map(|ingredient| ingredient_summary(ingredient))
            .collect();
        let department_name = DEPARTMENT_NAMES_MAP
            .get(recipe.potion_kind.department.as_str())
            .unwrap_or(&"");
        let potion_name = &recipe.potion_kind.name;
        let recipe_cost = &recipe.cost;
        println!(
            "{}:  {} [{:?} / {:?}] {}",
            department_name,
            potion_name,
            recipe.potency,
            recipe_cost,
            ingredients_summary.join(", ")
        );
    }
}
