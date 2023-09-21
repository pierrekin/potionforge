use crate::models::{
    self, Element, GetByParts, Ingredient, IngredientPart, IngredientParts, IngredientProcess,
    MainEffect, Recipe,
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

pub fn simulate(ingredients: &[Ingredient]) -> Option<Recipe> {
    let mut main_effect: Option<&MainEffect> = None;
    let mut element: Option<&Element> = None;
    let mut main_effect_count = 0;
    let mut element_count = 0;

    let parts = collect_parts(ingredients);
    for part in parts.iter() {
        match part {
            IngredientPart::MainEffect(me) => {
                main_effect_count += 1;
                main_effect = Some(me);
            }
            IngredientPart::Element(el) => {
                element_count += 1;
                element = Some(el);
            }
            _ => {}
        }
    }

    if main_effect_count != 1 || element_count != 1 {
        return None;
    }

    match (main_effect, element) {
        (Some(me), Some(el)) => {
            let potion_kind = models::POTION_KINDS.get_by_parts(me.clone(), el.clone());
            Some(Recipe {
                potion_kind: potion_kind.clone(),
                ingredients: ingredients.to_vec(),
            })
        }
        _ => None,
    }
}
