#![allow(missing_docs)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use per::add;

const ITERATION_COUNT: usize = 10000;

fn sum(criterion: &mut Criterion) {
    criterion.bench_function("add", |bencher| {
        let xs = (0..ITERATION_COUNT as usize).collect::<Vec<_>>();

        bencher.iter(|| {
            let mut sum = 0;

            for x in &xs {
                sum = add(sum, black_box(*x));
            }

            black_box(sum);
        })
    });
}

criterion_group!(benches, sum);

criterion_main!(benches);
