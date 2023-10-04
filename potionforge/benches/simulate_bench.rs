use criterion::{black_box, criterion_group, criterion_main, Criterion};
use potionforge::{
    models::{traits::GetByKey, IngredientKey, INGREDIENTS},
    recommend::{AlchemistAttributes, BrandingCounts, MarketConditions},
    simulate::SimulateConfig,
};

pub fn collect_parts(c: &mut Criterion) {
    let combination = vec![
        INGREDIENTS.get_by_key(&IngredientKey::Asporeus).clone(),
        INGREDIENTS.get_by_key(&IngredientKey::Wizards).clone(),
    ];

    let ingredients = combination.as_slice();

    c.bench_function("collect_parts", |b| {
        b.iter(|| {
            let parts = ::potionforge::simulate::collect_parts(ingredients);
            black_box(parts);
        });
    });
}

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
        b.iter(|| {
            let recipe = ::potionforge::simulate::simulate(ingredients, &config);
            black_box(recipe);
        });
    });
}

criterion_group!(benches, simulate, collect_parts);
criterion_main!(benches);
