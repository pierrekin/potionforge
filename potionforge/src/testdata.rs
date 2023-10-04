use crate::models::{traits::GetByKey, Ingredient, IngredientKey, PotionKindKey, INGREDIENTS};

lazy_static::lazy_static! {
    pub static ref INGREDIENT_COMBINATIONS: Vec<(Vec<Ingredient>, PotionKindKey)> = vec![
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Flyagaric).clone(), INGREDIENTS.get_by_key(&IngredientKey::Lupine).clone()],
            PotionKindKey::Vitality,
        ),
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Flyagaric).clone(), INGREDIENTS.get_by_key(&IngredientKey::Deathcap).clone()],
            PotionKindKey::Monster,
        ),
    ];
}
