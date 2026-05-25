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

    match shamir_split(secret, parts, threshold) {
        Ok(shares) => {
            if let Ok(recovered) = shamir_combine(&shares[0..threshold]) {
                assert_eq!(recovered, secret, "roundtrip failed");
            }
        }
        Err(ShamirError::EmptySecret) if secret.is_empty() => {}
        Err(_) => {}
    }
});
