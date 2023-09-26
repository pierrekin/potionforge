use crate::models::{Element, Ingredient, IngredientPart, IngredientParts, IngredientProcess};

pub fn process_crush(ingredient: &Ingredient) -> Option<Ingredient> {
    match &ingredient.parts {
        IngredientParts::Raw(_, b, c, _) => Some(Ingredient {
            key: ingredient.key.clone(),
            process: IngredientProcess::Crushed,
            kind: ingredient.kind.clone(),
            parts: IngredientParts::Crushed(b.clone(), c.clone()),
        }),
        _ => None,
    }
}

pub fn process_blanch(ingredient: &Ingredient) -> Option<Ingredient> {
    match &ingredient.parts {
        IngredientParts::Raw(a, b, _, _) => Some(Ingredient {
            key: ingredient.key.clone(),
            process: IngredientProcess::Blanched,
            kind: ingredient.kind.clone(),
            parts: IngredientParts::Blanched(a.clone(), b.clone()),
        }),
        _ => None,
    }
}

pub fn process_dry(ingredient: &Ingredient) -> Option<Ingredient> {
    match &ingredient.parts {
        IngredientParts::Raw(a, _, _, d) => Some(Ingredient {
            key: ingredient.key.clone(),
            process: IngredientProcess::Dried,
            kind: ingredient.kind.clone(),
            parts: IngredientParts::Dried(a.clone(), d.clone()),
        }),
        _ => None,
    }
}

pub fn process_pickle(ingredient: &Ingredient) -> Option<Ingredient> {
    match &ingredient.parts {
        IngredientParts::Raw(_, _, c, d) => Some(Ingredient {
            key: ingredient.key.clone(),
            process: IngredientProcess::Pickled,
            kind: ingredient.kind.clone(),
            parts: IngredientParts::Pickled(c.clone(), d.clone()),
        }),
        _ => None,
    }
}

pub fn process_ferment(ingredient: &Ingredient) -> Option<Ingredient> {
    let has_impurity = match &ingredient.parts {
        IngredientParts::Raw(a, b, c, d)
        | IngredientParts::Fermented(a, b, c, d)
        | IngredientParts::Infused(a, b, c, d)
        | IngredientParts::FermentedInfused(a, b, c, d) => {
            *a == IngredientPart::Impurity
                || *b == IngredientPart::Impurity
                || *c == IngredientPart::Impurity
                || *d == IngredientPart::Impurity
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
            *a == IngredientPart::Impurity || *b == IngredientPart::Impurity
        }
    };

    if !has_impurity {
        return None;
    }

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
        IngredientParts::Crushed(a, b) => {
            IngredientParts::CrushedInfused(swap_elements(a.clone()), swap_elements(b.clone()))
        }
        IngredientParts::Blanched(a, b) => {
            IngredientParts::BlanchedInfused(swap_elements(a.clone()), swap_elements(b.clone()))
        }
        IngredientParts::Dried(a, b) => {
            IngredientParts::DriedInfused(swap_elements(a.clone()), swap_elements(b.clone()))
        }
        IngredientParts::Pickled(a, b) => {
            IngredientParts::PickledInfused(swap_elements(a.clone()), swap_elements(b.clone()))
        }
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

        IngredientParts::Infused(_, _, _, _) => return None, // Already infused
        IngredientParts::CrushedInfused(_, _) => return None, // Already infused
        IngredientParts::BlanchedInfused(_, _) => return None, // Already infused
        IngredientParts::DriedInfused(_, _) => return None,  // Already infused
        IngredientParts::PickledInfused(_, _) => return None, // Already infused
        IngredientParts::FermentedInfused(_, _, _, _) => return None, // Already infused
        IngredientParts::CrushedFermentedInfused(_, _) => return None, // Already infused
        IngredientParts::BlanchedFermentedInfused(_, _) => return None, // Already infused
        IngredientParts::DriedFermentedInfused(_, _) => return None, // Already infused
        IngredientParts::PickledFermentedInfused(_, _) => return None, // Already infused
    };

    let new_process = match ingredient.process {
        IngredientProcess::Raw => IngredientProcess::Infused,
        IngredientProcess::Crushed => IngredientProcess::CrushedInfused,
        IngredientProcess::Blanched => IngredientProcess::BlanchedInfused,
        IngredientProcess::Dried => IngredientProcess::DriedInfused,
        IngredientProcess::Pickled => IngredientProcess::PickledInfused,
        IngredientProcess::Fermented => IngredientProcess::FermentedInfused,
        IngredientProcess::CrushedFermented => IngredientProcess::CrushedFermentedInfused,
        IngredientProcess::BlanchedFermented => IngredientProcess::BlanchedFermentedInfused,
        IngredientProcess::DriedFermented => IngredientProcess::DriedFermentedInfused,
        IngredientProcess::PickledFermented => IngredientProcess::PickledFermentedInfused,
        IngredientProcess::Infused => return None, // Already infused
        IngredientProcess::CrushedInfused => return None, // Already infused
        IngredientProcess::BlanchedInfused => return None, // Already infused
        IngredientProcess::DriedInfused => return None, // Already infused
        IngredientProcess::PickledInfused => return None, // Already infused
        IngredientProcess::FermentedInfused => return None, // Already infused
        IngredientProcess::CrushedFermentedInfused => return None, // Already infused
        IngredientProcess::BlanchedFermentedInfused => return None, // Already infused
        IngredientProcess::DriedFermentedInfused => return None, // Already infused
        IngredientProcess::PickledFermentedInfused => return None, // Already infused
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
