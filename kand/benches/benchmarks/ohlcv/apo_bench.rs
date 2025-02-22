use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use kand::ohlcv::apo::apo;

use crate::helpers::generate_test_data;

#[allow(dead_code)]
fn bench_apo(c: &mut Criterion) {
    let mut group = c.benchmark_group("apo");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input_close = generate_test_data(size);
        let mut output = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{}", size), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = apo(
                            black_box(&input_close),
                            black_box(period),
                            black_box(period),
                            black_box(MAType::Exponential),
                            black_box(&mut output),
                            black_box(&mut output),
                            black_box(&mut output),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_apo);
