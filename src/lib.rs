//! [![github]](https://github.com/andunieee/lowercase-hex)&ensp;[![crates-io]](https://crates.io/crates/lowercase-hex)&ensp;[![docs-rs]](https://docs.rs/lowercase-hex)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! This crate provides a fast conversion of byte arrays to **lowercase** hexadecimal strings,
//! both at compile time, and at run time.
//!
//! This is a fork of <https://github.com/danipopes/const-hex> that enforces strict lowercase.
//!
//! _Version requirement: rustc 1.64+_

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(
    feature = "nightly",
    feature(core_intrinsics, inline_const),
    allow(internal_features, stable_features)
)]
#![cfg_attr(feature = "portable-simd", feature(portable_simd))]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    clippy::missing_const_for_fn,
    clippy::missing_inline_in_public_items,
    clippy::all,
    rustdoc::all
)]
#![cfg_attr(not(any(test, feature = "__fuzzing")), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![allow(
    clippy::cast_lossless,
    clippy::inline_always,
    clippy::let_unit_value,
    clippy::must_use_candidate,
    clippy::wildcard_imports,
    unsafe_op_in_unsafe_fn,
    unused_unsafe
)]

#[cfg(feature = "alloc")]
#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

use cfg_if::cfg_if;

#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

// `cpufeatures` may be unused when `force-generic` is enabled.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use cpufeatures as _;

mod arch;
use arch::imp;

mod impl_core;

pub mod traits;
#[cfg(feature = "alloc")]
pub use traits::ToHexExt;

// If the `hex` feature is enabled, re-export the `hex` crate's traits.
// Otherwise, use our own with the more optimized implementation.
cfg_if! {
    if #[cfg(feature = "hex")] {
        pub use hex;
        #[doc(inline)]
        pub use hex::{FromHex, FromHexError, ToHex};
    } else {
        mod error;
        pub use error::FromHexError;

        #[allow(deprecated)]
        pub use traits::{FromHex, ToHex};
    }
}

// Support for nightly features.
cfg_if! {
    if #[cfg(feature = "nightly")] {
        // Branch prediction hints.
        #[allow(unused_imports)]
        use core::intrinsics::{likely, unlikely};

        // `inline_const`: [#76001](https://github.com/rust-lang/rust/issues/76001)
        macro_rules! maybe_const_assert {
            ($($tt:tt)*) => {
                const { assert!($($tt)*) }
            };
        }
    } else {
        #[allow(unused_imports)]
        use core::convert::{identity as likely, identity as unlikely};

        macro_rules! maybe_const_assert {
            ($($tt:tt)*) => {
                assert!($($tt)*)
            };
        }
    }
}

// Serde support.
cfg_if! {
    if #[cfg(feature = "serde")] {
        pub mod serde;

        #[doc(no_inline)]
        pub use self::serde::deserialize;
        #[cfg(feature = "alloc")]
        #[doc(no_inline)]
        pub use self::serde::serialize;
    }
}

mod buffer;
pub use buffer::Buffer;

/// The table of lowercase characters used for hex encoding.
pub const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";

/// The lookup table of hex byte to value, used for hex decoding.
///
/// [`NIL`] is used for invalid values.
pub const HEX_DECODE_LUT: &[u8; 256] = &make_decode_lut();

/// Represents an invalid value in the [`HEX_DECODE_LUT`] table.
pub const NIL: u8 = u8::MAX;

/// Encodes `input` as a hex string into a [`Buffer`].
///
/// # Examples
///
/// ```
/// const BUFFER: lowercase_hex::Buffer<4> = lowercase_hex::const_encode(b"kiwi");
/// assert_eq!(BUFFER.as_str(), "6b697769");
/// ```
#[inline]
pub const fn const_encode<const N: usize, const PREFIX: bool>(
    input: &[u8; N],
) -> Buffer<N, PREFIX> {
    Buffer::new().const_format(input)
}

/// Encodes `input` as a hex string using lowercase characters into a mutable
/// slice of bytes `output`.
///
/// # Errors
///
/// If the output buffer is not exactly `input.len() * 2` bytes long.
///
/// # Examples
///
/// ```
/// let mut bytes = [0u8; 4 * 2];
/// lowercase_hex::encode_to_slice(b"kiwi", &mut bytes)?;
/// assert_eq!(&bytes, b"6b697769");
/// # Ok::<_, lowercase_hex::FromHexError>(())
/// ```
#[inline]
pub fn encode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<(), FromHexError> {
    encode_to_slice_inner(input.as_ref(), output)
}

/// Encodes `data` as a hex string using lowercase characters.
///
/// Lowercase characters are used (e.g. `f9b4ca`). The resulting string's
/// length is always even, each byte in `data` is always encoded using two hex
/// digits. Thus, the resulting string contains exactly twice as many bytes as
/// the input data.
///
/// # Examples
///
/// ```
/// assert_eq!(lowercase_hex::encode("Hello world!"), "48656c6c6f20776f726c6421");
/// assert_eq!(lowercase_hex::encode([1, 2, 3, 15, 16]), "0102030f10");
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    encode_inner::<false>(data.as_ref())
}

/// Decode a hex string into a fixed-length byte-array.
///
/// Only lowercase characters are valid in the input string (e.g. `f9b4ca`).
///
/// Prefer using [`decode_to_array`] instead when possible (at runtime), as it is likely to be faster.
///
/// # Errors
///
/// This function returns an error if the input is not an even number of
/// characters long or contains invalid hex characters, or if the input is not
/// exactly `N * 2` bytes long.
///
/// # Example
///
/// ```
/// const _: () = {
///     let bytes = lowercase_hex::const_decode_to_array(b"6b697769");
///     assert!(matches!(bytes.as_ref(), Ok(b"kiwi")));
/// };
/// ```
#[inline]
pub const fn const_decode_to_array<const N: usize>(input: &[u8]) -> Result<[u8; N], FromHexError> {
    if input.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }
    if input.len() != N * 2 {
        return Err(FromHexError::InvalidStringLength);
    }
    match const_decode_to_array_impl(input) {
        Some(output) => Ok(output),
        None => Err(unsafe { invalid_hex_error(input) }),
    }
}

const fn const_decode_to_array_impl<const N: usize>(input: &[u8]) -> Option<[u8; N]> {
    macro_rules! next {
        ($var:ident, $i:expr) => {
            let hex = unsafe { *input.as_ptr().add($i) };
            let $var = HEX_DECODE_LUT[hex as usize];
            if $var == NIL {
                return None;
            }
        };
    }

    let mut output = [0; N];
    debug_assert!(input.len() == N * 2);
    let mut i = 0;
    while i < output.len() {
        next!(high, i * 2);
        next!(low, i * 2 + 1);
        output[i] = high << 4 | low;
        i += 1;
    }
    Some(output)
}

/// Decodes a hex string into raw bytes.
///
/// Only lowercase characters are valid in the input string (e.g. `f9b4ca`).
///
/// # Errors
///
/// This function returns an error if the input is not an even number of
/// characters long or contains invalid hex characters.
///
/// # Example
///
/// ```
/// assert_eq!(
///     lowercase_hex::decode("48656c6c6f20776f726c6421"),
///     Ok("Hello world!".to_owned().into_bytes())
/// );
///
/// assert_eq!(lowercase_hex::decode("123"), Err(lowercase_hex::FromHexError::OddLength));
/// assert!(lowercase_hex::decode("foo").is_err());
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, FromHexError> {
    fn decode_inner(input: &[u8]) -> Result<Vec<u8>, FromHexError> {
        if unlikely(input.len() % 2 != 0) {
            return Err(FromHexError::OddLength);
        }

        // Do not initialize memory since it will be entirely overwritten.
        let len = input.len() / 2;
        let mut output = Vec::with_capacity(len);
        // SAFETY: The entire vec is never read from, and gets dropped if decoding fails.
        #[allow(clippy::uninit_vec)]
        unsafe {
            output.set_len(len);
        }

        // SAFETY: Lengths are checked above.
        unsafe { decode_checked(input, &mut output) }.map(|()| output)
    }

    decode_inner(input.as_ref())
}

/// Decode a hex string into a mutable bytes slice.
///
/// Only lowercase characters are valid in the input string (e.g. `f9b4ca`).
///
/// # Errors
///
/// This function returns an error if the input is not an even number of
/// characters long or contains invalid hex characters, or if the output slice
/// is not exactly half the length of the input.
///
/// # Example
///
/// ```
/// let mut bytes = [0u8; 4];
/// lowercase_hex::decode_to_slice("6b697769", &mut bytes).unwrap();
/// assert_eq!(&bytes, b"kiwi");
///
/// let res = lowercase_hex::decode_to_slice("6B697769", &mut bytes);
/// assert!(res.is_err());
/// ```
#[inline]
pub fn decode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<(), FromHexError> {
    decode_to_slice_inner(input.as_ref(), output)
}

/// Decode a hex string into a fixed-length byte-array.
///
/// Only lowercase characters are valid in the input string (e.g. `f9b4ca`).
///
/// # Errors
///
/// This function returns an error if the input is not an even number of
/// characters long or contains invalid hex characters, or if the input is not
/// exactly `N / 2` bytes long.
///
/// # Example
///
/// ```
/// let bytes = lowercase_hex::decode_to_array(b"6b697769").unwrap();
/// assert_eq!(&bytes, b"kiwi");
/// ```
#[inline]
pub fn decode_to_array<T: AsRef<[u8]>, const N: usize>(input: T) -> Result<[u8; N], FromHexError> {
    fn decode_to_array_inner<const N: usize>(input: &[u8]) -> Result<[u8; N], FromHexError> {
        let mut output = impl_core::uninit_array();
        // SAFETY: The entire array is never read from.
        let output_slice = unsafe { impl_core::slice_assume_init_mut(&mut output) };
        // SAFETY: All elements are initialized.
        decode_to_slice_inner(input, output_slice)
            .map(|()| unsafe { impl_core::array_assume_init(output) })
    }

    decode_to_array_inner(input.as_ref())
}

#[cfg(feature = "alloc")]
fn encode_inner<const PREFIX: bool>(data: &[u8]) -> String {
    let capacity = PREFIX as usize * 2 + data.len() * 2;
    let mut buf = Vec::<u8>::with_capacity(capacity);
    // SAFETY: The entire vec is never read from, and gets dropped if decoding fails.
    #[allow(clippy::uninit_vec)]
    unsafe {
        buf.set_len(capacity)
    };
    let mut output = buf.as_mut_ptr();
    if PREFIX {
        // SAFETY: `output` is long enough.
        unsafe {
            output.add(0).write(b'0');
            output.add(1).write(b'x');
            output = output.add(2);
        }
    }
    // SAFETY: `output` is long enough (input.len() * 2).
    unsafe { imp::encode(data, output) };
    // SAFETY: We only write only ASCII bytes.
    unsafe { String::from_utf8_unchecked(buf) }
}

fn encode_to_slice_inner(input: &[u8], output: &mut [u8]) -> Result<(), FromHexError> {
    if unlikely(output.len() != 2 * input.len()) {
        return Err(FromHexError::InvalidStringLength);
    }
    // SAFETY: Lengths are checked above.
    unsafe { imp::encode(input, output.as_mut_ptr()) };
    Ok(())
}

fn decode_to_slice_inner(input: &[u8], output: &mut [u8]) -> Result<(), FromHexError> {
    if unlikely(input.len() % 2 != 0) {
        return Err(FromHexError::OddLength);
    }
    if unlikely(output.len() != input.len() / 2) {
        return Err(FromHexError::InvalidStringLength);
    }
    // SAFETY: Lengths are checked above.
    unsafe { decode_checked(input, output) }
}

/// # Safety
///
/// Assumes `output.len() == input.len() / 2`.
#[inline]
unsafe fn decode_checked(input: &[u8], output: &mut [u8]) -> Result<(), FromHexError> {
    debug_assert_eq!(output.len(), input.len() / 2);

    if imp::USE_CHECK_FN {
        // check then decode
        if imp::check(input) {
            unsafe { imp::decode_unchecked(input, output) };
            return Ok(());
        }
    } else {
        // check and decode at the same time
        if unsafe { imp::decode_checked(input, output) } {
            return Ok(());
        }
    }

    Err(unsafe { invalid_hex_error(input) })
}

#[inline]
const fn byte2hex(byte: u8) -> (u8, u8) {
    let table = HEX_CHARS_LOWER;
    let high = table[(byte >> 4) as usize];
    let low = table[(byte & 0x0f) as usize];
    (high, low)
}

/// Creates an invalid hex error from the input.
///
/// # Safety
///
/// Assumes `input` contains at least one invalid character.
#[cold]
#[cfg_attr(debug_assertions, track_caller)]
const unsafe fn invalid_hex_error(input: &[u8]) -> FromHexError {
    // Find the first invalid character.
    let mut index = None;
    let mut iter = input;
    while let [byte, rest @ ..] = iter {
        if HEX_DECODE_LUT[*byte as usize] == NIL {
            index = Some(input.len() - rest.len() - 1);
            break;
        }
        iter = rest;
    }

    let index = match index {
        Some(index) => index,
        None => {
            if cfg!(debug_assertions) {
                panic!("input was valid but `check` failed")
            } else {
                unsafe { core::hint::unreachable_unchecked() }
            }
        }
    };

    FromHexError::InvalidHexCharacter {
        c: input[index] as char,
        index,
    }
}

const fn make_decode_lut() -> [u8; 256] {
    let mut lut = [0; 256];
    let mut i = 0u8;
    loop {
        lut[i as usize] = match i {
            b'0'..=b'9' => i - b'0',
            b'a'..=b'f' => i - b'a' + 10,
            // use max value for invalid characters
            _ => NIL,
        };
        if i == NIL {
            break;
        }
        i += 1;
    }
    lut
}

#[allow(
    missing_docs,
    unused,
    clippy::all,
    clippy::missing_inline_in_public_items
)]
#[cfg(all(feature = "__fuzzing", not(miri)))]
#[doc(hidden)]
pub mod fuzzing {
    use super::*;
    use proptest::test_runner::TestCaseResult;
    use proptest::{prop_assert, prop_assert_eq};
    use std::fmt::Write;

    pub fn fuzz(data: &[u8]) -> TestCaseResult {
        self::encode(&data)?;
        self::decode(&data)?;
        Ok(())
    }

    pub fn encode(input: &[u8]) -> TestCaseResult {
        test_buffer::<8, 16>(input)?;
        test_buffer::<20, 40>(input)?;
        test_buffer::<32, 64>(input)?;
        test_buffer::<64, 128>(input)?;
        test_buffer::<128, 256>(input)?;

        let encoded = crate::encode(input);
        let expected = mk_expected(input);
        prop_assert_eq!(&encoded, &expected);

        let decoded = crate::decode(&encoded).unwrap();
        prop_assert_eq!(decoded, input);

        Ok(())
    }

    pub fn decode(input: &[u8]) -> TestCaseResult {
        if let Ok(decoded) = crate::decode(input) {
            let input_len = input.len() / 2;
            prop_assert_eq!(decoded.len(), input_len);
        }

        Ok(())
    }

    fn mk_expected(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for i in bytes {
            write!(s, "{i:02x}").unwrap();
        }
        s
    }

    fn test_buffer<const N: usize, const LEN: usize>(bytes: &[u8]) -> TestCaseResult {
        if let Ok(bytes) = <&[u8; N]>::try_from(bytes) {
            let mut buffer = Buffer::<N, false>::new();
            let string = buffer.format(bytes).to_string();
            prop_assert_eq!(string.len(), bytes.len() * 2);
            prop_assert_eq!(string.as_bytes(), buffer.as_byte_array::<LEN>());
            prop_assert_eq!(string.as_str(), buffer.as_str());
            prop_assert_eq!(string.as_str(), mk_expected(bytes));

            let mut buffer = Buffer::<N, true>::new();
            let prefixed = buffer.format(bytes).to_string();
            prop_assert_eq!(prefixed.len(), 2 + bytes.len() * 2);
            prop_assert_eq!(prefixed.as_str(), buffer.as_str());
            prop_assert_eq!(prefixed.as_str(), format!("0x{string}"));

            prop_assert_eq!(decode_to_array(&string), Ok(*bytes));
            prop_assert_eq!(decode_to_array(&prefixed), Ok(*bytes));
            prop_assert_eq!(const_decode_to_array(string.as_bytes()), Ok(*bytes));
            prop_assert_eq!(const_decode_to_array(prefixed.as_bytes()), Ok(*bytes));
        }

        Ok(())
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig {
            cases: 1024,
            ..Default::default()
        })]

        #[test]
        fn fuzz_encode(s in ".+") {
            encode(s.as_bytes())?;
        }

        #[test]
        fn fuzz_check_true(s in "[0-9a-f]+") {
            let s = s.as_bytes();
            prop_assert!(crate::check_raw(s));
            prop_assert!(crate::const_check_raw(s));
            if s.len() % 2 == 0 {
                prop_assert!(crate::check(s).is_ok());
                prop_assert!(crate::const_check(s).is_ok());
            }
        }

        #[test]
        fn fuzz_check_false(s in ".{16}[0-9a-f]+") {
            let s = s.as_bytes();
            prop_assert!(crate::check(s).is_err());
            prop_assert!(crate::const_check(s).is_err());
            prop_assert!(!crate::check_raw(s));
            prop_assert!(!crate::const_check_raw(s));
        }
    }
}

/// Returns `true` if the input is a valid hex string and can be decoded successfully.
///
/// Prefer using [`check`] instead when possible (at runtime), as it is likely to be faster.
///
/// # Examples
///
/// ```
/// const _: () = {
///     assert!(lowercase_hex::const_check(b"48656c6c6f20776f726c6421").is_ok());
///
///     assert!(lowercase_hex::const_check(b"48656c6c6f20776f726c642").is_err());
///     assert!(lowercase_hex::const_check(b"Hello world!").is_err());
/// };
/// ```
#[inline]
pub const fn const_check(input: &[u8]) -> Result<(), FromHexError> {
    if input.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }
    if const_check_raw(input) {
        Ok(())
    } else {
        Err(unsafe { invalid_hex_error(input) })
    }
}

/// Returns `true` if the input is a valid hex string.
///
/// Note that this does not check prefixes or length, but just the contents of the string.
///
/// Prefer using [`check_raw`] instead when possible (at runtime), as it is likely to be faster.
///
/// # Examples
///
/// ```
/// const _: () = {
///     assert!(lowercase_hex::const_check_raw(b"48656c6c6f20776f726c6421"));
///
///     // Odd length, but valid hex
///     assert!(lowercase_hex::const_check_raw(b"48656c6c6f20776f726c642"));
///
///     // Valid hex string, but the prefix is not valid
///     assert!(!lowercase_hex::const_check_raw(b"0x48656c6c6f20776f726c6421"));
///
///     assert!(!lowercase_hex::const_check_raw(b"Hello world!"));
/// };
/// ```
#[inline]
pub const fn const_check_raw(input: &[u8]) -> bool {
    let mut i = 0;
    while i < input.len() {
        let byte = input[i];
        if HEX_DECODE_LUT[byte as usize] == NIL {
            return false;
        }
        i += 1;
    }
    true
}

/// Returns `true` if the input is a valid hex string and can be decoded successfully.
///
/// # Examples
///
/// ```
/// assert!(lowercase_hex::check("48656c6c6f20776f726c6421").is_ok());
///
/// assert!(lowercase_hex::check("48656c6c6f20776f726c642").is_err());
/// assert!(lowercase_hex::check("Hello world!").is_err());
/// ```
#[inline]
pub fn check<T: AsRef<[u8]>>(input: T) -> Result<(), FromHexError> {
    #[allow(clippy::missing_const_for_fn)]
    fn check_inner(input: &[u8]) -> Result<(), FromHexError> {
        if input.len() % 2 != 0 {
            return Err(FromHexError::OddLength);
        }
        if imp::check(input) {
            Ok(())
        } else {
            Err(unsafe { invalid_hex_error(input) })
        }
    }

    check_inner(input.as_ref())
}

/// Returns `true` if the input is a valid hex string.
///
/// Note that this does not check prefixes or length, but just the contents of the string.
///
/// # Examples
///
/// ```
/// assert!(lowercase_hex::check_raw("48656c6c6f20776f726c6421"));
///
/// // Odd length, but valid hex
/// assert!(lowercase_hex::check_raw("48656c6c6f20776f726c642"));
///
/// // Valid hex string, but the prefix is not valid
/// assert!(!lowercase_hex::check_raw("0x48656c6c6f20776f726c6421"));
///
/// assert!(!lowercase_hex::check_raw("Hello world!"));
/// ```
#[inline]
pub fn check_raw<T: AsRef<[u8]>>(input: T) -> bool {
    imp::check(input.as_ref())
}
