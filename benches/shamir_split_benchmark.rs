use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use shamir_zero::shamir_split;
use std::hint::black_box;

fn shamir_split_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_split_benchmark");

    let size: usize = 64;
    let configs = [(2, 2), (3, 2), (3, 3), (5, 3), (5, 5)];
    let secret = vec![0xAAu8; size];

    for &(parts, threshold) in &configs {
        let id = BenchmarkId::new(format!("split_{}of{}_size{}", threshold, parts, size), size);

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(id, &secret, |b, secret| {
            b.iter(|| {
                let _ = black_box(shamir_split(
                    black_box(secret),
                    black_box(parts),
                    black_box(threshold),
                ));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, shamir_split_benchmark);
criterion_main!(benches);
