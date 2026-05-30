use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use shamir_zero::{ShamirError, shamir_combine, shamir_split};
use std::hint::black_box;

fn shamir_combine_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_combine_benchmark");

    let size: usize = 64;
    let configs = [(2, 2), (3, 2), (3, 3), (5, 3), (5, 5)];
    let secret = vec![0xAAu8; size];

    for &(parts, threshold) in &configs {
        let mut shares_out = vec![vec![0u8; secret.len() + 1]; parts];
        let mut shares_out_slices: Vec<&mut [u8]> =
            shares_out.iter_mut().map(|v| v.as_mut_slice()).collect();

        let shares_result = shamir_split(&secret, parts, threshold, &mut shares_out_slices);

        let shares = match shares_result {
            Ok(_) => shares_out,
            Err(ShamirError::EmptySecret) if size == 0 => continue,
            Err(_) => continue,
        };

        let mut recovered = vec![0u8; size];
        let id = BenchmarkId::new(
            format!("combine_{}of{}_size{}", threshold, parts, size),
            size,
        );

        let share_slices: Vec<&[u8]> = shares[0..threshold].iter().map(|s| s.as_slice()).collect();

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(id, &share_slices, |b, input_slices| {
            b.iter(|| {
                let _ = black_box(shamir_combine(
                    black_box(input_slices.as_slice()),
                    black_box(&mut recovered),
                ));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, shamir_combine_benchmark);
criterion_main!(benches);
