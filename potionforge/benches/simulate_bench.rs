use criterion::{criterion_group, criterion_main, Criterion};
use potionforge::{
    models::{traits::GetByKey, IngredientKey, INGREDIENTS},
    recommend::{AlchemistAttributes, BrandingCounts, MarketConditions},
    simulate::SimulateConfig,
};

pub fn simulate(c: &mut Criterion) {
    let combination = vec![
        INGREDIENTS.get_by_key(&IngredientKey::Asporeus).clone(),
        INGREDIENTS.get_by_key(&IngredientKey::Wizards).clone(),
    ];
    let config = SimulateConfig {
        alchemists_attributes: AlchemistAttributes::new(),
        market_conditions: MarketConditions::new(),
        branding_counts: BrandingCounts::new(),
    };

    let ingredients = combination.as_slice();

    c.bench_function("simulate", |b| {
        b.iter(|| ::potionforge::simulate::simulate(ingredients, &config));
    });
}

criterion_group!(benches, simulate);
criterion_main!(benches);
