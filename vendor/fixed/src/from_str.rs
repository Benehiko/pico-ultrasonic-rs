// Copyright © 2018–2023 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

use crate::{
    bytes::{DigitsExp, DigitsUnds},
    types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8},
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroU32,
    str::FromStr,
};
#[cfg(feature = "std")]
use std::error::Error;

// NOTE ON dec_to_bin
//
// dec_to_bin: Decode fractional decimal digits into nbits fractional bits.
//
// For an output with BIN = 8 bits, we can take DEC = 3 decimal digits.
//
//     0 ≤ val ≤ 999, 0 ≤ nbits ≤ 8
//
// In general,
//
//     0 ≤ val ≤ 10^DEC - 1, 0 ≤ nbits ≤ BIN
//
// We can either floor the result or round to the nearest, with ties rounded to
// even. If rounding results in more than nbits bits, returns None.
//
// Examples: (for DEC = 3, BIN = 8)
//
//    dec_to_bin(999, 8, Round::Floor) -> floor(999 × 256 / 1000) -> 255 -> Some(255)
//    dec_to_bin(999, 8, Round::Nearest) -> floor(999 × 256 / 1000 + 0.5) -> 256 -> None
//    dec_to_bin(999, 5, Round::Floor) -> floor(999 × 32 / 1000) -> 31 -> Some(31)
//    dec_to_bin(999, 5, Round::Nearest) -> floor(999 × 32 / 1000 + 0.5) -> 32 -> None
//    dec_to_bin(499, 0, Round::Floor) -> floor(499 / 1000) -> 0 -> Some(0)
//    dec_to_bin(499, 0, Round::Nearest) -> floor(499 / 1000 + 0.5) -> 0 -> Some(0)
//    dec_to_bin(500, 0, Round::Nearest) -> floor(500 / 1000 + 0.5) -> 1 -> None
//
// For flooring:
//
//     floor(val × 2^nbits / 10^3) = floor(val × 2^(nbits - 3) / 5^3)
//
// For rounding:
//
//     floor(val × 2^nbits / 10^3 + 0.5) = floor((val × 2^(nbits - 2) + 5^3) / (2 × 5^3))
//
// Using integer arithmetic, this is equal to:
//
//     ((val << 6 >> (8 - nbits)) + if rounding { 125 } else { 0 }) / 250
//
// Note that val << 6 cannot overflow u16, as val < 1000 and 1000 × 2^6 < 2^16.
//
// In general:
//
//     ((val << (BIN - DEC + 1) >> (8 - nbits)) + if rounding { 5^DEC } else { 0 }) / (2 × 5^DEC)
//
// And we ensure that 10^DEC × 2^(BIN - DEC + 1) < 2^(2 × BIN), which simplifies to
//
//     5^DEC × 2 < 2^BIN
//
// From this it also follows that val << (BIN - DEC + 1) never overflows a (2 × BIN)-bit number.
//
// So for u8, BIN = 8, DEC ≤ 3
// So for u16, BIN = 16, DEC ≤ 6
// So for u32, BIN = 32, DEC ≤ 13
// So for u64, BIN = 64, DEC ≤ 27
// So for u128, BIN = 128, DEC ≤ 54
//
// END NOTE ON dec_to_bin

// Expanded on all signed and unsigned integers.
// This should be expanded in a module named like the integer.
//
//   * Defines:
//       - pub const fn from_str_radix
//       - pub const fn saturating_from_str_radix
//       - pub const fn wrapping_from_str_radix
//       - pub const fn overflowing_from_str_radix
macro_rules! all {
    ($Single:ident) => {
        use crate::from_str::{ParseErrorKind, ParseFixedError, Sep};

        #[inline]
        pub const fn from_str_radix(
            s: &str,
            radix: u32,
            frac_nbits: u32,
        ) -> Result<$Single, ParseFixedError> {
            match overflowing_from_str_radix(s, radix, frac_nbits) {
                Ok((val, false)) => Ok(val),
                Ok((_, true)) => Err(ParseFixedError {
                    kind: ParseErrorKind::Overflow,
                }),
                Err(e) => Err(e),
            }
        }

        #[inline]
        pub const fn saturating_from_str_radix(
            s: &str,
            radix: u32,
            frac_nbits: u32,
        ) -> Result<$Single, ParseFixedError> {
            match overflowing_from_str_radix(s, radix, frac_nbits) {
                Ok((val, false)) => Ok(val),
                Ok((_, true)) => {
                    let bytes = s.as_bytes();
                    let starts_with_minus = match bytes.first() {
                        Some(s) => *s == b'-',
                        None => false,
                    };
                    if starts_with_minus {
                        Ok($Single::MIN)
                    } else {
                        Ok($Single::MAX)
                    }
                }
                Err(e) => Err(e),
            }
        }

        #[inline]
        pub const fn wrapping_from_str_radix(
            s: &str,
            radix: u32,
            frac_nbits: u32,
        ) -> Result<$Single, ParseFixedError> {
            match overflowing_from_str_radix(s, radix, frac_nbits) {
                Ok((val, _)) => Ok(val),
                Err(e) => Err(e),
            }
        }

        #[inline]
        pub const fn overflowing_from_str_radix(
            s: &str,
            radix: u32,
            frac_nbits: u32,
        ) -> Result<($Single, bool), ParseFixedError> {
            let bytes = s.as_bytes();
            match from_str(bytes, radix, Sep::Error, frac_nbits) {
                Ok(val) => Ok(val),
                Err(kind) => Err(ParseFixedError { kind }),
            }
        }
    };
}

// Expanded on all signed integers.
//
//   * Creates a module named like the integer.
//   * Expands `all` macro.
//   * Defines:
//       - pub const fn from_str
//       - pub const fn lit
macro_rules! signed {
    ($Single:ident, $Uns:ident) => {
        pub mod $Single {
            all! { $Single }

            pub(super) const fn from_str(
                bytes: &[u8],
                radix: u32,
                sep: Sep,
                frac_nbits: u32,
            ) -> Result<($Single, bool), ParseErrorKind> {
                let (neg, abs, mut overflow) = match crate::from_str::$Uns::get_int_frac(
                    bytes,
                    radix,
                    sep,
                    $Single::BITS - frac_nbits,
                    frac_nbits,
                ) {
                    Ok((neg, abs, overflow)) => (neg, abs, overflow),
                    Err(e) => return Err(e),
                };
                let bound = if !neg { $Single::MAX } else { $Single::MIN };
                if abs > bound.unsigned_abs() {
                    overflow = true;
                }
                let abs = if neg { abs.wrapping_neg() } else { abs } as $Single;
                Ok((abs, overflow))
            }

            pub const fn lit(s: &str, frac_nbits: u32) -> Result<$Single, ParseFixedError> {
                let mut bytes = s.as_bytes();
                if bytes.is_empty() {
                    return Err(ParseFixedError {
                        kind: ParseErrorKind::NoDigits,
                    });
                }
                let neg = if bytes[0] == b'-' {
                    bytes = bytes.split_at(1).1;
                    true
                } else {
                    false
                };
                let abs = match crate::from_str::$Uns::lit_no_sign(bytes, frac_nbits) {
                    Ok(val) => val,
                    Err(kind) => return Err(ParseFixedError { kind }),
                };
                let bound = if !neg { $Single::MAX } else { $Single::MIN };
                if abs > bound.unsigned_abs() {
                    return Err(ParseFixedError {
                        kind: ParseErrorKind::Overflow,
                    });
                }
                let val = if neg { abs.wrapping_neg() } else { abs } as $Single;
                Ok(val)
            }
        }
    };
}

signed! { i8, u8 }
signed! { i16, u16 }
signed! { i32, u32 }
signed! { i64, u64 }
signed! { i128, u128 }

// Expanded on all unsigned integers.
//
//   * Expands `all` macro.
//   * Defines:
//       - pub(super) const fn from_str
//       - pub const fn lit
//       - pub(super) const fn lit_no_sign
//       - pub(super) const fn get_int_frac
//       - pub(super) const fn get_int
//       - pub(super) const fn get_frac
//       - const fn bin_str_int_to_bin
//       - const fn bin_str_frac_to_bin
//       - const fn oct_str_int_to_bin
//       - const fn oct_str_frac_to_bin
//       - const fn hex_str_int_to_bin
//       - const fn hex_str_frac_to_bin
//       - pub(super) const fn dec_str_int_to_bin
//       - const fn dec_str_frac_to_bin
//       - const fn from_byte
//       - pub(super) const fn is_odd
macro_rules! unsigned {
    ($Uns:ident $(, $Half:ident)?) => {
        use crate::from_str::{
            frac_is_half, parse_bounds, unchecked_hex_digit, BitExp, DigitsExp, Parse, Round,
        };

        all! { $Uns }

        pub(super) const fn from_str(
            bytes: &[u8],
            radix: u32,
            sep: Sep,
            frac_nbits: u32,
        ) -> Result<($Uns, bool), ParseErrorKind> {
            let (neg, abs, mut overflow) =
                match get_int_frac(bytes, radix, sep, $Uns::BITS - frac_nbits, frac_nbits) {
                    Ok((neg, abs, overflow)) => (neg, abs, overflow),
                    Err(e) => return Err(e),
                };
            if neg && abs > 0 {
                overflow = true;
            }
            let abs = if neg { abs.wrapping_neg() } else { abs };
            Ok((abs, overflow))
        }

        #[inline]
        pub const fn lit(s: &str, frac_nbits: u32) -> Result<$Uns, ParseFixedError> {
            match lit_no_sign(s.as_bytes(), frac_nbits) {
                Ok(val) => Ok(val),
                Err(kind) => Err(ParseFixedError { kind }),
            }
        }

        pub(super) const fn lit_no_sign(
            mut bytes: &[u8],
            frac_nbits: u32,
        ) -> Result<$Uns, ParseErrorKind> {
            if bytes.is_empty() {
                return Err(ParseErrorKind::NoDigits);
            }
            let radix = if bytes.len() >= 2 && bytes[0] == b'0' {
                match bytes[1] {
                    b'b' => 2,
                    b'o' => 8,
                    b'x' => 16,
                    _ => 10,
                }
            } else {
                10
            };
            if radix != 10 {
                bytes = bytes.split_at(2).1;
                while let Some((b'_', rest)) = bytes.split_first() {
                    bytes = rest;
                }
            }
            if let Some((b'-' | b'+', _)) = bytes.split_first() {
                return Err(ParseErrorKind::MisplacedSign);
            }
            match from_str(bytes, radix, Sep::Skip, frac_nbits) {
                Ok((val, false)) => Ok(val),
                Ok((_, true)) => Err(ParseErrorKind::Overflow),
                Err(e) => Err(e),
            }
        }

        pub(super) const fn get_int_frac(
            bytes: &[u8],
            radix: u32,
            sep: Sep,
            int_nbits: u32,
            frac_nbits: u32,
        ) -> Result<(bool, $Uns, bool), ParseErrorKind> {
            let Parse {
                neg,
                int,
                frac,
                bit_exp,
            } = match parse_bounds(bytes, radix, sep) {
                Ok(o) => o,
                Err(e) => return Err(e),
            };
            let (int_val, mut overflow) = get_int(int, radix, int_nbits, bit_exp);
            let (frac_val, frac_overflow) = match get_frac(frac, radix, frac_nbits, bit_exp) {
                Some(val) => (val, false),
                None => (0, true),
            };
            let mut val = int_val | frac_val;
            // frac_overflow does not catch the case where:
            //  1. int is odd
            //  2. frac_nbits is 0
            //  3. frac_bytes is exactly half, e.g. "5" for decimal
            // In this case, get_frac returns 0.5 rounded to even 0.0,
            // as it does not have a way to know that int is odd.
            if frac_overflow || (is_odd(int_val) && frac_nbits == 0 && frac_is_half(frac, radix)) {
                let (new_val, new_overflow) = if int_nbits == 0 {
                    (val, true)
                } else {
                    val.overflowing_add(1 << frac_nbits)
                };
                if new_overflow {
                    overflow = true;
                }
                val = new_val;
            }
            Ok((neg, val, overflow))
        }

        pub(super) const fn get_int(
            int: DigitsExp,
            radix: u32,
            nbits: u32,
            bit_exp: Option<BitExp>,
        ) -> ($Uns, bool) {
            $(
                if nbits <= $Half::BITS {
                    let (half, overflow) =
                        crate::from_str::$Half::get_int(int, radix, nbits, bit_exp);
                    return ((half as $Uns) << $Half::BITS, overflow);
                }
            )?

            if int.is_empty() && bit_exp.is_none() {
                return (0, false);
            }
            let (mut parsed_int, mut overflow): ($Uns, bool) = match radix {
                2 => bin_str_int_to_bin(int),
                8 => oct_str_int_to_bin(int, bit_exp),
                16 => hex_str_int_to_bin(int, bit_exp),
                _ => {
                    debug_assert!(radix == 10);
                    dec_str_int_to_bin(int)
                }
            };
            let remove_bits = $Uns::BITS - nbits;
            if nbits == 0 {
                overflow = true;
                parsed_int = 0;
            } else if remove_bits > 0 {
                if (parsed_int >> nbits) != 0 {
                    overflow = true;
                }
                parsed_int <<= remove_bits;
            }
            (parsed_int, overflow)
        }

        pub(super) const fn get_frac(
            frac: DigitsExp,
            radix: u32,
            nbits: u32,
            bit_exp: Option<BitExp>,
        ) -> Option<$Uns> {
            $(
                if nbits <= $Half::BITS {
                    return match crate::from_str::$Half::get_frac(frac, radix, nbits, bit_exp) {
                        Some(half) => Some(half as $Uns),
                        None => None,
                    };
                }
            )?

            if frac.is_empty() {
                return Some(0);
            }
            match radix {
                2 => bin_str_frac_to_bin(frac, nbits),
                8 => oct_str_frac_to_bin(frac, nbits, bit_exp),
                16 => hex_str_frac_to_bin(frac, nbits, bit_exp),
                _ => {
                    debug_assert!(radix == 10);
                    dec_str_frac_to_bin(frac, nbits)
                }
            }
        }

        const fn bin_str_int_to_bin(digits: DigitsExp) -> ($Uns, bool) {
            let max_len = $Uns::BITS as usize;
            let (digits, overflow) = if digits.len() > max_len {
                let (_, last_max_len) = digits.split_at(digits.len() - max_len);
                (last_max_len, true)
            } else {
                (digits, false)
            };
            let mut acc = 0;
            let mut rem_digits = digits;
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                acc = (acc << 1) + from_byte(digit - b'0');
            }
            (acc, overflow)
        }

        const fn bin_str_frac_to_bin(digits: DigitsExp, nbits: u32) -> Option<$Uns> {
            let mut rem_bits = nbits;
            let mut acc = 0;
            let mut rem_digits = digits;
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                let val = digit - b'0';
                if rem_bits < 1 {
                    if val != 0 {
                        // half bit is true, round up if we have more
                        // significant bits or currently acc is odd
                        if !rem_digits.is_empty() || is_odd(acc) {
                            acc = match acc.checked_add(1) {
                                Some(acc) => acc,
                                None => return None,
                            };
                        }
                    }
                    if nbits != $Uns::BITS && acc >> nbits != 0 {
                        return None;
                    }
                    return Some(acc);
                }
                acc = (acc << 1) + from_byte(val);
                rem_bits -= 1;
            }
            Some(acc << rem_bits)
        }

        const fn oct_str_int_to_bin(digits: DigitsExp, bit_exp: Option<BitExp>) -> ($Uns, bool) {
            let (exp, exp_extra_digit) = match bit_exp {
                Some(s) => (s.exp.get(), s.first_frac_digit),
                None => (0, 0),
            };
            // handle condition where exp_extra_digit is only digit we have
            if digits.is_empty() {
                let val = (from_byte(exp_extra_digit - b'0')) >> (3 - exp);
                return (val, false)
            }
            // max_digits does not count exp_extra_digit, which holds exp bits we'll use.
            let max_digits = (($Uns::BITS - exp + 2) / 3) as usize;
            let (digits, mut overflow) = if digits.len() > max_digits {
                let (_, last_max_digits) = digits.split_at(digits.len() - max_digits);
                (last_max_digits, true)
            } else {
                (digits, false)
            };
            let Some((first_digit, mut rem_digits)) = digits.split_first() else {
                unreachable!();
            };
            let mut acc = from_byte(first_digit - b'0');
            if digits.len() == max_digits {
                let first_max_bits = $Uns::BITS - exp - (max_digits as u32 - 1) * 3;
                let first_max = (from_byte(1) << first_max_bits) - 1;
                if acc > first_max {
                    overflow = true;
                }
            }
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                acc = (acc << 3) + from_byte(digit - b'0');
            }
            if bit_exp.is_some() {
                let val = (exp_extra_digit - b'0') >> (3 - exp);
                acc = (acc << exp) + from_byte(val);
            }
            (acc, overflow)
        }

        const fn oct_str_frac_to_bin(
            digits: DigitsExp,
            nbits: u32,
            bit_exp: Option<BitExp>,
        ) -> Option<$Uns> {
            let mut rem_bits = nbits;
            let mut acc = 0;
            let mut rem_digits = digits;
            let mut val_bits = match bit_exp {
                Some(s) => 3 - s.exp.get(),
                None => 3,
            };
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                // If val_bits is not 3, we need to skip some bits
                let val = if val_bits != 3 {
                    let first_digit_mask = (1 << val_bits) - 1;
                    (digit - b'0') & first_digit_mask
                } else {
                    digit - b'0'
                };
                if rem_bits < val_bits {
                    acc = (acc << rem_bits) + from_byte(val >> (3 - rem_bits));
                    let half = 1 << (2 - rem_bits);
                    if val & half != 0 {
                        // half bit is true, round up if we have more
                        // significant bits or currently acc is odd
                        if val & (half - 1) != 0 || !rem_digits.is_empty() || is_odd(acc) {
                            acc = match acc.checked_add(1) {
                                Some(acc) => acc,
                                None => return None,
                            };
                        }
                    }
                    if nbits != $Uns::BITS && acc >> nbits != 0 {
                        return None;
                    }
                    return Some(acc);
                }
                acc = (acc << 3) + from_byte(val);
                rem_bits -= val_bits;
                val_bits = 3;
            }
            Some(acc << rem_bits)
        }

        const fn hex_str_int_to_bin(digits: DigitsExp, bit_exp: Option<BitExp>) -> ($Uns, bool) {
            let (exp, exp_extra_digit) = match bit_exp {
                Some(s) => (s.exp.get(), s.first_frac_digit),
                None => (0, 0),
            };
            // handle condition where exp_extra_digit is only digit we have
            if digits.is_empty() {
                let val = (from_byte(exp_extra_digit - b'0')) >> (3 - exp);
                return (val, false)
            }
            // max_digits does not count exp_extra_digit, which holds exp bits we'll use.
            let max_digits = (($Uns::BITS - exp + 3) / 4) as usize;
            let (digits, mut overflow) = if digits.len() > max_digits {
                let (_, last_max_digits) = digits.split_at(digits.len() - max_digits);
                (last_max_digits, true)
            } else {
                (digits, false)
            };
            let Some((first_digit, mut rem_digits)) = digits.split_first() else {
                unreachable!();
            };
            let mut acc = from_byte(unchecked_hex_digit(first_digit));
            if digits.len() == max_digits {
                let first_max_bits = $Uns::BITS - exp - (max_digits as u32 - 1) * 4;
                let first_max = (from_byte(1) << first_max_bits) - 1;
                if acc > first_max {
                    overflow = true;
                }
            }
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                acc = (acc << 4) + from_byte(unchecked_hex_digit(digit));
            }
            if bit_exp.is_some() {
                let val = unchecked_hex_digit(exp_extra_digit) >> (4 - exp);
                acc = (acc << exp) + from_byte(val);
            }
            (acc, overflow)
        }

        const fn hex_str_frac_to_bin(
            digits: DigitsExp,
            nbits: u32,
            bit_exp: Option<BitExp>,
        ) -> Option<$Uns> {
            let mut rem_bits = nbits;
            let mut acc = 0;
            let mut rem_digits = digits;
            let mut val_bits = match bit_exp {
                Some(s) => 4 - s.exp.get(),
                None => 4,
            };
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                // If val_bits is not 4, we need to skip some bits
                let val = if val_bits != 4 {
                    let first_digit_mask = (1 << val_bits) - 1;
                    unchecked_hex_digit(digit) & first_digit_mask
                } else {
                    unchecked_hex_digit(digit)
                };
                if rem_bits < val_bits {
                    acc = (acc << rem_bits) + from_byte(val >> (4 - rem_bits));
                    let half = 1 << (3 - rem_bits);
                    if val & half != 0 {
                        // half bit is true, round up if we have more
                        // significant bits or currently acc is odd
                        if val & (half - 1) != 0 || !rem_digits.is_empty() || is_odd(acc) {
                            acc = match acc.checked_add(1) {
                                Some(acc) => acc,
                                None => return None,
                            };
                        }
                    }
                    if nbits != $Uns::BITS && acc >> nbits != 0 {
                        return None;
                    }
                    return Some(acc);
                }
                acc = (acc << 4) + from_byte(val);
                rem_bits -= val_bits;
                val_bits = 4;
            }
            Some(acc << rem_bits)
        }

        pub(super) const fn dec_str_int_to_bin(digits: DigitsExp) -> ($Uns, bool) {
            let max_effective_len = $Uns::BITS as usize;
            let (digits, mut overflow) = if digits.len() > max_effective_len {
                let (_, last_max_effective_len) = digits.split_at(digits.len() - max_effective_len);
                (last_max_effective_len, true)
            } else {
                (digits, false)
            };
            let mut acc = 0;
            let mut rem_digits = digits;
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                let (prod, mul_overflow) = mul10_overflow(acc);
                let (add, add_overflow) = prod.overflowing_add(from_byte(digit - b'0'));
                acc = add;
                overflow = overflow || mul_overflow != 0 || add_overflow;
            }
            (acc, overflow)
        }

        const fn dec_str_frac_to_bin(digits: DigitsExp, nbits: u32) -> Option<$Uns> {
            let (val, is_short) = parse_is_short(digits);
            let one: $Uns = 1;
            let dump_bits = $Uns::BITS - nbits;
            // if is_short, dec_to_bin can round and give correct answer immediately
            let round = if is_short {
                Round::Nearest
            } else {
                Round::Floor
            };
            let Some(floor) = dec_to_bin(val, nbits, round) else {
                return None;
            };
            if is_short {
                return Some(floor);
            }
            // since !is_short, we have a floor and we have to check whether
            // we need to increment

            // add_5 is to add rounding when all bits are used
            let (mut boundary, mut add_5) = if nbits == 0 {
                (one << ($Uns::BITS - 1), false)
            } else if dump_bits == 0 {
                (floor, true)
            } else {
                ((floor << dump_bits) + (one << (dump_bits - 1)), false)
            };
            let mut tie = true;
            let mut rem_digits = digits;
            while let Some((digit, rem)) = rem_digits.split_first() {
                rem_digits = rem;

                if !add_5 && boundary == 0 {
                    // since zeros are trimmed, there must be some digit > 0 eventually
                    tie = false;
                    break;
                }
                let (prod, mut boundary_digit) = mul10_overflow(boundary);
                boundary = prod;
                if add_5 {
                    let (wrapped, overflow) = boundary.overflowing_add(5);
                    boundary = wrapped;
                    if overflow {
                        boundary_digit += 1;
                    }
                    add_5 = false;
                }
                if digit - b'0' < boundary_digit {
                    return Some(floor);
                }
                if digit - b'0' > boundary_digit {
                    tie = false;
                    break;
                }
            }
            if tie && !is_odd(floor) {
                return Some(floor);
            }
            let Some(next_up) = floor.checked_add(1) else  {
                return None;
            };
            if dump_bits != 0 && next_up >> nbits != 0 {
                None
            } else {
                Some(next_up)
            }
        }

        const fn from_byte(b: u8) -> $Uns {
            b as $Uns
        }

        pub(super) const fn is_odd(val: $Uns) -> bool {
            val & 1 != 0
        }
    };
}

// Expanded on all unsigned integers except u128.
//
//   * Creates a module named like the integer.
//   * Expands `unsigned` macro.
//   * Defines:
//       - const fn mul10_overflow
//       - pub(super) const fn dec_to_bin
//       - const fn parse_is_short
macro_rules! unsigned_not_u128 {
    ($Single:ident $(, $Half:ident)?; $Double:ident, $dec:expr, $bin:expr) => {
        pub mod $Single {
            unsigned! { $Single $(, $Half)? }

            #[inline]
            const fn mul10_overflow(x: $Single) -> ($Single, u8) {
                let prod = (x as $Double) * 10;
                (prod as $Single, (prod >> <$Single>::BITS) as u8)
            }

            pub(super) const fn dec_to_bin(
                val: $Double,
                nbits: u32,
                round: Round,
            ) -> Option<$Single> {
                debug_assert!(val < $Double::pow(10, $dec));
                debug_assert!(nbits <= $bin);
                let fives = $Double::pow(5, $dec);
                let denom = fives * 2;
                let mut numer = val << ($bin - $dec + 1) >> ($bin - nbits);
                match round {
                    Round::Nearest => {
                        // Round up, then round back down if we had a tie and the result is odd.
                        numer += fives;
                        // If unrounded division == 1 exactly, we actually have a tie at upper
                        // bound, which is rounded up to 1.0. This is even in all cases except
                        // when nbits == 0, in which case we must round it back down to 0.
                        if numer >> nbits >= denom {
                            // 0.5 exactly is 10^$dec / 2 = 5^dec * 2^dec / 2 = fives << ($dec - 1)
                            return if nbits == 0 && val == fives << ($dec - 1) {
                                Some(0)
                            } else {
                                None
                            };
                        }
                    }
                    Round::Floor => {}
                }
                let (mut div, tie) = (numer / denom, numer % denom == 0);
                if tie && crate::from_str::$Double::is_odd(div) {
                    div -= 1;
                }
                Some(div as $Single)
            }

            const fn parse_is_short(digits: DigitsExp) -> ($Double, bool) {
                let (is_short, slice, pad) =
                    if let Some(rem) = usize::checked_sub($dec, digits.len()) {
                        (true, digits, $Double::pow(10, rem as u32))
                    } else {
                        let (short, _) = digits.split_at($dec);
                        (false, short, 1)
                    };
                let val = crate::from_str::$Double::dec_str_int_to_bin(slice).0 * pad;
                (val, is_short)
            }
        }
    };
}

unsigned_not_u128! { u8; u16, 3, 8 }
unsigned_not_u128! { u16, u8; u32, 6, 16 }
unsigned_not_u128! { u32, u16; u64, 13, 32 }
unsigned_not_u128! { u64, u32; u128, 27, 64 }

pub mod u128 {
    unsigned! { u128, u64 }

    use crate::int256::{self, U256};
    use core::num::NonZeroU128;

    #[inline]
    const fn mul10_overflow(x: u128) -> (u128, u8) {
        const LO_MASK: u128 = !(!0 << 64);
        let hi = (x >> 64) * 10;
        let lo = (x & LO_MASK) * 10;
        // Generates better code than:
        //     let (wrapped, overflow) = (hi << 64).overflowing_add(lo);
        //     ((hi >> 64) as u8 + u8::from(overflow), wrapped)
        let (hi_lo, hi_hi) = (hi as u64, (hi >> 64) as u64);
        let (lo_lo, lo_hi) = (lo as u64, (lo >> 64) as u64);
        let (wrapped, overflow) = hi_lo.overflowing_add(lo_hi);
        (
            ((wrapped as u128) << 64) | (lo_lo as u128),
            (hi_hi as u8) + (overflow as u8),
        )
    }

    pub(super) const fn dec_to_bin(
        (hi, lo): (u128, u128),
        nbits: u32,
        round: Round,
    ) -> Option<u128> {
        debug_assert!(hi < 10u128.pow(27));
        debug_assert!(lo < 10u128.pow(27));
        debug_assert!(nbits <= 128);
        let fives = 5u128.pow(54);
        let denom = fives * 2;
        let Some(denom) = NonZeroU128::new(denom) else {
            unreachable!();
        };
        // we need to combine (10^27*hi + lo) << (128 - 54 + 1)
        let hi_e27 = int256::wide_mul_u128(hi, 10u128.pow(27));
        let (val_lo, overflow) = hi_e27.lo.overflowing_add(lo);
        let val_hi = hi_e27.hi + (overflow as u128);
        let (mut numer_lo, mut numer_hi) = (val_lo, val_hi);
        if nbits < (54 - 1) {
            let shr = (54 - 1) - nbits;
            numer_lo = (numer_lo >> shr) | (numer_hi << (128 - shr));
            numer_hi >>= shr;
        } else if nbits > (54 - 1) {
            let shl = nbits - (54 - 1);
            numer_hi = (numer_hi << shl) | (numer_lo >> (128 - shl));
            numer_lo <<= shl;
        }
        match round {
            Round::Nearest => {
                // Round up, then round back down if we had a tie and the result is odd.
                let (wrapped, overflow) = numer_lo.overflowing_add(fives);
                numer_lo = wrapped;
                if overflow {
                    numer_hi += 1;
                }
                let check_overflow = if nbits == 128 {
                    numer_hi
                } else if nbits == 0 {
                    numer_lo
                } else {
                    (numer_lo >> nbits) | (numer_hi << (128 - nbits))
                };
                // If unrounded division == 1 exactly, we actually have a tie at upper
                // bound, which is rounded up to 1.0. This is even in all cases except
                // when nbits == 0, in which case we must round it back down to 0.
                if check_overflow >= denom.get() {
                    // 0.5 exactly is 10^$dec / 2 = 5^dec * 2^dec / 2 = fives << ($dec - 1)
                    let half_hi = fives >> (128 - (54 - 1));
                    let half_lo = fives << (54 - 1);
                    return if nbits == 0 && val_hi == half_hi && val_lo == half_lo {
                        Some(0)
                    } else {
                        None
                    };
                }
            }
            Round::Floor => {}
        }
        let (mut div, tie) = div_tie(numer_hi, numer_lo, denom);
        if tie && is_odd(div) {
            div -= 1;
        }
        Some(div)
    }

    const fn parse_is_short(digits: DigitsExp) -> ((u128, u128), bool) {
        if let Some(rem) = 27usize.checked_sub(digits.len()) {
            let hi = dec_str_int_to_bin(digits).0 * 10u128.pow(rem as u32);
            ((hi, 0), true)
        } else {
            let (begin, end) = digits.split_at(27);
            let hi = dec_str_int_to_bin(begin).0;

            let (is_short, slice, pad) = if let Some(rem) = 54usize.checked_sub(digits.len()) {
                (true, end, 10u128.pow(rem as u32))
            } else {
                let (mid, _) = end.split_at(27);
                (false, mid, 1)
            };
            let lo = dec_str_int_to_bin(slice).0 * pad;
            ((hi, lo), is_short)
        }
    }

    const fn div_tie(dividend_hi: u128, dividend_lo: u128, divisor: NonZeroU128) -> (u128, bool) {
        let dividend = U256 {
            lo: dividend_lo,
            hi: dividend_hi,
        };
        let (quot, rem) = int256::div_rem_u256_u128(dividend, divisor);
        (quot.lo, rem == 0)
    }
}

const fn unchecked_hex_digit(byte: u8) -> u8 {
    // We know that byte is a valid hex:
    //   * b'0'..=b'9' (0x30..=0x39) => byte & 0x0f
    //   * b'A'..=b'F' (0x41..=0x46) => byte & 0x0f + 9
    //   * b'a'..=b'f' (0x61..=0x66) => byte & 0x0f + 9
    (byte & 0x0f) + if byte >= 0x40 { 9 } else { 0 }
}

#[derive(Clone, Copy, Debug)]
pub enum Round {
    Nearest,
    Floor,
}

#[derive(Clone, Copy, Debug)]
struct BitExp {
    exp: NonZeroU32,
    first_frac_digit: u8,
}

impl BitExp {
    const fn new(bit_exp: u32, frac: DigitsExp) -> Option<BitExp> {
        let Some(exp) = NonZeroU32::new(bit_exp) else {
            return None;
        };
        let first_frac_digit = match frac.split_first() {
            Some((digit, _)) => digit,
            None => b'0',
        };
        Some(BitExp {
            exp,
            first_frac_digit,
        })
    }
}

//   * bit_exp.exp can be {1,2} for octal or {1,2,3} for hex.
//   * For all other cases, bit_exp is None.
#[derive(Clone, Copy, Debug)]
struct Parse<'a> {
    neg: bool,
    int: DigitsExp<'a>,
    frac: DigitsExp<'a>,
    bit_exp: Option<BitExp>,
}

#[derive(Clone, Copy, Debug)]
pub enum Sep {
    Skip,
    Error,
}

/**
An error which can be returned when parsing a fixed-point number.

# Examples

```rust
use fixed::{types::I16F16, ParseFixedError};
// This string is not a fixed-point number.
let s = "something completely different (_!_!_)";
let error: ParseFixedError = match s.parse::<I16F16>() {
    Ok(_) => unreachable!(),
    Err(error) => error,
};
println!("Parse error: {error}");
```
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseFixedError {
    kind: ParseErrorKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParseErrorKind {
    InvalidDigit,
    MisplacedSign,
    MisplacedUnderscore,
    NoDigits,
    TooManyPoints,
    Overflow,
    ExpInvalidDigit,
    ExpNoDigits,
    TooManyExp,
    ExpOverflow,
}

impl ParseFixedError {
    #[inline]
    #[track_caller]
    pub(crate) const fn lit_message(self) -> &'static str {
        use self::ParseErrorKind::*;
        match self.kind {
            InvalidDigit => "invalid literal: invalid digit found in string",
            MisplacedSign => "invalid literal: misplaced sign found in string",
            MisplacedUnderscore => "invalid literal: misplaced underscore found in string",
            NoDigits => "invalid literal: string has no digits",
            TooManyPoints => "invalid literal: more than one point found in string",
            Overflow => "invalid literal: overflow",
            ExpInvalidDigit => "invalid literal: invalid digit found in exponent",
            ExpNoDigits => "invalid literal: exponent has no digits",
            TooManyExp => "invalid literal: more than one exponent found",
            ExpOverflow => "invalid literal: exponent overflow",
        }
    }

    #[inline]
    pub(crate) const fn message(self) -> &'static str {
        use self::ParseErrorKind::*;
        match self.kind {
            InvalidDigit => "invalid digit found in string",
            MisplacedSign => "misplaced sign found in string",
            MisplacedUnderscore => "misplaced underscore found in string",
            NoDigits => "string has no digits",
            TooManyPoints => "more than one point found in string",
            Overflow => "overflow",
            ExpInvalidDigit => "invalid digit found in exponent",
            ExpNoDigits => "exponent has no digits",
            TooManyExp => "more than one exponent found",
            ExpOverflow => "exponent overflow",
        }
    }
}

impl Display for ParseFixedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.message(), f)
    }
}

#[cfg(feature = "std")]
impl Error for ParseFixedError {
    fn description(&self) -> &str {
        self.message()
    }
}

// Zeros at start of int and at end of frac are trimmed.
// Leading underscores for either int or frac are never accepted, even for Sep::Skip.
const fn parse_bounds(bytes: &[u8], radix: u32, sep: Sep) -> Result<Parse<'_>, ParseErrorKind> {
    let mut sign: Option<bool> = None;
    let mut int_start: Option<usize> = None;
    let mut point: Option<usize> = None;
    let mut frac_end: Option<usize> = None;
    let mut has_int_digit = false;
    let mut has_frac_digit = false;
    let mut exp_sep: Option<usize> = None;
    let mut exp_sign: Option<bool> = None;
    let mut exp: i32 = 0;
    let mut has_exp_digit = false;
    let mut exp_is_bit = false;

    let mut next_index = 0;
    let mut rem_bytes = bytes;
    while let Some((&byte, rem)) = rem_bytes.split_first() {
        let index = next_index;
        next_index += 1;
        rem_bytes = rem;

        match (byte, radix) {
            (b'+', _) => {
                if exp_sep.is_none() {
                    if sign.is_some() || has_int_digit || point.is_some() {
                        return Err(ParseErrorKind::MisplacedSign);
                    }
                    sign = Some(false);
                } else {
                    if exp_sign.is_some() || has_exp_digit {
                        return Err(ParseErrorKind::MisplacedSign);
                    }
                    exp_sign = Some(false);
                }
            }
            (b'-', _) => {
                if exp_sep.is_none() {
                    if sign.is_some() || has_int_digit || point.is_some() {
                        return Err(ParseErrorKind::MisplacedSign);
                    }
                    sign = Some(true);
                } else {
                    if exp_sign.is_some() || has_exp_digit {
                        return Err(ParseErrorKind::MisplacedSign);
                    }
                    exp_sign = Some(true);
                }
            }
            (b'.', _) => {
                if exp_sep.is_some() {
                    return Err(ParseErrorKind::ExpInvalidDigit);
                }
                if point.is_some() {
                    return Err(ParseErrorKind::TooManyPoints);
                }
                point = Some(index);
                frac_end = Some(index + 1);
            }
            (b'_', _) => {
                if matches!(sep, Sep::Error) {
                    if exp_sep.is_some() {
                        return Err(ParseErrorKind::ExpInvalidDigit);
                    }
                    return Err(ParseErrorKind::InvalidDigit);
                };
                if (point.is_none() && exp_sep.is_none() && !has_int_digit)
                    || (point.is_some() && exp_sep.is_none() && !has_frac_digit)
                    || (exp_sep.is_some() && !has_exp_digit)
                {
                    return Err(ParseErrorKind::MisplacedUnderscore);
                }
            }
            (b'e' | b'E', 2 | 8 | 10) | (b'@', _) => {
                if exp_sep.is_some() {
                    return Err(ParseErrorKind::TooManyExp);
                }
                exp_sep = Some(index);
            }
            (b'p' | b'P', 2 | 8 | 16) => {
                if exp_sep.is_some() {
                    return Err(ParseErrorKind::TooManyExp);
                }
                exp_sep = Some(index);
                exp_is_bit = true;
            }
            (b'0'..=b'1', 2)
            | (b'0'..=b'7', 8)
            | (b'0'..=b'9', 10)
            | (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F', 16)
                if exp_sep.is_none() =>
            {
                if point.is_none() {
                    has_int_digit = true;
                    if int_start.is_none() && byte != b'0' {
                        int_start = Some(index);
                    }
                } else {
                    has_frac_digit = true;
                    if byte != b'0' {
                        frac_end = Some(index + 1);
                    }
                }
            }
            (b'0'..=b'9', _) if exp_sep.is_some() => {
                exp = match exp.checked_mul(10) {
                    Some(s) => s,
                    None => {
                        return Err(ParseErrorKind::ExpOverflow);
                    }
                };
                let add = match exp_sign {
                    Some(true) => -((byte - b'0') as i32),
                    Some(false) | None => (byte - b'0') as i32,
                };
                exp = match exp.checked_add(add) {
                    Some(s) => s,
                    None => {
                        return Err(ParseErrorKind::ExpOverflow);
                    }
                };
                has_exp_digit = true;
            }
            _ => {
                if exp_sep.is_some() {
                    return Err(ParseErrorKind::ExpInvalidDigit);
                }
                return Err(ParseErrorKind::InvalidDigit);
            }
        }
    }
    if !has_int_digit && !has_frac_digit {
        return Err(ParseErrorKind::NoDigits);
    }
    if exp_sep.is_some() && !has_exp_digit {
        return Err(ParseErrorKind::ExpNoDigits);
    }
    let neg = match sign {
        Some(s) => s,
        None => false,
    };
    let int = match (int_start, point, exp_sep) {
        (Some(begin), Some(end), _) | (Some(begin), None, Some(end)) => {
            let (up_to_end, _) = bytes.split_at(end);
            let (_, from_begin) = up_to_end.split_at(begin);
            DigitsUnds::new(from_begin)
        }
        (Some(begin), None, None) => {
            let (_, from_begin) = bytes.split_at(begin);
            DigitsUnds::new(from_begin)
        }
        (None, _, _) => DigitsUnds::EMPTY,
    };
    let frac = match (point, frac_end) {
        (Some(point), Some(end)) => {
            let (up_to_end, _) = bytes.split_at(end);
            let (_, from_after_point) = up_to_end.split_at(point + 1);
            DigitsUnds::new(from_after_point)
        }
        _ => DigitsUnds::EMPTY,
    };
    let bit_exp = if exp_is_bit {
        match radix {
            2 => 0,
            8 => {
                let (q, r) = (exp.div_euclid(3), exp.rem_euclid(3));
                exp = q;
                r.unsigned_abs()
            }
            16 => {
                let (q, r) = (exp.div_euclid(4), exp.rem_euclid(4));
                exp = q;
                r.unsigned_abs()
            }
            _ => unreachable!(),
        }
    } else {
        0
    };
    let Some((int, frac)) = DigitsExp::new_int_frac(int, frac, exp) else {
        return Err(ParseErrorKind::ExpOverflow);
    };
    let bit_exp = BitExp::new(bit_exp, frac);
    Ok(Parse {
        neg,
        int,
        frac,
        bit_exp,
    })
}

const fn frac_is_half(digits: DigitsExp, radix: u32) -> bool {
    // since zeros are trimmed, when the value is one half there has to be
    // exatly one digit, and rest has to be empty
    match digits.split_first() {
        Some((digit, rest)) => digit - b'0' == (radix as u8) / 2 && rest.is_empty(),
        None => false,
    }
}

macro_rules! impl_from_str {
    ($Fixed:ident, $LeEqU:ident) => {
        impl<Frac: $LeEqU> FromStr for $Fixed<Frac> {
            type Err = ParseFixedError;
            /// Parses a string slice to return a fixed-point number.
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_str(s)
            }
        }
    };
}
impl_from_str! { FixedI8, LeEqU8 }
impl_from_str! { FixedI16, LeEqU16 }
impl_from_str! { FixedI32, LeEqU32 }
impl_from_str! { FixedI64, LeEqU64 }
impl_from_str! { FixedI128, LeEqU128 }
impl_from_str! { FixedU8, LeEqU8 }
impl_from_str! { FixedU16, LeEqU16 }
impl_from_str! { FixedU32, LeEqU32 }
impl_from_str! { FixedU64, LeEqU64 }
impl_from_str! { FixedU128, LeEqU128 }

#[cfg(test)]
mod tests {
    use crate::{
        bytes::DigitsExp,
        from_str::{self, parse_bounds, Parse, ParseErrorKind, ParseFixedError, Round, Sep},
        types::*,
    };
    use std::{
        format,
        string::{String, ToString},
    };

    #[test]
    fn overflowing() {
        let overflow = ParseFixedError {
            kind: ParseErrorKind::Overflow,
        };
        assert_eq!(
            U4F4::overflowing_from_str("15.5"),
            Ok((U4F4::from_bits(0xF8), false))
        );
        assert_eq!(U4F4::from_str("15.5"), Ok(U4F4::from_bits(0xF8)));
        assert_eq!(
            U4F4::overflowing_from_str("31.5"),
            Ok((U4F4::from_bits(0xF8), true))
        );
        assert_eq!(U4F4::from_str("31.5"), Err(overflow));
        assert_eq!(
            U4F4::overflowing_from_str("271.5"),
            Ok((U4F4::from_bits(0xF8), true))
        );
        assert_eq!(
            U8F0::overflowing_from_str("271"),
            Ok((U8F0::from_bits(0x0F), true))
        );
        let longer_than_8 = format!("{}", (1 << 30) + 15);
        assert_eq!(
            U8F0::overflowing_from_str(&longer_than_8),
            Ok((U8F0::from_bits(0x0F), true))
        );

        assert_eq!(
            U4F4::overflowing_from_str_binary("1111.1000"),
            Ok((U4F4::from_bits(0xF8), false))
        );
        assert_eq!(
            U4F4::from_str_binary("1111.1000"),
            Ok(U4F4::from_bits(0xF8))
        );
        assert_eq!(
            U4F4::overflowing_from_str_binary("11111.1000"),
            Ok((U4F4::from_bits(0xF8), true))
        );
        assert_eq!(U4F4::from_str_binary("11111.1000"), Err(overflow));
        assert_eq!(
            U8F0::overflowing_from_str_binary("100001111"),
            Ok((U8F0::from_bits(0x0F), true))
        );

        assert_eq!(
            U4F4::overflowing_from_str_octal("17.7"),
            Ok((U4F4::from_bits(0xFE), false))
        );
        assert_eq!(U4F4::from_str_octal("17.7"), Ok(U4F4::from_bits(0xFE)));
        assert_eq!(
            U4F4::overflowing_from_str_octal("77.7"),
            Ok((U4F4::from_bits(0xFE), true))
        );
        assert_eq!(U4F4::from_str_octal("77.7"), Err(overflow));
        assert_eq!(
            U4F4::overflowing_from_str_octal("707.7"),
            Ok((U4F4::from_bits(0x7E), true))
        );
        assert_eq!(
            U8F0::overflowing_from_str_octal("1307"),
            Ok((U8F0::from_bits(0o307), true))
        );

        assert_eq!(
            U6F10::overflowing_from_str_hex("3F.8"),
            Ok((U6F10::from_bits(0xFE00), false))
        );
        assert_eq!(U6F10::from_str_hex("3F.8"), Ok(U6F10::from_bits(0xFE00)));
        assert_eq!(
            U6F10::overflowing_from_str_hex("FF.8"),
            Ok((U6F10::from_bits(0xFE00), true))
        );
        assert_eq!(U6F10::from_str_hex("FF.8"), Err(overflow));
        assert_eq!(
            U6F10::overflowing_from_str_hex("F0F.8"),
            Ok((U6F10::from_bits(0x3E00), true))
        );
        assert_eq!(
            U16F0::overflowing_from_str_hex("100FF"),
            Ok((U16F0::from_bits(0x00FF), true))
        );
    }

    #[test]
    fn check_dec_8() {
        let two_pow = 8f64.exp2();
        let limit = 1000;
        for i in 0..limit {
            let ans = from_str::u8::dec_to_bin(i, 8, Round::Nearest);
            let approx = two_pow * f64::from(i) / f64::from(limit);
            let error = (ans.map_or(two_pow, f64::from) - approx).abs();
            assert!(
                error <= 0.5,
                "i {i} ans {ans:?}  approx {approx} error {error}"
            );
        }
    }

    #[test]
    fn check_dec_16() {
        let two_pow = 16f64.exp2();
        let limit = 1_000_000;
        for i in 0..limit {
            let ans = from_str::u16::dec_to_bin(i, 16, Round::Nearest);
            let approx = two_pow * f64::from(i) / f64::from(limit);
            let error = (ans.map_or(two_pow, f64::from) - approx).abs();
            assert!(
                error <= 0.5,
                "i {i} ans {ans:?}  approx {approx} error {error}"
            );
        }
    }

    #[test]
    fn check_dec_32() {
        let two_pow = 32f64.exp2();
        let limit = 10_000_000_000_000;
        for iter in 0..1_000_000 {
            for &i in &[
                iter,
                limit / 4 - 1 - iter,
                limit / 4 + iter,
                limit / 3 - 1 - iter,
                limit / 3 + iter,
                limit / 2 - 1 - iter,
                limit / 2 + iter,
                limit - iter - 1,
            ] {
                let ans = from_str::u32::dec_to_bin(i, 32, Round::Nearest);
                let approx = two_pow * i as f64 / limit as f64;
                let error = (ans.map_or(two_pow, f64::from) - approx).abs();
                assert!(
                    error <= 0.5,
                    "i {i} ans {ans:?}  approx {approx} error {error}"
                );
            }
        }
    }

    #[test]
    fn check_dec_64() {
        let two_pow = 64f64.exp2();
        let limit = 1_000_000_000_000_000_000_000_000_000;
        for iter in 0..200_000 {
            for &i in &[
                iter,
                limit / 4 - 1 - iter,
                limit / 4 + iter,
                limit / 3 - 1 - iter,
                limit / 3 + iter,
                limit / 2 - 1 - iter,
                limit / 2 + iter,
                limit - iter - 1,
            ] {
                let ans = from_str::u64::dec_to_bin(i, 64, Round::Nearest);
                let approx = two_pow * i as f64 / limit as f64;
                let error = (ans.map_or(two_pow, |x| x as f64) - approx).abs();
                assert!(
                    error <= 0.5,
                    "i {i} ans {ans:?}  approx {approx} error {error}"
                );
            }
        }
    }

    #[test]
    fn check_dec_128() {
        let nines = 10u128.pow(27) - 1;
        let zeros = 0;
        let too_big = from_str::u128::dec_to_bin((nines, nines), 128, Round::Nearest);
        assert_eq!(too_big, None);
        let big = from_str::u128::dec_to_bin((nines, zeros), 128, Round::Nearest);
        assert_eq!(
            big,
            Some(340_282_366_920_938_463_463_374_607_091_485_844_535)
        );
        let small = from_str::u128::dec_to_bin((zeros, nines), 128, Round::Nearest);
        assert_eq!(small, Some(340_282_366_921));
        let zero = from_str::u128::dec_to_bin((zeros, zeros), 128, Round::Nearest);
        assert_eq!(zero, Some(0));
        let x = from_str::u128::dec_to_bin(
            (
                123_456_789_012_345_678_901_234_567,
                987_654_321_098_765_432_109_876_543,
            ),
            128,
            Round::Nearest,
        );
        assert_eq!(x, Some(42_010_168_377_579_896_403_540_037_811_203_677_112));

        let eights = 888_888_888_888_888_888_888_888_888;
        let narrow = from_str::u128::dec_to_bin((eights, zeros), 40, Round::Nearest);
        assert_eq!(narrow, Some(977_343_669_134));
    }

    fn digits_eq_bytes(mut digits: DigitsExp, bytes: &[u8]) -> bool {
        let mut bytes = bytes.iter().copied();
        while let Some((digit, rem)) = digits.split_first() {
            digits = rem;
            match bytes.next() {
                Some(byte) => {
                    if byte != digit {
                        return false;
                    }
                }
                None => return false,
            }
        }
        bytes.next().is_none()
    }

    #[track_caller]
    fn check_parse_bounds_ok(bytes: &str, radix: u32, sep: Sep, check: (bool, &str, &str, u32)) {
        let bytes = bytes.as_bytes();
        let Parse {
            neg,
            int,
            frac,
            bit_exp,
        } = parse_bounds(bytes, radix, sep).unwrap();
        assert_eq!(neg, check.0);
        assert!(digits_eq_bytes(int, check.1.as_bytes()));
        assert!(digits_eq_bytes(frac, check.2.as_bytes()));
        match bit_exp {
            Some(bit_exp) => {
                assert_eq!(bit_exp.exp.get(), check.3);
                assert_eq!(
                    bit_exp.first_frac_digit,
                    frac.split_first().map_or(b'0', |x| x.0)
                );
            }
            None => assert_eq!(0, check.3),
        }
    }

    #[track_caller]
    fn check_parse_bounds_err(bytes: &str, radix: u32, sep: Sep, check: ParseErrorKind) {
        let bytes = bytes.as_bytes();
        let kind = parse_bounds(bytes, radix, sep).unwrap_err();
        assert_eq!(kind, check);
    }

    #[test]
    fn check_parse_bounds() {
        let sep = Sep::Error;

        check_parse_bounds_ok("-12.34", 10, sep, (true, "12", "34", 0));
        check_parse_bounds_ok("012.", 10, sep, (false, "12", "", 0));
        check_parse_bounds_ok("+.340", 10, sep, (false, "", "34", 0));
        check_parse_bounds_ok("0", 10, sep, (false, "", "", 0));
        check_parse_bounds_ok("-.C1A0", 16, sep, (true, "", "C1A", 0));
        check_parse_bounds_ok("-.C1A0@1", 16, sep, (true, "C", "1A", 0));
        check_parse_bounds_ok("-.C1A0@+1", 16, sep, (true, "C", "1A", 0));
        check_parse_bounds_ok("-.C1A0@-1", 16, sep, (true, "", "0C1A", 0));
        check_parse_bounds_ok("-C1A0@-2", 16, sep, (true, "C1", "A", 0));
        check_parse_bounds_ok("-.C1A0p5", 16, sep, (true, "C", "1A", 1));
        check_parse_bounds_ok("-C1A0P-2", 16, sep, (true, "C1A", "", 2));

        check_parse_bounds_err("0 ", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("+-", 10, sep, ParseErrorKind::MisplacedSign);
        check_parse_bounds_err("1+2", 10, sep, ParseErrorKind::MisplacedSign);
        check_parse_bounds_err("1-2", 10, sep, ParseErrorKind::MisplacedSign);
        check_parse_bounds_err("+.", 10, sep, ParseErrorKind::NoDigits);
        check_parse_bounds_err(".1.", 10, sep, ParseErrorKind::TooManyPoints);
        check_parse_bounds_err("C1A0@2F", 16, sep, ParseErrorKind::ExpInvalidDigit);
        check_parse_bounds_err("12.34E", 10, sep, ParseErrorKind::ExpNoDigits);
        check_parse_bounds_err("C1A0@1P1", 16, sep, ParseErrorKind::TooManyExp);
        check_parse_bounds_err("1E3000000000", 10, sep, ParseErrorKind::ExpOverflow);

        check_parse_bounds_err("-_12.34", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("-1_2.34", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("-12_.34", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("-12._34", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("-12.3_4", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("-123E4_", 10, sep, ParseErrorKind::ExpInvalidDigit);
        check_parse_bounds_err("-12.34_", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err(
            "-0_1__2___.3____4_____0",
            10,
            sep,
            ParseErrorKind::InvalidDigit,
        );
        check_parse_bounds_err("-1_2__.3_4__e+0___5", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("-1_2__.3_4__E-0___5", 10, sep, ParseErrorKind::InvalidDigit);
    }

    #[test]
    fn check_parse_bounds_underscore() {
        let sep = Sep::Skip;

        check_parse_bounds_ok("-12.34", 10, sep, (true, "12", "34", 0));
        check_parse_bounds_ok("012.", 10, sep, (false, "12", "", 0));
        check_parse_bounds_ok("+.340", 10, sep, (false, "", "34", 0));
        check_parse_bounds_ok("0", 10, sep, (false, "", "", 0));
        check_parse_bounds_ok("-.C1A0", 16, sep, (true, "", "C1A", 0));
        check_parse_bounds_ok("-.C1A0@1", 16, sep, (true, "C", "1A", 0));
        check_parse_bounds_ok("-.C1A0@+1", 16, sep, (true, "C", "1A", 0));
        check_parse_bounds_ok("-.C1A0@-1", 16, sep, (true, "", "0C1A", 0));
        check_parse_bounds_ok("-C1A0@-2", 16, sep, (true, "C1", "A", 0));
        check_parse_bounds_ok("-.C1A0p5", 16, sep, (true, "C", "1A", 1));
        check_parse_bounds_ok("-C1A0P-2", 16, sep, (true, "C1A", "", 2));

        check_parse_bounds_err("0 ", 10, sep, ParseErrorKind::InvalidDigit);
        check_parse_bounds_err("+-", 10, sep, ParseErrorKind::MisplacedSign);
        check_parse_bounds_err("1+2", 10, sep, ParseErrorKind::MisplacedSign);
        check_parse_bounds_err("1-2", 10, sep, ParseErrorKind::MisplacedSign);
        check_parse_bounds_err("+.", 10, sep, ParseErrorKind::NoDigits);
        check_parse_bounds_err(".1.", 10, sep, ParseErrorKind::TooManyPoints);
        check_parse_bounds_err("C1A0@2F", 16, sep, ParseErrorKind::ExpInvalidDigit);
        check_parse_bounds_err("12.34E", 10, sep, ParseErrorKind::ExpNoDigits);
        check_parse_bounds_err("C1A0@1P1", 16, sep, ParseErrorKind::TooManyExp);
        check_parse_bounds_err("1E3000000000", 10, sep, ParseErrorKind::ExpOverflow);

        check_parse_bounds_err("-_12.34", 10, sep, ParseErrorKind::MisplacedUnderscore);
        check_parse_bounds_ok("-1_2.34", 10, sep, (true, "12", "34", 0));
        check_parse_bounds_ok("-12_.34", 10, sep, (true, "12", "34", 0));
        check_parse_bounds_err("-12._34", 10, sep, ParseErrorKind::MisplacedUnderscore);
        check_parse_bounds_ok("-12.3_4", 10, sep, (true, "12", "34", 0));
        check_parse_bounds_ok("-12.34_", 10, sep, (true, "12", "34", 0));
        check_parse_bounds_ok("-123E4_", 10, sep, (true, "1230000", "", 0));
        check_parse_bounds_ok("-0_1__2___.3____4_____0", 10, sep, (true, "12", "34", 0));
        check_parse_bounds_ok("-1_2__.3_4__e+0___5", 10, sep, (true, "1234000", "", 0));
        check_parse_bounds_ok("-1_2__.3_4__E-0___5", 10, sep, (true, "", "0001234", 0));
    }

    macro_rules! assert_ok {
        ($T:ty, $str:expr, $radix:expr, $bits:expr, $overflow:expr) => {
            let m = match $radix {
                2 => <$T>::overflowing_from_str_binary($str),
                8 => <$T>::overflowing_from_str_octal($str),
                10 => <$T>::overflowing_from_str($str),
                16 => <$T>::overflowing_from_str_hex($str),
                _ => unreachable!(),
            };
            match m {
                Ok((f, o)) => {
                    assert_eq!(f.to_bits(), $bits, "{} -> ({f}, {o})", $str);
                    assert_eq!(o, $overflow, "{} -> ({f}, {o})", $str);
                }
                Err(e) => panic!("could not parse {}: {e}", $str),
            }
        };
    }

    #[test]
    fn check_i8_u8_from_str() {
        assert_ok!(I0F8, "-1", 10, 0x00, true);
        assert_ok!(I0F8, "-0.502", 10, 0x7F, true);
        assert_ok!(I0F8, "-0.501", 10, -0x80, false);
        assert_ok!(I0F8, "0.498", 10, 0x7F, false);
        assert_ok!(I0F8, "0.499", 10, -0x80, true);
        assert_ok!(I0F8, "1", 10, 0x00, true);

        assert_ok!(I4F4, "-8.04", 10, 0x7F, true);
        assert_ok!(I4F4, "-8.03", 10, -0x80, false);
        assert_ok!(I4F4, "7.96", 10, 0x7F, false);
        assert_ok!(I4F4, "7.97", 10, -0x80, true);

        assert_ok!(I8F0, "-128.501", 10, 0x7F, true);
        // exact tie, round up to even
        assert_ok!(I8F0, "-128.5", 10, -0x80, false);
        assert_ok!(I8F0, "127.499", 10, 0x7F, false);
        // exact tie, round up to even
        assert_ok!(I8F0, "127.5", 10, -0x80, true);

        assert_ok!(U0F8, "-0", 10, 0x00, false);
        assert_ok!(U0F8, "0.498", 10, 0x7F, false);
        assert_ok!(U0F8, "0.499", 10, 0x80, false);
        assert_ok!(U0F8, "0.998", 10, 0xFF, false);
        assert_ok!(U0F8, "0.999", 10, 0x00, true);
        assert_ok!(U0F8, "1", 10, 0x00, true);

        assert_ok!(U4F4, "7.96", 10, 0x7F, false);
        assert_ok!(U4F4, "7.97", 10, 0x80, false);
        assert_ok!(U4F4, "15.96", 10, 0xFF, false);
        assert_ok!(U4F4, "15.97", 10, 0x00, true);

        assert_ok!(U8F0, "127.499", 10, 0x7F, false);
        // exact tie, round up to even
        assert_ok!(U8F0, "127.5", 10, 0x80, false);
        assert_ok!(U8F0, "255.499", 10, 0xFF, false);
        // exact tie, round up to even
        assert_ok!(U8F0, "255.5", 10, 0x00, true);
    }

    #[test]
    fn check_i16_u16_from_str() {
        assert_ok!(I0F16, "-1", 10, 0x00, true);
        assert_ok!(I0F16, "-0.500008", 10, 0x7FFF, true);
        assert_ok!(I0F16, "-0.500007", 10, -0x8000, false);
        assert_ok!(I0F16, "+0.499992", 10, 0x7FFF, false);
        assert_ok!(I0F16, "+0.499993", 10, -0x8000, true);
        assert_ok!(I0F16, "1", 10, 0x0000, true);

        assert_ok!(I8F8, "-128.002", 10, 0x7FFF, true);
        assert_ok!(I8F8, "-128.001", 10, -0x8000, false);
        assert_ok!(I8F8, "+127.998", 10, 0x7FFF, false);
        assert_ok!(I8F8, "+127.999", 10, -0x8000, true);

        assert_ok!(I16F0, "-32768.500001", 10, 0x7FFF, true);
        // exact tie, round up to even
        assert_ok!(I16F0, "-32768.5", 10, -0x8000, false);
        assert_ok!(I16F0, "+32767.499999", 10, 0x7FFF, false);
        // exact tie, round up to even
        assert_ok!(I16F0, "+32767.5", 10, -0x8000, true);

        assert_ok!(U0F16, "-0", 10, 0x0000, false);
        assert_ok!(U0F16, "0.499992", 10, 0x7FFF, false);
        assert_ok!(U0F16, "0.499993", 10, 0x8000, false);
        assert_ok!(U0F16, "0.999992", 10, 0xFFFF, false);
        assert_ok!(U0F16, "0.999993", 10, 0x0000, true);
        assert_ok!(U0F16, "1", 10, 0x0000, true);

        assert_ok!(U8F8, "127.998", 10, 0x7FFF, false);
        assert_ok!(U8F8, "127.999", 10, 0x8000, false);
        assert_ok!(U8F8, "255.998", 10, 0xFFFF, false);
        assert_ok!(U8F8, "255.999", 10, 0x0000, true);

        assert_ok!(U16F0, "32767.499999", 10, 0x7FFF, false);
        // exact tie, round up to even
        assert_ok!(U16F0, "32767.5", 10, 0x8000, false);
        assert_ok!(U16F0, "65535.499999", 10, 0xFFFF, false);
        // exact tie, round up to even
        assert_ok!(U16F0, "65535.5", 10, 0x0000, true);
    }

    #[test]
    fn check_i32_u32_from_str() {
        assert_ok!(I0F32, "-1", 10, 0x0000_0000, true);
        assert_ok!(I0F32, "-0.5000000002", 10, 0x7FFF_FFFF, true);
        assert_ok!(I0F32, "-0.5000000001", 10, -0x8000_0000, false);
        assert_ok!(I0F32, "0.4999999998", 10, 0x7FFF_FFFF, false);
        assert_ok!(I0F32, "0.4999999999", 10, -0x8000_0000, true);
        assert_ok!(I0F32, "1", 10, 0x0000_0000, true);

        assert_ok!(I16F16, "-32768.000008", 10, 0x7FFF_FFFF, true);
        assert_ok!(I16F16, "-32768.000007", 10, -0x8000_0000, false);
        assert_ok!(I16F16, "32767.999992", 10, 0x7FFF_FFFF, false);
        assert_ok!(I16F16, "32767.999993", 10, -0x8000_0000, true);

        assert_ok!(I32F0, "-2147483648.5000000001", 10, 0x7FFF_FFFF, true);
        // exact tie, round up to even
        assert_ok!(I32F0, "-2147483648.5", 10, -0x8000_0000, false);
        assert_ok!(I32F0, "2147483647.4999999999", 10, 0x7FFF_FFFF, false);
        // exact tie, round up to even
        assert_ok!(I32F0, "2147483647.5", 10, -0x8000_0000, true);

        assert_ok!(U0F32, "-0", 10, 0x0000_0000, false);
        assert_ok!(U0F32, "0.4999999998", 10, 0x7FFF_FFFF, false);
        assert_ok!(U0F32, "0.4999999999", 10, 0x8000_0000, false);
        assert_ok!(U0F32, "0.9999999998", 10, 0xFFFF_FFFF, false);
        assert_ok!(U0F32, "0.9999999999", 10, 0x0000_0000, true);
        assert_ok!(U0F32, "1", 10, 0x0000_0000, true);

        assert_ok!(U16F16, "32767.999992", 10, 0x7FFF_FFFF, false);
        assert_ok!(U16F16, "32767.999993", 10, 0x8000_0000, false);
        assert_ok!(U16F16, "65535.999992", 10, 0xFFFF_FFFF, false);
        assert_ok!(U16F16, "65535.999993", 10, 0x0000_0000, true);

        assert_ok!(U32F0, "2147483647.4999999999", 10, 0x7FFF_FFFF, false);
        // exact tie, round up to even
        assert_ok!(U32F0, "2147483647.5", 10, 0x8000_0000, false);
        assert_ok!(U32F0, "4294967295.4999999999", 10, 0xFFFF_FFFF, false);
        // exact tie, round up to even
        assert_ok!(U32F0, "4294967295.5", 10, 0x0000_0000, true);
    }

    #[test]
    fn check_i64_u64_from_str() {
        assert_ok!(I0F64, "-1", 10, 0x0000_0000_0000_0000, true);
        assert_ok!(
            I0F64,
            "-0.50000000000000000003",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            true
        );
        assert_ok!(
            I0F64,
            "-0.50000000000000000002",
            10,
            -0x8000_0000_0000_0000,
            false
        );
        assert_ok!(
            I0F64,
            "+0.49999999999999999997",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            I0F64,
            "+0.49999999999999999998",
            10,
            -0x8000_0000_0000_0000,
            true
        );
        assert_ok!(I0F64, "1", 10, 0x0000_0000_0000_0000, true);

        assert_ok!(
            I32F32,
            "-2147483648.0000000002",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            true
        );
        assert_ok!(
            I32F32,
            "-2147483648.0000000001",
            10,
            -0x8000_0000_0000_0000,
            false
        );
        assert_ok!(
            I32F32,
            "2147483647.9999999998",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            I32F32,
            "2147483647.9999999999",
            10,
            -0x8000_0000_0000_0000,
            true
        );

        assert_ok!(
            I64F0,
            "-9223372036854775808.50000000000000000001",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            true
        );
        // exact tie, round up to even
        assert_ok!(
            I64F0,
            "-9223372036854775808.5",
            10,
            -0x8000_0000_0000_0000,
            false
        );
        assert_ok!(
            I64F0,
            "9223372036854775807.49999999999999999999",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            false
        );
        // exact tie, round up to even
        assert_ok!(
            I64F0,
            "9223372036854775807.5",
            10,
            -0x8000_0000_0000_0000,
            true
        );

        assert_ok!(U0F64, "-0", 10, 0x0000_0000_0000_0000, false);
        assert_ok!(
            U0F64,
            "0.49999999999999999997",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U0F64,
            "0.49999999999999999998",
            10,
            0x8000_0000_0000_0000,
            false
        );
        assert_ok!(
            U0F64,
            "0.99999999999999999997",
            10,
            0xFFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U0F64,
            "0.99999999999999999998",
            10,
            0x0000_0000_0000_0000,
            true
        );
        assert_ok!(U0F64, "1", 10, 0x0000_0000_0000_0000, true);

        assert_ok!(
            U32F32,
            "2147483647.9999999998",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U32F32,
            "2147483647.9999999999",
            10,
            0x8000_0000_0000_0000,
            false
        );
        assert_ok!(
            U32F32,
            "4294967295.9999999998",
            10,
            0xFFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U32F32,
            "4294967295.9999999999",
            10,
            0x0000_0000_0000_0000,
            true
        );

        assert_ok!(
            U64F0,
            "9223372036854775807.49999999999999999999",
            10,
            0x7FFF_FFFF_FFFF_FFFF,
            false
        );
        // exact tie, round up to even
        assert_ok!(
            U64F0,
            "9223372036854775807.5",
            10,
            0x8000_0000_0000_0000,
            false
        );
        assert_ok!(
            U64F0,
            "18446744073709551615.49999999999999999999",
            10,
            0xFFFF_FFFF_FFFF_FFFF,
            false
        );
        // exact tie, round up to even
        assert_ok!(
            U64F0,
            "18446744073709551615.5",
            10,
            0x0000_0000_0000_0000,
            true
        );
    }

    #[test]
    fn check_i128_u128_from_str() {
        assert_ok!(
            I0F128,
            "-1",
            10,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
        assert_ok!(
            I0F128,
            "-0.500000000000000000000000000000000000002",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            true
        );
        assert_ok!(
            I0F128,
            "-0.500000000000000000000000000000000000001",
            10,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I0F128,
            "0.499999999999999999999999999999999999998",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            I0F128,
            "0.499999999999999999999999999999999999999",
            10,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
        assert_ok!(
            I0F128,
            "1",
            10,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            I64F64,
            "-9223372036854775808.00000000000000000003",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            true
        );
        assert_ok!(
            I64F64,
            "-9223372036854775808.00000000000000000002",
            10,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I64F64,
            "9223372036854775807.99999999999999999997",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            I64F64,
            "9223372036854775807.99999999999999999998",
            10,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            I128F0,
            "-170141183460469231731687303715884105728.5000000000000000000000000000000000000001",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            true
        );
        // exact tie, round up to even
        assert_ok!(
            I128F0,
            "-170141183460469231731687303715884105728.5",
            10,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I128F0,
            "170141183460469231731687303715884105727.4999999999999999999999999999999999999999",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        // exact tie, round up to even
        assert_ok!(
            I128F0,
            "170141183460469231731687303715884105727.5",
            10,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            U0F128,
            "-0",
            10,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U0F128,
            "0.499999999999999999999999999999999999998",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U0F128,
            "0.499999999999999999999999999999999999999",
            10,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U0F128,
            "0.999999999999999999999999999999999999998",
            10,
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U0F128,
            "0.999999999999999999999999999999999999999",
            10,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
        assert_ok!(
            U0F128,
            "1",
            10,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            U64F64,
            "9223372036854775807.99999999999999999997",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U64F64,
            "9223372036854775807.99999999999999999998",
            10,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U64F64,
            "18446744073709551615.99999999999999999997",
            10,
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U64F64,
            "18446744073709551615.99999999999999999998",
            10,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            U128F0,
            "170141183460469231731687303715884105727.4999999999999999999999999999999999999999",
            10,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        // exact tie(ound up to even
        assert_ok!(
            U128F0,
            "170141183460469231731687303715884105727.5",
            10,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U128F0,
            "340282366920938463463374607431768211455.4999999999999999999999999999999999999999",
            10,
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        // exact tie, round up to even
        assert_ok!(
            U128F0,
            "340282366920938463463374607431768211455.5",
            10,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
    }

    #[test]
    fn check_i16_u16_from_str_binary() {
        assert_ok!(I0F16, "-1", 2, 0x0000, true);
        assert_ok!(I0F16, "-0.100000000000000011", 2, 0x7FFF, true);
        assert_ok!(I0F16, "-0.100000000000000010", 2, -0x8000, false);
        assert_ok!(I0F16, "-0.011111111111111110", 2, -0x8000, false);
        assert_ok!(I0F16, "+0.011111111111111101", 2, 0x7FFF, false);
        assert_ok!(I0F16, "+0.011111111111111110", 2, -0x8000, true);
        assert_ok!(I0F16, "1", 2, 0x0000, true);

        assert_ok!(I8F8, "-10000000.0000000011", 2, 0x7FFF, true);
        assert_ok!(I8F8, "-10000000.0000000010", 2, -0x8000, false);
        assert_ok!(I8F8, "-01111111.1111111110", 2, -0x8000, false);
        assert_ok!(I8F8, "+01111111.1111111101", 2, 0x7FFF, false);
        assert_ok!(I8F8, "+01111111.1111111110", 2, -0x8000, true);

        assert_ok!(I16F0, "-1000000000000000.11", 2, 0x7FFF, true);
        assert_ok!(I16F0, "-1000000000000000.10", 2, -0x8000, false);
        assert_ok!(I16F0, "-0111111111111111.10", 2, -0x8000, false);
        assert_ok!(I16F0, "+0111111111111111.01", 2, 0x7FFF, false);
        assert_ok!(I16F0, "+0111111111111111.10", 2, -0x8000, true);

        assert_ok!(U0F16, "-0", 2, 0x0000, false);
        assert_ok!(U0F16, "0.011111111111111101", 2, 0x7FFF, false);
        assert_ok!(U0F16, "0.011111111111111110", 2, 0x8000, false);
        assert_ok!(U0F16, "0.111111111111111101", 2, 0xFFFF, false);
        assert_ok!(U0F16, "0.111111111111111110", 2, 0x0000, true);
        assert_ok!(U0F16, "1", 2, 0x0000, true);

        assert_ok!(U8F8, "01111111.1111111101", 2, 0x7FFF, false);
        assert_ok!(U8F8, "01111111.1111111110", 2, 0x8000, false);
        assert_ok!(U8F8, "11111111.1111111101", 2, 0xFFFF, false);
        assert_ok!(U8F8, "11111111.1111111110", 2, 0x0000, true);

        assert_ok!(U16F0, "0111111111111111.01", 2, 0x7FFF, false);
        assert_ok!(U16F0, "0111111111111111.10", 2, 0x8000, false);
        assert_ok!(U16F0, "1111111111111111.01", 2, 0xFFFF, false);
        assert_ok!(U16F0, "1111111111111111.10", 2, 0x0000, true);

        assert_ok!(U0F16, "00111.11111111111101e-4", 2, 0x7FFF, false);
        assert_ok!(U16F0, "011111111111.111101e4", 2, 0x7FFF, false);
        assert_ok!(U8F8, "011.110P3", 2, 0x1E00, false);
        assert_ok!(U8F8, "011.110P-3", 2, 0x0078, false);
    }

    #[test]
    fn check_i16_u16_from_str_octal() {
        assert_ok!(I0F16, "-1", 8, 0x0000, true);
        assert_ok!(I0F16, "-0.400003", 8, 0x7FFF, true);
        assert_ok!(I0F16, "-0.400002", 8, -0x8000, false);
        assert_ok!(I0F16, "-0.377776", 8, -0x8000, false);
        assert_ok!(I0F16, "+0.377775", 8, 0x7FFF, false);
        assert_ok!(I0F16, "+0.377776", 8, -0x8000, true);
        assert_ok!(I0F16, "1", 8, 0x0000, true);

        assert_ok!(I8F8, "-200.0011", 8, 0x7FFF, true);
        assert_ok!(I8F8, "-200.0010", 8, -0x8000, false);
        assert_ok!(I8F8, "-177.7770", 8, -0x8000, false);
        assert_ok!(I8F8, "+177.7767", 8, 0x7FFF, false);
        assert_ok!(I8F8, "+177.7770", 8, -0x8000, true);

        assert_ok!(I16F0, "-100000.5", 8, 0x7FFF, true);
        assert_ok!(I16F0, "-100000.4", 8, -0x8000, false);
        assert_ok!(I16F0, "-077777.4", 8, -0x8000, false);
        assert_ok!(I16F0, "+077777.3", 8, 0x7FFF, false);
        assert_ok!(I16F0, "+077777.4", 8, -0x8000, true);

        assert_ok!(U0F16, "-0", 8, 0x0000, false);
        assert_ok!(U0F16, "0.377775", 8, 0x7FFF, false);
        assert_ok!(U0F16, "0.377776", 8, 0x8000, false);
        assert_ok!(U0F16, "0.777775", 8, 0xFFFF, false);
        assert_ok!(U0F16, "0.777776", 8, 0x0000, true);
        assert_ok!(U0F16, "1", 8, 0x0000, true);

        assert_ok!(U8F8, "177.7767", 8, 0x7FFF, false);
        assert_ok!(U8F8, "177.7770", 8, 0x8000, false);
        assert_ok!(U8F8, "377.7767", 8, 0xFFFF, false);
        assert_ok!(U8F8, "377.7770", 8, 0x0000, true);

        assert_ok!(U16F0, "077777.3", 8, 0x7FFF, false);
        assert_ok!(U16F0, "077777.4", 8, 0x8000, false);
        assert_ok!(U16F0, "177777.3", 8, 0xFFFF, false);
        assert_ok!(U16F0, "177777.4", 8, 0x0000, true);

        assert_ok!(U0F16, "037.7775e-2", 8, 0x7FFF, false);
        assert_ok!(U16F0, "0777.773e2", 8, 0x7FFF, false);
        assert_ok!(U8F8, "037.450P4", 8, 0xF940, true);
        assert_ok!(U8F8, "037.450P3", 8, 0xFCA0, false);
        assert_ok!(U8F8, "037.450P2", 8, 0x7E50, false);
        assert_ok!(U8F8, "037.450P1", 8, 0x3F28, false);
        assert_ok!(U8F8, "037.450P0", 8, 0x1F94, false);
        assert_ok!(U8F8, "037.450P-1", 8, 0x0FCA, false);
        assert_ok!(U8F8, "037.450P-2", 8, 0x07E5, false);
        assert_ok!(U8F8, "037.450P-3", 8, 0x03F2, false);
        assert_ok!(U8F8, "037.450P-4", 8, 0x01F9, false);
    }

    #[test]
    fn check_i16_u16_from_str_hex() {
        assert_ok!(I0F16, "-1", 16, 0x0000, true);
        assert_ok!(I0F16, "-0.80009", 16, 0x7FFF, true);
        assert_ok!(I0F16, "-0.80008", 16, -0x8000, false);
        assert_ok!(I0F16, "-0.7FFF8", 16, -0x8000, false);
        assert_ok!(I0F16, "+0.7FFF7", 16, 0x7FFF, false);
        assert_ok!(I0F16, "+0.7FFF8", 16, -0x8000, true);
        assert_ok!(I0F16, "1", 16, 0x0000, true);

        assert_ok!(I8F8, "-80.009", 16, 0x7FFF, true);
        assert_ok!(I8F8, "-80.008", 16, -0x8000, false);
        assert_ok!(I8F8, "-7F.FF8", 16, -0x8000, false);
        assert_ok!(I8F8, "+7F.FF7", 16, 0x7FFF, false);
        assert_ok!(I8F8, "+7F.FF8", 16, -0x8000, true);

        assert_ok!(I16F0, "-8000.9", 16, 0x7FFF, true);
        assert_ok!(I16F0, "-8000.8", 16, -0x8000, false);
        assert_ok!(I16F0, "-7FFF.8", 16, -0x8000, false);
        assert_ok!(I16F0, "+7FFF.7", 16, 0x7FFF, false);
        assert_ok!(I16F0, "+7FFF.8", 16, -0x8000, true);

        assert_ok!(U0F16, "-0", 16, 0x0000, false);
        assert_ok!(U0F16, "0.7FFF7", 16, 0x7FFF, false);
        assert_ok!(U0F16, "0.7FFF8", 16, 0x8000, false);
        assert_ok!(U0F16, "0.FFFF7", 16, 0xFFFF, false);
        assert_ok!(U0F16, "0.FFFF8", 16, 0x0000, true);
        assert_ok!(U0F16, "1", 16, 0x0000, true);

        assert_ok!(U8F8, "7F.FF7", 16, 0x7FFF, false);
        assert_ok!(U8F8, "7F.FF8", 16, 0x8000, false);
        assert_ok!(U8F8, "FF.FF7", 16, 0xFFFF, false);
        assert_ok!(U8F8, "FF.FF8", 16, 0x0000, true);

        assert_ok!(U16F0, "7FFF.7", 16, 0x7FFF, false);
        assert_ok!(U16F0, "7FFF.8", 16, 0x8000, false);
        assert_ok!(U16F0, "FFFF.7", 16, 0xFFFF, false);
        assert_ok!(U16F0, "FFFF.8", 16, 0x0000, true);

        assert_ok!(U0F16, "07F.FF7@-2", 16, 0x7FFF, false);
        assert_ok!(U16F0, "7F.FF7@2", 16, 0x7FFF, false);
        assert_ok!(U8F8, "13.B8P4", 16, 0x3B80, true);
        assert_ok!(U8F8, "13.B8P3", 16, 0x9DC0, false);
        assert_ok!(U8F8, "13.B8P2", 16, 0x4EE0, false);
        assert_ok!(U8F8, "13.B8P1", 16, 0x2770, false);
        assert_ok!(U8F8, "13.B8P0", 16, 0x13B8, false);
        assert_ok!(U8F8, "13.B8P-1", 16, 0x09DC, false);
        assert_ok!(U8F8, "13.B8P-2", 16, 0x04EE, false);
        assert_ok!(U8F8, "13.B8P-3", 16, 0x0277, false);
        assert_ok!(U8F8, "13.B8P-4", 16, 0x013C, false);
    }

    #[test]
    fn check_i128_u128_from_str_hex() {
        assert_ok!(
            I0F128,
            "-1",
            16,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
        assert_ok!(
            I0F128,
            "-0.800000000000000000000000000000009",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            true
        );
        assert_ok!(
            I0F128,
            "-0.800000000000000000000000000000008",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I0F128,
            "-0.7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I0F128,
            "+0.7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            I0F128,
            "+0.7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
        assert_ok!(
            I0F128,
            "1",
            16,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            I64F64,
            "-8000000000000000.00000000000000009",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            true
        );
        assert_ok!(
            I64F64,
            "-8000000000000000.00000000000000008",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I64F64,
            "-7FFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF8",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I64F64,
            "+7FFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF7",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            I64F64,
            "+7FFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF8",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            I128F0,
            "-80000000000000000000000000000000.9",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            true
        );
        assert_ok!(
            I128F0,
            "-80000000000000000000000000000000.8",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I128F0,
            "-7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.8",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            I128F0,
            "+7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.7",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            I128F0,
            "+7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.8",
            16,
            -0x8000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            U0F128,
            "-0",
            16,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U0F128,
            "0.7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U0F128,
            "0.7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8",
            16,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U0F128,
            "0.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7",
            16,
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U0F128,
            "0.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8",
            16,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
        assert_ok!(
            U0F128,
            "1",
            16,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            U64F64,
            "7FFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF7",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U64F64,
            "7FFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF8",
            16,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U64F64,
            "FFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF7",
            16,
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U64F64,
            "FFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFF8",
            16,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );

        assert_ok!(
            U128F0,
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.7",
            16,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U128F0,
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.8",
            16,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
            false
        );
        assert_ok!(
            U128F0,
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.7",
            16,
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            false
        );
        assert_ok!(
            U128F0,
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.8",
            16,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            true
        );
    }

    // For an odd prefix, e.g. eps = 0.125
    // zero = 0.125
    // gt_0 = 0.125000001
    // max = max_int.874999999
    // overflow = max_int.875
    struct Fractions {
        zero: String,
        gt_0: String,
        max: String,
        over: String,
    }
    fn without_last(a: &str) -> &str {
        &a[..a.len() - 1]
    }
    fn make_fraction_strings(max_int: &str, eps_frac: &str) -> Fractions {
        let eps_frac_compl: String = eps_frac
            .chars()
            .map(|digit| (b'0' + b'9' - digit as u8) as char)
            .collect();

        let zero = String::from("0.") + eps_frac;
        let gt_0 = String::from(&*zero) + "000001";
        let max = String::from(max_int) + &eps_frac_compl + "999999";
        let over = String::from(max_int) + without_last(&eps_frac_compl) + "5";
        Fractions {
            zero,
            gt_0,
            max,
            over,
        }
    }

    // check that for example for four fractional bits,
    //   * 0.03125 (1/32) is parsed as 0
    //   * 0.03125000001 (just above 1/32) is parsed as 0.0625 (1/16)
    //   * odd.96874999999 (just below 31/32) is parsed as 0.9375 (15/16)
    //   * odd.96875 (31/32) is parsed as odd + 1
    #[test]
    fn check_exact_decimal() {
        let max_int_0 = String::from("0.");
        let max_int_4 = String::from("15.");
        let max_int_8 = format!("{}.", !0u8);
        let max_int_16 = format!("{}.", !0u16);
        let max_int_28 = format!("{}.", !0u32 >> 4);
        let max_int_32 = format!("{}.", !0u32);
        let max_int_64 = format!("{}.", !0u64);
        let max_int_124 = format!("{}.", !0u128 >> 4);
        let max_int_128 = format!("{}.", !0u128);

        // Note: fractions can be generated with this:
        //
        //     use rug::Integer;
        //     for &i in &[0, 4, 8, 16, 28, 32, 64, 124, 128] {
        //         let eps = Integer::from(Integer::u_pow_u(5, i + 1));
        //         println!("let eps_{} = \"{:02$}\";", i, eps, i as usize + 1);
        //     }

        // eps_0 = 0.5 >> 0 = 0.5
        // eps_4 = 0.5 >> 4 = 0.03125
        // eps_8 = 0.5 >> 8 = 0.001953125
        // etc.
        let eps_0 = "5";
        let eps_4 = "03125";
        let eps_8 = "001953125";
        let eps_16 = "00000762939453125";
        let eps_28 = "00000000186264514923095703125";
        let eps_32 = "000000000116415321826934814453125";
        let eps_64 = "00000000000000000002710505431213761085018632002174854278564453125";
        let eps_124 = "0000000000000000000000000000000000000235098870164457501593747307\
                       4444491355637331113544175043017503412556834518909454345703125";
        let eps_128 = "0000000000000000000000000000000000000014693679385278593849609206\
                       71527807097273331945965109401885939632848021574318408966064453125";

        let frac_0_8 = make_fraction_strings(&max_int_0, eps_8);
        assert_ok!(U0F8, &frac_0_8.zero, 10, 0, false);
        assert_ok!(U0F8, &frac_0_8.gt_0, 10, 1, false);
        assert_ok!(U0F8, &frac_0_8.max, 10, !0, false);
        assert_ok!(U0F8, &frac_0_8.over, 10, 0, true);

        let frac_4_4 = make_fraction_strings(&max_int_4, eps_4);
        assert_ok!(U4F4, &frac_4_4.zero, 10, 0, false);
        assert_ok!(U4F4, &frac_4_4.gt_0, 10, 1, false);
        assert_ok!(U4F4, &frac_4_4.max, 10, !0, false);
        assert_ok!(U4F4, &frac_4_4.over, 10, 0, true);

        let frac_8_0 = make_fraction_strings(&max_int_8, eps_0);
        assert_ok!(U8F0, &frac_8_0.zero, 10, 0, false);
        assert_ok!(U8F0, &frac_8_0.gt_0, 10, 1, false);
        assert_ok!(U8F0, &frac_8_0.max, 10, !0, false);
        assert_ok!(U8F0, &frac_8_0.over, 10, 0, true);

        let frac_0_32 = make_fraction_strings(&max_int_0, eps_32);
        assert_ok!(U0F32, &frac_0_32.zero, 10, 0, false);
        assert_ok!(U0F32, &frac_0_32.gt_0, 10, 1, false);
        assert_ok!(U0F32, &frac_0_32.max, 10, !0, false);
        assert_ok!(U0F32, &frac_0_32.over, 10, 0, true);

        let frac_4_28 = make_fraction_strings(&max_int_4, eps_28);
        assert_ok!(U4F28, &frac_4_28.zero, 10, 0, false);
        assert_ok!(U4F28, &frac_4_28.gt_0, 10, 1, false);
        assert_ok!(U4F28, &frac_4_28.max, 10, !0, false);
        assert_ok!(U4F28, &frac_4_28.over, 10, 0, true);

        let frac_16_16 = make_fraction_strings(&max_int_16, eps_16);
        assert_ok!(U16F16, &frac_16_16.zero, 10, 0, false);
        assert_ok!(U16F16, &frac_16_16.gt_0, 10, 1, false);
        assert_ok!(U16F16, &frac_16_16.max, 10, !0, false);
        assert_ok!(U16F16, &frac_16_16.over, 10, 0, true);

        let frac_28_4 = make_fraction_strings(&max_int_28, eps_4);
        assert_ok!(U28F4, &frac_28_4.zero, 10, 0, false);
        assert_ok!(U28F4, &frac_28_4.gt_0, 10, 1, false);
        assert_ok!(U28F4, &frac_28_4.max, 10, !0, false);
        assert_ok!(U28F4, &frac_28_4.over, 10, 0, true);

        let frac_32_0 = make_fraction_strings(&max_int_32, eps_0);
        assert_ok!(U32F0, &frac_32_0.zero, 10, 0, false);
        assert_ok!(U32F0, &frac_32_0.gt_0, 10, 1, false);
        assert_ok!(U32F0, &frac_32_0.max, 10, !0, false);
        assert_ok!(U32F0, &frac_32_0.over, 10, 0, true);

        let frac_0_128 = make_fraction_strings(&max_int_0, eps_128);
        assert_ok!(U0F128, &frac_0_128.zero, 10, 0, false);
        assert_ok!(U0F128, &frac_0_128.gt_0, 10, 1, false);
        assert_ok!(U0F128, &frac_0_128.max, 10, !0, false);
        assert_ok!(U0F128, &frac_0_128.over, 10, 0, true);

        let frac_4_124 = make_fraction_strings(&max_int_4, eps_124);
        assert_ok!(U4F124, &frac_4_124.zero, 10, 0, false);
        assert_ok!(U4F124, &frac_4_124.gt_0, 10, 1, false);
        assert_ok!(U4F124, &frac_4_124.max, 10, !0, false);
        assert_ok!(U4F124, &frac_4_124.over, 10, 0, true);

        let frac_64_64 = make_fraction_strings(&max_int_64, eps_64);
        assert_ok!(U64F64, &frac_64_64.zero, 10, 0, false);
        assert_ok!(U64F64, &frac_64_64.gt_0, 10, 1, false);
        assert_ok!(U64F64, &frac_64_64.max, 10, !0, false);
        assert_ok!(U64F64, &frac_64_64.over, 10, 0, true);

        let frac_124_4 = make_fraction_strings(&max_int_124, eps_4);
        assert_ok!(U124F4, &frac_124_4.zero, 10, 0, false);
        assert_ok!(U124F4, &frac_124_4.gt_0, 10, 1, false);
        assert_ok!(U124F4, &frac_124_4.max, 10, !0, false);
        assert_ok!(U124F4, &frac_124_4.over, 10, 0, true);

        let frac_128_0 = make_fraction_strings(&max_int_128, eps_0);
        assert_ok!(U128F0, &frac_128_0.zero, 10, 0, false);
        assert_ok!(U128F0, &frac_128_0.gt_0, 10, 1, false);
        assert_ok!(U128F0, &frac_128_0.max, 10, !0, false);
        assert_ok!(U128F0, &frac_128_0.over, 10, 0, true);

        // some other cases
        // 13/32 = 6.5/16, to even 6/16
        assert_ok!(
            U4F4,
            "0.40624999999999999999999999999999999999999999999999",
            10,
            0x06,
            false
        );
        assert_ok!(U4F4, "0.40625", 10, 0x06, false);
        assert_ok!(
            U4F4,
            "0.40625000000000000000000000000000000000000000000001",
            10,
            0x07,
            false
        );
        // 14/32 = 7/16
        assert_ok!(U4F4, "0.4375", 10, 0x07, false);
        // 15/32 = 7.5/16, to even 8/16
        assert_ok!(
            U4F4,
            "0.46874999999999999999999999999999999999999999999999",
            10,
            0x07,
            false
        );
        assert_ok!(U4F4, "0.46875", 10, 0x08, false);
        assert_ok!(
            U4F4,
            "0.46875000000000000000000000000000000000000000000001",
            10,
            0x08,
            false
        );
        // 16/32 = 8/16
        assert_ok!(U4F4, "0.5", 10, 0x08, false);
        // 17/32 = 8.5/16, to even 8/16
        assert_ok!(
            U4F4,
            "0.53124999999999999999999999999999999999999999999999",
            10,
            0x08,
            false
        );
        assert_ok!(U4F4, "0.53125", 10, 0x08, false);
        assert_ok!(
            U4F4,
            "0.53125000000000000000000000000000000000000000000001",
            10,
            0x09,
            false
        );
        // 18/32 = 9/16
        assert_ok!(U4F4, "0.5625", 10, 0x09, false);
    }

    #[test]
    fn frac4() {
        for u in 0..=255u8 {
            let (ifix, ufix) = (I4F4::from_bits(u as i8), U4F4::from_bits(u));
            let (ifix_str, ufix_str) = (ifix.to_string(), ufix.to_string());
            assert_eq!(I4F4::from_str(&ifix_str).unwrap(), ifix);
            assert_eq!(U4F4::from_str(&ufix_str).unwrap(), ufix);
        }
    }

    #[test]
    fn frac17() {
        for u in 0..(1 << 17) {
            let fix = U15F17::from_bits(u) + U15F17::from_num(99);
            let fix_pos = I15F17::from_num(fix);
            let fix_neg = -fix_pos;
            let fix_str = fix.to_string();
            let fix_pos_str = fix_pos.to_string();
            let fix_neg_str = fix_neg.to_string();
            assert_eq!(fix_str, fix_pos_str);
            if u != 0 {
                assert_eq!(&fix_neg_str[..1], "-");
                assert_eq!(&fix_neg_str[1..], fix_pos_str);
            }
            assert_eq!(U15F17::from_str(&fix_str).unwrap(), fix);
            assert_eq!(I15F17::from_str(&fix_pos_str).unwrap(), fix_pos);
            assert_eq!(I15F17::from_str(&fix_neg_str).unwrap(), fix_neg);

            let fix_str3 = format!("{fix:.3}");
            let fix_pos_str3 = format!("{fix_pos:.3}");
            let fix_neg_str3 = format!("{fix_neg:.3}");
            assert_eq!(fix_str3, fix_pos_str3);
            if u != 0 {
                assert_eq!(&fix_neg_str3[..1], "-");
                assert_eq!(&fix_neg_str3[1..], fix_pos_str3);
            }
            let max_diff = U15F17::from_bits((5 << 17) / 10000 + 1);
            let from_fix_str3 = U15F17::from_str(&fix_str3).unwrap();
            assert!(from_fix_str3.dist(fix) <= max_diff);
            let from_fix_pos_str3 = I15F17::from_str(&fix_pos_str3).unwrap();
            assert!(from_fix_pos_str3.dist(fix_pos) <= max_diff);
            let from_fix_neg_str3 = I15F17::from_str(&fix_neg_str3).unwrap();
            assert!(from_fix_neg_str3.dist(fix_neg) <= max_diff);

            let fix_str9 = format!("{fix:.9}");
            let fix_pos_str9 = format!("{fix_pos:.9}");
            let fix_neg_str9 = format!("{fix_neg:.9}");
            assert_eq!(fix_str9, fix_pos_str9);
            if u != 0 {
                assert_eq!(&fix_neg_str9[..1], "-");
                assert_eq!(&fix_neg_str9[1..], fix_pos_str9);
            }
            assert_eq!(U15F17::from_str(&fix_str9).unwrap(), fix);
            assert_eq!(I15F17::from_str(&fix_pos_str9).unwrap(), fix_pos);
            assert_eq!(I15F17::from_str(&fix_neg_str9).unwrap(), fix_neg);
        }
    }
}
