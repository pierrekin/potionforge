use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use potionforge::{
    models::{
        traits::{GetByKey, GetName},
        IngredientKey, INGREDIENTS, POTION_KINDS,
    },
    recommend::{AlchemistAttributes, BrandingCounts, MarketConditions},
    simulate::SimulateConfig,
    testdata::INGREDIENT_COMBINATIONS,
};

pub fn collect_parts(c: &mut Criterion) {
    let mut group = c.benchmark_group("collect_parts");
    for (ingredients, potion_kind_key) in INGREDIENT_COMBINATIONS.iter() {
        let potion_kind_name = POTION_KINDS.get_by_key(potion_kind_key).name();
        group.bench_with_input(
            BenchmarkId::from_parameter(potion_kind_name),
            potion_kind_key,
            |b, _| {
                b.iter(|| {
                    let parts = ::potionforge::simulate::collect_parts(ingredients);
                    black_box(parts);
                });
            },
        );
    }
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
