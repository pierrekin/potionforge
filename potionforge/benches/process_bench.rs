use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use potionforge::models::{traits::GetName, Process, INGREDIENTS};

pub fn process_ingredient(c: &mut Criterion) {
    let processes = vec![Process::Crush, Process::Infuse, Process::Ferment];

    let mut group = c.benchmark_group("process_ingredient");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for (ingredient_key, ingredient) in INGREDIENTS.0.iter() {
        let ingredient_name = ingredient.name();

        group.bench_with_input(
            BenchmarkId::from_parameter(ingredient_name),
            ingredient_key,
            |b, _| {
                b.iter(|| {
                    let processed_ingredient =
                        ::potionforge::process::process_ingredient(ingredient, &processes);
                    black_box(processed_ingredient);
                });
            },
        );
    }
}

criterion_group!(benches, process_ingredient);
criterion_main!(benches);
