pub mod ingredients;
pub mod potion;

pub use ingredients::*;
pub use potion::*;

#[cfg(test)]
mod tests {
    use crate::models::{
        ingredients::IngredientKey,
        potion::{GetByKey, GetName},
    };

    use super::potion::{INGREDIENTS, POTION_KINDS};

    #[test]
    fn test_get_potion_kind_by_key() {
        let potion_kinds = &POTION_KINDS;
        for i in 0..16 {
            let key = &potion_kinds[i].0;
            let expected = &potion_kinds[i].1;

            assert_eq!(potion_kinds.get_by_key(key), expected);
        }
    }

    #[test]
    fn test_get_ingredient_by_key() {
        let ingredients = &INGREDIENTS;
        for i in 0..14 {
            let key = &ingredients[i].0;
            let expected = &ingredients[i].1;

            assert_eq!(ingredients.get_by_key(key), expected);
        }
    }

    #[test]
    fn test_get_name() {
        let ingredient = INGREDIENTS.get_by_key(&IngredientKey::Sage);
        assert_eq!(ingredient.name(), "Sage");

        let ingredient = INGREDIENTS.get_by_key(&IngredientKey::Catnip);
        assert_eq!(ingredient.name(), "Catnip");

        let ingredient = INGREDIENTS.get_by_key(&IngredientKey::Elven);
        assert_eq!(ingredient.name(), "Elven Saddle");
    }
}
