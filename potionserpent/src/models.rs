use ::potionforge::models as forge;
use potionforge::models::traits::ToHumanReadable;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub enum IngredientPart {
    // Main effect
    Cat,
    Bone,
    Soul,
    Beast,
    // Purity
    Stimulant,
    Impurity,
    // Toxicity
    Toxin,
    Antitoxin,
    // Element
    Fire,
    Aether,
    Water,
    Earth,
    // Taste
    Tasty,
    Unsavory,
    Bitter,
    Sweet,
}

impl From<forge::IngredientPart> for IngredientPart {
    fn from(part: forge::IngredientPart) -> Self {
        match part {
            // Main effect
            forge::IngredientPart::MainEffect(forge::MainEffect::Cat) => IngredientPart::Cat,
            forge::IngredientPart::MainEffect(forge::MainEffect::Bone) => IngredientPart::Bone,
            forge::IngredientPart::MainEffect(forge::MainEffect::Soul) => IngredientPart::Soul,
            forge::IngredientPart::MainEffect(forge::MainEffect::Beast) => IngredientPart::Beast,
            // Purity
            forge::IngredientPart::Stimulant => IngredientPart::Stimulant,
            forge::IngredientPart::Impurity => IngredientPart::Impurity,
            // Toxicity
            forge::IngredientPart::Toxin => IngredientPart::Toxin,
            forge::IngredientPart::Antitoxin => IngredientPart::Antitoxin,
            // Element
            forge::IngredientPart::Element(forge::Element::Fire) => IngredientPart::Fire,
            forge::IngredientPart::Element(forge::Element::Aether) => IngredientPart::Aether,
            forge::IngredientPart::Element(forge::Element::Water) => IngredientPart::Water,
            forge::IngredientPart::Element(forge::Element::Earth) => IngredientPart::Earth,
            // Taste
            forge::IngredientPart::Taste(forge::Taste::Sweetness(forge::Sweetness::Bitter)) => {
                IngredientPart::Bitter
            }
            forge::IngredientPart::Taste(forge::Taste::Sweetness(forge::Sweetness::Sweet)) => {
                IngredientPart::Sweet
            }
            forge::IngredientPart::Taste(forge::Taste::Tastiness(forge::Tastiness::Tasty)) => {
                IngredientPart::Tasty
            }
            forge::IngredientPart::Taste(forge::Taste::Tastiness(forge::Tastiness::Unsavory)) => {
                IngredientPart::Unsavory
            }
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct IngredientParts(
    pub IngredientPart,
    pub IngredientPart,
    pub Option<IngredientPart>,
    pub Option<IngredientPart>,
);

#[pymethods]
impl IngredientParts {
    fn __len__(&self) -> PyResult<usize> {
        Ok(4)
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Option<IngredientPart>> {
        match idx {
            0 => Ok(Some(self.0)),
            1 => Ok(Some(self.1)),
            2 => Ok(self.2),
            3 => Ok(self.3),
            _ => Ok(None),
        }
    }
}

impl From<forge::IngredientParts> for IngredientParts {
    fn from(parts: forge::IngredientParts) -> Self {
        let convert_full = |a: forge::IngredientPart,
                            b: forge::IngredientPart,
                            c: forge::IngredientPart,
                            d: forge::IngredientPart| {
            IngredientParts(
                IngredientPart::from(a),
                IngredientPart::from(b),
                Some(IngredientPart::from(c)),
                Some(IngredientPart::from(d)),
            )
        };

        let convert_partial = |a: forge::IngredientPart, b: forge::IngredientPart| {
            IngredientParts(IngredientPart::from(a), IngredientPart::from(b), None, None)
        };

        match parts {
            // Single processes
            forge::IngredientParts::Raw(a, b, c, d) => convert_full(a, b, c, d),
            forge::IngredientParts::Fermented(a, b, c, d) => convert_full(a, b, c, d),
            forge::IngredientParts::Infused(a, b, c, d) => convert_full(a, b, c, d),
            forge::IngredientParts::Crushed(a, b) => convert_partial(a, b),
            forge::IngredientParts::Blanched(a, b) => convert_partial(a, b),
            forge::IngredientParts::Dried(a, b) => convert_partial(a, b),
            forge::IngredientParts::Pickled(a, b) => convert_partial(a, b),
            // Chained processes: Cut and then fermented
            forge::IngredientParts::CrushedFermented(a, b) => convert_partial(a, b),
            forge::IngredientParts::BlanchedFermented(a, b) => convert_partial(a, b),
            forge::IngredientParts::DriedFermented(a, b) => convert_partial(a, b),
            forge::IngredientParts::PickledFermented(a, b) => convert_partial(a, b),
            // Chained processes: Cut and then infused
            forge::IngredientParts::CrushedInfused(a, b) => convert_partial(a, b),
            forge::IngredientParts::BlanchedInfused(a, b) => convert_partial(a, b),
            forge::IngredientParts::DriedInfused(a, b) => convert_partial(a, b),
            forge::IngredientParts::PickledInfused(a, b) => convert_partial(a, b),
            // Chained processes: Fermented and then infused
            forge::IngredientParts::FermentedInfused(a, b, c, d) => convert_full(a, b, c, d),
            // Chained processes: Cut, fermented, and then infused
            forge::IngredientParts::CrushedFermentedInfused(a, b) => convert_partial(a, b),
            forge::IngredientParts::BlanchedFermentedInfused(a, b) => convert_partial(a, b),
            forge::IngredientParts::DriedFermentedInfused(a, b) => convert_partial(a, b),
            forge::IngredientParts::PickledFermentedInfused(a, b) => convert_partial(a, b),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct IngredientPartsWrapper {
    inner: forge::IngredientParts,

    #[pyo3(get)]
    value: IngredientParts,
}

#[pymethods]
impl IngredientPartsWrapper {
    fn to_human(&self) -> PyResult<String> {
        Ok(self.inner.to_human())
    }
}

#[pyclass]
#[derive(Clone)]
pub enum IngredientKind {
    Herb,
    Mushroom,
}

impl From<forge::IngredientKind> for IngredientKind {
    fn from(kind: forge::IngredientKind) -> Self {
        match kind {
            forge::IngredientKind::Herb => IngredientKind::Herb,
            forge::IngredientKind::Mushroom => IngredientKind::Mushroom,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct IngredientKindWrapper {
    inner: forge::IngredientKind,

    #[pyo3(get)]
    value: IngredientKind,
}

#[pymethods]
impl IngredientKindWrapper {
    fn to_human(&self) -> PyResult<String> {
        Ok(self.inner.to_human())
    }
}

#[pyclass]
#[derive(Clone)]
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

impl From<forge::IngredientProcess> for IngredientProcess {
    fn from(process: forge::IngredientProcess) -> Self {
        match process {
            // Single Processes
            forge::IngredientProcess::Raw => IngredientProcess::Raw,
            forge::IngredientProcess::Crushed => IngredientProcess::Crushed,
            forge::IngredientProcess::Blanched => IngredientProcess::Blanched,
            forge::IngredientProcess::Dried => IngredientProcess::Dried,
            forge::IngredientProcess::Pickled => IngredientProcess::Pickled,
            forge::IngredientProcess::Fermented => IngredientProcess::Fermented,
            forge::IngredientProcess::Infused => IngredientProcess::Infused,
            // Chained processes: Cut and then fermente => IngredientProcess::d
            forge::IngredientProcess::CrushedFermented => IngredientProcess::CrushedFermented,
            forge::IngredientProcess::BlanchedFermented => IngredientProcess::BlanchedFermented,
            forge::IngredientProcess::DriedFermented => IngredientProcess::DriedFermented,
            forge::IngredientProcess::PickledFermented => IngredientProcess::PickledFermented,
            // Chained processes: Cut and then infuse => IngredientProcess::d
            forge::IngredientProcess::CrushedInfused => IngredientProcess::CrushedInfused,
            forge::IngredientProcess::BlanchedInfused => IngredientProcess::BlanchedInfused,
            forge::IngredientProcess::DriedInfused => IngredientProcess::DriedInfused,
            forge::IngredientProcess::PickledInfused => IngredientProcess::PickledInfused,
            // Chained processes: Fermented and then infuse => IngredientProcess::d
            forge::IngredientProcess::FermentedInfused => IngredientProcess::FermentedInfused,
            // Chained processes: Cut, fermented, and then infuse => IngredientProcess::d
            forge::IngredientProcess::CrushedFermentedInfused => {
                IngredientProcess::CrushedFermentedInfused
            }
            forge::IngredientProcess::BlanchedFermentedInfused => {
                IngredientProcess::BlanchedFermentedInfused
            }
            forge::IngredientProcess::DriedFermentedInfused => {
                IngredientProcess::DriedFermentedInfused
            }
            forge::IngredientProcess::PickledFermentedInfused => {
                IngredientProcess::PickledFermentedInfused
            }
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct IngredientProcessWrapper {
    inner: forge::IngredientProcess,

    #[pyo3(get)]
    value: IngredientProcess,
}

#[pymethods]
impl IngredientProcessWrapper {
    fn to_human(&self) -> PyResult<String> {
        Ok(self.inner.to_human())
    }
}

#[pyclass]
#[derive(Clone)]
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

impl From<forge::IngredientKey> for IngredientKey {
    fn from(key: forge::IngredientKey) -> Self {
        match key {
            forge::IngredientKey::Catnip => IngredientKey::Catnip,
            forge::IngredientKey::Lupine => IngredientKey::Lupine,
            forge::IngredientKey::Mandrake => IngredientKey::Mandrake,
            forge::IngredientKey::Nightshade => IngredientKey::Nightshade,
            forge::IngredientKey::Sage => IngredientKey::Sage,
            forge::IngredientKey::Thyme => IngredientKey::Thyme,
            forge::IngredientKey::Wormwood => IngredientKey::Wormwood,
            forge::IngredientKey::Anise => IngredientKey::Anise,
            forge::IngredientKey::Deadmans => IngredientKey::Deadmans,
            forge::IngredientKey::Deathcap => IngredientKey::Deathcap,
            forge::IngredientKey::Elven => IngredientKey::Elven,
            forge::IngredientKey::Flyagaric => IngredientKey::Flyagaric,
            forge::IngredientKey::Pluteus => IngredientKey::Pluteus,
            forge::IngredientKey::Wizards => IngredientKey::Wizards,
            forge::IngredientKey::Asporeus => IngredientKey::Asporeus,
            forge::IngredientKey::Stinkhorn => IngredientKey::Stinkhorn,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct IngredientKeyWrapper {
    inner: forge::IngredientKey,

    #[pyo3(get)]
    value: IngredientKey,
}

#[pymethods]
impl IngredientKeyWrapper {
    fn to_human(&self) -> PyResult<String> {
        Ok(self.inner.to_human())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Ingredient {
    inner: forge::Ingredient,

    #[pyo3(get)]
    pub key: IngredientKeyWrapper,

    #[pyo3(get)]
    pub process: IngredientProcessWrapper,

    #[pyo3(get)]
    pub kind: IngredientKindWrapper,

    #[pyo3(get)]
    pub parts: IngredientPartsWrapper,
}

#[pymethods]
impl Ingredient {
    fn to_human(&self) -> PyResult<String> {
        Ok(self.inner.to_human())
    }
}

impl From<forge::Ingredient> for Ingredient {
    fn from(ingredient: forge::Ingredient) -> Self {
        Ingredient {
            inner: ingredient.clone(),
            key: IngredientKeyWrapper {
                inner: ingredient.key.clone(),
                value: IngredientKey::from(ingredient.key),
            },
            process: IngredientProcessWrapper {
                inner: ingredient.process.clone(),
                value: IngredientProcess::from(ingredient.process),
            },
            kind: IngredientKindWrapper {
                inner: ingredient.kind.clone(),
                value: IngredientKind::from(ingredient.kind),
            },
            parts: IngredientPartsWrapper {
                inner: ingredient.parts.clone(),
                value: IngredientParts::from(ingredient.parts),
            },
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum PotionKindKey {
    Speed,
    Slow,
    Mana,
    Warding,
    Strength,
    Weakness,
    Necromancy,
    Skeleton,
    Speech,
    Silence,
    Conjuring,
    Exorcism,
    Vitality,
    Sleep,
    Summoning,
    Monster,
}

#[pyclass]
#[derive(Clone)]
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

#[pyclass]
#[derive(Clone)]
pub enum OverallToxicity {
    VeryToxic,
    Toxic,
    Neutral,
    Antitoxic,
    Veryantitoxic,
}

#[pyclass]
#[derive(Clone)]
pub enum OverallPurity {
    Neutral,
    Impure,
}

#[pyclass]
pub struct Recipe {
    #[pyo3(get)]
    pub potion_kind_key: PotionKindKey,

    #[pyo3(get)]
    pub ingredients: Vec<Ingredient>,

    #[pyo3(get)]
    pub overall_taste: OverallTaste,

    #[pyo3(get)]
    pub overall_toxicity: OverallToxicity,

    #[pyo3(get)]
    pub overall_purity: OverallPurity,

    #[pyo3(get)]
    pub overall_appeal: i32,

    #[pyo3(get)]
    pub overall_potency: i32,
}
