use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use shamir_zero::{shamir_combine, shamir_split};
use std::hint::black_box;

fn shamir_zero_benchmark_full(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_zero_benchmark_full");

    // Representative secret sizes (bytes)
    let secret_sizes = [
        8, 16, 32, 64, 128, 256, 512, 1024, 4096, 8192, 16384, 32768usize,
    ];

    // Comprehensive (parts, threshold) permutations — all valid: 2 ≤ threshold ≤ parts ≤ 255
    let configs = [
        (2, 2),
        (3, 2),
        (3, 3),
        (5, 3),
        (5, 5),
        (7, 4),
        (10, 5),
        (10, 10),
        (15, 8),
        (15, 15),
        (20, 10),
        (20, 15),
        (50, 25),
        (50, 50),
        (100, 50),
        (100, 100),
        (200, 100),
        (200, 150),
        (200, 200),
        (255, 128),
        (255, 200),
        (255, 254),
        (255, 255),
    ];

    for &size in &secret_sizes {
        let secret = vec![0xAAu8; size];

        for &(parts, threshold) in &configs {
            let id = BenchmarkId::new(
                format!("roundtrip_{}of{}_size{}", threshold, parts, size),
                size,
            );

            group.throughput(Throughput::Bytes(size as u64));
            group.bench_with_input(id, &secret, |b, secret| {
                b.iter(|| {
                    let shares = black_box(shamir_split(
                        black_box(secret),
                        black_box(parts),
                        black_box(threshold),
                    ))
                    .unwrap();

                    let _ = black_box(shamir_combine(black_box(&shares[0..threshold])));
                });
            });
        }
    }

    group.finish();
}

criterion_group!(benches, shamir_zero_benchmark_full);
criterion_main!(benches);
