use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum IngredientPart {
    MainEffect(MainEffect),
    Element(Element),
    Taste(Taste),
    Stimulant,
    Impurity,
    Toxin,
    Antitoxin,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum MainEffect {
    Cat,
    Bone,
    Soul,
    Beast,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Element {
    Fire,
    Aether,
    Water,
    Earth,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Taste {
    Tastiness(Tastiness),
    Sweetness(Sweetness),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Tastiness {
    Tasty,
    Unsavory,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Herb,
    Mushroom,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Department {
    Health,
    Sourcery,
    Provisions,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ingredient {
    pub key: IngredientKey,
    pub process: IngredientProcess,
    pub kind: Kind,
    pub parts: IngredientParts,
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum PotionKindKey {
    Speed,
    Slow,
    Mana,
    Warding,
    Strength,
    Weakness,
    Necromancy,
    Skelleton,
    Speech,
    Silence,
    Conjuring,
    Exorcism,
    Vitality,
    Sleep,
    Summoning,
    Monster,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ToxicityEffect {
    ToxicPositive,
    ToxicNegative,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TasteEffect {
    TastyPositive,
    TastyNeutral,
    TastyNegative,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OverallPurity {
    VeryStimulant,
    Stimulant,
    Neutral,
    Impure,
    VeryImpure,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OverallToxicity {
    VeryToxic,
    Toxic,
    Neutral,
    Antitoxic,
    Veryantitoxic,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OverallTaste {
    Tasty,
    Flavorful,
    Bitter,
    Foul,
    Unsavory,
    Icky,
    Sweet,
    Delicious,
    Bland,
}

pub static APPEAL_MAP_POSITIVE: [(OverallTaste, i32); 9] = [
    (OverallTaste::Tasty, 5),
    (OverallTaste::Flavorful, 15),
    (OverallTaste::Bitter, 5),
    (OverallTaste::Foul, -20),
    (OverallTaste::Unsavory, -10),
    (OverallTaste::Icky, -20),
    (OverallTaste::Sweet, 5),
    (OverallTaste::Delicious, 15),
    (OverallTaste::Bland, 0),
];

pub static APPEAL_MAP_NEGATIVE: [(OverallTaste, i32); 9] = [
    (OverallTaste::Tasty, -10),
    (OverallTaste::Flavorful, -20),
    (OverallTaste::Bitter, 5),
    (OverallTaste::Foul, 15),
    (OverallTaste::Unsavory, 5),
    (OverallTaste::Icky, 15),
    (OverallTaste::Sweet, -10),
    (OverallTaste::Delicious, -20),
    (OverallTaste::Bland, 0),
];

pub struct AppealMapPositive;
pub struct AppealMapNegative;

pub trait AppealLookup {
    fn get_appeal(&self, overall_taste: OverallTaste) -> i32;
}

impl AppealLookup for AppealMapPositive {
    fn get_appeal(&self, overall_taste: OverallTaste) -> i32 {
        match overall_taste {
            OverallTaste::Tasty => APPEAL_MAP_POSITIVE[0].1,
            OverallTaste::Flavorful => APPEAL_MAP_POSITIVE[1].1,
            OverallTaste::Bitter => APPEAL_MAP_POSITIVE[2].1,
            OverallTaste::Foul => APPEAL_MAP_POSITIVE[3].1,
            OverallTaste::Unsavory => APPEAL_MAP_POSITIVE[4].1,
            OverallTaste::Icky => APPEAL_MAP_POSITIVE[5].1,
            OverallTaste::Sweet => APPEAL_MAP_POSITIVE[6].1,
            OverallTaste::Delicious => APPEAL_MAP_POSITIVE[7].1,
            OverallTaste::Bland => APPEAL_MAP_POSITIVE[7].1,
        }
    }
}

impl AppealLookup for AppealMapNegative {
    fn get_appeal(&self, overall_taste: OverallTaste) -> i32 {
        match overall_taste {
            OverallTaste::Tasty => APPEAL_MAP_NEGATIVE[0].1,
            OverallTaste::Flavorful => APPEAL_MAP_NEGATIVE[1].1,
            OverallTaste::Bitter => APPEAL_MAP_NEGATIVE[2].1,
            OverallTaste::Foul => APPEAL_MAP_NEGATIVE[3].1,
            OverallTaste::Unsavory => APPEAL_MAP_NEGATIVE[4].1,
            OverallTaste::Icky => APPEAL_MAP_NEGATIVE[5].1,
            OverallTaste::Sweet => APPEAL_MAP_NEGATIVE[6].1,
            OverallTaste::Delicious => APPEAL_MAP_NEGATIVE[7].1,
            OverallTaste::Bland => APPEAL_MAP_NEGATIVE[7].1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PotionKind {
    pub key: PotionKindKey,
    pub department: Department,
    pub parts: (MainEffect, Element),
    pub toxicity_effect: ToxicityEffect,
    pub taste_effect: TasteEffect,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Recipe {
    pub potion_kind: PotionKind,
    pub ingredients: Vec<Ingredient>,
    pub overall_taste: OverallTaste,
    pub overall_toxicity: OverallToxicity,
    pub overall_purity: OverallPurity,
    pub overall_appeal: i32,
}

pub type IngredientCounts = HashMap<IngredientKey, i32>;

pub static INGREDIENTS: [(IngredientKey, Ingredient); 16] = [
    (
        IngredientKey::Catnip,
        Ingredient {
            key: IngredientKey::Catnip,
            process: IngredientProcess::Raw,
            kind: Kind::Herb,
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
            kind: Kind::Herb,
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
            kind: Kind::Herb,
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
            kind: Kind::Herb,
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
            kind: Kind::Herb,
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
            kind: Kind::Herb,
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
            kind: Kind::Herb,
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
            kind: Kind::Herb,
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
            kind: Kind::Mushroom,
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
            kind: Kind::Mushroom,
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
            kind: Kind::Mushroom,
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
            kind: Kind::Mushroom,
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
            kind: Kind::Mushroom,
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
            kind: Kind::Mushroom,
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
            kind: Kind::Mushroom,
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
            kind: Kind::Mushroom,
            parts: IngredientParts::Raw(
                IngredientPart::Taste(Taste::Tastiness(Tastiness::Unsavory)),
                IngredientPart::MainEffect(MainEffect::Soul),
                IngredientPart::Taste(Taste::Sweetness(Sweetness::Sweet)),
                IngredientPart::Stimulant,
            ),
        },
    ),
];

pub static _INGREDIENTS_VALUES: [&Ingredient; 16] = [
    &INGREDIENTS[0].1,
    &INGREDIENTS[1].1,
    &INGREDIENTS[2].1,
    &INGREDIENTS[3].1,
    &INGREDIENTS[4].1,
    &INGREDIENTS[5].1,
    &INGREDIENTS[6].1,
    &INGREDIENTS[7].1,
    &INGREDIENTS[8].1,
    &INGREDIENTS[9].1,
    &INGREDIENTS[10].1,
    &INGREDIENTS[11].1,
    &INGREDIENTS[12].1,
    &INGREDIENTS[13].1,
    &INGREDIENTS[14].1,
    &INGREDIENTS[15].1,
];

pub trait GetName {
    fn name(&self) -> &'static str;
}

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

pub static POTION_KINDS: [(PotionKindKey, PotionKind); 16] = [
    (
        PotionKindKey::Speed,
        PotionKind {
            key: PotionKindKey::Speed,
            department: Department::Health,
            parts: (MainEffect::Cat, Element::Fire),
            toxicity_effect: ToxicityEffect::ToxicNegative,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Slow,
        PotionKind {
            key: PotionKindKey::Slow,
            department: Department::Provisions,
            parts: (MainEffect::Cat, Element::Water),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNeutral,
        },
    ),
    (
        PotionKindKey::Mana,
        PotionKind {
            key: PotionKindKey::Mana,
            department: Department::Sourcery,
            parts: (MainEffect::Cat, Element::Aether),
            toxicity_effect: ToxicityEffect::ToxicNegative,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Warding,
        PotionKind {
            key: PotionKindKey::Warding,
            department: Department::Sourcery,
            parts: (MainEffect::Cat, Element::Earth),
            toxicity_effect: ToxicityEffect::ToxicNegative,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Strength,
        PotionKind {
            key: PotionKindKey::Strength,
            department: Department::Health,
            parts: (MainEffect::Bone, Element::Fire),
            toxicity_effect: ToxicityEffect::ToxicNegative,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Weakness,
        PotionKind {
            key: PotionKindKey::Weakness,
            department: Department::Provisions,
            parts: (MainEffect::Bone, Element::Water),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNeutral,
        },
    ),
    (
        PotionKindKey::Necromancy,
        PotionKind {
            key: PotionKindKey::Necromancy,
            department: Department::Sourcery,
            parts: (MainEffect::Bone, Element::Aether),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNeutral,
        },
    ),
    (
        PotionKindKey::Skelleton,
        PotionKind {
            key: PotionKindKey::Skelleton,
            department: Department::Provisions,
            parts: (MainEffect::Bone, Element::Earth),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNegative,
        },
    ),
    (
        PotionKindKey::Speech,
        PotionKind {
            key: PotionKindKey::Speech,
            department: Department::Health,
            parts: (MainEffect::Soul, Element::Fire),
            toxicity_effect: ToxicityEffect::ToxicNegative,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Silence,
        PotionKind {
            key: PotionKindKey::Silence,
            department: Department::Provisions,
            parts: (MainEffect::Soul, Element::Water),
            toxicity_effect: ToxicityEffect::ToxicNegative,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Conjuring,
        PotionKind {
            key: PotionKindKey::Conjuring,
            department: Department::Sourcery,
            parts: (MainEffect::Soul, Element::Aether),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNeutral,
        },
    ),
    (
        PotionKindKey::Exorcism,
        PotionKind {
            key: PotionKindKey::Exorcism,
            department: Department::Sourcery,
            parts: (MainEffect::Soul, Element::Earth),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNegative,
        },
    ),
    (
        PotionKindKey::Vitality,
        PotionKind {
            key: PotionKindKey::Vitality,
            department: Department::Health,
            parts: (MainEffect::Beast, Element::Fire),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Sleep,
        PotionKind {
            key: PotionKindKey::Sleep,
            department: Department::Provisions,
            parts: (MainEffect::Beast, Element::Water),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyPositive,
        },
    ),
    (
        PotionKindKey::Summoning,
        PotionKind {
            key: PotionKindKey::Summoning,
            department: Department::Sourcery,
            parts: (MainEffect::Beast, Element::Aether),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNeutral,
        },
    ),
    (
        PotionKindKey::Monster,
        PotionKind {
            key: PotionKindKey::Monster,
            department: Department::Sourcery,
            parts: (MainEffect::Beast, Element::Earth),
            toxicity_effect: ToxicityEffect::ToxicPositive,
            taste_effect: TasteEffect::TastyNegative,
        },
    ),
];

pub trait GetByParts {
    fn get_by_parts(&self, valid_combination: ValidCombination) -> &PotionKind;
}

impl GetByParts for [(PotionKindKey, PotionKind); 16] {
    fn get_by_parts(&self, valid_combination: ValidCombination) -> &PotionKind {
        match valid_combination {
            ValidCombination(MainEffect::Cat, Element::Fire) => &self[0].1, // Speed
            ValidCombination(MainEffect::Cat, Element::Water) => &self[1].1, // Slow
            ValidCombination(MainEffect::Cat, Element::Aether) => &self[2].1, // Mana
            ValidCombination(MainEffect::Cat, Element::Earth) => &self[3].1, // Warding
            ValidCombination(MainEffect::Bone, Element::Fire) => &self[4].1, // Strength
            ValidCombination(MainEffect::Bone, Element::Water) => &self[5].1, // Weakness
            ValidCombination(MainEffect::Bone, Element::Aether) => &self[6].1, // Necromancy
            ValidCombination(MainEffect::Bone, Element::Earth) => &self[7].1, // Skeleton
            ValidCombination(MainEffect::Soul, Element::Fire) => &self[8].1, // Speech
            ValidCombination(MainEffect::Soul, Element::Water) => &self[9].1, // Silence
            ValidCombination(MainEffect::Soul, Element::Aether) => &self[10].1, // Conjuring
            ValidCombination(MainEffect::Soul, Element::Earth) => &self[11].1, // Excorcism
            ValidCombination(MainEffect::Beast, Element::Fire) => &self[12].1, // Vitality
            ValidCombination(MainEffect::Beast, Element::Water) => &self[13].1, // Sleep
            ValidCombination(MainEffect::Beast, Element::Aether) => &self[14].1, // Summoning
            ValidCombination(MainEffect::Beast, Element::Earth) => &self[15].1, // Monster
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ValidCombination(MainEffect, Element);

impl ValidCombination {
    pub fn new(main_effect: MainEffect, element: Element) -> Result<Self, &'static str> {
        // Validate the combination here, if needed
        Ok(ValidCombination(main_effect, element))
    }
}

pub trait GetByKey<K, V> {
    fn get_by_key(&self, key: &K) -> &V;
}

impl GetByKey<IngredientKey, Ingredient> for [(IngredientKey, Ingredient); 16] {
    fn get_by_key(&self, key: &IngredientKey) -> &Ingredient {
        match key {
            IngredientKey::Catnip => &self[0].1,
            IngredientKey::Lupine => &self[1].1,
            IngredientKey::Mandrake => &self[2].1,
            IngredientKey::Nightshade => &self[3].1,
            IngredientKey::Sage => &self[4].1,
            IngredientKey::Thyme => &self[5].1,
            IngredientKey::Wormwood => &self[6].1,
            IngredientKey::Anise => &self[7].1,
            IngredientKey::Deadmans => &self[8].1,
            IngredientKey::Deathcap => &self[9].1,
            IngredientKey::Elven => &self[10].1,
            IngredientKey::Flyagaric => &self[11].1,
            IngredientKey::Pluteus => &self[12].1,
            IngredientKey::Wizards => &self[13].1,
            IngredientKey::Asporeus => &self[14].1,
            IngredientKey::Stinkhorn => &self[15].1,
        }
    }
}

impl GetByKey<PotionKindKey, PotionKind> for [(PotionKindKey, PotionKind); 16] {
    fn get_by_key(&self, key: &PotionKindKey) -> &PotionKind {
        match key {
            PotionKindKey::Speed => &self[0].1,
            PotionKindKey::Slow => &self[1].1,
            PotionKindKey::Mana => &self[2].1,
            PotionKindKey::Warding => &self[3].1,
            PotionKindKey::Strength => &self[4].1,
            PotionKindKey::Weakness => &self[5].1,
            PotionKindKey::Necromancy => &self[6].1,
            PotionKindKey::Skelleton => &self[7].1,
            PotionKindKey::Speech => &self[8].1,
            PotionKindKey::Silence => &self[9].1,
            PotionKindKey::Conjuring => &self[10].1,
            PotionKindKey::Exorcism => &self[11].1,
            PotionKindKey::Vitality => &self[12].1,
            PotionKindKey::Sleep => &self[13].1,
            PotionKindKey::Summoning => &self[14].1,
            PotionKindKey::Monster => &self[15].1,
        }
    }
}

impl GetByKey<Department, &'static str> for [(Department, &'static str); 3] {
    fn get_by_key(&self, key: &Department) -> &&'static str {
        match key {
            Department::Health => &&self[0].1,
            Department::Sourcery => &&self[1].1,
            Department::Provisions => &&self[2].1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
