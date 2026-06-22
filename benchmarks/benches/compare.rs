use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::{TryRng, rngs::SysRng};
use std::hint::black_box;
use std::time::Duration;

fn main_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_roundtrip");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(500);

    let sizes = [1024usize, 2048];
    let num_shares = 10u8;
    let threshold = 5u8;

    for &secret_len in &sizes {
        let mut secret = vec![0u8; secret_len];
        SysRng.try_fill_bytes(&mut secret).unwrap();

        let id = format!("{}B_t{}_n{}", secret_len, threshold, num_shares);

        // shamir-zero (high-level API)
        group.bench_with_input(
            BenchmarkId::new("shamir-zero", &id),
            &secret,
            |b, secret| {
                b.iter(|| {
                    let shares = black_box(shamir_zero::ShamirZero::split(
                        secret,
                        num_shares as usize,
                        threshold as usize,
                    ))
                    .unwrap();
                    let recovered = black_box(shamir_zero::ShamirZero::combine(
                        &shares[0..threshold as usize],
                    ))
                    .unwrap();
                    black_box(recovered);
                });
            },
        );

        // shamir-zero (zero-copy API)
        group.bench_with_input(
            BenchmarkId::new("shamir-zero-zero-copy", &id),
            &secret,
            |b, secret| {
                let mut shares_buf: Vec<Vec<u8>> =
                    vec![vec![0u8; secret.len() + 1]; num_shares as usize];
                let mut recovered = vec![0u8; secret.len()];

                b.iter(|| {
                    let mut shares_out: Vec<&mut [u8]> =
                        shares_buf.iter_mut().map(|v| v.as_mut_slice()).collect();

                    shamir_zero::shamir_split(
                        black_box(secret),
                        num_shares as usize,
                        threshold as usize,
                        &mut shares_out,
                    )
                    .unwrap();

                    let share_slices: Vec<&[u8]> =
                        shares_buf.iter().map(|v| v.as_slice()).collect();

                    shamir_zero::shamir_combine(
                        &share_slices[0..threshold as usize],
                        &mut recovered,
                    )
                    .unwrap();

                    black_box(&recovered);
                });
            },
        );

        // sharks
        group.bench_with_input(BenchmarkId::new("sharks", &id), &secret, |b, secret| {
            let sharks = sharks::Sharks(threshold);
            b.iter(|| {
                let dealer = sharks.dealer(black_box(secret));
                let shares: Vec<_> = dealer.take(num_shares as usize).collect();
                let recovered = black_box(sharks.recover(&shares[0..threshold as usize])).unwrap();
                black_box(recovered);
            });
        });

        // ssskit
        group.bench_with_input(BenchmarkId::new("ssskit", &id), &secret, |b, secret| {
            let sss = ssskit::SecretSharing::<0x11b>(threshold);
            b.iter(|| {
                let dealer = sss.dealer(black_box(secret));
                let shares: Vec<ssskit::Share<0x11b>> = dealer.take(num_shares as usize).collect();
                let recovered = black_box(sss.recover(&shares[0..threshold as usize])).unwrap();
                black_box(recovered);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, main_benches);
criterion_main!(benches);
