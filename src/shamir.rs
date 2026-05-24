use crate::math::{Polynomial, interpolate_polynomial};
use rand::seq::SliceRandom;
use rand::{TryRng, rngs::SysRng};

#[derive(Debug, PartialEq)]
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
    // `parts` must be at least two bytes.
    MinimumPartByteLength,
    // All `parts` must be the same length.
    PartsLengthMismatch,
}

/// Split takes an arbitrarily long secret and generates a `parts`
/// number of shares, `threshold` of which are required to reconstruct
/// the secret. The parts and threshold must be at least 2, and less
/// than 256. The returned shares are each one byte longer than the secret
/// as they attach a tag used to reconstruct the secret.
pub fn shamir_split(
    secret: &[u8],
    parts: usize,
    threshold: usize,
) -> Result<Vec<Vec<u8>>, ShamirError> {
    if secret.is_empty() || !(2..=255).contains(&threshold) || parts < threshold || parts > 255 {
        return Err(if secret.is_empty() {
            ShamirError::EmptySecret
        } else if threshold < 2 {
            ShamirError::ThresholdLessThanMinimumLength
        } else if threshold > 255 {
            ShamirError::ThresholdExceedsMaximumLength
        } else if parts < threshold {
            ShamirError::PartsLessThanThresholdLength
        } else {
            ShamirError::PartsExceedMaximumLength
        });
    }

    let n = secret.len();
    let degree = (threshold - 1) as u8;
    let share_len = n + 1;
    let mut shares = Vec::with_capacity(parts);

    for i in 1..=parts {
        let mut share = vec![0u8; share_len];
        share[n] = i as u8;
        shares.push(share);
    }

    for (byte_idx, &secret_byte) in secret.iter().enumerate() {
        let p = Polynomial::new(secret_byte, degree)
            .map_err(|e| ShamirError::FailedToGeneratePolynomial)?;

        // Evaluate at every x using the fast Horner `evaluate`
        for (share_idx, share) in shares.iter_mut().enumerate() {
            let x = (share_idx + 1) as u8;
            share[byte_idx] = p.evaluate(x);
        }
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

    let mut x_samples = vec![0u8; n];
    let mut seen = [false; 256];

    for (i, part) in parts.iter().enumerate() {
        if part.len() != share_len {
            return Err(ShamirError::PartsLengthMismatch);
        }
        let x = part[share_len - 1];
        if seen[x as usize] {
            return Err(ShamirError::DuplicatePartDetected);
        }
        seen[x as usize] = true;
        x_samples[i] = x;
    }

    // Reuse this buffer for every byte
    let mut y_samples = vec![0u8; n];

    let mut secret = vec![0u8; secret_len];

    for byte_idx in 0..secret_len {
        // Fill y_samples for this byte position
        for (i, part) in parts.iter().enumerate() {
            y_samples[i] = part[byte_idx];
        }

        secret[byte_idx] = interpolate_polynomial(&x_samples, &y_samples, 0);
    }

    Ok(secret)
}
