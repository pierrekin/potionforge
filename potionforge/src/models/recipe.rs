use super::{
    ingredients::{Element, Ingredient, MainEffect},
    traits::{GetByKey, GetName},
};

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Department {
    Health,
    Sourcery,
    Provisions,
}

impl GetName for Department {
    fn name(&self) -> &'static str {
        match self {
            Department::Health => "Health",
            Department::Sourcery => "Sourcery",
            Department::Provisions => "Provisions",
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ToxicityEffect {
    ToxicPositive,
    ToxicNegative,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TasteEffect {
    TastyPositive,
    TastyNeutral,
    TastyNegative,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OverallPurity {
    Neutral,
    Impure,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OverallToxicity {
    VeryToxic,
    Toxic,
    Neutral,
    Antitoxic,
    Veryantitoxic,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
            OverallTaste::Bland => APPEAL_MAP_NEGATIVE[8].1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PotionKind {
    pub key: PotionKindKey,
    pub department: Department,
    pub parts: (MainEffect, Element),
    pub toxicity_effect: ToxicityEffect,
    pub taste_effect: TasteEffect,
}

impl GetName for PotionKind {
    fn name(&self) -> &'static str {
        match self.key {
            PotionKindKey::Speed => "Speed",
            PotionKindKey::Slow => "Slow",
            PotionKindKey::Mana => "Mana",
            PotionKindKey::Warding => "Warding",
            PotionKindKey::Strength => "Strength",
            PotionKindKey::Weakness => "Weakness",
            PotionKindKey::Necromancy => "Necromancy",
            PotionKindKey::Skelleton => "Skelleton",
            PotionKindKey::Speech => "Speech",
            PotionKindKey::Silence => "Silence",
            PotionKindKey::Conjuring => "Conjuring",
            PotionKindKey::Exorcism => "Exorcism",
            PotionKindKey::Vitality => "Vitality",
            PotionKindKey::Sleep => "Sleep",
            PotionKindKey::Summoning => "Summoning",
            PotionKindKey::Monster => "Monster",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Recipe {
    pub potion_kind_key: PotionKindKey,
    pub ingredients: Vec<Ingredient>,
    pub overall_taste: OverallTaste,
    pub overall_toxicity: OverallToxicity,
    pub overall_purity: OverallPurity,
    pub overall_appeal: i32,
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
    use crate::models::traits::GetByKey;

    use super::POTION_KINDS;

    #[test]
    fn test_get_potion_kind_by_key() {
        let potion_kinds = &POTION_KINDS;
        for i in 0..16 {
            let key = &potion_kinds[i].0;
            let expected = &potion_kinds[i].1;

            assert_eq!(potion_kinds.get_by_key(key), expected);
        }
    }
}
