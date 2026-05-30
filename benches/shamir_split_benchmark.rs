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
                // Allocate exactly once per iteration to measure true zero-copy overhead
                let mut shares_out = vec![vec![0u8; secret.len() + 1]; parts];
                let shares_out_slices: Vec<&mut [u8]> =
                    shares_out.iter_mut().map(|v| v.as_mut_slice()).collect();

                let _ = black_box(shamir_split(
                    black_box(secret),
                    black_box(parts),
                    black_box(threshold),
                    &mut black_box(shares_out_slices),
                ));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, shamir_split_benchmark);
criterion_main!(benches);
