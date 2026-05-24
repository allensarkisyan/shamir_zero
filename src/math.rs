use rand::{TryRng, rngs::SysRng};

/// Represents a polynomial of arbitrary degree over GF(2^8)
pub(crate) struct Polynomial {
    coefficients: Vec<u8>,
}

impl Polynomial {
    /// Constructs a random polynomial of the given degree with the provided intercept value.
    pub fn new(intercept: u8, degree: u8) -> Result<Self, String> {
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
pub(crate) fn add(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Multiplies two numbers in GF(2^8) - Branchless GF(2^8) multiplication
#[inline(always)]
pub(crate) fn mult(a: u8, b: u8) -> u8 {
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
pub fn inverse(a: u8) -> u8 {
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

/// Divides two numbers in GF(2^8)
#[inline(always)]
pub(crate) fn div(a: u8, b: u8) -> u8 {
    if b == 0 {
        return 0;
    }

    mult(a, inverse(b))
}

/// Takes N sample points and returns the value at a given x using Lagrange interpolation over GF(2^8).
#[inline(always)]
pub(crate) fn interpolate_polynomial(x_samples: &[u8], y_samples: &[u8], x: u8) -> u8 {
    let n = x_samples.len();

    if n != y_samples.len() {
        return 0;
    }

    match n {
        0 => return 0,
        1 => return y_samples[0],
        _ => {}
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
    fn test_interpolate_rand() {
        for i in 0..=255u8 {
            let p = Polynomial::new(i, 2).unwrap();
            let x_vals = [1u8, 2, 3];
            let y_vals = [p.evaluate(1), p.evaluate(2), p.evaluate(3)];
            let out = interpolate_polynomial(&x_vals, &y_vals, 0);
            assert_eq!(out, i, "Failed for intercept {}", i);
        }
    }
}
