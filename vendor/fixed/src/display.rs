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
    debug_hex::{self, IsDebugHex},
    int_helper,
    types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8, Unsigned},
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};
use az_crate::{WrappingAs, WrappingCast};
use core::{
    cmp::{self, Ordering},
    fmt::{
        Alignment, Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal,
        Result as FmtResult, UpperExp, UpperHex,
    },
    ops::{Add, Shl, Shr},
    str,
};

// We need 129 digit bytes: 128 digits, one leading zero.
//
// The leading zero has two purposes:
//
//  1. If there are no integer digits, we still want to start with "0.".
//  2. If rounding causes a carry, we can overflow into this extra zero.
//
// In the end the layout should be:
//
//   * data[0..1 + int_digits]: integer digits with 0, 1, or 2 extra zeros
//   * data[1 + int_digits..1 + int_digits + frac_digits]: fractional digits
//
// exp is only used for decimal, so its range is from -39 to 38 inclusive.
struct Buffer {
    int_digits: usize,
    frac_digits: usize,
    digits: [u8; 129],
    exp_len: usize,
    exp: [u8; 4],
}

impl Buffer {
    fn new(int_digits: u32, frac_digits: u32) -> Buffer {
        assert!(int_digits + frac_digits <= 128, "out of bounds");
        Buffer {
            int_digits: int_digits as usize,
            frac_digits: frac_digits as usize,
            digits: [0; 129],
            exp_len: 0,
            exp: [0; 4],
        }
    }

    // does not include leading zero
    fn int(&mut self) -> &mut [u8] {
        let begin = 1;
        let end = 1 + self.int_digits;
        &mut self.digits[begin..end]
    }

    fn frac(&mut self) -> &mut [u8] {
        let begin = 1 + self.int_digits;
        let end = 1 + self.int_digits + self.frac_digits;
        &mut self.digits[begin..end]
    }

    fn format_exp_dec(&mut self, exp_is_upper: bool) {
        self.exp_len = 1;
        self.exp[0] = if exp_is_upper { b'E' } else { b'e' };
        let digits_end = self.int_digits + self.frac_digits;
        match self.digits[..digits_end].iter().position(|&x| x > 0) {
            None => {
                self.int_digits = 0;
                self.frac_digits = 0;
                self.exp_len = 2;
                self.exp[1] = b'0';
            }
            Some(first) => {
                // point should be between digits[int_digits] and digits[1 + int_digits],
                // so digits[int_digits] should be digits[first]
                let neg_exp = self.int_digits < first;
                let abs_exp = self.int_digits.abs_diff(first);
                if neg_exp {
                    self.int_digits += abs_exp;
                    self.frac_digits -= abs_exp;
                    self.exp_len = 2;
                    self.exp[1] = b'-';
                } else {
                    self.int_digits -= abs_exp;
                    self.frac_digits += abs_exp;
                }
                // exp should be between -38 and 39 inclusive
                debug_assert!(abs_exp <= 39);
                if abs_exp > 9 {
                    self.exp_len += 2;
                    let (e0, e1) = (abs_exp as u8 % 10, abs_exp as u8 / 10);
                    self.exp[self.exp_len - 2] = b'0' + e1;
                    self.exp[self.exp_len - 1] = b'0' + e0;
                } else {
                    self.exp_len += 1;
                    self.exp[self.exp_len - 1] = b'0' + abs_exp as u8;
                }
            }
        }
    }

    fn finish(
        &mut self,
        format: Format,
        is_neg: bool,
        frac_rem_cmp_tie: Ordering,
        fmt: &mut Formatter,
    ) -> FmtResult {
        self.round_and_trim(format.max_digit(), frac_rem_cmp_tie);
        self.encode_digits(format == Format::UpHex);
        self.pad_and_print(is_neg, format.prefix(), fmt)
    }

    // rounds, and then trims trailing zeros from frac
    fn round_and_trim(&mut self, max: u8, frac_rem_cmp_tie: Ordering) {
        let len = 1 + self.int_digits + self.frac_digits;

        // round up if cropped is greater than tie, or if it is tie and current is odd
        let is_odd = self.digits[len - 1] & 1 != 0;
        let round_up =
            frac_rem_cmp_tie == Ordering::Greater || frac_rem_cmp_tie == Ordering::Equal && is_odd;
        if round_up {
            for b in self.digits[0..len].iter_mut().rev() {
                if *b < max {
                    *b += 1;
                    break;
                }
                *b = 0;
                // trim
                if self.frac_digits > 0 {
                    self.frac_digits -= 1;
                }
            }
        } else {
            for b in self.digits[len - self.frac_digits..len].iter_mut().rev() {
                if *b != 0 {
                    break;
                }
                self.frac_digits -= 1;
            }
        }
    }

    fn encode_digits(&mut self, upper: bool) {
        for digit in &mut self.digits[..1 + self.int_digits + self.frac_digits] {
            if *digit < 10 {
                *digit += b'0';
            } else {
                *digit += if upper { b'A' - 10 } else { b'a' - 10 };
            }
        }
    }

    fn pad_and_print(&self, is_neg: bool, maybe_prefix: &str, fmt: &mut Formatter) -> FmtResult {
        use core::fmt::Write;

        let sign = if is_neg {
            "-"
        } else if fmt.sign_plus() {
            "+"
        } else {
            ""
        };
        let prefix = if fmt.alternate() { maybe_prefix } else { "" };

        // For numbers with a negative exponent:
        //   * digits[int_digits] is the first non-zero digit
        //
        // For numbers with no significant integer bits:
        //   * digits starts with "0" and begin = 0
        //
        // For numbers with some significant integer bits, data can have:
        //   * no leading zeros => begin = 0
        //   * one leading zero => begin = 1
        //   * two leading zeros => begin = 2
        //
        // Two leading zeros can happen for decimal only. For example
        // with four significant integer bits, we could get anything
        // between 8 and 15, so two decimal digits are allocated apart
        // from the initial padding zero. This means that for 8, data
        // would begin as "008.", and begin = 2.
        let abs_begin = if self.exp[1] == b'-' {
            self.int_digits
        } else if self.int_digits == 0 || self.digits[0] != b'0' {
            0
        } else if self.digits[1] != b'0' {
            1
        } else {
            2
        };
        let end_zeros = fmt.precision().map_or(0, |x| x - self.frac_digits);
        let has_frac = self.frac_digits > 0 || end_zeros > 0;

        let digits_width = 1 + self.int_digits + self.frac_digits - abs_begin;
        let req_width = sign.len()
            + prefix.len()
            + digits_width
            + usize::from(has_frac)
            + end_zeros
            + self.exp_len;
        let pad = fmt
            .width()
            .and_then(|w| w.checked_sub(req_width))
            .unwrap_or(0);
        let (pad_left, pad_zeros, pad_right) = if fmt.sign_aware_zero_pad() {
            (0, pad, 0)
        } else {
            match fmt.align() {
                Some(Alignment::Left) => (0, 0, pad),
                Some(Alignment::Center) => (pad / 2, 0, pad - pad / 2),
                None | Some(Alignment::Right) => (pad, 0, 0),
            }
        };
        let fill = fmt.fill();

        for _ in 0..pad_left {
            fmt.write_char(fill)?;
        }
        fmt.write_str(sign)?;
        fmt.write_str(prefix)?;
        for _ in 0..pad_zeros {
            fmt.write_char('0')?;
        }
        let int_bytes = &self.digits[abs_begin..1 + self.int_digits];
        fmt.write_str(str::from_utf8(int_bytes).unwrap())?;
        if has_frac {
            fmt.write_char('.')?;
            let frac_bytes =
                &self.digits[1 + self.int_digits..1 + self.int_digits + self.frac_digits];
            fmt.write_str(str::from_utf8(frac_bytes).unwrap())?;
            for _ in 0..end_zeros {
                fmt.write_char('0')?;
            }
        }
        fmt.write_str(str::from_utf8(&self.exp[..self.exp_len]).unwrap())?;
        for _ in 0..pad_right {
            fmt.write_char(fill)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Format {
    Bin,
    Oct,
    LowHex,
    UpHex,
    Dec,
    LowExp,
    UpExp,
}

impl Format {
    fn digit_bits(self) -> u32 {
        match self {
            Format::Bin => 1,
            Format::Oct => 3,
            Format::LowHex | Format::UpHex => 4,
            Format::Dec | Format::LowExp | Format::UpExp => 4,
        }
    }
    fn max_digit(self) -> u8 {
        match self {
            Format::Bin => 1,
            Format::Oct => 7,
            Format::LowHex | Format::UpHex => 15,
            Format::Dec | Format::LowExp | Format::UpExp => 9,
        }
    }
    fn prefix(self) -> &'static str {
        match self {
            Format::Bin => "0b",
            Format::Oct => "0o",
            Format::LowHex | Format::UpHex => "0x",
            Format::Dec | Format::LowExp | Format::UpExp => "",
        }
    }
}

trait FmtHelper
where
    Self: Copy + Ord,
    Self: Shl<u32, Output = Self> + Shr<u32, Output = Self> + Add<Output = Self>,
    Self: WrappingCast<u8> + Mul10 + From<u8>,
{
    const ZERO: Self;
    const MSB: Self;
    const BITS: u32;

    type Half: FmtHelper;

    fn int_used_nbits(int: Self) -> u32;
    fn frac_used_nbits(frac: Self) -> u32;
    fn as_half(val: Self) -> Self::Half;
    fn div_rem_10(val: Self) -> (Self, u8);
    fn wrapping_neg(val: Self) -> Self;

    fn write_int_radix2(mut int: Self, format: Format, nbits: u32, buf: &mut Buffer) {
        if Self::Half::BITS == Self::BITS / 2 && nbits <= Self::Half::BITS {
            return FmtHelper::write_int_radix2(Self::as_half(int), format, nbits, buf);
        }
        let digit_bits = format.digit_bits();
        let mask = format.max_digit();
        for b in buf.int().iter_mut().rev() {
            debug_assert!(int != Self::ZERO);
            *b = int.wrapping_as::<u8>() & mask;
            int = int >> digit_bits;
        }
        debug_assert!(int == Self::ZERO);
    }

    fn write_frac_radix2(mut frac: Self, format: Format, nbits: u32, buf: &mut Buffer) -> Ordering {
        if Self::Half::BITS == Self::BITS / 2 && nbits <= Self::Half::BITS {
            return FmtHelper::write_frac_radix2(
                Self::as_half(frac >> Self::Half::BITS),
                format,
                nbits,
                buf,
            );
        }
        let digit_bits = format.digit_bits();
        let compl_digit_bits = Self::BITS - digit_bits;
        for b in &mut *buf.frac() {
            debug_assert!(frac != Self::ZERO);
            *b = (frac >> compl_digit_bits).wrapping_as::<u8>();
            frac = frac << digit_bits;
        }
        frac.cmp(&Self::MSB)
    }

    // returns the number of significant digits
    fn write_int_dec(mut int: Self, nbits: u32, buf: &mut Buffer) -> usize {
        if Self::Half::BITS == Self::BITS / 2 && nbits <= Self::Half::BITS {
            return FmtHelper::write_int_dec(Self::as_half(int), nbits, buf);
        }
        let mut sig = 0;
        for b in buf.int().iter_mut().rev() {
            let (q, r) = Self::div_rem_10(int);
            int = q;
            *b = r;
            if r != 0 || sig != 0 {
                sig += 1;
            }
        }
        debug_assert!(int == Self::ZERO);
        sig
    }

    fn write_frac_dec(
        mut frac: Self,
        nbits: u32,
        frac_format: DecFracFormat,
        buf: &mut Buffer,
    ) -> Ordering {
        if Self::Half::BITS == Self::BITS / 2 && nbits <= Self::Half::BITS {
            return FmtHelper::write_frac_dec(
                Self::as_half(frac >> Self::Half::BITS),
                nbits,
                frac_format,
                buf,
            );
        }

        let mut is_past_point = !frac_format.has_exp || frac_format.int_sig_digits > 0;
        let (auto_prec, mut rem_prec) = match frac_format.precision {
            Some(prec) => (false, prec),
            None => (true, 0),
        };
        if !auto_prec && frac_format.has_exp && is_past_point {
            rem_prec = rem_prec.saturating_sub(frac_format.int_sig_digits - 1);
        }
        if !auto_prec && is_past_point && buf.frac_digits > rem_prec {
            buf.frac_digits = rem_prec;
        }

        // add_5 is to add rounding when all bits are used
        let (mut tie, mut add_5) = if nbits == Self::BITS {
            (Self::ZERO, true)
        } else {
            (Self::MSB >> nbits, false)
        };
        for (i, b) in buf.frac().iter_mut().enumerate() {
            *b = Mul10::mul10_assign(&mut frac);

            // Check if very close to zero, to avoid things like 0.19999999 and 0.20000001.
            if auto_prec && frac < Self::from(10) || Self::wrapping_neg(frac) < Self::from(10) {
                buf.frac_digits = i + 1;
                break;
            }

            if auto_prec {
                // tie might overflow in last iteration when i = frac_digits - 1,
                // but it has no effect as all it can do is set frac_digits to i + 1
                Mul10::mul10_assign(&mut tie);
                if add_5 {
                    tie = tie + Self::from(5);
                    add_5 = false;
                }
                if frac < tie || Self::wrapping_neg(frac) < tie {
                    buf.frac_digits = i + 1;
                    break;
                }
            } else if frac_format.has_exp {
                debug_assert!(rem_prec > 0);
                if is_past_point {
                    rem_prec -= 1;
                    if rem_prec == 0 {
                        buf.frac_digits = i + 1;
                        break;
                    }
                } else if *b != 0 {
                    is_past_point = true;
                    // *b is still before point, so do not decrement rem_prec here.
                }
            }
        }
        frac.cmp(&Self::MSB)
    }
}

macro_rules! impl_format_helper {
    ($U:ident, $H:ident) => {
        impl FmtHelper for $U {
            const ZERO: $U = 0;
            const MSB: $U = 1 << ($U::BITS - 1);
            const BITS: u32 = $U::BITS;

            type Half = $H;

            fn int_used_nbits(int: $U) -> u32 {
                $U::BITS - int.leading_zeros()
            }

            fn frac_used_nbits(frac: $U) -> u32 {
                $U::BITS - frac.trailing_zeros()
            }

            fn as_half(val: $U) -> Self::Half {
                val as Self::Half
            }

            fn div_rem_10(val: $U) -> ($U, u8) {
                (val / 10, (val % 10).wrapping_cast())
            }

            fn wrapping_neg(val: $U) -> $U {
                val.wrapping_neg()
            }
        }
    };
}

impl_format_helper! { u8, u8 }
impl_format_helper! { u16, u8 }
impl_format_helper! { u32, u16 }
impl_format_helper! { u64, u32 }
impl_format_helper! { u128, u64 }

fn fmt<U: FmtHelper>(
    (neg, abs): (bool, U),
    frac_nbits: u32,
    format: Format,
    fmt: &mut Formatter,
) -> FmtResult {
    let (int, frac) = if frac_nbits == 0 {
        (abs, U::ZERO)
    } else if frac_nbits == U::BITS {
        (U::ZERO, abs)
    } else {
        (abs >> frac_nbits, abs << (U::BITS - frac_nbits))
    };
    match format {
        Format::Bin | Format::Oct | Format::LowHex | Format::UpHex => {
            fmt_radix2((neg, int, frac), format, fmt)
        }
        Format::Dec | Format::LowExp | Format::UpExp => {
            fmt_dec((neg, int, frac), frac_nbits, format, fmt)
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct DecFracFormat {
    int_sig_digits: usize,
    has_exp: bool,
    precision: Option<usize>,
}

fn fmt_dec<U: FmtHelper>(
    (neg, int, frac): (bool, U, U),
    frac_nbits: u32,
    format: Format,
    fmt: &mut Formatter,
) -> FmtResult {
    let int_used_nbits = FmtHelper::int_used_nbits(int);
    let frac_used_nbits = FmtHelper::frac_used_nbits(frac);
    let int_max_len = ceil_log10_2_times(int_used_nbits);
    let frac_max_len = if fmt.precision().is_some() {
        // for specified precision, we want exact fractions till the very end
        frac_used_nbits
    } else {
        // for auto precision, we don't need more than ceil(log10(2) × frac_nbits)
        ceil_log10_2_times(frac_nbits)
    };
    let mut buf = Buffer::new(int_max_len, frac_max_len);

    let int_sig_digits = FmtHelper::write_int_dec(int, int_used_nbits, &mut buf);
    let has_exp = matches!(format, Format::UpExp | Format::LowExp);
    let exp_is_upper = matches!(format, Format::UpExp);
    let frac_format = DecFracFormat {
        int_sig_digits,
        has_exp,
        precision: fmt.precision(),
    };
    let frac_rem_cmp_tie = FmtHelper::write_frac_dec(frac, frac_nbits, frac_format, &mut buf);
    if has_exp {
        buf.format_exp_dec(exp_is_upper);
    }
    buf.finish(format, neg, frac_rem_cmp_tie, fmt)
}

fn fmt_radix2<U: FmtHelper>(
    (neg, int, frac): (bool, U, U),
    format: Format,
    fmt: &mut Formatter,
) -> FmtResult {
    let digit_bits = format.digit_bits();
    let int_used_nbits = FmtHelper::int_used_nbits(int);
    let int_digits = (int_used_nbits + digit_bits - 1) / digit_bits;
    let frac_used_nbits = FmtHelper::frac_used_nbits(frac);
    let mut frac_digits = (frac_used_nbits + digit_bits - 1) / digit_bits;
    if let Some(precision) = fmt.precision() {
        // frac_digits fits in usize, but precision might wrap to 0 in u32
        frac_digits = cmp::min(frac_digits as usize, precision) as u32;
    }

    let mut buf = Buffer::new(int_digits, frac_digits);
    FmtHelper::write_int_radix2(int, format, int_used_nbits, &mut buf);
    // for bin, oct, hex, we can simply pass frac_used_bits to write_frac
    let frac_rem_cmp_tie = FmtHelper::write_frac_radix2(frac, format, frac_used_nbits, &mut buf);
    buf.finish(format, neg, frac_rem_cmp_tie, fmt)
}

macro_rules! impl_fmt {
    ($Fixed:ident($LeEqU:ident, $Inner:ident)) => {
        impl<Frac: $LeEqU> Display for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                fmt(neg_abs, Self::FRAC_NBITS, Format::Dec, f)
            }
        }

        impl<Frac: Unsigned> Debug for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                if Frac::U32 > $Inner::BITS {
                    match debug_hex::is_debug_hex(f) {
                        IsDebugHex::Lower => {
                            f.write_fmt(format_args!("(0x{:x}", self.to_bits()))?;
                        }
                        IsDebugHex::Upper => {
                            f.write_fmt(format_args!("(0x{:X}", self.to_bits()))?;
                        }
                        IsDebugHex::No => {
                            f.write_fmt(format_args!("({}", self.to_bits()))?;
                        }
                    }
                    return f.write_fmt(format_args!(" >> {})", Frac::U32));
                }
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                match debug_hex::is_debug_hex(f) {
                    IsDebugHex::Lower => fmt(neg_abs, Frac::U32, Format::LowHex, f),
                    IsDebugHex::Upper => fmt(neg_abs, Frac::U32, Format::UpHex, f),
                    IsDebugHex::No => fmt(neg_abs, Frac::U32, Format::Dec, f),
                }
            }
        }

        impl<Frac: $LeEqU> Binary for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                fmt(neg_abs, Self::FRAC_NBITS, Format::Bin, f)
            }
        }

        impl<Frac: $LeEqU> Octal for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                fmt(neg_abs, Self::FRAC_NBITS, Format::Oct, f)
            }
        }

        impl<Frac: $LeEqU> LowerHex for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                fmt(neg_abs, Self::FRAC_NBITS, Format::LowHex, f)
            }
        }

        impl<Frac: $LeEqU> UpperHex for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                fmt(neg_abs, Self::FRAC_NBITS, Format::UpHex, f)
            }
        }

        impl<Frac: $LeEqU> LowerExp for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                fmt(neg_abs, Self::FRAC_NBITS, Format::LowExp, f)
            }
        }

        impl<Frac: $LeEqU> UpperExp for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                let neg_abs = int_helper::$Inner::neg_abs(self.to_bits());
                fmt(neg_abs, Self::FRAC_NBITS, Format::UpExp, f)
            }
        }
    };
}

impl_fmt! { FixedU8(LeEqU8, u8) }
impl_fmt! { FixedU16(LeEqU16, u16) }
impl_fmt! { FixedU32(LeEqU32, u32) }
impl_fmt! { FixedU64(LeEqU64, u64) }
impl_fmt! { FixedU128(LeEqU128, u128) }
impl_fmt! { FixedI8(LeEqU8, i8) }
impl_fmt! { FixedI16(LeEqU16, i16) }
impl_fmt! { FixedI32(LeEqU32, i32) }
impl_fmt! { FixedI64(LeEqU64, i64) }
impl_fmt! { FixedI128(LeEqU128, i128) }

// ceil(i × log_10 2), works for input < 112_816
fn ceil_log10_2_times(int_bits: u32) -> u32 {
    debug_assert!(int_bits < 112_816);
    ((u64::from(int_bits) * 0x4D10_4D43 + 0xFFFF_FFFF) >> 32) as u32
}

pub(crate) trait Mul10: Sized {
    fn mul10_assign(slf: &mut Self) -> u8;
}
macro_rules! mul10_widen {
    ($Single:ty, $Double:ty) => {
        impl Mul10 for $Single {
            #[inline]
            fn mul10_assign(x: &mut $Single) -> u8 {
                let prod = <$Double>::from(*x) * 10;
                *x = prod as $Single;
                (prod >> <$Single>::BITS) as u8
            }
        }
    };
}
mul10_widen! { u8, u16 }
mul10_widen! { u16, u32 }
mul10_widen! { u32, u64 }
mul10_widen! { u64, u128 }
impl Mul10 for u128 {
    #[inline]
    fn mul10_assign(x: &mut u128) -> u8 {
        const LO_MASK: u128 = !(!0 << 64);
        let hi = (*x >> 64) * 10;
        let lo = (*x & LO_MASK) * 10;
        // Workaround for https://github.com/rust-lang/rust/issues/63384
        // let (wrapped, overflow) = (hi << 64).overflowing_add(lo);
        // ((hi >> 64) as u8 + u8::from(overflow), wrapped)
        let (hi_lo, hi_hi) = (hi as u64, (hi >> 64) as u64);
        let (lo_lo, lo_hi) = (lo as u64, (lo >> 64) as u64);
        let (wrapped, overflow) = hi_lo.overflowing_add(lo_hi);
        *x = (u128::from(wrapped) << 64) | u128::from(lo_lo);
        hi_hi as u8 + u8::from(overflow)
    }
}

#[cfg(test)]
mod tests {
    use crate::{display, types::*};
    use std::{
        format,
        string::{String, ToString},
    };

    #[test]
    fn format() {
        let pos = I16F16::from_num(12.3);
        assert_eq!(format!("{pos:+}"), "+12.3");
        assert_eq!(format!("{pos:.20}"), "12.30000305175781250000");
        assert_eq!(format!("{pos:+08}"), "+00012.3");
        assert_eq!(format!("{pos:+#08}"), "+00012.3");
        assert_eq!(format!("{pos:+08X}"), "+0C.4CCD");
        assert_eq!(format!("{pos:+08.1X}"), "+0000C.5");
        assert_eq!(format!("{pos:+#08X}"), "+0xC.4CCD");
        assert_eq!(format!("{pos:+#08.1X}"), "+0x00C.5");

        assert_eq!(format!("{pos:#<8}"), "12.3####");
        assert_eq!(format!("{pos:#^8}"), "##12.3##");
        assert_eq!(format!("{pos:#^9}"), "##12.3###");
        assert_eq!(format!("{pos:#>8}"), "####12.3");
        assert_eq!(format!("{pos:#^08}"), "000012.3");

        assert_eq!(format!("{pos:^8e}"), " 1.23e1 ");
        assert_eq!(format!("{pos:^08E}"), "001.23E1");
        assert_eq!(format!("{pos:.20e}"), "1.23000030517578125000e1");

        let pos = I16F16::from_bits(9);
        assert_eq!(format!("{pos}"), "0.00014");
        assert_eq!(format!("{pos:.16}"), "0.0001373291015625");
        assert_eq!(format!("{pos:e}"), "1.4e-4");
        assert_eq!(format!("{pos:.16e}"), "1.3732910156250000e-4");
    }

    fn trim_frac_zeros(mut x: &str) -> &str {
        while x.ends_with('0') {
            x = &x[..x.len() - 1];
        }
        if x.ends_with('.') {
            x = &x[..x.len() - 1];
        }
        x
    }

    fn up_frac_digits(x: &mut String, frac_digits: usize) {
        if let Some(point) = x.find('.') {
            if let Some(additional) = frac_digits.checked_sub(x.len() - point - 1) {
                x.reserve(additional);
                for _ in 0..additional {
                    x.push('0');
                }
            }
        } else {
            x.reserve(frac_digits + 1);
            x.push('.');
            for _ in 0..frac_digits {
                x.push('0');
            }
        }
    }

    #[test]
    fn hex() {
        for i in 0..(1u32 << 7) {
            let p = 0x1234_5678_9abc_def0u64 ^ u64::from(i);
            let n = -0x1234_5678_9abc_def0i64 ^ i64::from(i);
            let f_p = U57F7::from_bits(p);
            let f_n = I57F7::from_bits(n);
            let mut check_p = format!("{:x}.{:02x}", p >> 7, (p & 0x7f) << 1);
            up_frac_digits(&mut check_p, 1000);
            let trimmed_p = trim_frac_zeros(&check_p);
            let mut check_n = format!("-{:x}.{:02x}", n.abs() >> 7, (n.abs() & 0x7f) << 1);
            up_frac_digits(&mut check_n, 1000);
            let trimmed_n = trim_frac_zeros(&check_n);
            assert_eq!(format!("{f_p:.1000x}"), check_p);
            assert_eq!(format!("{f_p:x}"), trimmed_p);
            assert_eq!(format!("{f_n:.1000x}"), check_n);
            assert_eq!(format!("{f_n:x}"), trimmed_n);
        }
    }

    #[test]
    fn debug_hex() {
        let v = I16F16::MAX;
        assert_eq!(format!("{v:?}"), "32767.99998");
        assert_eq!(format!("{v:x?}"), "7fff.ffff");
        assert_eq!(format!("{v:X?}"), "7FFF.FFFF");
        assert_eq!(format!("{v:010X?}"), "07FFF.FFFF");
    }

    #[test]
    fn dec() {
        for i in 0..(1 << 7) {
            // use 24 bits of precision to be like f32
            let bits = (!0u32 >> 8) ^ i;
            let fix = U25F7::from_bits(bits);
            let flt = (bits as f32) / 7f32.exp2();
            assert_eq!(format!("{fix}"), format!("{flt}"));
            assert_eq!(U25F7::from_num(flt), fix);
            assert_eq!(fix.to_num::<f32>(), flt);
        }
    }

    #[test]
    fn display_frac() {
        assert_eq!(
            format!("{:X}", I0F128::from_bits(!0)),
            "-0.00000000000000000000000000000001"
        );
        assert_eq!(format!("{:X}", I0F64::from_bits(!0)), "-0.0000000000000001");
        assert_eq!(format!("{:X}", I0F32::from_bits(!0)), "-0.00000001");
        assert_eq!(format!("{:X}", I0F16::from_bits(!0)), "-0.0001");
        assert_eq!(format!("{:X}", I0F8::from_bits(!0)), "-0.01");
        assert_eq!(
            format!("{:X}", U0F128::from_bits(!0)),
            "0.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        );
        assert_eq!(format!("{:X}", U0F64::from_bits(!0)), "0.FFFFFFFFFFFFFFFF");
        assert_eq!(format!("{:X}", U0F32::from_bits(!0)), "0.FFFFFFFF");
        assert_eq!(format!("{:X}", U0F16::from_bits(!0)), "0.FFFF");
        assert_eq!(format!("{:X}", U0F8::from_bits(!0)), "0.FF");

        assert_eq!(
            format!("{}", I0F128::from_bits(!0)),
            "-0.000000000000000000000000000000000000003"
        );
        assert_eq!(
            format!("{}", I0F64::from_bits(!0)),
            "-0.00000000000000000005"
        );
        assert_eq!(format!("{}", I0F32::from_bits(!0)), "-0.0000000002");
        assert_eq!(format!("{}", I0F16::from_bits(!0)), "-0.00002");
        assert_eq!(format!("{}", I0F8::from_bits(!0)), "-0.004");
        assert_eq!(
            format!("{}", U0F128::from_bits(!0)),
            "0.999999999999999999999999999999999999997"
        );
        assert_eq!(
            format!("{}", U0F64::from_bits(!0)),
            "0.99999999999999999995"
        );
        assert_eq!(format!("{}", U0F32::from_bits(!0)), "0.9999999998");
        assert_eq!(format!("{}", U0F16::from_bits(!0)), "0.99998");
        assert_eq!(format!("{}", U0F8::from_bits(!0)), "0.996");

        // check overflow issues in <u128 as Mul10>::mul10
        let no_internal_overflow_bits = 0xe666_6666_6666_6665_ffff_ffff_ffff_ffffu128;
        let internal_overflow_bits = 0xe666_6666_6666_6666_ffff_ffff_ffff_ffffu128;
        assert_eq!(
            format!("{:X}", U0F128::from_bits(no_internal_overflow_bits)),
            "0.E666666666666665FFFFFFFFFFFFFFFF"
        );
        assert_eq!(
            format!("{:X}", U0F128::from_bits(internal_overflow_bits)),
            "0.E666666666666666FFFFFFFFFFFFFFFF"
        );
        assert_eq!(
            format!("{}", U0F128::from_bits(no_internal_overflow_bits)),
            "0.899999999999999999978315956550289911317"
        );
        assert_eq!(
            format!("{}", U0F128::from_bits(internal_overflow_bits)),
            "0.900000000000000000032526065174565133017"
        );
    }

    #[test]
    fn close_to_round_decimal() {
        for i in 0..1000u16 {
            // f32 has 24 bits of precision, so we use 1 bit for the
            // integer part to have exactly 23 bits for the fraction
            let float = f32::from(i + 1000) / 1000.;
            let fix = U9F23::from_num(float);
            let check = format!("1.{i:03}");
            assert_eq!(format!("{fix}"), trim_frac_zeros(&check));
            assert_eq!(format!("{fix}"), format!("{float}"));
            for prec in 0..10 {
                assert_eq!(format!("{fix:.prec$}"), format!("{float:.prec$}"));
            }
        }
    }

    #[test]
    fn check_ceil_log10_2_times() {
        for i in 0..112_816 {
            let check = (f64::from(i) * 2f64.log10()).ceil() as u32;
            assert_eq!(display::ceil_log10_2_times(i), check);
        }
    }

    #[test]
    fn rounding() {
        let i = U8F8::from_bits(0xFF80);
        assert_eq!(format!("{i}"), "255.5");
        assert_eq!(format!("{i:?}"), "255.5");
        assert_eq!(format!("{i:.0}"), "256");
        assert_eq!(format!("{i:b}"), "11111111.1");
        assert_eq!(format!("{i:.0b}"), "100000000");
        assert_eq!(format!("{i:o}"), "377.4");
        assert_eq!(format!("{i:.0o}"), "400");
        assert_eq!(format!("{i:X}"), "FF.8");
        assert_eq!(format!("{i:.0X}"), "100");

        let i = U8F8::from_bits(0xFE80);
        assert_eq!(format!("{i}"), "254.5");
        assert_eq!(format!("{i:?}"), "254.5");
        assert_eq!(format!("{i:.0}"), "254");
        assert_eq!(format!("{i:b}"), "11111110.1");
        assert_eq!(format!("{i:.0b}"), "11111110");
        assert_eq!(format!("{i:o}"), "376.4");
        assert_eq!(format!("{i:.0o}"), "376");
        assert_eq!(format!("{i:X}"), "FE.8");
        assert_eq!(format!("{i:.0X}"), "FE");

        let i = U8F8::from_bits(0xDDDD);
        assert_eq!(format!("{i}"), "221.863");
        assert_eq!(format!("{i:?}"), "221.863");
        assert_eq!(format!("{i:.0}"), "222");
        assert_eq!(format!("{i:.1}"), "221.9");
        assert_eq!(format!("{i:.2}"), "221.86");
        assert_eq!(format!("{i:.3}"), "221.863");
        assert_eq!(format!("{i:.4}"), "221.8633");
        assert_eq!(format!("{i:.5}"), "221.86328");
        assert_eq!(format!("{i:.6}"), "221.863281");
        assert_eq!(format!("{i:.7}"), "221.8632812");
        assert_eq!(format!("{i:.8}"), "221.86328125");
        assert_eq!(format!("{i:.9}"), "221.863281250");
        assert_eq!(format!("{i:b}"), "11011101.11011101");
        assert_eq!(format!("{i:.0b}"), "11011110");
        assert_eq!(format!("{i:.1b}"), "11011110.0");
        assert_eq!(format!("{i:.2b}"), "11011101.11");
        assert_eq!(format!("{i:.3b}"), "11011101.111");
        assert_eq!(format!("{i:.4b}"), "11011101.1110");
        assert_eq!(format!("{i:.5b}"), "11011101.11100");
        assert_eq!(format!("{i:.6b}"), "11011101.110111");
        assert_eq!(format!("{i:.7b}"), "11011101.1101110");
        assert_eq!(format!("{i:.8b}"), "11011101.11011101");
        assert_eq!(format!("{i:.9b}"), "11011101.110111010");
        assert_eq!(format!("{i:o}"), "335.672");
        assert_eq!(format!("{i:.0o}"), "336");
        assert_eq!(format!("{i:.1o}"), "335.7");
        assert_eq!(format!("{i:.2o}"), "335.67");
        assert_eq!(format!("{i:.3o}"), "335.672");
        assert_eq!(format!("{i:.4o}"), "335.6720");
        assert_eq!(format!("{i:X}"), "DD.DD");
        assert_eq!(format!("{i:.0X}"), "DE");
        assert_eq!(format!("{i:.0X}"), "DE");
        assert_eq!(format!("{i:.1X}"), "DD.E");
        assert_eq!(format!("{i:.2X}"), "DD.DD");
        assert_eq!(format!("{i:.3X}"), "DD.DD0");
    }

    #[test]
    fn compare_frac0_int() {
        for u in 0..=255u8 {
            let i = u as i8;
            let (ifix, ufix) = (I8F0::from_bits(i), U8F0::from_bits(u));
            assert_eq!(ifix.to_string(), i.to_string());
            assert_eq!(ufix.to_string(), u.to_string());
            if i >= 0 {
                assert_eq!(format!("{ifix:#X}"), format!("{i:#X}"));
                assert_eq!(format!("{ifix:#b}"), format!("{i:#b}"));
            } else {
                let abs_i = i.wrapping_neg() as u8;
                assert_eq!(format!("{ifix:#X}"), format!("-{abs_i:#X}"));
                assert_eq!(format!("{ifix:#b}"), format!("-{abs_i:#b}"));
            }
            assert_eq!(format!("{ufix:#x}"), format!("{u:#x}"));
            assert_eq!(format!("{ufix:#o}"), format!("{u:#o}"));
        }
    }

    #[test]
    fn compare_frac4_float() {
        for u in 0..=255u8 {
            // I4F4 and U4F4 are displayed like f32 when the f32
            // display precision is the number of fractional digits
            // displayed for fixed-point. This verifies correct display
            // of the integer part.
            let (ifix, ufix) = (I4F4::from_bits(u as i8), U4F4::from_bits(u));
            let (iflo, uflo) = (ifix.to_num::<f32>(), ufix.to_num::<f32>());
            let (sifix, sufix) = (ifix.to_string(), ufix.to_string());
            let pifix = sifix.find('.').map_or(0, |p| sifix.len() - 1 - p);
            let pufix = sufix.find('.').map_or(0, |p| sufix.len() - 1 - p);
            let (siflo, suflo) = (format!("{iflo:.pifix$}"), format!("{uflo:.pufix$}"));
            assert_eq!(sifix, siflo);
            assert_eq!(sufix, suflo);

            // I28F4 and U28F4 are displayed like f32 when the f32 has
            // four bits of precision dedicated to the fractional
            // part. For f32, this requires the magnitude’s integer
            // part to have 20 significant bits: (1 << 19)..(1 << 20).
            let ifixed =
                I28F4::from(ifix) + I28F4::from_num(i32::from(ifix.to_bits().signum()) << 19);
            let ufixed = U28F4::from(ufix) + U28F4::from_num(1 << 19);
            let (ifloat, ufloat) = (ifixed.to_num::<f32>(), ufixed.to_num::<f32>());
            let (sifixed, sufixed) = (ifixed.to_string(), ufixed.to_string());
            let (sifloat, sufloat) = (ifloat.to_string(), ufloat.to_string());
            assert_eq!(sifixed, sifloat);
            assert_eq!(sufixed, sufloat);

            // The fractional parts of I4F4 and U4F4 are displayed
            // like the fractional parts of I28F4 and U28F4
            // respectively.
            let sifix_frac = sifix.find('.').map(|i| &sifix[i..]);
            let sifixed_frac = sifixed.find('.').map(|i| &sifixed[i..]);
            assert_eq!(sifix_frac, sifixed_frac);
            let sufix_frac = sufix.find('.').map(|i| &sufix[i..]);
            let sufixed_frac = sufixed.find('.').map(|i| &sufixed[i..]);
            assert_eq!(sufix_frac, sufixed_frac);
        }
    }

    #[test]
    fn compare_frac17_float() {
        for u in 0..(1 << 17) {
            // 24 bits of precision: 17 fractional bits + 7 significant integer bits
            let fix = U15F17::from_bits(u) + U15F17::from_num(99);
            let fix_pos = I15F17::from_num(fix);
            let fix_neg = -fix_pos;
            let (flo, flo_neg) = (fix.to_num::<f32>(), fix_neg.to_num::<f32>());

            let fix_str = fix.to_string();
            let fix_pos_str = fix_pos.to_string();
            let fix_neg_str = fix_neg.to_string();
            assert_eq!(fix_str, flo.to_string());
            assert_eq!(fix_str, fix_pos_str);
            assert_eq!(fix_neg_str, flo_neg.to_string());
            if u != 0 {
                assert_eq!(&fix_neg_str[..1], "-");
                assert_eq!(&fix_neg_str[1..], fix_pos_str);
            }

            let fix_str3 = format!("{fix:.3}");
            let fix_pos_str3 = format!("{fix_pos:.3}");
            let fix_neg_str3 = format!("{fix_neg:.3}");
            assert_eq!(fix_str3, format!("{flo:.3}"));
            assert_eq!(fix_str3, fix_pos_str3);
            assert_eq!(fix_neg_str3, format!("{flo_neg:.3}"));
            if u != 0 {
                assert_eq!(&fix_neg_str3[..1], "-");
                assert_eq!(&fix_neg_str3[1..], fix_pos_str3);
            }
        }
    }
}
