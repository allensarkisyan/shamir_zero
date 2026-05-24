use rand::{TryRng, rngs::SysRng};

/// Represents a polynomial of arbitrary degree over GF(2^8)
pub(crate) struct Polynomial {
    coefficients: Vec<u8>,
}

impl Polynomial {
    /// Constructs a random polynomial of the given degree with the provided intercept value.
    pub fn new(intercept: u8, degree: u8) -> Result<Self, Box<dyn std::error::Error>> {
        let mut coefficients = vec![0u8; (degree + 1) as usize];
        coefficients[0] = intercept;

        SysRng.try_fill_bytes(&mut coefficients[1..]).ok();
        Ok(Polynomial { coefficients })
    }

    /// Returns the value of the polynomial for the given x
    #[inline(always)]
    pub fn evaluate(&self, x: u8) -> u8 {
        if x == 0 {
            return self.coefficients[0];
        }

        // Compute the polynomial value using Horner's method. - start with highest coefficient
        let mut result = *self.coefficients.last().unwrap_or(&0);

        // Iterate in reverse order, skipping the leading coefficient
        for &coeff in self.coefficients.iter().rev().skip(1) {
            result = mult(result, x) ^ coeff;
        }

        result
    }
}

/// Combines two numbers in GF(2^8) (XOR)
#[inline(always)]
pub(crate) const fn add(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Multiplies two numbers in GF(2^8) - Branchless GF(2^8) multiplication
#[inline(always)]
pub(crate) const fn mult(a: u8, b: u8) -> u8 {
    let mut r = 0u8;
    let mut i = 8u8;

    // Iterate from MSB (7) down to LSB (0)
    while i > 0 {
        i -= 1;

        // 0x00 or 0xFF
        let mask = 0u8.wrapping_sub((b >> i) & 1);
        let reduce = 0u8.wrapping_sub((r >> 7) & 1);

        r = (mask & a) ^ (reduce & 0x1B) ^ (r << 1);
    }
    r
}

/// Calculates the inverse of a number in GF(2^8) (AES field / a^254)
/// Equivalent to a^254. Uses the optimized 11-multiplication chain.
/// The inverse of `a` in GF(2^8) is `a^254`.
#[inline(always)]
const fn inverse_11x(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }

    let mut b = mult(a, a);
    let mut c = mult(a, b);

    b = mult(c, c);
    b = mult(b, b);
    c = mult(b, c);
    b = mult(b, b);
    b = mult(b, b);
    b = mult(b, c);
    b = mult(b, b);
    b = mult(a, b);

    mult(b, b)
}

/// Helper that generates the 256-byte inverse lookup table at compile time.
const fn generate_inverse_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        table[i] = inverse_11x(i as u8);
        i += 1;
    }
    table
}

/// 256-byte lookup table for multiplicative inverse in GF(2^8) (a^254).
const INV_TABLE: [u8; 256] = generate_inverse_table();

/// Calculates the inverse of a number in GF(2^8) (a^254)
pub(crate) const fn inverse(a: u8) -> u8 {
    INV_TABLE[a as usize]
}

/// Divides two numbers in GF(2^8)
#[inline(always)]
pub(crate) const fn div(a: u8, b: u8) -> u8 {
    if b == 0 {
        return 0;
    }

    mult(a, inverse(b))
}

/// Takes N sample points and returns the value at a given x using Lagrange interpolation over GF(2^8).
#[inline(always)]
pub(crate) fn interpolate_polynomial(x_samples: &[u8], y_samples: &[u8], x: u8) -> u8 {
    let n = x_samples.len();

    if n != y_samples.len() || n == 0 {
        return 0;
    }

    if n == 1 {
        return y_samples[0];
    }

    // Precompute delta_inv[i]
    let mut delta_inv = vec![0u8; n];

    for i in 0..n {
        let xi = x_samples[i];
        let mut prod = 1u8;

        for j in 0..n {
            if i != j {
                prod = mult(prod, xi ^ x_samples[j]);
            }
        }

        delta_inv[i] = inverse(prod);
    }

    let mut result = 0u8;

    for i in 0..n {
        let mut num = 1u8;

        for j in 0..n {
            if i != j {
                num = mult(num, x ^ x_samples[j]);
            }
        }

        let basis = mult(num, delta_inv[i]);

        result ^= mult(y_samples[i], basis);
    }

    result
}

#[cfg(test)]
mod shamir_math_tests {
    use super::*;

    #[test]
    fn test_field_add() {
        assert_eq!(add(16, 16), 0);
        assert_eq!(add(3, 4), 7);
    }

    #[test]
    fn test_field_mult() {
        assert_eq!(mult(3, 7), 9);
        assert_eq!(mult(3, 0), 0);
        assert_eq!(mult(0, 3), 0);
    }

    #[test]
    fn test_field_divide() {
        assert_eq!(div(0, 7), 0);
        assert_eq!(div(3, 3), 1);
        assert_eq!(div(6, 3), 2);
        assert_eq!(div(12, 0), 0);
    }

    #[test]
    fn test_polynomial_random() {
        let p = Polynomial::new(42, 2).unwrap();
        assert_eq!(p.coefficients[0], 42);
    }

    #[test]
    fn test_polynomial_eval() {
        let p = Polynomial::new(42, 1).unwrap();
        assert_eq!(p.evaluate(0), 42);
        let exp = add(42, mult(1, p.coefficients[1]));
        assert_eq!(p.evaluate(1), exp);
    }

    #[test]
    fn test_interpolate() {
        let out = interpolate_polynomial(&[0, 1, 2], &[1], 0);
        assert_eq!(out, 0);

        let out = interpolate_polynomial(&[], &[], 3);
        assert_eq!(out, 0);

        let out = interpolate_polynomial(&[8], &[11], 3);
        assert_eq!(out, 11);
    }

    #[test]
    fn test_interpolate_rand() {
        for i in 0..=255u8 {
            let p = Polynomial::new(i, 2).unwrap();
            let x_vals = [1u8, 2, 3];
            let y_vals = [p.evaluate(1), p.evaluate(2), p.evaluate(3)];
            let out = interpolate_polynomial(&x_vals, &y_vals, 0);
            assert_eq!(out, i, "Failed for intercept {}", i);
        }
    }

    /// Comprehensive unit test for the reference inverse implementation (`inverse_11x`).
    /// This verifies the mathematical properties of the GF(2^8) inverse (a^254):
    #[test]
    fn test_inverse_11x() {
        // 1. Zero and identity edge cases
        assert_eq!(inverse_11x(0), 0, "inverse(0) must be 0");
        assert_eq!(inverse_11x(1), 1, "inverse(1) must be 1");

        // 2. Exhaustive correctness test over the entire field (256 values)
        for a in 0u8..=255 {
            let inv = inverse_11x(a);

            // a * inv(a) == 1 (when a != 0)
            if a != 0 {
                let product = mult(a, inv);
                assert_eq!(
                    product, 1,
                    "Round-trip failure for a = 0x{:02x}: a * inv(a) = 0x{:02x} (expected 1)",
                    a, product
                );
            }

            // involution: inv(inv(a)) == a
            let inv_inv = inverse_11x(inv);
            assert_eq!(
                inv_inv, a,
                "Involution failure for a = 0x{:02x}: inv(inv(a)) = 0x{:02x} (expected 0x{:02x})",
                a, inv_inv, a
            );
        }

        // 3. Must be identical to the fast lookup table (single source of truth)
        for a in 0u8..=255 {
            assert_eq!(
                inverse_11x(a),
                INV_TABLE[a as usize],
                "inverse_11x(0x{:02x}) does not match INV_TABLE entry",
                a
            );
        }

        // 4. Well-known test vectors from the GF(2^8) field (AES S-box related)
        let known_cases = [
            (0x00, 0x00),
            (0x01, 0x01),
            (0x02, 0x8d),
            (0x03, 0xf6),
            (0x53, 0xca),
            (0xff, 0x1c),
        ];

        for (a, expected) in known_cases {
            let actual = inverse_11x(a);
            assert_eq!(
                actual, expected,
                "Known value failed: inverse(0x{:02x}) = 0x{:02x} (expected 0x{:02x})",
                a, actual, expected
            );
        }

        println!("✅ All tests passed for inverse_11x (256 values fully verified)");
    }

    #[test]
    fn generates_inverse_lut() {
        let gen_lut = generate_inverse_table();
        let inverse_table: [u8; 256] = [
            0, 1, 141, 246, 203, 82, 123, 209, 232, 79, 41, 192, 176, 225, 229, 199, 116, 180, 170,
            75, 153, 43, 96, 95, 88, 63, 253, 204, 255, 64, 238, 178, 58, 110, 90, 241, 85, 77,
            168, 201, 193, 10, 152, 21, 48, 68, 162, 194, 44, 69, 146, 108, 243, 57, 102, 66, 242,
            53, 32, 111, 119, 187, 89, 25, 29, 254, 55, 103, 45, 49, 245, 105, 167, 100, 171, 19,
            84, 37, 233, 9, 237, 92, 5, 202, 76, 36, 135, 191, 24, 62, 34, 240, 81, 236, 97, 23,
            22, 94, 175, 211, 73, 166, 54, 67, 244, 71, 145, 223, 51, 147, 33, 59, 121, 183, 151,
            133, 16, 181, 186, 60, 182, 112, 208, 6, 161, 250, 129, 130, 131, 126, 127, 128, 150,
            115, 190, 86, 155, 158, 149, 217, 247, 2, 185, 164, 222, 106, 50, 109, 216, 138, 132,
            114, 42, 20, 159, 136, 249, 220, 137, 154, 251, 124, 46, 195, 143, 184, 101, 72, 38,
            200, 18, 74, 206, 231, 210, 98, 12, 224, 31, 239, 17, 117, 120, 113, 165, 142, 118, 61,
            189, 188, 134, 87, 11, 40, 47, 163, 218, 212, 228, 15, 169, 39, 83, 4, 27, 252, 172,
            230, 122, 7, 174, 99, 197, 219, 226, 234, 148, 139, 196, 213, 157, 248, 144, 107, 177,
            13, 214, 235, 198, 14, 207, 173, 8, 78, 215, 227, 93, 80, 30, 179, 91, 35, 56, 52, 104,
            70, 3, 140, 221, 156, 125, 160, 205, 26, 65, 28,
        ];

        assert_eq!(INV_TABLE, inverse_table);
        assert_eq!(gen_lut, inverse_table);
    }
}
