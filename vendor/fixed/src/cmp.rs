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

#![allow(deprecated)]

use crate::{
    float_helper,
    int_helper::{self, IntFixed},
    types::extra::Unsigned,
    F128Bits, FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32,
    FixedU64, FixedU8, F128,
};
use core::cmp::Ordering;
use core::ops::{Shl, Shr};
use half::{bf16, f16};

macro_rules! fixed_cmp_int {
    ($Fixed:ident, $Int:ident) => {
        impl<Frac: Unsigned> PartialEq<$Int> for $Fixed<Frac> {
            #[inline]
            fn eq(&self, rhs: &$Int) -> bool {
                let fixed_rhs = IntFixed(*rhs).fixed();
                PartialEq::eq(self, &fixed_rhs)
            }
        }

        impl<Frac: Unsigned> PartialEq<$Fixed<Frac>> for $Int {
            #[inline]
            fn eq(&self, rhs: &$Fixed<Frac>) -> bool {
                let fixed_lhs = IntFixed(*self).fixed();
                PartialEq::eq(&fixed_lhs, rhs)
            }
        }

        impl<Frac: Unsigned> PartialOrd<$Int> for $Fixed<Frac> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Int) -> Option<Ordering> {
                let fixed_rhs = IntFixed(*rhs).fixed();
                PartialOrd::partial_cmp(self, &fixed_rhs)
            }

            #[inline]
            fn lt(&self, rhs: &$Int) -> bool {
                let fixed_rhs = IntFixed(*rhs).fixed();
                PartialOrd::lt(self, &fixed_rhs)
            }

            #[inline]
            fn le(&self, rhs: &$Int) -> bool {
                let fixed_rhs = IntFixed(*rhs).fixed();
                PartialOrd::le(self, &fixed_rhs)
            }

            #[inline]
            fn gt(&self, rhs: &$Int) -> bool {
                let fixed_rhs = IntFixed(*rhs).fixed();
                PartialOrd::gt(self, &fixed_rhs)
            }

            #[inline]
            fn ge(&self, rhs: &$Int) -> bool {
                let fixed_rhs = IntFixed(*rhs).fixed();
                PartialOrd::ge(self, &fixed_rhs)
            }
        }

        impl<Frac: Unsigned> PartialOrd<$Fixed<Frac>> for $Int {
            #[inline]
            fn partial_cmp(&self, rhs: &$Fixed<Frac>) -> Option<Ordering> {
                let fixed_lhs = IntFixed(*self).fixed();
                PartialOrd::partial_cmp(&fixed_lhs, rhs)
            }

            #[inline]
            fn lt(&self, rhs: &$Fixed<Frac>) -> bool {
                let fixed_lhs = IntFixed(*self).fixed();
                PartialOrd::lt(&fixed_lhs, rhs)
            }

            #[inline]
            fn le(&self, rhs: &$Fixed<Frac>) -> bool {
                let fixed_lhs = IntFixed(*self).fixed();
                PartialOrd::le(&fixed_lhs, rhs)
            }

            #[inline]
            fn gt(&self, rhs: &$Fixed<Frac>) -> bool {
                let fixed_lhs = IntFixed(*self).fixed();
                PartialOrd::gt(&fixed_lhs, rhs)
            }

            #[inline]
            fn ge(&self, rhs: &$Fixed<Frac>) -> bool {
                let fixed_lhs = IntFixed(*self).fixed();
                PartialOrd::ge(&fixed_lhs, rhs)
            }
        }
    };
}

// Zero must NOT be neg
struct Value<U> {
    neg: bool,
    abs: U,
    bits: u32,
    frac_bits: i32,
}

#[inline]
// lhs_frac >= rhs_frac
fn float_rhs_shl<U>(rhs_abs: U, bits: u32, lhs_frac: i32, rhs_frac: i32) -> Option<U>
where
    U: Copy + Eq + TryFrom<u32> + Shl<u32, Output = U> + Shr<u32, Output = U>,
{
    debug_assert!(lhs_frac >= rhs_frac);
    let rhs_shl = lhs_frac.wrapping_sub(rhs_frac) as u32;
    let Ok(rhs_zero) = U::try_from(0u32) else {
        unreachable!();
    };
    if rhs_abs == rhs_zero {
        Some(rhs_zero)
    } else if rhs_shl >= bits {
        None
    } else {
        let shifted = rhs_abs << rhs_shl;
        if (shifted >> rhs_shl) == rhs_abs {
            Some(shifted)
        } else {
            None
        }
    }
}

#[inline]
fn float_eq_even<U>(lhs: Value<U>, rhs: Value<U>) -> bool
where
    U: Copy + Eq + TryFrom<u32> + Shl<u32, Output = U> + Shr<u32, Output = U>,
{
    if lhs.frac_bits < rhs.frac_bits {
        return float_eq_even(rhs, lhs);
    }

    if lhs.neg != rhs.neg {
        return false;
    }

    // lhs.frac_bits >= rhs.frac_bits
    match float_rhs_shl(rhs.abs, rhs.bits, lhs.frac_bits, rhs.frac_bits) {
        None => false,
        Some(shifted_rhs_abs) => lhs.abs == shifted_rhs_abs,
    }
}

#[inline]
fn float_eq<Lhs, Rhs>(lhs: Value<Lhs>, rhs: Value<Rhs>) -> bool
where
    Lhs: Copy + Eq + TryFrom<u32> + TryFrom<Rhs> + Shl<u32, Output = Lhs> + Shr<u32, Output = Lhs>,
    Rhs: Copy + Eq + TryFrom<u32> + TryFrom<Lhs> + Shl<u32, Output = Rhs> + Shr<u32, Output = Rhs>,
{
    if lhs.bits >= rhs.bits {
        let Ok(rhs_abs) = Lhs::try_from(rhs.abs) else {
            unreachable!();
        };
        let rhs = Value {
            neg: rhs.neg,
            abs: rhs_abs,
            bits: lhs.bits,
            frac_bits: rhs.frac_bits,
        };
        float_eq_even(lhs, rhs)
    } else {
        let Ok(lhs_abs) = Rhs::try_from(lhs.abs) else {
            unreachable!();
        };
        let lhs = Value {
            neg: lhs.neg,
            abs: lhs_abs,
            bits: rhs.bits,
            frac_bits: lhs.frac_bits,
        };
        float_eq_even(lhs, rhs)
    }
}

#[inline]
fn float_cmp_even<U>(lhs: Value<U>, rhs: Value<U>) -> Ordering
where
    U: Copy + Ord + TryFrom<u32> + Shl<u32, Output = U> + Shr<u32, Output = U>,
{
    if lhs.frac_bits < rhs.frac_bits {
        return float_cmp_even(rhs, lhs).reverse();
    }

    if !lhs.neg && rhs.neg {
        return Ordering::Greater;
    }
    if lhs.neg && !rhs.neg {
        return Ordering::Less;
    }

    match float_rhs_shl(rhs.abs, rhs.bits, lhs.frac_bits, rhs.frac_bits) {
        None => {
            // rhs is so large it doesn't fit
            if lhs.neg {
                // both lhs and rhs are negative, and rhs is even more negative
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        Some(shifted_rhs_abs) => {
            if lhs.neg {
                // both lhs are negative, so reverse order
                shifted_rhs_abs.cmp(&lhs.abs)
            } else {
                lhs.abs.cmp(&shifted_rhs_abs)
            }
        }
    }
}

#[inline]
fn float_cmp<Lhs, Rhs>(lhs: Value<Lhs>, rhs: Value<Rhs>) -> Ordering
where
    Lhs: Copy + Ord + TryFrom<u32> + TryFrom<Rhs> + Shl<u32, Output = Lhs> + Shr<u32, Output = Lhs>,
    Rhs: Copy + Ord + TryFrom<u32> + TryFrom<Lhs> + Shl<u32, Output = Rhs> + Shr<u32, Output = Rhs>,
{
    if lhs.bits >= rhs.bits {
        let Ok(rhs_abs) = Lhs::try_from(rhs.abs) else {
            unreachable!();
        };
        let rhs = Value {
            neg: rhs.neg,
            abs: rhs_abs,
            bits: lhs.bits,
            frac_bits: rhs.frac_bits,
        };
        float_cmp_even(lhs, rhs)
    } else {
        let Ok(lhs_abs) = Rhs::try_from(lhs.abs) else {
            unreachable!();
        };
        let lhs = Value {
            neg: lhs.neg,
            abs: lhs_abs,
            bits: rhs.bits,
            frac_bits: lhs.frac_bits,
        };
        float_cmp_even(lhs, rhs)
    }
}

macro_rules! fixed_cmp_float {
    ($Fix:ident($Inner:ident), $Float:ident, $FloatBits:ident) => {
        impl<Frac: Unsigned> PartialEq<$Float> for $Fix<Frac> {
            #[inline]
            fn eq(&self, rhs: &$Float) -> bool {
                use float_helper::$Float::Kind;
                let (lhs_neg, lhs_abs) = int_helper::$Inner::neg_abs(self.to_bits());
                let lhs = Value {
                    neg: lhs_neg,
                    abs: lhs_abs,
                    bits: $Inner::BITS,
                    frac_bits: Frac::to_i32(),
                };
                let Kind::Finite {
                    neg: rhs_neg,
                    abs: rhs_abs,
                    frac_bits: rhs_frac,
                } = float_helper::$Float::kind(*rhs)
                else {
                    return false;
                };
                let rhs = Value {
                    neg: rhs_neg,
                    abs: rhs_abs,
                    bits: $FloatBits::BITS,
                    frac_bits: rhs_frac,
                };

                float_eq(lhs, rhs)
            }
        }

        impl<Frac: Unsigned> PartialEq<$Fix<Frac>> for $Float {
            #[inline]
            fn eq(&self, rhs: &$Fix<Frac>) -> bool {
                rhs.eq(self)
            }
        }

        impl<Frac: Unsigned> PartialOrd<$Float> for $Fix<Frac> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Float) -> Option<Ordering> {
                use float_helper::$Float::Kind;
                let (lhs_neg, lhs_abs) = int_helper::$Inner::neg_abs(self.to_bits());
                let lhs = Value {
                    neg: lhs_neg,
                    abs: lhs_abs,
                    bits: $Inner::BITS,
                    frac_bits: Frac::to_i32(),
                };
                let (rhs_neg, rhs_abs, rhs_frac) = match float_helper::$Float::kind(*rhs) {
                    Kind::Finite {
                        neg,
                        abs,
                        frac_bits,
                    } => (neg, abs, frac_bits),
                    Kind::Infinite { neg } => {
                        return if neg {
                            Some(Ordering::Greater)
                        } else {
                            Some(Ordering::Less)
                        };
                    }
                    Kind::NaN => return None,
                };
                let rhs = Value {
                    neg: rhs_neg,
                    abs: rhs_abs,
                    bits: $FloatBits::BITS,
                    frac_bits: rhs_frac,
                };

                Some(float_cmp(lhs, rhs))
            }
        }

        impl<Frac: Unsigned> PartialOrd<$Fix<Frac>> for $Float {
            #[inline]
            fn partial_cmp(&self, rhs: &$Fix<Frac>) -> Option<Ordering> {
                rhs.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}

macro_rules! fixed_cmp_all {
    ($Fix:ident($LeEqU:ident, $Inner:ident)) => {
        fixed_cmp_int! { $Fix, i8 }
        fixed_cmp_int! { $Fix, i16 }
        fixed_cmp_int! { $Fix, i32 }
        fixed_cmp_int! { $Fix, i64 }
        fixed_cmp_int! { $Fix, i128 }
        fixed_cmp_int! { $Fix, isize }
        fixed_cmp_int! { $Fix, u8 }
        fixed_cmp_int! { $Fix, u16 }
        fixed_cmp_int! { $Fix, u32 }
        fixed_cmp_int! { $Fix, u64 }
        fixed_cmp_int! { $Fix, u128 }
        fixed_cmp_int! { $Fix, usize }
        fixed_cmp_float! { $Fix($Inner), f16, u16 }
        fixed_cmp_float! { $Fix($Inner), bf16, u16 }
        fixed_cmp_float! { $Fix($Inner), f32, u32 }
        fixed_cmp_float! { $Fix($Inner), f64, u64 }
        fixed_cmp_float! { $Fix($Inner), F128, u128 }
        fixed_cmp_float! { $Fix($Inner), F128Bits, u128 }
    };
}

fixed_cmp_all! { FixedI8(LeEqU8, i8) }
fixed_cmp_all! { FixedI16(LeEqU16, i16) }
fixed_cmp_all! { FixedI32(LeEqU32, i32) }
fixed_cmp_all! { FixedI64(LeEqU64, i64) }
fixed_cmp_all! { FixedI128(LeEqU128, i128) }
fixed_cmp_all! { FixedU8(LeEqU8, u8) }
fixed_cmp_all! { FixedU16(LeEqU16, u16) }
fixed_cmp_all! { FixedU32(LeEqU32, u32) }
fixed_cmp_all! { FixedU64(LeEqU64, u64) }
fixed_cmp_all! { FixedU128(LeEqU128, u128) }

#[cfg(test)]
mod tests {
    use crate::*;
    use core::{cmp::Ordering, f32};

    #[test]
    fn cmp_signed() {
        use core::cmp::Ordering::*;
        let neg1_16 = FixedI32::<types::extra::U16>::NEG_ONE;
        let neg1_20 = FixedI32::<types::extra::U20>::NEG_ONE;
        let mut a = neg1_16;
        let mut b = neg1_20;
        // a = ffff.0000 = -1, b = fff.00000 = -1
        assert!(a.eq(&b) && b.eq(&a));
        assert_eq!(a.partial_cmp(&b), Some(Equal));
        assert_eq!(b.partial_cmp(&a), Some(Equal));
        assert_eq!(a, -1i8);
        assert_eq!(b, -1i128);
        a >>= 16;
        b >>= 16;
        // a = ffff.ffff = -2^-16, b = fff.ffff0 = -2^-16
        assert!(a.eq(&b) && b.eq(&a));
        assert_eq!(a.partial_cmp(&b), Some(Equal));
        assert_eq!(b.partial_cmp(&a), Some(Equal));
        assert!(a < 0.0);
        assert_eq!(a.partial_cmp(&f32::INFINITY), Some(Less));
        assert!(a < f32::INFINITY);
        assert!(a != f32::INFINITY);
        assert_eq!(a.partial_cmp(&f32::NEG_INFINITY), Some(Greater));
        assert!(a > f32::NEG_INFINITY);
        assert_eq!(a, -(-16f32).exp2());
        assert!(a <= -(-16f32).exp2());
        assert!(a >= -(-16f32).exp2());
        assert!(a < (-16f32).exp2());
        assert_ne!(a, -0.75 * (-16f32).exp2());
        assert!(a < -0.75 * (-16f32).exp2());
        assert!(a <= -0.75 * (-16f32).exp2());
        assert!(a > -1.25 * (-16f32).exp2());
        assert!(a >= -1.25 * (-16f32).exp2());
        a >>= 1;
        b >>= 1;
        // a = ffff.ffff = -2^-16, b = fff.ffff8 = -2^-17
        assert!(a.ne(&b) && b.ne(&a));
        assert_eq!(a.partial_cmp(&b), Some(Less));
        assert_eq!(b.partial_cmp(&a), Some(Greater));
        a = neg1_16 << 11;
        b = neg1_20 << 11;
        // a = f800.0000 = -2^11, b = 800.00000 = -2^11
        assert!(a.eq(&b) && b.eq(&a));
        assert_eq!(a.partial_cmp(&b), Some(Equal));
        assert_eq!(b.partial_cmp(&a), Some(Equal));
        assert_eq!(a, -1i16 << 11);
        assert_eq!(b, -1i64 << 11);
        a <<= 1;
        b <<= 1;
        // a = f000.0000 = -2^-12, b = 000.00000 = 0
        assert!(a.ne(&b) && b.ne(&a));
        assert_eq!(a.partial_cmp(&b), Some(Less));
        assert_eq!(b.partial_cmp(&a), Some(Greater));
        assert!(a < 1u8);
        assert_eq!(b, 0);
    }

    #[test]
    fn cmp_unsigned() {
        use core::cmp::Ordering::*;
        let one_16 = FixedU32::<types::extra::U16>::ONE;
        let one_20 = FixedU32::<types::extra::U20>::ONE;
        let mut a = one_16;
        let mut b = one_20;
        // a = 0001.0000 = 1, b = 001.00000 = 1
        assert!(a.eq(&b) && b.eq(&a));
        assert_eq!(a.partial_cmp(&b), Some(Equal));
        assert_eq!(b.partial_cmp(&a), Some(Equal));
        assert_eq!(a, 1u8);
        assert_eq!(b, 1i128);
        a >>= 16;
        b >>= 16;
        // a = 0000.0001 = 2^-16, b = 000.00010 = 2^-16
        assert!(a.eq(&b) && b.eq(&a));
        assert_eq!(a.partial_cmp(&b), Some(Equal));
        assert_eq!(b.partial_cmp(&a), Some(Equal));
        assert!(a > 0.0);
        assert_eq!(a.partial_cmp(&f32::INFINITY), Some(Less));
        assert!(a < f32::INFINITY);
        assert!(a != f32::INFINITY);
        assert_eq!(a.partial_cmp(&f32::NEG_INFINITY), Some(Greater));
        assert!(a > f32::NEG_INFINITY);
        assert_eq!(a, (-16f64).exp2());
        assert!(a <= (-16f64).exp2());
        assert!(a >= (-16f64).exp2());
        assert!(a > -(-16f64).exp2());
        assert_ne!(a, 0.75 * (-16f64).exp2());
        assert!(a > 0.75 * (-16f64).exp2());
        assert!(a >= 0.75 * (-16f64).exp2());
        assert!(a < 1.25 * (-16f64).exp2());
        assert!(a <= 1.25 * (-16f64).exp2());
        a >>= 1;
        b >>= 1;
        // a = 0000.0000 = 0, b = 000.00008 = 2^-17
        assert!(a.ne(&b) && b.ne(&a));
        assert_eq!(a.partial_cmp(&b), Some(Less));
        assert_eq!(b.partial_cmp(&a), Some(Greater));
        a = one_16 << 11;
        b = one_20 << 11;
        // a = 0800.0000 = 2^11, b = 800.00000 = 2^11
        assert!(a.eq(&b) && b.eq(&a));
        assert_eq!(a.partial_cmp(&b), Some(Equal));
        assert_eq!(b.partial_cmp(&a), Some(Equal));
        assert_eq!(a, 1i16 << 11);
        assert_eq!(b, 1u64 << 11);
        a <<= 1;
        b <<= 1;
        // a = 1000.0000 = 2^12, b = 000.00000 = 0
        assert!(a.ne(&b) && b.ne(&a));
        assert_eq!(a.partial_cmp(&b), Some(Greater));
        assert_eq!(b.partial_cmp(&a), Some(Less));
        assert!(a > -1i8);
        assert_eq!(a, 1i32 << 12);
        assert_eq!(b, 0);
    }

    #[test]
    fn cmp_i0() {
        use crate::types::*;
        assert_eq!(I0F32::checked_from_num(0.5), None);
        for &float in &[
            -0.5,
            -0.5 + f32::EPSILON,
            -0.25,
            -f32::EPSILON,
            0.0,
            f32::EPSILON,
            0.25,
            0.5 - f32::EPSILON,
        ] {
            let fixed = I0F32::from_num(float);
            let half = U0F32::from_num(0.5);
            assert_eq!(fixed < half, float < 0.5, "{fixed} < {half}");
            assert_eq!(fixed <= half, float <= 0.5, "{fixed} <= {half}");
            assert_eq!(fixed == half, float == 0.5, "{fixed} == {half}");
            assert_eq!(fixed >= half, float >= 0.5, "{fixed} >= {half}");
            assert_eq!(fixed > half, float > 0.5, "{fixed} > {half}");
            assert_eq!(
                fixed.partial_cmp(&half),
                float.partial_cmp(&0.5),
                "{fixed}.partial_cmp(&{half})"
            );
            assert_eq!(half < fixed, fixed > half);
            assert_eq!(half <= fixed, fixed >= half);
            assert_eq!(half == fixed, fixed == half);
            assert_eq!(half >= fixed, fixed <= half);
            assert_eq!(half > fixed, fixed < half);
            assert_eq!(
                half.partial_cmp(&fixed),
                fixed.partial_cmp(&half).map(Ordering::reverse)
            );

            let half = I1F31::from_num(0.5);
            assert_eq!(fixed < half, float < 0.5, "{fixed} < {half}");
            assert_eq!(fixed <= half, float <= 0.5, "{fixed} <= {half}");
            assert_eq!(fixed == half, float == 0.5, "{fixed} == {half}");
            assert_eq!(fixed >= half, float >= 0.5, "{fixed} >= {half}");
            assert_eq!(fixed > half, float > 0.5, "{fixed} > {half}");
            assert_eq!(
                fixed.partial_cmp(&half),
                float.partial_cmp(&0.5),
                "{fixed}.partial_cmp(&{half})"
            );
            assert_eq!(half < fixed, fixed > half);
            assert_eq!(half <= fixed, fixed >= half);
            assert_eq!(half == fixed, fixed == half);
            assert_eq!(half >= fixed, fixed <= half);
            assert_eq!(half > fixed, fixed < half);
            assert_eq!(
                half.partial_cmp(&fixed),
                fixed.partial_cmp(&half).map(Ordering::reverse)
            );

            let half = 0.5f32;
            assert_eq!(fixed < half, float < 0.5, "{fixed} < {half}");
            assert_eq!(fixed <= half, float <= 0.5, "{fixed} <= {half}");
            assert_eq!(fixed == half, float == 0.5, "{fixed} == {half}");
            assert_eq!(fixed >= half, float >= 0.5, "{fixed} >= {half}");
            assert_eq!(fixed > half, float > 0.5, "{fixed} > {half}");
            assert_eq!(
                fixed.partial_cmp(&half),
                float.partial_cmp(&0.5),
                "{fixed}.partial_cmp(&{half})"
            );
            assert_eq!(half < fixed, fixed > half);
            assert_eq!(half <= fixed, fixed >= half);
            assert_eq!(half == fixed, fixed == half);
            assert_eq!(half >= fixed, fixed <= half);
            assert_eq!(half > fixed, fixed < half);
            assert_eq!(
                half.partial_cmp(&fixed),
                fixed.partial_cmp(&half).map(Ordering::reverse)
            );

            let m1 = I32F0::from_num(-1.0);
            assert_eq!(fixed < m1, float < -1.0, "{fixed} < {m1}");
            assert_eq!(fixed <= m1, float <= -1.0, "{fixed} <= {m1}");
            assert_eq!(fixed == m1, float == -1.0, "{fixed} == {m1}");
            assert_eq!(fixed >= m1, float >= -1.0, "{fixed} >= {m1}");
            assert_eq!(fixed > m1, float > -1.0, "{fixed} > {m1}");
            assert_eq!(
                fixed.partial_cmp(&m1),
                float.partial_cmp(&-1.0),
                "{fixed}.partial_cmp(&{m1})"
            );
            assert_eq!(m1 < fixed, fixed > m1);
            assert_eq!(m1 <= fixed, fixed >= m1);
            assert_eq!(m1 == fixed, fixed == m1);
            assert_eq!(m1 >= fixed, fixed <= m1);
            assert_eq!(m1 > fixed, fixed < m1);
            assert_eq!(
                m1.partial_cmp(&fixed),
                fixed.partial_cmp(&m1).map(Ordering::reverse)
            );

            let m1 = I1F31::from_num(-1.0);
            assert_eq!(fixed < m1, float < -1.0, "{fixed} < {m1}");
            assert_eq!(fixed <= m1, float <= -1.0, "{fixed} <= {m1}");
            assert_eq!(fixed == m1, float == -1.0, "{fixed} == {m1}");
            assert_eq!(fixed >= m1, float >= -1.0, "{fixed} >= {m1}");
            assert_eq!(fixed > m1, float > -1.0, "{fixed} > {m1}");
            assert_eq!(
                fixed.partial_cmp(&m1),
                float.partial_cmp(&-1.0),
                "{fixed}.partial_cmp(&{m1})"
            );
            assert_eq!(m1 < fixed, fixed > m1);
            assert_eq!(m1 <= fixed, fixed >= m1);
            assert_eq!(m1 == fixed, fixed == m1);
            assert_eq!(m1 >= fixed, fixed <= m1);
            assert_eq!(m1 > fixed, fixed < m1);
            assert_eq!(
                m1.partial_cmp(&fixed),
                fixed.partial_cmp(&m1).map(Ordering::reverse)
            );

            let m1 = -1.0f32;
            assert_eq!(fixed < m1, float < -1.0, "{fixed} < {m1}");
            assert_eq!(fixed <= m1, float <= -1.0, "{fixed} <= {m1}");
            assert_eq!(fixed == m1, float == -1.0, "{fixed} == {m1}");
            assert_eq!(fixed >= m1, float >= -1.0, "{fixed} >= {m1}");
            assert_eq!(fixed > m1, float > -1.0, "{fixed} > {m1}");
            assert_eq!(
                fixed.partial_cmp(&m1),
                float.partial_cmp(&-1.0),
                "{fixed}.partial_cmp(&{m1})"
            );
            assert_eq!(m1 < fixed, fixed > m1);
            assert_eq!(m1 <= fixed, fixed >= m1);
            assert_eq!(m1 == fixed, fixed == m1);
            assert_eq!(m1 >= fixed, fixed <= m1);
            assert_eq!(m1 > fixed, fixed < m1);
            assert_eq!(
                m1.partial_cmp(&fixed),
                fixed.partial_cmp(&m1).map(Ordering::reverse)
            );

            let mhalf = I1F31::from_num(-0.5);
            assert_eq!(fixed < mhalf, float < -0.5, "{fixed} < {mhalf}");
            assert_eq!(fixed <= mhalf, float <= -0.5, "{fixed} <= {mhalf}");
            assert_eq!(fixed == mhalf, float == -0.5, "{fixed} == {mhalf}");
            assert_eq!(fixed >= mhalf, float >= -0.5, "{fixed} >= {mhalf}");
            assert_eq!(fixed > mhalf, float > -0.5, "{fixed} > {mhalf}");
            assert_eq!(
                fixed.partial_cmp(&mhalf),
                float.partial_cmp(&-0.5),
                "{fixed}.partial_cmp(&{mhalf})"
            );
            assert_eq!(mhalf < fixed, fixed > mhalf);
            assert_eq!(mhalf <= fixed, fixed >= mhalf);
            assert_eq!(mhalf == fixed, fixed == mhalf);
            assert_eq!(mhalf >= fixed, fixed <= mhalf);
            assert_eq!(mhalf > fixed, fixed < mhalf);
            assert_eq!(
                mhalf.partial_cmp(&fixed),
                fixed.partial_cmp(&mhalf).map(Ordering::reverse)
            );

            let mhalf = -0.5f32;
            assert_eq!(fixed < mhalf, float < -0.5, "{fixed} < {mhalf}");
            assert_eq!(fixed <= mhalf, float <= -0.5, "{fixed} <= {mhalf}");
            assert_eq!(fixed == mhalf, float == -0.5, "{fixed} == {mhalf}");
            assert_eq!(fixed >= mhalf, float >= -0.5, "{fixed} >= {mhalf}");
            assert_eq!(fixed > mhalf, float > -0.5, "{fixed} > {mhalf}");
            assert_eq!(
                fixed.partial_cmp(&mhalf),
                float.partial_cmp(&-0.5),
                "{fixed}.partial_cmp(&{mhalf})"
            );
            assert_eq!(mhalf < fixed, fixed > mhalf);
            assert_eq!(mhalf <= fixed, fixed >= mhalf);
            assert_eq!(mhalf == fixed, fixed == mhalf);
            assert_eq!(mhalf >= fixed, fixed <= mhalf);
            assert_eq!(mhalf > fixed, fixed < mhalf);
            assert_eq!(
                mhalf.partial_cmp(&fixed),
                fixed.partial_cmp(&mhalf).map(Ordering::reverse)
            );
        }
    }
}
