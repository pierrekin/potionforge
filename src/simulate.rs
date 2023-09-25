use crate::models::{
    self, AppealLookup, AppealMapNegative, AppealMapPositive, Element, GetByParts, Ingredient,
    IngredientPart, IngredientParts, IngredientProcess, MainEffect, OverallTaste, PotionKind,
    Recipe, Sweetness, Taste, TasteEffect, Tastiness, ToxicityEffect, ValidCombination,
};

pub fn process_cut(ingredient: &Ingredient) -> Option<Vec<Ingredient>> {
    match &ingredient.parts {
        IngredientParts::Raw(a, b, c, d) => Some(vec![
            Ingredient {
                key: ingredient.key.clone(),
                process: IngredientProcess::Crushed,
                kind: ingredient.kind.clone(),
                parts: IngredientParts::Crushed(b.clone(), c.clone()),
            },
            Ingredient {
                key: ingredient.key.clone(),
                process: IngredientProcess::Blanched,
                kind: ingredient.kind.clone(),
                parts: IngredientParts::Blanched(a.clone(), b.clone()),
            },
            Ingredient {
                key: ingredient.key.clone(),
                process: IngredientProcess::Dried,
                kind: ingredient.kind.clone(),
                parts: IngredientParts::Dried(a.clone(), c.clone()),
            },
            Ingredient {
                key: ingredient.key.clone(),
                process: IngredientProcess::Pickled,
                kind: ingredient.kind.clone(),
                parts: IngredientParts::Pickled(c.clone(), d.clone()),
            },
        ]),
        _ => None,
    }
}

pub fn process_ferment(ingredient: &Ingredient) -> Option<Ingredient> {
    let new_parts = match &ingredient.parts {
        IngredientParts::Raw(a, b, c, d) => IngredientParts::Fermented(
            if *a == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                a.clone()
            },
            if *b == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                b.clone()
            },
            if *c == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                c.clone()
            },
            if *d == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                d.clone()
            },
        ),
        IngredientParts::Crushed(a, b) => IngredientParts::CrushedFermented(
            if *a == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                a.clone()
            },
            if *b == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                b.clone()
            },
        ),
        IngredientParts::Blanched(a, b) => IngredientParts::BlanchedFermented(
            if *a == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                a.clone()
            },
            if *b == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                b.clone()
            },
        ),
        IngredientParts::Dried(a, b) => IngredientParts::DriedFermented(
            if *a == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                a.clone()
            },
            if *b == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                b.clone()
            },
        ),
        IngredientParts::Pickled(a, b) => IngredientParts::PickledFermented(
            if *a == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                a.clone()
            },
            if *b == IngredientPart::Impurity {
                IngredientPart::Stimulant
            } else {
                b.clone()
            },
        ),
        IngredientParts::Fermented(_, _, _, _) => return None, // Already fermented
        IngredientParts::Infused(_, _, _, _) => return None,   // Cannot ferment after infusion
        IngredientParts::CrushedFermented(_, _) => return None, // Already fermented
        IngredientParts::BlanchedFermented(_, _) => return None, // Already fermented
        IngredientParts::DriedFermented(_, _) => return None,  // Already fermented
        IngredientParts::PickledFermented(_, _) => return None, // Already fermented
        IngredientParts::CrushedInfused(_, _) => return None,  // Cannot ferment after infusion
        IngredientParts::BlanchedInfused(_, _) => return None, // Cannot ferment after infusion
        IngredientParts::DriedInfused(_, _) => return None,    // Cannot ferment after infusion
        IngredientParts::PickledInfused(_, _) => return None,  // Cannot ferment after infusion
        IngredientParts::FermentedInfused(_, _, _, _) => return None, // Already fermented
        IngredientParts::CrushedFermentedInfused(_, _) => return None, // Already fermented
        IngredientParts::BlanchedFermentedInfused(_, _) => return None, // Already fermented
        IngredientParts::DriedFermentedInfused(_, _) => return None, // Already fermented
        IngredientParts::PickledFermentedInfused(_, _) => return None, // Already fermented
    };

    let new_process = match ingredient.process {
        IngredientProcess::Raw => IngredientProcess::Fermented,
        IngredientProcess::Crushed => IngredientProcess::CrushedFermented,
        IngredientProcess::Blanched => IngredientProcess::BlanchedFermented,
        IngredientProcess::Dried => IngredientProcess::DriedFermented,
        IngredientProcess::Pickled => IngredientProcess::PickledFermented,
        _ => return None,
    };

    Some(Ingredient {
        key: ingredient.key.clone(),
        process: new_process,
        kind: ingredient.kind.clone(),
        parts: new_parts,
    })
}

pub fn process_infuse(ingredient: &Ingredient) -> Option<Ingredient> {
    let new_parts = match &ingredient.parts {
        IngredientParts::Raw(a, b, c, d) => IngredientParts::Infused(
            swap_elements(a.clone()),
            swap_elements(b.clone()),
            swap_elements(c.clone()),
            swap_elements(d.clone()),
        ),
        IngredientParts::Fermented(a, b, c, d) => IngredientParts::FermentedInfused(
            swap_elements(a.clone()),
            swap_elements(b.clone()),
            swap_elements(c.clone()),
            swap_elements(d.clone()),
        ),
        IngredientParts::CrushedFermented(a, b) => IngredientParts::CrushedFermentedInfused(
            swap_elements(a.clone()),
            swap_elements(b.clone()),
        ),
        IngredientParts::BlanchedFermented(a, b) => IngredientParts::BlanchedFermentedInfused(
            swap_elements(a.clone()),
            swap_elements(b.clone()),
        ),
        IngredientParts::DriedFermented(a, b) => IngredientParts::DriedFermentedInfused(
            swap_elements(a.clone()),
            swap_elements(b.clone()),
        ),

        IngredientParts::PickledFermented(a, b) => IngredientParts::PickledFermentedInfused(
            swap_elements(a.clone()),
            swap_elements(b.clone()),
        ),
        _ => return None,
    };

    let new_process = match ingredient.process {
        IngredientProcess::Raw => IngredientProcess::Infused,
        IngredientProcess::Fermented => IngredientProcess::FermentedInfused,
        IngredientProcess::CrushedFermented => IngredientProcess::CrushedFermentedInfused,
        IngredientProcess::BlanchedFermented => IngredientProcess::BlanchedFermentedInfused,
        IngredientProcess::DriedFermented => IngredientProcess::DriedFermentedInfused,
        IngredientProcess::PickledFermented => IngredientProcess::PickledFermentedInfused,
        _ => return None,
    };

    Some(Ingredient {
        key: ingredient.key.clone(),
        process: new_process,
        kind: ingredient.kind.clone(),
        parts: new_parts,
    })
}

fn swap_elements(part: IngredientPart) -> IngredientPart {
    match part {
        IngredientPart::Element(Element::Fire) => IngredientPart::Element(Element::Water),
        IngredientPart::Element(Element::Water) => IngredientPart::Element(Element::Fire),
        IngredientPart::Element(Element::Aether) => IngredientPart::Element(Element::Earth),
        IngredientPart::Element(Element::Earth) => IngredientPart::Element(Element::Aether),
        _ => part,
    }
}

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

fn determine_toxicity_appeal(potion_kind: &PotionKind, parts: &Vec<IngredientPart>) -> i32 {
    let toxicity: i32 = parts
        .iter()
        .map(|part| match part {
            IngredientPart::Toxin => 1,
            IngredientPart::Antitoxin => -1,
            _ => 0,
        })
        .sum();

    match potion_kind.toxicity_effect {
        ToxicityEffect::ToxicPositive => {
            if toxicity >= 2 {
                20
            } else if toxicity == 1 {
                10
            } else if toxicity == 0 {
                0
            } else if toxicity == -1 {
                -20
            } else if toxicity <= -2 {
                -50
            } else {
                unreachable!()
            }
        }
        ToxicityEffect::ToxicNegative => {
            if toxicity >= 2 {
                -50
            } else if toxicity == 1 {
                -20
            } else if toxicity == 0 {
                0
            } else if toxicity == -1 {
                10
            } else if toxicity <= -2 {
                20
            } else {
                unreachable!()
            }
        }
    }
}

fn determine_purity_appeal(parts: &Vec<IngredientPart>) -> i32 {
    parts
        .iter()
        .map(|part| match part {
            IngredientPart::Impurity => -10,
            _ => 0,
        })
        .sum()
}

fn determine_overall_appeal(
    potion_kind: &PotionKind,
    parts: &Vec<IngredientPart>,
    overall_taste: OverallTaste,
) -> i32 {
    determine_purity_appeal(&parts)
        + determine_taste_appeal(potion_kind, overall_taste)
        + determine_toxicity_appeal(potion_kind, parts)
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

    let overall_taste = determine_overall_taste(&parts);
    let overall_appeal = determine_overall_appeal(potion_kind, &parts, overall_taste);

    Some(Recipe {
        potion_kind: potion_kind.clone(),
        ingredients: ingredients.to_vec(),
        overall_taste: overall_taste,
        overall_appeal: overall_appeal,
    })
}

#[cfg(test)]
mod tests {
    use crate::models::{GetByKey, IngredientKey, PotionKindKey, INGREDIENTS};

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
            (
                vec![IngredientKey::Deadmans, IngredientKey::Elven],
                PotionKindKey::Silence,
            ),
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

            dbg!(&ingredient_keys, &expected_potion);

            let result = simulate(&ingredients);
            assert!(result.is_some());

            let recipe = result.unwrap();
            assert_eq!(recipe.potion_kind.key, expected_potion);
        }
    }
}
