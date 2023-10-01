use serde::Deserialize;

use super::traits::{GetByKey, GetName, ToHumanReadable};

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IngredientKey {
    Catnip,
    Lupine,
    Mandrake,
    Nightshade,
    Sage,
    Thyme,
    Wormwood,
    Anise,
    Deadmans,
    Deathcap,
    Elven,
    Flyagaric,
    Pluteus,
    Wizards,
    Asporeus,
    Stinkhorn,
}

impl ToHumanReadable for IngredientKey {
    fn to_human(&self) -> String {
        match self {
            IngredientKey::Catnip => "Catnip",
            IngredientKey::Lupine => "Lupine",
            IngredientKey::Mandrake => "Mandrake",
            IngredientKey::Nightshade => "Nightshade",
            IngredientKey::Sage => "Sage",
            IngredientKey::Thyme => "Thyme",
            IngredientKey::Wormwood => "Wormwood",
            IngredientKey::Anise => "Anise",
            IngredientKey::Deadmans => "Deadmans",
            IngredientKey::Deathcap => "Deathcap",
            IngredientKey::Elven => "Elven",
            IngredientKey::Flyagaric => "Flyagaric",
            IngredientKey::Pluteus => "Pluteus",
            IngredientKey::Wizards => "Wizards",
            IngredientKey::Asporeus => "Asporeus",
            IngredientKey::Stinkhorn => "Stinkhorn",
        }
        .to_string()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum IngredientPart {
    MainEffect(MainEffect),
    Element(Element),
    Taste(Taste),
    Stimulant,
    Impurity,
    Toxin,
    Antitoxin,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum MainEffect {
    Cat,
    Bone,
    Soul,
    Beast,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Element {
    Fire,
    Aether,
    Water,
    Earth,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Taste {
    Tastiness(Tastiness),
    Sweetness(Sweetness),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Tastiness {
    Tasty,
    Unsavory,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Sweetness {
    Bitter,
    Sweet,
}

impl From<MainEffect> for IngredientPart {
    fn from(main_effect: MainEffect) -> Self {
        IngredientPart::MainEffect(main_effect)
    }
}

impl From<Element> for IngredientPart {
    fn from(element: Element) -> Self {
        IngredientPart::Element(element)
    }
}

impl From<IngredientPart> for MainEffect {
    fn from(part: IngredientPart) -> Self {
        match part {
            IngredientPart::MainEffect(main_effect) => main_effect,
            _ => unreachable!(), // Handle error or default case as needed
        }
    }
}

impl From<IngredientPart> for Element {
    fn from(part: IngredientPart) -> Self {
        match part {
            IngredientPart::Element(element) => element,
            _ => unreachable!(), // Handle error or default case as needed
        }
    }
}

impl ToHumanReadable for IngredientPart {
    fn to_human(&self) -> String {
        match self {
            // Main effect
            IngredientPart::MainEffect(MainEffect::Cat) => "Cat",
            IngredientPart::MainEffect(MainEffect::Bone) => "Bone",
            IngredientPart::MainEffect(MainEffect::Soul) => "Soul",
            IngredientPart::MainEffect(MainEffect::Beast) => "Beast",
            // Purity
            IngredientPart::Stimulant => "Stimulant",
            IngredientPart::Impurity => "Impurity",
            // Toxicity
            IngredientPart::Toxin => "Toxin",
            IngredientPart::Antitoxin => "Antitoxin",
            // Element
            IngredientPart::Element(Element::Fire) => "Fire",
            IngredientPart::Element(Element::Aether) => "Aether",
            IngredientPart::Element(Element::Water) => "Water",
            IngredientPart::Element(Element::Earth) => "Earth",
            // Taste
            IngredientPart::Taste(Taste::Sweetness(Sweetness::Bitter)) => "Bitter",
            IngredientPart::Taste(Taste::Sweetness(Sweetness::Sweet)) => "Sweet",
            IngredientPart::Taste(Taste::Tastiness(Tastiness::Tasty)) => "Tasty",
            IngredientPart::Taste(Taste::Tastiness(Tastiness::Unsavory)) => "Unsavory",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum IngredientKind {
    Herb,
    Mushroom,
}

impl ToHumanReadable for IngredientKind {
    fn to_human(&self) -> String {
        match self {
            IngredientKind::Herb => "Herb",
            IngredientKind::Mushroom => "Mushroom",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ingredient {
    pub key: IngredientKey,
    pub process: IngredientProcess,
    pub kind: IngredientKind,
    pub parts: IngredientParts,
}

impl ToHumanReadable for Ingredient {
    fn to_human(&self) -> String {
        format!("{} ({})", self.key.to_human(), self.process.to_human())
    }
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum IngredientParts {
    // Single processes
    Raw(
        IngredientPart,
        IngredientPart,
        IngredientPart,
        IngredientPart,
    ),
    Fermented(
        IngredientPart,
        IngredientPart,
        IngredientPart,
        IngredientPart,
    ),
    Infused(
        IngredientPart,
        IngredientPart,
        IngredientPart,
        IngredientPart,
    ),
    Crushed(IngredientPart, IngredientPart),
    Blanched(IngredientPart, IngredientPart),
    Dried(IngredientPart, IngredientPart),
    Pickled(IngredientPart, IngredientPart),

    // Chained processes: Cut and then fermented
    CrushedFermented(IngredientPart, IngredientPart),
    BlanchedFermented(IngredientPart, IngredientPart),
    DriedFermented(IngredientPart, IngredientPart),
    PickledFermented(IngredientPart, IngredientPart),

    // Chained processes: Cut and then infused
    CrushedInfused(IngredientPart, IngredientPart),
    BlanchedInfused(IngredientPart, IngredientPart),
    DriedInfused(IngredientPart, IngredientPart),
    PickledInfused(IngredientPart, IngredientPart),

    // Chained processes: Fermented and then infused
    FermentedInfused(
        IngredientPart,
        IngredientPart,
        IngredientPart,
        IngredientPart,
    ),

    // Chained processes: Cut, fermented, and then infused
    CrushedFermentedInfused(IngredientPart, IngredientPart),
    BlanchedFermentedInfused(IngredientPart, IngredientPart),
    DriedFermentedInfused(IngredientPart, IngredientPart),
    PickledFermentedInfused(IngredientPart, IngredientPart),
}

impl ToHumanReadable for IngredientParts {
    fn to_human(&self) -> String {
        let convert_full =
            |a: &IngredientPart, b: &IngredientPart, c: &IngredientPart, d: &IngredientPart| {
                format!(
                    "({}, {}, {}, {})",
                    a.to_human(),
                    b.to_human(),
                    c.to_human(),
                    d.to_human()
                )
            };

        let convert_partial = |a: &IngredientPart, b: &IngredientPart| {
            format!("({}, {})", a.to_human(), b.to_human(),)
        };

        match self {
            // Single processes
            IngredientParts::Raw(a, b, c, d) => convert_full(a, b, c, d),
            IngredientParts::Fermented(a, b, c, d) => convert_full(a, b, c, d),
            IngredientParts::Infused(a, b, c, d) => convert_full(a, b, c, d),
            IngredientParts::Crushed(a, b) => convert_partial(a, b),
            IngredientParts::Blanched(a, b) => convert_partial(a, b),
            IngredientParts::Dried(a, b) => convert_partial(a, b),
            IngredientParts::Pickled(a, b) => convert_partial(a, b),
            // Chained processes: Cut and then fermented
            IngredientParts::CrushedFermented(a, b) => convert_partial(a, b),
            IngredientParts::BlanchedFermented(a, b) => convert_partial(a, b),
            IngredientParts::DriedFermented(a, b) => convert_partial(a, b),
            IngredientParts::PickledFermented(a, b) => convert_partial(a, b),
            // Chained processes: Cut and then infused
            IngredientParts::CrushedInfused(a, b) => convert_partial(a, b),
            IngredientParts::BlanchedInfused(a, b) => convert_partial(a, b),
            IngredientParts::DriedInfused(a, b) => convert_partial(a, b),
            IngredientParts::PickledInfused(a, b) => convert_partial(a, b),
            // Chained processes: Fermented and then infused
            IngredientParts::FermentedInfused(a, b, c, d) => convert_full(a, b, c, d),
            // Chained processes: Cut, fermented, and then infused
            IngredientParts::CrushedFermentedInfused(a, b) => convert_partial(a, b),
            IngredientParts::BlanchedFermentedInfused(a, b) => convert_partial(a, b),
            IngredientParts::DriedFermentedInfused(a, b) => convert_partial(a, b),
            IngredientParts::PickledFermentedInfused(a, b) => convert_partial(a, b),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum IngredientProcess {
    // Single Processes
    Raw,
    Crushed,
    Blanched,
    Dried,
    Pickled,
    Fermented,
    Infused,
    // Chained processes: Cut and then fermented
    CrushedFermented,
    BlanchedFermented,
    DriedFermented,
    PickledFermented,
    // Chained processes: Cut and then infused
    CrushedInfused,
    BlanchedInfused,
    DriedInfused,
    PickledInfused,
    // Chained processes: Fermented and then infused
    FermentedInfused,
    // Chained processes: Cut, fermented, and then infused
    CrushedFermentedInfused,
    BlanchedFermentedInfused,
    DriedFermentedInfused,
    PickledFermentedInfused,
}

impl ToHumanReadable for IngredientProcess {
    fn to_human(&self) -> String {
        match self {
            IngredientProcess::Raw => "Raw",
            IngredientProcess::Crushed => "Crushed",
            IngredientProcess::Blanched => "Blanched",
            IngredientProcess::Dried => "Dried",
            IngredientProcess::Pickled => "Pickled",
            IngredientProcess::Fermented => "Fermented",
            IngredientProcess::Infused => "Infused",
            IngredientProcess::CrushedFermented => "Crushed, Fermented",
            IngredientProcess::BlanchedFermented => "Blanched, Fermented",
            IngredientProcess::DriedFermented => "Dried, Fermented",
            IngredientProcess::PickledFermented => "Pickled, Fermented",
            IngredientProcess::CrushedInfused => "Crushed, Infused",
            IngredientProcess::BlanchedInfused => "Blanched, Infused",
            IngredientProcess::DriedInfused => "Dried, Infused",
            IngredientProcess::PickledInfused => "Pickled, Infused",
            IngredientProcess::FermentedInfused => "Fermented, Infused",
            IngredientProcess::CrushedFermentedInfused => "Crushed, Fermented, Infused",
            IngredientProcess::BlanchedFermentedInfused => "Blanched, Fermented, Infused",
            IngredientProcess::DriedFermentedInfused => "Dried, Fermented, Infused",
            IngredientProcess::PickledFermentedInfused => "Pickled, Fermented, Infused",
        }
        .to_string()
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Process {
    Crush,
    Blanch,
    Dry,
    Pickle,
    Ferment,
    Infuse,
}

pub static INGREDIENTS: Ingredients = Ingredients([
    (
        IngredientKey::Catnip,
        Ingredient {
            key: IngredientKey::Catnip,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::Stimulant,
                IngredientPart::Impurity,
                IngredientPart::MainEffect(MainEffect::Cat),
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Tasty)),
            ),
        },
    ),
    (
        IngredientKey::Lupine,
        Ingredient {
            key: IngredientKey::Lupine,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::Toxin,
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Tasty)),
                IngredientPart::Element(Element::Fire),
                IngredientPart::Stimulant,
            ),
        },
    ),
    (
        IngredientKey::Mandrake,
        Ingredient {
            key: IngredientKey::Mandrake,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Unsavory)),
                IngredientPart::Stimulant,
                IngredientPart::Taste(Taste::Sweetness(Sweetness::Bitter)),
                IngredientPart::MainEffect(MainEffect::Bone),
            ),
        },
    ),
    (
        IngredientKey::Nightshade,
        Ingredient {
            key: IngredientKey::Nightshade,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::Toxin,
                IngredientPart::Element(Element::Aether),
                IngredientPart::Stimulant,
                IngredientPart::Element(Element::Water),
            ),
        },
    ),
    (
        IngredientKey::Sage,
        Ingredient {
            key: IngredientKey::Sage,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::Element(Element::Water),
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Tasty)),
                IngredientPart::Impurity,
                IngredientPart::Taste(Taste::Sweetness(Sweetness::Sweet)),
            ),
        },
    ),
    (
        IngredientKey::Thyme,
        Ingredient {
            key: IngredientKey::Thyme,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Tasty)),
                IngredientPart::Stimulant,
                IngredientPart::Impurity,
                IngredientPart::MainEffect(MainEffect::Cat),
            ),
        },
    ),
    (
        IngredientKey::Wormwood,
        Ingredient {
            key: IngredientKey::Wormwood,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::Element(Element::Fire),
                IngredientPart::Antitoxin,
                IngredientPart::Taste(Taste::Sweetness(Sweetness::Bitter)),
                IngredientPart::Element(Element::Earth),
            ),
        },
    ),
    (
        IngredientKey::Anise,
        Ingredient {
            key: IngredientKey::Anise,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Herb,
            parts: IngredientParts::Raw(
                IngredientPart::MainEffect(MainEffect::Bone),
                IngredientPart::Antitoxin,
                IngredientPart::Taste(Taste::Sweetness(Sweetness::Sweet)),
                IngredientPart::Impurity,
            ),
        },
    ),
    (
        IngredientKey::Deadmans,
        Ingredient {
            key: IngredientKey::Deadmans,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Stimulant,
                IngredientPart::MainEffect(MainEffect::Soul),
                IngredientPart::Toxin,
                IngredientPart::Taste(Taste::Sweetness(Sweetness::Bitter)),
            ),
        },
    ),
    (
        IngredientKey::Deathcap,
        Ingredient {
            key: IngredientKey::Deathcap,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Impurity,
                IngredientPart::Stimulant,
                IngredientPart::Toxin,
                IngredientPart::Element(Element::Earth),
            ),
        },
    ),
    (
        IngredientKey::Elven,
        Ingredient {
            key: IngredientKey::Elven,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Element(Element::Earth),
                IngredientPart::Stimulant,
                IngredientPart::Element(Element::Water),
                IngredientPart::Antitoxin,
            ),
        },
    ),
    (
        IngredientKey::Flyagaric,
        Ingredient {
            key: IngredientKey::Flyagaric,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Stimulant,
                IngredientPart::Toxin,
                IngredientPart::MainEffect(MainEffect::Beast),
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Tasty)),
            ),
        },
    ),
    (
        IngredientKey::Pluteus,
        Ingredient {
            key: IngredientKey::Pluteus,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Toxin,
                IngredientPart::Element(Element::Fire),
                IngredientPart::Stimulant,
                IngredientPart::Element(Element::Aether),
            ),
        },
    ),
    (
        IngredientKey::Wizards,
        Ingredient {
            key: IngredientKey::Wizards,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Impurity,
                IngredientPart::Element(Element::Aether),
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Unsavory)),
                IngredientPart::Stimulant,
            ),
        },
    ),
    (
        IngredientKey::Asporeus,
        Ingredient {
            key: IngredientKey::Asporeus,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::MainEffect(MainEffect::Beast),
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Unsavory)),
                IngredientPart::Stimulant,
                IngredientPart::Impurity,
            ),
        },
    ),
    (
        IngredientKey::Stinkhorn,
        Ingredient {
            key: IngredientKey::Stinkhorn,
            process: IngredientProcess::Raw,
            kind: IngredientKind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Unsavory)),
                IngredientPart::MainEffect(MainEffect::Soul),
                IngredientPart::Taste(Taste::Sweetness(Sweetness::Sweet)),
                IngredientPart::Stimulant,
            ),
        },
    ),
]);

impl GetName for Ingredient {
    fn name(&self) -> &'static str {
        match self.key {
            IngredientKey::Catnip => "Catnip",
            IngredientKey::Lupine => "Lupine",
            IngredientKey::Mandrake => "Mandrake",
            IngredientKey::Nightshade => "Nightshade",
            IngredientKey::Sage => "Sage",
            IngredientKey::Thyme => "Thyme",
            IngredientKey::Wormwood => "Wormwood",
            IngredientKey::Anise => "Anise",
            IngredientKey::Deadmans => "Dead Man's Finger",
            IngredientKey::Deathcap => "Death Cap",
            IngredientKey::Elven => "Elven Saddle",
            IngredientKey::Flyagaric => "Fly Agaric",
            IngredientKey::Pluteus => "Pluteus",
            IngredientKey::Wizards => "Wizard's Hat",
            IngredientKey::Asporeus => "Asporeus",
            IngredientKey::Stinkhorn => "Stinkhorn",
        }
    }
}

pub struct Ingredients(pub [(IngredientKey, Ingredient); 16]);

impl GetByKey<IngredientKey, Ingredient> for Ingredients {
    fn get_by_key(&self, key: &IngredientKey) -> &Ingredient {
        match key {
            IngredientKey::Catnip => &self.0[0].1,
            IngredientKey::Lupine => &self.0[1].1,
            IngredientKey::Mandrake => &self.0[2].1,
            IngredientKey::Nightshade => &self.0[3].1,
            IngredientKey::Sage => &self.0[4].1,
            IngredientKey::Thyme => &self.0[5].1,
            IngredientKey::Wormwood => &self.0[6].1,
            IngredientKey::Anise => &self.0[7].1,
            IngredientKey::Deadmans => &self.0[8].1,
            IngredientKey::Deathcap => &self.0[9].1,
            IngredientKey::Elven => &self.0[10].1,
            IngredientKey::Flyagaric => &self.0[11].1,
            IngredientKey::Pluteus => &self.0[12].1,
            IngredientKey::Wizards => &self.0[13].1,
            IngredientKey::Asporeus => &self.0[14].1,
            IngredientKey::Stinkhorn => &self.0[15].1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        ingredients::IngredientKey,
        traits::{GetByKey, GetName},
        INGREDIENTS, POTION_KINDS,
    };

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
        for i in 0..14 {
            let key = &INGREDIENTS.0[i].0;
            let expected = &INGREDIENTS.0[i].1;

            assert_eq!(INGREDIENTS.get_by_key(key), expected);
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
