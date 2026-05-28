// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (C) 2026 Allen Sarkisyan

use crate::math::{compute_lagrange_basis_at_zero, mult};
use rand::{TryRng, rngs::SysRng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShamirError {
    /// `parts` cannot be less than `threshold`.
    PartsLessThanThresholdLength,
    /// `parts` cannot exceed the maximum value of `255`.
    PartsExceedMaximumLength,
    /// `threshold` is less than the minimum value of `2`.
    ThresholdLessThanMinimumLength,
    /// `threshold` cannot exceed the maximum value of `255`.
    ThresholdExceedsMaximumLength,
    /// The `secret` value cannot be empty.
    EmptySecret,
    /// Failed to generate Polynomial.
    FailedToGeneratePolynomial,
    /// Duplicate `part` detected.
    DuplicatePartDetected,
    /// Less than two `parts` cannot be used to reconstruct the `secret`.
    RequiredMinimumParts,
    /// `parts` must be at least two bytes.
    MinimumPartByteLength,
    /// All `parts` must be the same length.
    PartsLengthMismatch,
    /// x-value of a secret share cannot be 0
    InvalidShareXValue,
}

/// Split takes an arbitrarily long secret and generates a `parts`
/// number of shares, `threshold` of which are required to reconstruct
/// the secret. The parts and threshold must be at least 2, and less
/// than 256. The returned shares are each one byte longer than the secret
/// as they attach a tag used to reconstruct the secret.
///
/// # Examples
///
/// ```
/// use shamir_zero::{shamir_split, shamir_combine};
///
/// let secret_key = b"top secret security key";
///
/// let secret_shares = shamir_split(secret_key, 5, 2).expect("valid params");
///
/// let recovered = shamir_combine(&secret_shares[0..3]).expect("valid shares");
///
/// assert_eq!(secret_key.to_vec(), recovered);
/// ```
pub fn shamir_split(
    secret: &[u8],
    parts: usize,
    threshold: usize,
) -> Result<Vec<Vec<u8>>, ShamirError> {
    if secret.is_empty() || !(2..=255).contains(&threshold) || parts < threshold || parts > 255 {
        return Err(match () {
            _ if secret.is_empty() => ShamirError::EmptySecret,
            _ if threshold < 2 => ShamirError::ThresholdLessThanMinimumLength,
            _ if threshold > 255 => ShamirError::ThresholdExceedsMaximumLength,
            _ if parts < threshold => ShamirError::PartsLessThanThresholdLength,
            _ => ShamirError::PartsExceedMaximumLength,
        });
    }

    let n = secret.len();
    let degree = threshold - 1;
    let share_len = n + 1;
    let total_random = n * degree;

    let mut shares = Vec::with_capacity(parts);
    let mut random_coeffs = vec![0u8; total_random];
    let mut rand_offset = 0usize;

    for i in 1..=parts {
        let mut share = vec![0u8; share_len];
        share[n] = i as u8;
        shares.push(share);
    }

    // Bulk-generate all random coefficients
    if total_random > 0 {
        SysRng
            .try_fill_bytes(&mut random_coeffs)
            .map_err(|_| ShamirError::FailedToGeneratePolynomial)?;
    }

    for (byte_idx, &secret_byte) in secret.iter().enumerate() {
        let poly_randoms = &random_coeffs[rand_offset..rand_offset + degree];

        // Fully inlined Horner evaluation
        for (share_idx, share) in shares.iter_mut().enumerate() {
            let x = (share_idx + 1) as u8;

            let mut result = poly_randoms[degree - 1];

            for k in (0..degree.saturating_sub(1)).rev() {
                result = mult(result, x) ^ poly_randoms[k];
            }

            result = mult(result, x) ^ secret_byte;
            share[byte_idx] = result;
        }

        rand_offset += degree;
    }

    Ok(shares)
}

/// Combine is used to reverse a Split and reconstruct a secret
/// once a `threshold` number of parts are available.
#[inline]
pub fn shamir_combine(parts: &[Vec<u8>]) -> Result<Vec<u8>, ShamirError> {
    let n = parts.len();
    if n < 2 {
        return Err(ShamirError::RequiredMinimumParts);
    }

    let share_len = match parts.first() {
        Some(p) if p.len() >= 2 => p.len(),
        _ => return Err(ShamirError::MinimumPartByteLength),
    };

    let secret_len = share_len - 1;

    let mut x_samples = [0u8; 256];
    let mut seen = [false; 256];

    for (i, part) in parts.iter().enumerate() {
        if part.len() != share_len {
            return Err(ShamirError::PartsLengthMismatch);
        }
        let x = part[share_len - 1];
        if x == 0 {
            return Err(ShamirError::InvalidShareXValue);
        }
        if seen[x as usize] {
            return Err(ShamirError::DuplicatePartDetected);
        }
        seen[x as usize] = true;
        x_samples[i] = x;
    }

    // Precompute Lagrange basis coefficients on stack, zero-copy
    let mut basis = [0u8; 256];
    compute_lagrange_basis_at_zero(&x_samples[..n], &mut basis[..n]);

    let mut secret = vec![0u8; secret_len];

    for byte_idx in 0..secret_len {
        let mut val = 0u8;
        for i in 0..n {
            val ^= mult(parts[i][byte_idx], basis[i]);
        }
        secret[byte_idx] = val;
    }

    Ok(secret)
}
