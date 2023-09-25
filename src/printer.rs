use crate::models::{Department, Recipe};

#[allow(dead_code)]
pub fn print_recipes(recipes: Vec<Recipe>) {
    for (i, recipe) in recipes.iter().enumerate() {
        println!("Recipe {}:", i + 1);
        println!("====================");
        println!(
            "Department: {}",
            match recipe.potion_kind.department {
                Department::Health => "Health",
                Department::Sourcery => "Sourcery",
                Department::Provisions => "Provisions",
            }
        );
        println!("Potion: {:?}", recipe.potion_kind.key);
        println!("Main Effect: {:?}", recipe.potion_kind.parts.0);
        println!("Element: {:?}", recipe.potion_kind.parts.1);
        println!("Ingredients:");
        for ingredient in &recipe.ingredients {
            println!(
                "  - {:?} ({})",
                ingredient.key,
                ingredient.process.to_human()
            );
        }
        println!("Overall Taste: {:?}", recipe.overall_taste);
        println!("Overall Appeal: {}", recipe.overall_appeal);
        println!();
    }
}

#[allow(dead_code)]
pub fn print_recipes_compact(recipes: Vec<Recipe>) {
    for recipe in recipes {
        let potion_kind = &recipe.potion_kind;
        let ingredients = &recipe.ingredients;
        let appeal = recipe.overall_appeal;

        print!(
            "Dept: {:?} | Kind: {:?} | Parts: {:?}, {:?} | Ingredients: [",
            potion_kind.department, potion_kind.key, potion_kind.parts.0, potion_kind.parts.1
        );

        for (i, ingredient) in ingredients.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{:?} ({}) ", ingredient.key, ingredient.process.to_human());
        }

        println!("] | Appeal: {}", appeal);
    }
}

#[allow(dead_code)]
pub fn print_recipes_semi_compact(recipes: Vec<Recipe>) {
    for (i, recipe) in recipes.iter().enumerate() {
        let dept = match recipe.potion_kind.department {
            Department::Health => "Health",
            Department::Sourcery => "Sourcery",
            Department::Provisions => "Provisions",
        };

        print!(
            "Recipe {}: Dept: {} | Kind: {:?} | Parts: {:?}, {:?} | Ingredients: [",
            i + 1,
            dept,
            recipe.potion_kind.key,
            recipe.potion_kind.parts.0,
            recipe.potion_kind.parts.1
        );

        for (j, ingredient) in recipe.ingredients.iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{:?} ({})", ingredient.key, ingredient.process.to_human());
        }

        println!("] | Appeal: {}", recipe.overall_appeal);
    }
}
