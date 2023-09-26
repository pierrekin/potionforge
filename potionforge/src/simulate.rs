use crate::models::{
    self, AppealLookup, AppealMapNegative, AppealMapPositive, Element, GetByParts, Ingredient,
    IngredientPart, IngredientParts, MainEffect, OverallPurity, OverallTaste,
    OverallToxicity, PotionKind, Recipe, Sweetness, Taste, TasteEffect, Tastiness, ToxicityEffect,
    ValidCombination,
};

pub fn collect_parts(ingredients: &[Ingredient]) -> Vec<IngredientPart> {
    let mut result = Vec::new();
    for ingredient in ingredients {
        match &ingredient.parts {
            IngredientParts::Raw(a, b, c, d)
            | IngredientParts::Fermented(a, b, c, d)
            | IngredientParts::Infused(a, b, c, d)
            | IngredientParts::FermentedInfused(a, b, c, d) => {
                result.push(a.clone());
                result.push(b.clone());
                result.push(c.clone());
                result.push(d.clone());
            }
            IngredientParts::Crushed(a, b)
            | IngredientParts::Blanched(a, b)
            | IngredientParts::Dried(a, b)
            | IngredientParts::Pickled(a, b)
            | IngredientParts::CrushedFermented(a, b)
            | IngredientParts::BlanchedFermented(a, b)
            | IngredientParts::DriedFermented(a, b)
            | IngredientParts::PickledFermented(a, b)
            | IngredientParts::CrushedInfused(a, b)
            | IngredientParts::BlanchedInfused(a, b)
            | IngredientParts::DriedInfused(a, b)
            | IngredientParts::PickledInfused(a, b)
            | IngredientParts::CrushedFermentedInfused(a, b)
            | IngredientParts::BlanchedFermentedInfused(a, b)
            | IngredientParts::DriedFermentedInfused(a, b)
            | IngredientParts::PickledFermentedInfused(a, b) => {
                result.push(a.clone());
                result.push(b.clone());
            }
        }
    }
    result
}

pub fn find_dominant_element(parts: &Vec<IngredientPart>) -> Option<Element> {
    let mut counts = [0; 4];

    for part in parts {
        if let IngredientPart::Element(element) = part {
            match element {
                Element::Fire => counts[0] += 1,
                Element::Water => counts[1] += 1,
                Element::Earth => counts[2] += 1,
                Element::Aether => counts[3] += 1,
            }
        }
    }

    counts[0] -= counts[1];
    counts[2] -= counts[3];

    let max_count = *counts.iter().max().unwrap();
    if counts.iter().filter(|&&x| x == max_count).count() > 1 {
        return None;
    }

    match counts.iter().position(|&x| x == max_count) {
        Some(0) => Some(Element::Fire),
        Some(1) => Some(Element::Water),
        Some(2) => Some(Element::Earth),
        Some(3) => Some(Element::Aether),
        _ => None,
    }
}

pub fn find_dominant_main_effect(parts: &[IngredientPart]) -> Option<MainEffect> {
    let mut counts = [0; 4];

    for part in parts {
        if let IngredientPart::MainEffect(effect) = part {
            match effect {
                MainEffect::Cat => counts[0] += 1,
                MainEffect::Bone => counts[1] += 1,
                MainEffect::Soul => counts[2] += 1,
                MainEffect::Beast => counts[3] += 1,
            }
        }
    }

    let max_count = *counts.iter().max().unwrap();
    if counts.iter().filter(|&&x| x == max_count).count() > 1 {
        return None;
    }

    match counts.iter().position(|&x| x == max_count) {
        Some(0) => Some(MainEffect::Cat),
        Some(1) => Some(MainEffect::Bone),
        Some(2) => Some(MainEffect::Soul),
        Some(3) => Some(MainEffect::Beast),
        _ => None,
    }
}

fn determine_overall_purity(parts: &[IngredientPart]) -> OverallPurity {
    for part in parts {
        match part {
            IngredientPart::Impurity => return OverallPurity::Impure,
            _ => {}
        }
    }
    OverallPurity::Neutral
}

pub fn determine_overall_taste(parts: &Vec<IngredientPart>) -> OverallTaste {
    let mut tastiness: i32 = 0;
    let mut sweetness: i32 = 0;

    for part in parts {
        match part {
            IngredientPart::Taste(Taste::Tastiness(Tastiness::Tasty)) => tastiness += 1,
            IngredientPart::Taste(Taste::Tastiness(Tastiness::Unsavory)) => tastiness -= 1,
            IngredientPart::Taste(Taste::Sweetness(Sweetness::Sweet)) => sweetness += 1,
            IngredientPart::Taste(Taste::Sweetness(Sweetness::Bitter)) => sweetness -= 1,
            _ => {}
        }
    }

    // Normalize
    tastiness = tastiness.signum();
    sweetness = sweetness.signum();

    // Map to Option<Taste> enum
    let final_tastiness: Option<Tastiness> = match tastiness {
        1 => Some(Tastiness::Tasty),
        -1 => Some(Tastiness::Unsavory),
        _ => None,
    };

    let final_sweetness: Option<Sweetness> = match sweetness {
        1 => Some(Sweetness::Sweet),
        -1 => Some(Sweetness::Bitter),
        _ => None,
    };

    // Determine overall taste
    match (final_tastiness, final_sweetness) {
        (Some(Tastiness::Tasty), None) => OverallTaste::Tasty,
        (Some(Tastiness::Tasty), Some(Sweetness::Bitter)) => OverallTaste::Flavorful,
        (None, Some(Sweetness::Bitter)) => OverallTaste::Bitter,
        (Some(Tastiness::Unsavory), Some(Sweetness::Bitter)) => OverallTaste::Foul,
        (Some(Tastiness::Unsavory), None) => OverallTaste::Unsavory,
        (Some(Tastiness::Unsavory), Some(Sweetness::Sweet)) => OverallTaste::Icky,
        (None, Some(Sweetness::Sweet)) => OverallTaste::Sweet,
        (Some(Tastiness::Tasty), Some(Sweetness::Sweet)) => OverallTaste::Delicious,
        (None, None) => OverallTaste::Bland,
    }
}

pub fn determine_taste_appeal(potion_kind: &PotionKind, overall_taste: OverallTaste) -> i32 {
    match potion_kind.taste_effect {
        TasteEffect::TastyNeutral => 0,
        TasteEffect::TastyPositive => AppealMapPositive.get_appeal(overall_taste) as i32,
        TasteEffect::TastyNegative => AppealMapNegative.get_appeal(overall_taste) as i32,
    }
}

fn determine_overall_toxicity(parts: &Vec<IngredientPart>) -> OverallToxicity {
    let toxicity: i32 = parts
        .iter()
        .map(|part| match part {
            IngredientPart::Toxin => 1,
            IngredientPart::Antitoxin => -1,
            _ => 0,
        })
        .sum();

    if toxicity >= 2 {
        OverallToxicity::VeryToxic
    } else if toxicity == 1 {
        OverallToxicity::Toxic
    } else if toxicity == 0 {
        OverallToxicity::Neutral
    } else if toxicity == -1 {
        OverallToxicity::Antitoxic
    } else if toxicity <= -2 {
        OverallToxicity::Veryantitoxic
    } else {
        unreachable!()
    }
}

fn determine_toxicity_appeal(potion_kind: &PotionKind, overall_toxicity: OverallToxicity) -> i32 {
    match potion_kind.toxicity_effect {
        ToxicityEffect::ToxicPositive => match overall_toxicity {
            OverallToxicity::VeryToxic => 20,
            OverallToxicity::Toxic => 10,
            OverallToxicity::Neutral => 0,
            OverallToxicity::Antitoxic => -20,
            OverallToxicity::Veryantitoxic => -50,
        },
        ToxicityEffect::ToxicNegative => match overall_toxicity {
            OverallToxicity::VeryToxic => -50,
            OverallToxicity::Toxic => -20,
            OverallToxicity::Neutral => 0,
            OverallToxicity::Antitoxic => 10,
            OverallToxicity::Veryantitoxic => 20,
        },
    }
}

fn determine_purity_appeal(overall_purity: OverallPurity) -> i32 {
    match overall_purity {
        OverallPurity::Impure => -10,
        OverallPurity::Neutral => 0,
    }
}

fn determine_overall_appeal(
    potion_kind: &PotionKind,
    overall_purity: OverallPurity,
    overall_taste: OverallTaste,
    overall_toxicity: OverallToxicity,
) -> i32 {
    determine_purity_appeal(overall_purity)
        + determine_taste_appeal(potion_kind, overall_taste)
        + determine_toxicity_appeal(potion_kind, overall_toxicity)
}

pub fn simulate(ingredients: &[Ingredient]) -> Option<Recipe> {
    let parts = collect_parts(ingredients);
    let element: Option<Element> = find_dominant_element(&parts);
    let main_effect: Option<MainEffect> = find_dominant_main_effect(&parts);

    if element.is_none() || main_effect.is_none() {
        return None;
    }

    // Safe to unwrap due to check above.
    let valid_combination = ValidCombination::new(main_effect.unwrap(), element.unwrap()).unwrap();
    let potion_kind = models::POTION_KINDS.get_by_parts(valid_combination);

    let overall_purity = determine_overall_purity(&parts);
    let overall_taste = determine_overall_taste(&parts);
    let overall_toxicity = determine_overall_toxicity(&parts);
    let overall_appeal =
        determine_overall_appeal(potion_kind, overall_purity, overall_taste, overall_toxicity);

    Some(Recipe {
        potion_kind_key: potion_kind.key.clone(),
        ingredients: ingredients.to_vec(),
        overall_purity: overall_purity,
        overall_taste: overall_taste,
        overall_toxicity: overall_toxicity,
        overall_appeal: overall_appeal,
    })
}

#[cfg(test)]
mod tests {
    use crate::models::{traits::GetByKey, IngredientKey, PotionKindKey, INGREDIENTS};

    use super::*;

    #[test]
    fn test_simulate() {
        let test_cases = vec![
            // Beast, Fire -> Vitality
            (
                vec![IngredientKey::Flyagaric, IngredientKey::Lupine],
                PotionKindKey::Vitality,
            ),
            // (vec![IngredientKey::Flyagaric, IngredientKey::ElvenCrushed], PotionKindKey::Sleep),
            // (vec![IngredientKey::Flyagaric, IngredientKey::WizardsHat], PotionKindKey::Summoning),
            (
                vec![IngredientKey::Flyagaric, IngredientKey::Deathcap],
                PotionKindKey::Monster,
            ),
            // Cat
            (
                vec![IngredientKey::Catnip, IngredientKey::Lupine],
                PotionKindKey::Speed,
            ),
            // (vec![IngredientKey::Catnip, IngredientKey::ElvenCrushed], PotionKindKey::Slow),
            // (vec![IngredientKey::Catnip, IngredientKey::WizardsHat], PotionKindKey::Mana),
            (
                vec![IngredientKey::Catnip, IngredientKey::Deathcap],
                PotionKindKey::Warding,
            ),
            // Bone
            (
                vec![IngredientKey::Anise, IngredientKey::Lupine],
                PotionKindKey::Strength,
            ),
            // (vec![IngredientKey::Anise, IngredientKey::ElvenCrushed], PotionKindKey::Weakness),
            // ( vec![IngredientKey::Anise, IngredientKey::Pluteus], PotionKindKey::Necromancy,),
            (
                vec![IngredientKey::Anise, IngredientKey::Deathcap],
                PotionKindKey::Skelleton,
            ),
            // Soul, Fire -> Speech
            (
                vec![IngredientKey::Deadmans, IngredientKey::Lupine],
                PotionKindKey::Speech,
            ),
            // ( vec![IngredientKey::Deadmans, IngredientKey::Elven], PotionKindKey::Silence,),
            // (vec![IngredientKey::Deadmans, IngredientKey::WizardsHat], PotionKindKey::Conjuring),
            (
                vec![IngredientKey::Deadmans, IngredientKey::Deathcap],
                PotionKindKey::Exorcism,
            ),
        ];

        for (ingredient_keys, expected_potion) in test_cases {
            let ingredients: Vec<Ingredient> = ingredient_keys
                .iter()
                .map(|&key| INGREDIENTS.get_by_key(&key).clone())
                .collect();

            let result = simulate(&ingredients);
            assert!(result.is_some());

            let recipe = result.unwrap();
            assert_eq!(recipe.potion_kind_key, expected_potion);
        }
    }
}
