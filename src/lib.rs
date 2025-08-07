//! Create an unsigned integer with the high/low `n` bits set to `1`
//!
//! All functions are `const`
//!
//! This crate can be particularly useful for creating bit masks
//!
//! ## Features
//!
//! The `inline` feature is enabled by default and inlines all the functions
//! with a suggestion that they should be inline (`#[inline]`)
//!
//! ## Attribution
//!
//! Original code for [`setbits_u32_lo`] taken from:
//! <https://users.rust-lang.org/t/how-to-make-an-integer-with-n-bits-set-without-overflow/63078/3>

use akin::akin;

akin! {
    let &uint_type = [u8, u16, u32, u64, u128];

    #[inline]
    const fn shift_bits_~*uint_type(n: u8) -> u32 {
        if *uint_type::BITS > n as u32 {
            *uint_type::BITS - n as u32
        } else {
            0
        }
    }

    /// Create a [`*uint_type`] with the low `n` bits set to `1`
    #[cfg_attr(feature = "inline", inline)]
    pub const fn setbits_~*uint_type_lo(n: u8) -> *uint_type {
        *uint_type::MAX >> shift_bits_~*uint_type(n)
    }

    /// Create a [`*uint_type`] with the high `n` bits set to `1`
    #[inline]
    pub const fn setbits_~*uint_type_hi(n: u8) -> *uint_type {
        *uint_type::MAX << shift_bits_~*uint_type(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_lo_check() {
        assert_eq!(setbits_u8_lo(3), 0b111);
        assert_eq!(setbits_u16_lo(13), 0b1111111111111);
        assert_eq!(setbits_u32_lo(25), 0b1111111111111111111111111);
        assert_eq!(
            setbits_u64_lo(53),
            0b11111111111111111111111111111111111111111111111111111
        );
        assert_eq!(setbits_u128_lo(107), 0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111);
    }

    #[test]
    fn set_hi_check() {
        assert_eq!(setbits_u8_hi(3), 0b11100000);
        assert_eq!(setbits_u16_hi(12), 0b1111111111110000);
        assert_eq!(setbits_u32_hi(25), 0b11111111111111111111111110000000);
        assert_eq!(
            setbits_u64_hi(53),
            0b1111111111111111111111111111111111111111111111111111100000000000
        );
        assert_eq!(setbits_u128_hi(107), 0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111000000000000000000000);
    }

    #[test]
    fn overflow_check() {
        akin! {
            let &uint_type = [u8, u16, u32, u64, u128];

            let bits = setbits_~*uint_type_lo((*uint_type::BITS + 1) as u8);
            assert_eq!(bits, *uint_type::MAX);
        }
    }
}
