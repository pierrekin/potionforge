use serde::Deserialize;

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

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum IngredientKind {
    Herb,
    Mushroom,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ingredient {
    pub key: IngredientKey,
    pub process: IngredientProcess,
    pub kind: IngredientKind,
    pub parts: IngredientParts,
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

impl IngredientProcess {
    pub fn to_human(&self) -> String {
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
