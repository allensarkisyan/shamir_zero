#[cfg(test)]
mod shamir_tests {
    use rand::{TryRng, rngs::SysRng};
    use shamir_zero::{shamir_combine, shamir_split};

    /// Helper method to generate random sized bytes
    fn generate_random_bytes_sized<const N: usize>() -> [u8; N] {
        let mut data = [0u8; N];
        SysRng.try_fill_bytes(&mut data).unwrap();
        data
    }

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));
    }

    #[test]
    fn test_split_invalid() {
        let secret = b"test";

        assert!(shamir_split(secret, 0, 0).is_err());
        assert!(shamir_split(secret, 2, 3).is_err());
        assert!(shamir_split(secret, 1000, 3).is_err());
        assert!(shamir_split(secret, 10, 1).is_err());
        assert!(shamir_split(&[], 3, 2).is_err());
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
        assert!(out.is_err());
    }

    #[test]
    fn test_combine_invalid() {
        // Not enough parts
        assert!(shamir_combine(&[]).is_err());

        // Length mismatch
        let parts = vec![b"foo".to_vec(), b"ba".to_vec()];
        assert!(shamir_combine(&parts).is_err());

        // Too short (< 2 bytes)
        let parts = vec![b"f".to_vec(), b"b".to_vec()];
        assert!(shamir_combine(&parts).is_err());

        // Duplicate x value
        let parts = vec![b"foo".to_vec(), b"foo".to_vec()];
        assert!(shamir_combine(&parts).is_err());
    }

    #[test]
    fn test_combine() {
        let secret = b"test";
        let out = shamir_split(secret, 5, 3).unwrap();

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
                    let parts = vec![out[i].clone(), out[j].clone(), out[k].clone()];
                    let recomb = shamir_combine(&parts).unwrap();
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

        // Verify that ANY combination of `threshold` shares reconstructs the key
        for i in 0..parts {
            for j in (i + 1)..parts {
                permutations += 1;

                let subset = vec![shares[i].clone(), shares[j].clone()];

                log::info!("combined: {:?}", subset);

                let reconstructed = shamir_combine(&subset).unwrap();

                log::info!("reconstructed: {:?}", reconstructed);

                assert_eq!(
                    key.to_vec(),
                    reconstructed,
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
        let secret = b"0xdeadbeef";
        let shares = shamir_split(secret, 10, 5).unwrap();
        let recovered = shamir_combine(&shares[2..7]).unwrap(); // any 5 shares
        assert_eq!(secret.to_vec(), recovered);

        let secret = "0xcafe".to_string();
        let shares = shamir_split(secret.as_bytes(), 7, 4).unwrap();
        let recovered_bytes = shamir_combine(&shares[0..4]).unwrap();
        let recovered = String::from_utf8(recovered_bytes).unwrap();
        assert_eq!(secret, recovered);

        let secret: Vec<u8> = vec![0x01, 0x02, 0x03, 0xFF, 0xAA];
        let shares = shamir_split(&secret, 8, 3).unwrap();
        let recovered = shamir_combine(&shares[3..6]).unwrap();
        assert_eq!(secret, recovered);

        let secret: [u8; 32] = [0x42; 32]; // 256-bit key
        let shares = shamir_split(&secret, 6, 4).unwrap();

        let recovered: Vec<u8> = shamir_combine(&shares[1..5]).unwrap();
        let recovered_array: [u8; 32] = recovered.try_into().unwrap();
        assert_eq!(secret, recovered_array);

        let number: u128 = 12345678901234567890;
        let secret_bytes = number.to_le_bytes();

        let shares = shamir_split(&secret_bytes, 5, 3).unwrap();
        let recovered_bytes = shamir_combine(&shares[0..3]).unwrap();
        let recovered_number = u128::from_le_bytes(recovered_bytes.try_into().unwrap());
        assert_eq!(number, recovered_number);
    }
}
