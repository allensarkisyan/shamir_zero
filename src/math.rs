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
}
