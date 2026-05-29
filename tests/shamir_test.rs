#[cfg(test)]
mod shamir_tests {
    use rand::{TryRng, rngs::SysRng};
    use shamir_zero::{ShamirError, shamir_combine, shamir_split};

    /// Helper method to generate random sized bytes
    fn generate_random_bytes_sized<const N: usize>() -> [u8; N] {
        let mut data = [0u8; N];
        SysRng.try_fill_bytes(&mut data).unwrap();
        data
    }

    fn init() {
        let _ = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));
    }

    #[test]
    fn test_split_invalid() {
        let secret = b"test";

        assert_eq!(
            shamir_split(secret, 0, 0),
            Err(ShamirError::ThresholdLessThanMinimumLength)
        );
        assert_eq!(
            shamir_split(secret, 2, 3),
            Err(ShamirError::PartsLessThanThresholdLength)
        );
        assert_eq!(
            shamir_split(secret, 1000, 3),
            Err(ShamirError::PartsExceedMaximumLength)
        );
        assert_eq!(
            shamir_split(secret, 10, 1),
            Err(ShamirError::ThresholdLessThanMinimumLength)
        );
        assert_eq!(shamir_split(&[], 3, 2), Err(ShamirError::EmptySecret));
    }

    #[test]
    fn test_split() {
        let secret = b"test";
        let out = shamir_split(secret, 5, 3).unwrap();

        assert_eq!(out.len(), 5);
        for share in &out {
            assert_eq!(share.len(), secret.len() + 1);
        }

        // Threshold max length
        let out = shamir_split(secret, 5, 256);
        assert_eq!(out, Err(ShamirError::ThresholdExceedsMaximumLength));
    }

    #[test]
    fn test_combine_invalid() {
        let mut dummy = [0u8; 0];
        assert_eq!(
            shamir_combine(&[], &mut dummy),
            Err(ShamirError::RequiredMinimumParts)
        );

        // Length mismatch
        let parts = vec![b"foo".to_vec(), b"ba".to_vec()];
        let slices: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
        let mut out = vec![0u8; 2]; // first part len=3 → secret_len=2
        assert_eq!(
            shamir_combine(&slices, &mut out),
            Err(ShamirError::PartsLengthMismatch)
        );
    }

    #[test]
    fn test_combine_invalid_output_length() {
        let secret = b"test";
        let shares = shamir_split(secret, 3, 2).unwrap();
        let slices: Vec<&[u8]> = shares.iter().map(|s| s.as_slice()).collect();

        let mut wrong_size = vec![0u8; 10];
        assert_eq!(
            shamir_combine(&slices[0..2], &mut wrong_size),
            Err(ShamirError::InvalidOutputLength)
        );
    }

    #[test]
    fn test_combine_invalid_bytes() {
        // Too short (< 2 bytes)
        let parts = vec![b"f".to_vec(), b"b".to_vec()];
        let slices: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
        let mut dummy = [0u8; 0];
        assert_eq!(
            shamir_combine(&slices, &mut dummy),
            Err(ShamirError::MinimumPartByteLength)
        );

        // Duplicate x value
        let parts = vec![b"foo".to_vec(), b"foo".to_vec()];
        let slices: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
        let mut out = vec![0u8; 2]; // foo.len() = 3 → secret_len = 2
        assert_eq!(
            shamir_combine(&slices, &mut out),
            Err(ShamirError::DuplicatePartDetected)
        );
    }

    #[test]
    fn test_combine_invalid_share() {
        // Invalid Share x value
        let secret = b"test";
        let mut parts = shamir_split(secret, 5, 3).unwrap();
        let x_index = secret.len();
        parts[2][x_index] = 0;

        let slices: Vec<&[u8]> = parts[0..3].iter().map(|s| s.as_slice()).collect();
        let mut out = vec![0u8; secret.len()];

        assert_eq!(
            shamir_combine(&slices, &mut out),
            Err(ShamirError::InvalidShareXValue)
        );
    }

    #[test]
    fn test_combine() {
        let secret = b"test";
        let shares = shamir_split(secret, 5, 3).unwrap();

        // Brute-force all combinations of 3 shares
        for i in 0..5 {
            for j in 0..5 {
                if j == i {
                    continue;
                }
                for k in 0..5 {
                    if k == i || k == j {
                        continue;
                    }
                    let parts = vec![
                        shares[i].as_slice(),
                        shares[j].as_slice(),
                        shares[k].as_slice(),
                    ];
                    let mut recomb = vec![0u8; secret.len()];

                    shamir_combine(&parts, &mut recomb).unwrap();

                    assert_eq!(
                        recomb.as_slice(),
                        secret,
                        "Failed for i:{}, j:{}, k:{}",
                        i,
                        j,
                        k
                    );
                }
            }
        }
    }

    #[test]
    fn test_split_and_combine() {
        init();
        let key: [u8; 64] = generate_random_bytes_sized::<64>();
        let parts = 32;
        let threshold = 2;
        let shares = shamir_split(&key, parts, threshold).unwrap();
        let mut permutations = 0;
        let mut recovered = vec![0u8; key.len()];

        // Verify that ANY combination of `threshold` shares reconstructs the key
        for i in 0..parts {
            for j in (i + 1)..parts {
                permutations += 1;

                let subset = vec![shares[i].as_slice(), shares[j].as_slice()];

                shamir_combine(&subset, &mut recovered).unwrap();

                assert_eq!(
                    key.as_slice(),
                    recovered.as_slice(),
                    "Reconstruction failed for shares {} and {}",
                    i,
                    j
                );
            }
        }

        log::info!("Tested key permutations: {:?}", permutations);
    }

    #[test]
    fn test_split_and_combine_data_types() {
        // 1. Byte string
        let secret = b"0xdeadbeef";
        let shares = shamir_split(secret, 10, 5).unwrap();
        let slices: Vec<&[u8]> = shares[2..7].iter().map(|s| s.as_slice()).collect();
        let mut recovered = vec![0u8; secret.len()];
        shamir_combine(&slices, &mut recovered).unwrap();
        assert_eq!(secret.to_vec(), recovered);

        // 2. String
        let secret = "0xcafe".to_string();
        let shares = shamir_split(secret.as_bytes(), 7, 4).unwrap();
        let slices: Vec<&[u8]> = shares[0..4].iter().map(|s| s.as_slice()).collect();
        let mut recovered_bytes = vec![0u8; secret.len()];
        shamir_combine(&slices, &mut recovered_bytes).unwrap();
        let recovered = String::from_utf8(recovered_bytes).unwrap();
        assert_eq!(secret, recovered);

        // 3. Vec<u8>
        let secret: Vec<u8> = vec![0x01, 0x02, 0x03, 0xFF, 0xAA];
        let shares = shamir_split(&secret, 8, 3).unwrap();
        let slices: Vec<&[u8]> = shares[3..6].iter().map(|s| s.as_slice()).collect();
        let mut recovered = vec![0u8; secret.len()];
        shamir_combine(&slices, &mut recovered).unwrap();
        assert_eq!(secret, recovered);

        // 4. Fixed array [u8; 32]
        let secret: [u8; 32] = [0x42; 32];
        let shares = shamir_split(&secret, 6, 4).unwrap();
        let slices: Vec<&[u8]> = shares[1..5].iter().map(|s| s.as_slice()).collect();
        let mut recovered = vec![0u8; 32];
        shamir_combine(&slices, &mut recovered).unwrap();
        assert_eq!(secret.to_vec(), recovered);

        // 5. Numeric type (u128)
        let number: u128 = 12345678901234567890;
        let secret_bytes = number.to_le_bytes();
        let shares = shamir_split(&secret_bytes, 5, 3).unwrap();
        let slices: Vec<&[u8]> = shares[0..3].iter().map(|s| s.as_slice()).collect();
        let mut recovered_bytes = vec![0u8; 16];
        shamir_combine(&slices, &mut recovered_bytes).unwrap();
        let recovered_number = u128::from_le_bytes(recovered_bytes.try_into().unwrap());
        assert_eq!(number, recovered_number);
    }
}
