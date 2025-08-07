# `setbits`: Easily and efficiently create bitmasks

Create an unsigned integer with the high/low `n` bits set to `1`

All functions are `const`

This crate can be particularly useful for creating bit masks

## Features

The `inline` feature is enabled by default and inlines all the functions
with a suggestion that they should be inline (`#[inline]`)

## Attribution

Original code for `setbits_u32_lo` taken from:
<https://users.rust-lang.org/t/how-to-make-an-integer-with-n-bits-set-without-overflow/63078/3>

## LICENSE

[MIT](LICENSE)
