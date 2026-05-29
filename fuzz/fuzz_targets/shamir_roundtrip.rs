#![no_main]

use libfuzzer_sys::fuzz_target;
use shamir_zero::{ShamirError, shamir_combine, shamir_split};

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    let parts = (data[0] as usize % 254) + 2; // 2..=255
    let threshold = (data[1] as usize % parts) + 1; // 1..=parts
    let secret = &data[2..];

    if threshold < 2 {
        return;
    }

    if secret.is_empty() {
        return;
    }

    let mut shares = vec![vec![0u8; secret.len() + 1]; parts];
    let mut shares_out: Vec<&mut [u8]> = shares.iter_mut().map(|v| v.as_mut_slice()).collect();

    match shamir_split(secret, parts, threshold, &mut shares_out) {
        Ok(()) => {
            let share_slices: Vec<&[u8]> =
                shares[0..threshold].iter().map(|s| s.as_slice()).collect();

            let mut recovered = vec![0u8; secret.len()];

            if let Ok(()) = shamir_combine(&share_slices, &mut recovered) {
                assert_eq!(recovered.as_slice(), secret, "roundtrip failed");
            }
        }
        Err(ShamirError::EmptySecret) if secret.is_empty() => {}
        Err(_) => {}
    }
});
