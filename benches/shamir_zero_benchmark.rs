use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use shamir_zero::{shamir_combine, shamir_split};
use std::hint::black_box;

fn shamir_zero_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_zero_benchmark");

    let size: usize = 64;
    let configs = [(2, 2), (3, 2), (3, 3), (5, 3), (5, 5)];
    let secret = vec![0xAAu8; size];

    for &(parts, threshold) in &configs {
        let mut recovered = vec![0u8; size];
        let id = BenchmarkId::new(
            format!("roundtrip_{}of{}_size{}", threshold, parts, size),
            size,
        );

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(id, &secret, |b, secret| {
            b.iter(|| {
                // Full roundtrip: split then immediately combine using threshold shares
                let shares = black_box(shamir_split(
                    black_box(secret),
                    black_box(parts),
                    black_box(threshold),
                ))
                .unwrap();

                let share_slices: Vec<&[u8]> =
                    shares[0..threshold].iter().map(|s| s.as_slice()).collect();

                let _ = black_box(shamir_combine(
                    black_box(share_slices.as_slice()),
                    black_box(&mut recovered),
                ));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, shamir_zero_benchmark);
criterion_main!(benches);
