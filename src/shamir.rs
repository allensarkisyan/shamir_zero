// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (C) 2026 Allen Sarkisyan

use crate::math::{compute_lagrange_basis_at_zero, mult};
use rand::{TryRng, rngs::SysRng};

/// A high-level convenience wrapper around the zero-copy `shamir_split` and `shamir_combine` core APIs.
///
/// This struct provides a familiar `Vec<Vec<u8>>` interface while internally allocating
/// exactly once and delegating to the optimized, zero-copy core implementations.
///
/// # Examples
///
/// ```
/// use shamir_zero::{ShamirZero, ShamirError};
///
/// let secret = b"top secret security key";
/// let shares = ShamirZero::split(secret, 5, 3).unwrap();
/// let recovered = ShamirZero::combine(&shares[0..3]).unwrap();
/// assert_eq!(recovered, secret);
/// ```
pub struct ShamirZero;

impl ShamirZero {
    /// Splits a secret into `parts` shares, requiring `threshold` shares to reconstruct.
    /// Internally allocates exactly once and delegates to the zero-copy `shamir_split`.
    ///
    /// # Examples
    ///
    /// ```
    /// use shamir_zero::{ShamirZero, ShamirError};
    ///
    /// let secret = b"test secret";
    /// let shares = ShamirZero::split(secret, 3, 2).unwrap();
    /// assert_eq!(shares.len(), 3);
    /// assert_eq!(shares[0].len(), secret.len() + 1);
    /// ```
    pub fn split(
        secret: &[u8],
        parts: usize,
        threshold: usize,
    ) -> Result<Vec<Vec<u8>>, ShamirError> {
        let share_len = secret.len() + 1;

        let mut shares = vec![vec![0u8; share_len]; parts];
        let mut shares_out: Vec<&mut [u8]> = shares.iter_mut().map(|v| v.as_mut_slice()).collect();

        shamir_split(secret, parts, threshold, &mut shares_out)?;

        Ok(shares)
    }

    /// Reconstructs the secret from `threshold` or more shares.
    /// Internally allocates exactly once and delegates to the zero-copy `shamir_combine`.
    ///
    /// # Examples
    ///
    /// ```
    /// use shamir_zero::{ShamirZero, ShamirError};
    ///
    /// let secret = b"test secret";
    /// let shares = ShamirZero::split(secret, 3, 2).unwrap();
    /// let recovered = ShamirZero::combine(&shares[0..2]).unwrap();
    /// assert_eq!(recovered, secret);
    /// ```
    pub fn combine(parts: &[Vec<u8>]) -> Result<Vec<u8>, ShamirError> {
        if parts.len() < 2 {
            return Err(ShamirError::RequiredMinimumParts);
        }

        let share_len = match parts.first() {
            Some(p) if p.len() >= 2 => p.len(),
            _ => return Err(ShamirError::MinimumPartByteLength),
        };

        if parts.iter().any(|p| p.len() != share_len) {
            return Err(ShamirError::PartsLengthMismatch);
        }

        let secret_len = share_len - 1;
        let mut recovered = vec![0u8; secret_len];

        let slices: Vec<&[u8]> = parts.iter().map(|s| s.as_slice()).collect();

        shamir_combine(&slices, &mut recovered)?;

        Ok(recovered)
    }
}

#[derive(Debug, PartialEq, Eq)]
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
    /// The provided output buffer length does not match the expected secret length (`share_len - 1`).
    InvalidOutputLength,
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
/// let parts = 5;
/// let threshold = 3;
///
/// // Caller pre-allocates exactly once
/// let mut shares_out = vec![vec![0u8; secret_key.len() + 1]; parts];
/// let mut shares_out_slices: Vec<&mut [u8]> = shares_out.iter_mut()
///     .map(|v| v.as_mut_slice())
///     .collect();
///
/// shamir_split(secret_key, parts, threshold, &mut shares_out_slices).unwrap();
///
/// let share_slices: Vec<&[u8]> = shares_out[0..3]
///     .iter()
///     .map(|s| s.as_slice())
///     .collect();
///
/// let mut recovered = vec![0u8; secret_key.len()];
/// shamir_combine(&share_slices, &mut recovered).unwrap();
///
/// assert_eq!(secret_key, recovered.as_slice());
/// ```
pub fn shamir_split(
    secret: &[u8],
    parts: usize,
    threshold: usize,
    shares_out: &mut [&mut [u8]],
) -> Result<(), ShamirError> {
    if secret.is_empty() || !(2..=255).contains(&threshold) || parts < threshold || parts > 255 {
        return Err(match () {
            _ if secret.is_empty() => ShamirError::EmptySecret,
            _ if threshold < 2 => ShamirError::ThresholdLessThanMinimumLength,
            _ if threshold > 255 => ShamirError::ThresholdExceedsMaximumLength,
            _ if parts < threshold => ShamirError::PartsLessThanThresholdLength,
            _ => ShamirError::PartsExceedMaximumLength,
        });
    }

    if shares_out.len() != parts {
        return Err(ShamirError::InvalidOutputLength);
    }

    let n = secret.len();
    let degree = threshold - 1;
    let share_len = n + 1;
    let total_random = n * degree;

    for (i, share) in shares_out.iter_mut().enumerate() {
        if share.len() != share_len {
            return Err(ShamirError::InvalidOutputLength);
        }
        share[n] = (i + 1) as u8;
    }

    // Bulk-generate all random coefficients
    // TODO: add feature flag for `rand` usage
    let mut random_coeffs = vec![0u8; total_random];
    SysRng
        .try_fill_bytes(&mut random_coeffs)
        .map_err(|_| ShamirError::FailedToGeneratePolynomial)?;

    // Fully inlined Horner evaluation
    for (share_idx, share) in shares_out.iter_mut().enumerate() {
        let x = (share_idx + 1) as u8;

        for byte_idx in 0..n {
            // Compute offset based on byte index, as coefficients are laid out byte-by-byte
            let poly_offset = byte_idx * degree;
            let poly_randoms = &random_coeffs[poly_offset..poly_offset + degree];

            let mut result = poly_randoms[degree - 1];

            for k in (0..degree.saturating_sub(1)).rev() {
                result = mult(result, x) ^ poly_randoms[k];
            }

            result = mult(result, x) ^ secret[byte_idx];
            share[byte_idx] = result;
        }
    }

    Ok(())
}

/// Combine is used to reverse a Split and reconstruct a secret
/// once a `threshold` number of parts are available.
#[inline(always)]
pub fn shamir_combine(parts: &[&[u8]], secret_out: &mut [u8]) -> Result<(), ShamirError> {
    let n = parts.len();
    if n < 2 {
        return Err(ShamirError::RequiredMinimumParts);
    }

    let share_len = match parts.first() {
        Some(p) if p.len() >= 2 => p.len(),
        _ => return Err(ShamirError::MinimumPartByteLength),
    };

    let secret_len = share_len - 1;
    if secret_out.len() != secret_len {
        return Err(ShamirError::InvalidOutputLength);
    }

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

    for byte_idx in 0..secret_len {
        let mut val = 0u8;
        for i in 0..n {
            val ^= mult(parts[i][byte_idx], basis[i]);
        }
        secret_out[byte_idx] = val;
    }

    Ok(())
}
