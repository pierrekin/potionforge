use crate::{
    models::{
        self,
        AlchemistAttribute::{self, Acclaimed},
        AppealLookup, AppealMapNegative, AppealMapPositive,
        BrandingCategory::{Bulk, Health, Provisions, Sourcery},
        Department, Element, GetByParts, Ingredient, IngredientKind, IngredientPart,
        IngredientParts, MainEffect, MarketCondition, OverallPurity, OverallTaste, OverallToxicity,
        PotionKind, Recipe, Sweetness, Taste, TasteEffect, Tastiness, ToxicityEffect,
        ValidCombination,
    },
    recommend::{AlchemistAttributes, BrandingCounts, MarketConditions},
};

#[derive(Debug)]
pub struct SimulateConfig {
    pub alchemists_attributes: AlchemistAttributes,
    pub market_conditions: MarketConditions,
    pub branding_counts: BrandingCounts,
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
    let mut initial_counts = [0; 4];

    for part in parts {
        if let IngredientPart::Element(element) = part {
            match element {
                Element::Fire => initial_counts[0] += 1,
                Element::Water => initial_counts[1] += 1,
                Element::Earth => initial_counts[2] += 1,
                Element::Aether => initial_counts[3] += 1,
            }
        }
    }

    let initial_total: i32 = initial_counts.iter().sum();

    let final_counts = [
        initial_counts[0] - initial_counts[1],
        initial_counts[1] - initial_counts[0],
        initial_counts[2] - initial_counts[3],
        initial_counts[3] - initial_counts[2],
    ];

    match final_counts
        .iter()
        .position(|&count| count > initial_total / 2)
    {
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

fn determine_purity_potency(parts: &Vec<IngredientPart>) -> i32 {
    parts
        .iter()
        .map(|part| match part {
            IngredientPart::Stimulant => 50,
            IngredientPart::Impurity => -50,
            _ => 0,
        })
        .sum()
}

fn determine_toxicity_potency(potion_kind: &PotionKind, parts: &Vec<IngredientPart>) -> i32 {
    parts
        .iter()
        .map(|part| match part {
            IngredientPart::Toxin => match potion_kind.toxicity_effect {
                ToxicityEffect::ToxicPositive => 20,
                ToxicityEffect::ToxicNegative => -20,
            },
            _ => 0,
        })
        .sum()
}

fn determine_main_effect_potency(main_effect: &MainEffect, parts: &Vec<IngredientPart>) -> i32 {
    parts
        .iter()
        .map(|part| match part {
            IngredientPart::MainEffect(part_main_effect) if part_main_effect == main_effect => 50,
            _ => 0,
        })
        .sum()
}

fn determine_element_potency(element: &Element, parts: &Vec<IngredientPart>) -> i32 {
    parts
        .iter()
        .map(|part| match part {
            IngredientPart::Element(part_element) if part_element == element => 50,
            _ => 0,
        })
        .sum()
}

fn determine_branding_appeal(
    potion_kind: &PotionKind,
    potency: i32,
    branding_counts: &BrandingCounts,
) -> i32 {
    // Bulk appeal only applies if the potion is offered with bulk pricing.
    // As a rule of thumb, bulk pricing only makes sense if potency is below
    // around 400.
    let bulk_appeal: i32 = if potency < 400 {
        *branding_counts.get(&Bulk).unwrap_or(&0)
    } else {
        0
    };

    let department_appeal = match potion_kind.department {
        Department::Health => branding_counts.get(&Health),
        Department::Sourcery => branding_counts.get(&Sourcery),
        Department::Provisions => branding_counts.get(&Provisions),
    }
    .unwrap_or(&0);

    bulk_appeal + department_appeal
}

fn determine_market_appeal(potion_kind: &PotionKind, market_conditions: &MarketConditions) -> i32 {
    let market_condition = market_conditions.get(&potion_kind.key);

    if market_condition.is_none() {
        return 0;
    }

    market_condition
        .unwrap()
        .iter()
        .map(|effect| match effect {
            MarketCondition::HighDemand => 20,
            MarketCondition::InDemand => 15,
            MarketCondition::LowDemand => 10,
            MarketCondition::Trendy => 10,
        })
        .sum()
}

fn determine_alchemist_appeal(attribute_counts: &AlchemistAttributes) -> i32 {
    attribute_counts.get(&Acclaimed).unwrap_or(&0) * 3
}

fn determine_alchemist_potency(
    alchemists_attributes: &AlchemistAttributes,
    ingredients: &[Ingredient],
    parts: &[IngredientPart],
) -> i32 {
    let mut stimulant_count: i32 = 0;
    let mut herb_count: i32 = 0;
    let mut mushroom_count: i32 = 0;

    for ingredient in ingredients {
        match ingredient.kind {
            IngredientKind::Herb => herb_count += 1,
            IngredientKind::Mushroom => mushroom_count += 1,
        }
    }

    for part in parts {
        match part {
            IngredientPart::Stimulant => stimulant_count += 1,
            _ => {}
        }
    }

    let optimiser_count = *alchemists_attributes
        .get(&AlchemistAttribute::Optimiser)
        .unwrap_or(&0);
    let herbalist_count = *alchemists_attributes
        .get(&AlchemistAttribute::Herbalist)
        .unwrap_or(&0);
    let fungi_connoisseur_count = *alchemists_attributes
        .get(&AlchemistAttribute::FungiConnoisseur)
        .unwrap_or(&0);

    (stimulant_count * optimiser_count * 10)
        + (herb_count.signum() * herbalist_count * 10)
        + (mushroom_count.signum() * fungi_connoisseur_count * 10)
}

pub fn simulate(ingredients: &[Ingredient], simulate_config: &SimulateConfig) -> Option<Recipe> {
    let parts = collect_parts(ingredients);
    let element: Option<Element> = find_dominant_element(&parts);
    let main_effect: Option<MainEffect> = find_dominant_main_effect(&parts);

    if element.is_none() || main_effect.is_none() {
        return None;
    }

    let element = element.unwrap();
    let main_effect = main_effect.unwrap();

    // Safe to unwrap due to check above.
    let valid_combination = ValidCombination::new(main_effect, element).unwrap();
    let potion_kind = models::POTION_KINDS.get_by_parts(valid_combination);

    let overall_purity = determine_overall_purity(&parts);
    let overall_taste = determine_overall_taste(&parts);
    let overall_toxicity = determine_overall_toxicity(&parts);

    let purity_potency = determine_purity_potency(&parts);
    let toxicity_potency = determine_toxicity_potency(potion_kind, &parts);
    let element_potency = determine_element_potency(&element, &parts);
    let main_effect_potency = determine_main_effect_potency(&main_effect, &parts);
    let alchemist_potency =
        determine_alchemist_potency(&simulate_config.alchemists_attributes, &ingredients, &parts);
    let overall_potency = purity_potency
        + toxicity_potency
        + element_potency
        + main_effect_potency
        + alchemist_potency;

    let purity_appeal = determine_purity_appeal(overall_purity);
    let taste_appeal = determine_taste_appeal(potion_kind, overall_taste);
    let toxicity_appeal = determine_toxicity_appeal(potion_kind, overall_toxicity);
    let market_appeal = determine_market_appeal(potion_kind, &simulate_config.market_conditions);
    let alchemist_appeal = determine_alchemist_appeal(&simulate_config.alchemists_attributes);
    let branding_appeal = determine_branding_appeal(
        potion_kind,
        overall_potency,
        &simulate_config.branding_counts,
    );
    let overall_appeal = purity_appeal
        + taste_appeal
        + toxicity_appeal
        + market_appeal
        + alchemist_appeal
        + branding_appeal;

    Some(Recipe {
        potion_kind_key: potion_kind.key.clone(),
        ingredients: ingredients.to_vec(),
        overall_purity: overall_purity,
        overall_taste: overall_taste,
        overall_toxicity: overall_toxicity,
        overall_appeal: overall_appeal,
        overall_potency: overall_potency,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{traits::GetByKey, IngredientKey, PotionKindKey, INGREDIENTS},
        recommend::{AlchemistAttributes, MarketConditions},
    };

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
                PotionKindKey::Skeleton,
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

            let simulate_config = SimulateConfig {
                alchemists_attributes: AlchemistAttributes::new(),
                market_conditions: MarketConditions::new(),
                branding_counts: BrandingCounts::new(),
            };
            let result = simulate(&ingredients, &simulate_config);
            assert!(result.is_some());

            let recipe = result.unwrap();
            assert_eq!(recipe.potion_kind_key, expected_potion);
        }
    }
}
