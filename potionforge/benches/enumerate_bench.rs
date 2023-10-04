use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use potionforge::models::{traits::GetName, Process, INGREDIENTS};

pub fn permute_ingredient(c: &mut Criterion) {
    let processes = vec![Process::Crush];

    let mut group = c.benchmark_group("permute_ingredient");
    for (ingredient_key, ingredient) in INGREDIENTS.0.iter() {
        let ingredient_name = ingredient.name();

        group.bench_with_input(
            BenchmarkId::from_parameter(ingredient_name),
            ingredient_key,
            |b, _| {
                b.iter(|| {
                    let all_ingredients =
                        ::potionforge::enumerate::permute_ingredient(ingredient, &processes);
                    black_box(all_ingredients);
                });
            },
        );
    }
}

pub fn permute_ingredients(c: &mut Criterion) {
    let processes = vec![Process::Crush];
    let ingredients: Vec<_> = INGREDIENTS
        .0
        .iter()
        .map(|(_, ingredient)| ingredient)
        .collect();
    let ingredients = ingredients.as_slice();

    c.bench_function("permute_ingredients", |b| {
        b.iter(|| {
            let all_ingredients =
                ::potionforge::enumerate::permute_ingredients(ingredients, &processes);
            black_box(all_ingredients);
        });
    });
}

criterion_group!(benches, permute_ingredient, permute_ingredients);
criterion_main!(benches);
