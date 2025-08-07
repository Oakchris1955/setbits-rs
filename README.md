# `setbits`: Easily and efficiently create bitmasks

[![CI Status](https://github.com/Oakchris1955/setbits-rs/actions/workflows/test.yml/badge.svg)](https://github.com/Oakchris1955/setbits-rs/actions/workflows/test.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/setbits)](https://crates.io/crates/setbits)
[![Documentation](https://docs.rs/setbits/badge.svg)](https://docs.rs/setbits)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.78+-blue.svg)](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0/)

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
