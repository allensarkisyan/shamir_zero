use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use shamir_zero::{ShamirError, shamir_combine, shamir_split};
use std::hint::black_box;

fn shamir_combine_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_combine_benchmark");

    let size: usize = 64;
    let configs = [(2, 2), (3, 2), (3, 3), (5, 3), (5, 5)];
    let secret = vec![0xAAu8; size];

    for &(parts, threshold) in &configs {
        let shares = match shamir_split(&secret, parts, threshold) {
            Ok(shares) => shares,
            Err(ShamirError::EmptySecret) if size == 0 => continue,
            Err(_) => continue,
        };

        let id = BenchmarkId::new(
            format!("combine_{}of{}_size{}", threshold, parts, size),
            size,
        );

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(id, &shares, |b, shares| {
            b.iter(|| {
                let _ = black_box(shamir_combine(black_box(&shares[0..threshold])));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, shamir_combine_benchmark);
criterion_main!(benches);
