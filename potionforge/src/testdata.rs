use crate::models::{traits::GetByKey, Ingredient, IngredientKey, PotionKindKey, INGREDIENTS};

lazy_static::lazy_static! {
    pub static ref INGREDIENT_COMBINATIONS: Vec<(Vec<Ingredient>, PotionKindKey)> = vec![
        // Beast + Fire = Vitality
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Flyagaric).clone(), INGREDIENTS.get_by_key(&IngredientKey::Lupine).clone()],
            PotionKindKey::Vitality,
        ),
        // Beast + Water = Sleep
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Flyagaric).clone(), INGREDIENTS.get_by_key(&IngredientKey::Sage).clone()],
            PotionKindKey::Sleep,
        ),
        // Beast + Aether = Sumonning
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Flyagaric).clone(), INGREDIENTS.get_by_key(&IngredientKey::Wizards).clone()],
            PotionKindKey::Summoning,
        ),
        // Beast + Earth = Monster
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Flyagaric).clone(), INGREDIENTS.get_by_key(&IngredientKey::Deathcap).clone()],
            PotionKindKey::Monster,
        ),
        // Soul + Fire = Speech
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Deadmans).clone(), INGREDIENTS.get_by_key(&IngredientKey::Lupine).clone()],
            PotionKindKey::Speech,
        ),
        // Soul + Water = Silence
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Deadmans).clone(), INGREDIENTS.get_by_key(&IngredientKey::Sage).clone()],
            PotionKindKey::Silence,
        ),
        // Soul + Aether = Conjuring
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Deadmans).clone(), INGREDIENTS.get_by_key(&IngredientKey::Wizards).clone()],
            PotionKindKey::Conjuring,
        ),
        // Soul + Earth = Exorcism
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Deadmans).clone(), INGREDIENTS.get_by_key(&IngredientKey::Deathcap).clone()],
            PotionKindKey::Exorcism,
        ),
        // Bone + Fire = Strength
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Anise).clone(), INGREDIENTS.get_by_key(&IngredientKey::Lupine).clone()],
            PotionKindKey::Strength,
        ),
        // Bone + Water = Weakness
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Anise).clone(), INGREDIENTS.get_by_key(&IngredientKey::Sage).clone()],
            PotionKindKey::Weakness,
        ),
        // Bone + Aether = Necromancy
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Anise).clone(), INGREDIENTS.get_by_key(&IngredientKey::Wizards).clone()],
            PotionKindKey::Necromancy,
        ),
        // Bone + Earth = Skeleton
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Anise).clone(), INGREDIENTS.get_by_key(&IngredientKey::Deathcap).clone()],
            PotionKindKey::Skeleton,
        ),
        // Cat + Fire = Speed
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Catnip).clone(), INGREDIENTS.get_by_key(&IngredientKey::Lupine).clone()],
            PotionKindKey::Speed,
        ),
        // Cat + Water = Slow
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Catnip).clone(), INGREDIENTS.get_by_key(&IngredientKey::Sage).clone()],
            PotionKindKey::Slow,
        ),
        // Cat + Aether = Mana
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Catnip).clone(), INGREDIENTS.get_by_key(&IngredientKey::Wizards).clone()],
            PotionKindKey::Mana,
        ),
        // Cat + Earth = Warding
        (
            vec![INGREDIENTS.get_by_key(&IngredientKey::Catnip).clone(), INGREDIENTS.get_by_key(&IngredientKey::Deathcap).clone()],
            PotionKindKey::Warding,
        ),
    ];
}
