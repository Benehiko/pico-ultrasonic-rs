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

macro_rules! fixed_from_to {
    (
        $Fixed:ident[$s_fixed:expr](
            $Inner:ident[$s_inner:expr], $LeEqU:tt, $s_nbits:expr
        ),
        $Signedness:tt
    ) => {
        comment! {
            r#"Creates a fixed-point number from another number.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds
    to the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[to\_fixed][ToFixed::to_fixed]\()</code>.

# Panics

For floating-point numbers, panics if the value is not [finite].

When debug assertions are enabled, panics if the value does not fit.
When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future
it panics; if wrapping is required use [`wrapping_from_num`] instead.

# Examples

```rust
use fixed::{types::extra::U4, types::I16F16, "#, $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::from_num(src), Fix::from_bits(0b111 << (4 - 2)));

assert_eq!(Fix::from_num(3i32), Fix::from_bits(3 << 4));
assert_eq!(Fix::from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-3i64), Fix::from_bits(-",
                "3i64), Fix::from_bits(",
            ),
            "3 << 4));

assert_eq!(Fix::from_num(1.75f32), Fix::from_bits(0b111 << (4 - 2)));
assert_eq!(Fix::from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-1.75f64), Fix::from_bits(-",
                "1.75f64), Fix::from_bits(",
            ),
            "0b111 << (4-2)));
```

[`bf16`]: half::bf16
[`f16`]: half::f16
[`wrapping_from_num`]: Self::wrapping_from_num
[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn from_num<Src: ToFixed>(src: Src) -> $Fixed<Frac> {
                src.to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[from\_fixed][FromFixed::from_fixed]\(self)</code>.

# Panics

When debug assertions are enabled, panics if the value does not fit.
When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future
it panics; if wrapping is required use [`wrapping_to_num`] instead.

# Examples

```rust
use fixed::{types::extra::U4, types::I30F2, "#, $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(src.to_num::<I30F2>(), I30F2::from_bits(0b111));
// src >> 2 is 0.0111, which for I30F2 is truncated to 0.01
assert_eq!((src >> 2u32).to_num::<I30F2>(), I30F2::from_bits(0b1));

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.to_num::<i32>(), 2);
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                "(-two_point_5).to_num::<i64>(), -3",
                "two_point_5.to_num::<i64>(), 2",
            ),
            ");

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.to_num::<f32>(), 1.625f32);
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                "(-one_point_625).to_num::<f64>(), -",
                "one_point_625.to_num::<f64>(), "
            ),
            "1.625f64);
```

[`bf16`]: half::bf16
[`f16`]: half::f16
[`wrapping_to_num`]: Self::wrapping_to_num
";
            #[inline]
            #[track_caller]
            #[must_use]
            pub fn to_num<Dst: FromFixed>(self) -> Dst {
                Dst::from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number if it
fits, otherwise returns [`None`].

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[checked\_to\_fixed][ToFixed::checked_to_fixed]\()</code>.

# Examples

```rust
use fixed::{
    types::extra::{U2, U4},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::checked_from_num(src), Some(Fix::from_bits(0b111 << (4 - 2))));
let too_large = ", $s_fixed, "::<U2>::MAX;
assert!(Fix::checked_from_num(too_large).is_none());

assert_eq!(Fix::checked_from_num(3), Some(Fix::from_bits(3 << 4)));
let too_large = ", $s_inner, "::MAX;
assert!(Fix::checked_from_num(too_large).is_none());
let too_small = ",
            if_signed_unsigned!(
                $Signedness,
                concat!($s_inner, "::MIN"),
                "-1",
            ),
            ";
assert!(Fix::checked_from_num(too_small).is_none());

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::checked_from_num(1.75f32), Some(expected));
assert_eq!(Fix::checked_from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-1.75f64), Some(-",
                "1.75f64), Some(",
            ),
            "expected));
assert!(Fix::checked_from_num(2e38).is_none());
assert!(Fix::checked_from_num(std::f64::NAN).is_none());
```

[`bf16`]: half::bf16
[`f16`]: half::f16
";
            #[inline]
            pub fn checked_from_num<Src: ToFixed>(src: Src) -> Option<$Fixed<Frac>> {
                src.checked_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number if it
fits, otherwise returns [`None`].

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[checked\_from\_fixed][FromFixed::checked_from_fixed]\(self)</code>.

# Examples

```rust
use fixed::{
    types::extra::{U0, U4, U6},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.checked_to_num::<I16F16>(), Some(expected));
type TooFewIntBits = ", $s_fixed, "<U6>;
assert!(Fix::MAX.checked_to_num::<TooFewIntBits>().is_none());

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.checked_to_num::<i32>(), Some(2));
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                "(-two_point_5).checked_to_num::<i64>(), Some(-3",
                "two_point_5.checked_to_num::<i64>(), Some(2",
            ),
            "));
type AllInt = ", $s_fixed, "<U0>;
assert!(AllInt::",
            if_signed_unsigned!(
                $Signedness,
                "from_bits(-1).checked_to_num::<u",
                "MAX.checked_to_num::<i",
            ),
            $s_nbits, ">().is_none());

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.checked_to_num::<f32>(), Some(1.625f32));
```

[`bf16`]: half::bf16
[`f16`]: half::f16
";
            #[inline]
            #[must_use]
            pub fn checked_to_num<Dst: FromFixed>(self) -> Option<Dst> {
                Dst::checked_from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number,
saturating if it does not fit.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[saturating\_to\_fixed][ToFixed::saturating_to_fixed]\()</code>.

# Panics

This method panics if the value is a floating-point [NaN].

# Examples

```rust
use fixed::{
    types::extra::{U2, U4},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::saturating_from_num(src), Fix::from_bits(0b111 << (4 - 2)));
let too_large = ", $s_fixed, "::<U2>::MAX;
assert_eq!(Fix::saturating_from_num(too_large), Fix::MAX);

assert_eq!(Fix::saturating_from_num(3), Fix::from_bits(3 << 4));
let too_small = ",
            if_signed_unsigned!(
                $Signedness,
                concat!($s_inner, "::MIN"),
                "-1",
            ),
            ";
assert_eq!(Fix::saturating_from_num(too_small), Fix::MIN);

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::saturating_from_num(1.75f32), expected);
assert_eq!(Fix::saturating_from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-1.75f64), -",
                "1.75f64), ",
            ),
            "expected);
assert_eq!(Fix::saturating_from_num(2e38), Fix::MAX);
assert_eq!(Fix::saturating_from_num(std::f64::NEG_INFINITY), Fix::MIN);
```

[NaN]: f64::is_nan
[`bf16`]: half::bf16
[`f16`]: half::f16
";
            #[inline]
            #[track_caller]
            pub fn saturating_from_num<Src: ToFixed>(src: Src) -> $Fixed<Frac> {
                src.saturating_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number,
saturating the value if it does not fit.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[saturating\_from\_fixed][FromFixed::saturating_from_fixed]\(self)</code>.

# Examples

```rust
use fixed::{
    types::extra::{U0, U4, U6},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.saturating_to_num::<I16F16>(), expected);
type TooFewIntBits = ", $s_fixed, "<U6>;
let saturated = Fix::MAX.saturating_to_num::<TooFewIntBits>();
assert_eq!(saturated, TooFewIntBits::MAX);

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.saturating_to_num::<i32>(), 2);
type AllInt = ", $s_fixed, "<U0>;
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                concat!("AllInt::from_bits(-1).saturating_to_num::<u", $s_nbits, ">(), 0"),
                concat!(
                    "AllInt::MAX.saturating_to_num::<i", $s_nbits, ">(), ",
                    "i", $s_nbits, "::MAX",
                ),
            ),
            ");

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.saturating_to_num::<f32>(), 1.625f32);
```

[`bf16`]: half::bf16
[`f16`]: half::f16
";
            #[inline]
            #[must_use]
            pub fn saturating_to_num<Dst: FromFixed>(self) -> Dst {
                Dst::saturating_from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number,
wrapping the value on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[wrapping\_to\_fixed][ToFixed::wrapping_to_fixed]\()</code>.

# Panics

For floating-point numbers, panics if the value is not [finite].

# Examples

```rust
use fixed::{
    types::extra::{U0, U4},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::wrapping_from_num(src), Fix::from_bits(0b111 << (4 - 2)));
// integer 0b1101 << (", $s_nbits, " - 7) will wrap to fixed-point 1010...
let too_large = ", $s_fixed, "::<U0>::from_bits(0b1101 << (", $s_nbits, " - 7));
let wrapped = Fix::from_bits(0b1010 << (", $s_nbits, " - 4));
assert_eq!(Fix::wrapping_from_num(too_large), wrapped);

// integer 0b1101 << (", $s_nbits, " - 7) will wrap to fixed-point 1010...
let large: ", $s_inner, " = 0b1101 << (", $s_nbits, " - 7);
let wrapped = Fix::from_bits(0b1010 << (", $s_nbits, " - 4));
assert_eq!(Fix::wrapping_from_num(large), wrapped);

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::wrapping_from_num(1.75f32), expected);
// 1.75 << (", $s_nbits, " - 4) wraps to binary 11000...
let large = 1.75 * 2f32.powi(", $s_nbits, " - 4);
let wrapped = Fix::from_bits(0b1100 << (", $s_nbits, " - 4));
assert_eq!(Fix::wrapping_from_num(large), wrapped);
```

[`bf16`]: half::bf16
[`f16`]: half::f16
[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn wrapping_from_num<Src: ToFixed>(src: Src) -> $Fixed<Frac> {
                src.wrapping_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number,
wrapping the value on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[wrapping\_from\_fixed][FromFixed::wrapping_from_fixed]\(self)</code>.

# Examples

```rust
use fixed::{
    types::extra::{U0, U4, U6},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.wrapping_to_num::<I16F16>(), expected);
type TooFewIntBits = ", $s_fixed, "<U6>;
let wrapped = TooFewIntBits::from_bits(Fix::MAX.to_bits() << 2);
assert_eq!(Fix::MAX.wrapping_to_num::<TooFewIntBits>(), wrapped);

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.wrapping_to_num::<i32>(), 2);
type AllInt = ", $s_fixed, "<U0>;
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                concat!(
                    "AllInt::from_bits(-1).wrapping_to_num::<u", $s_nbits, ">(), ",
                    "u", $s_nbits, "::MAX",
                ),
                concat!("AllInt::MAX.wrapping_to_num::<i", $s_nbits, ">(), -1"),
            ),
            ");

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.wrapping_to_num::<f32>(), 1.625f32);
```

[`bf16`]: half::bf16
[`f16`]: half::f16
";
            #[inline]
            #[must_use]
            pub fn wrapping_to_num<Dst: FromFixed>(self) -> Dst {
                Dst::wrapping_from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number,
panicking on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[unwrapped\_to\_fixed][ToFixed::unwrapped_to_fixed]\()</code>.

# Panics

Panics if the value does not fit.

For floating-point numbers, also panics if the value is not [finite].

# Examples

```rust
use fixed::{
    types::{extra::U4, I16F16},
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::unwrapped_from_num(src), Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of overflow.

```should_panic
use fixed::{
    types::extra::{U0, U4},
    ", $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;
let too_large = ", $s_fixed, "::<U0>::from_bits(0b1101 << (", $s_nbits, " - 7));
let _overflow = Fix::unwrapped_from_num(too_large);
```

[`bf16`]: half::bf16
[`f16`]: half::f16
[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn unwrapped_from_num<Src: ToFixed>(src: Src) -> $Fixed<Frac> {
                match src.overflowing_to_fixed() {
                    (_, true) => panic!("overflow"),
                    (ans, false) => ans,
                }
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number,
panicking on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[unwrapped\_from\_fixed][FromFixed::unwrapped_from_fixed]\(self)</code>.

# Panics

Panics if the value does not fit.

# Examples

```rust
use fixed::{
    types::{extra::U4, I16F16},
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.unwrapped_to_num::<I16F16>(), expected);
```

The following panics because of overflow.

```should_panic
use fixed::{
    types::extra::{U4, U6},
    ", $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;
type TooFewIntBits = ", $s_fixed, "<U6>;
let _overflow = Fix::MAX.unwrapped_to_num::<TooFewIntBits>();
```

[`bf16`]: half::bf16
[`f16`]: half::f16
";
            #[inline]
            #[track_caller]
            #[must_use]
            pub fn unwrapped_to_num<Dst: FromFixed>(self) -> Dst {
                match Dst::overflowing_from_fixed(self) {
                    (_, true) => panic!("overflow"),
                    (ans, false) => ans,
                }
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[overflowing\_to\_fixed][ToFixed::overflowing_to_fixed]\()</code>.

# Panics

For floating-point numbers, panics if the value is not [finite].

# Examples

```rust
use fixed::{
    types::extra::{U0, U4},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::overflowing_from_num(src), (expected, false));
// integer 0b1101 << (", $s_nbits, " - 7) will wrap to fixed-point 1010...
let too_large = ", $s_fixed, "::<U0>::from_bits(0b1101 << (", $s_nbits, " - 7));
let wrapped = Fix::from_bits(0b1010 << (", $s_nbits, " - 4));
assert_eq!(Fix::overflowing_from_num(too_large), (wrapped, true));

assert_eq!(Fix::overflowing_from_num(3), (Fix::from_bits(3 << 4), false));
// integer 0b1101 << (", $s_nbits, " - 7) will wrap to fixed-point 1010...
let large: ", $s_inner, " = 0b1101 << (", $s_nbits, " - 7);
let wrapped = Fix::from_bits(0b1010 << (", $s_nbits, " - 4));
assert_eq!(Fix::overflowing_from_num(large), (wrapped, true));

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::overflowing_from_num(1.75f32), (expected, false));
// 1.75 << (", $s_nbits, " - 4) wraps to binary 11000...
let large = 1.75 * 2f32.powi(", $s_nbits, " - 4);
let wrapped = Fix::from_bits(0b1100 << (", $s_nbits, " - 4));
assert_eq!(Fix::overflowing_from_num(large), (wrapped, true));
```

[`bf16`]: half::bf16
[`f16`]: half::f16
[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn overflowing_from_num<Src: ToFixed>(src: Src) -> ($Fixed<Frac>, bool) {
                src.overflowing_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number.

Returns a [tuple] of the number and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`bf16`], [`f32`],
    [`f64`] or [`F128`]. For this conversion, the method rounds to
    the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[overflowing\_from\_fixed][FromFixed::overflowing_from_fixed]\(self)</code>.

# Examples

```rust
use fixed::{
    types::extra::{U0, U4, U6},
    types::I16F16,
    "#, $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.overflowing_to_num::<I16F16>(), (expected, false));
type TooFewIntBits = ", $s_fixed, "<U6>;
let wrapped = TooFewIntBits::from_bits(Fix::MAX.to_bits() << 2);
assert_eq!(Fix::MAX.overflowing_to_num::<TooFewIntBits>(), (wrapped, true));

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.overflowing_to_num::<i32>(), (2, false));
let does_not_fit = ", $s_fixed, "::<U0>::",
            if_signed_unsigned!($Signedness, "from_bits(-1)", "MAX"),
            ";
let wrapped = ",
            if_signed_unsigned!(
                $Signedness,
                concat!("1u", $s_nbits, ".wrapping_neg()"),
                concat!("-1i", $s_nbits),
            ),
            ";
assert_eq!(does_not_fit.overflowing_to_num::<",
            if_signed_unsigned!($Signedness, "u", "i"),
            $s_nbits, ">(), (wrapped, true));

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.overflowing_to_num::<f32>(), (1.625f32, false));
```

[`bf16`]: half::bf16
[`f16`]: half::f16
";
            #[inline]
            #[must_use]
            pub fn overflowing_to_num<Dst: FromFixed>(self) -> (Dst, bool) {
                Dst::overflowing_from_fixed(self)
            }
        }

        /// Creates a fixed-point number from a fixed-point number with the same
        /// underlying integer type. Usable in constant context.
        ///
        /// This is equivalent to the [`unwrapped_from_num`] method with
        #[doc = concat!("<code>[", $s_fixed, "]&lt;SrcFrac></code>")]
        /// as its generic parameter, but can also be used in constant context.
        /// Unless required in constant context, use [`unwrapped_from_num`] or
        /// [`from_num`] instead.
        ///
        /// # Planned deprecation
        ///
        /// This method will be deprecated when the [`unwrapped_from_num`]
        /// method is usable in constant context.
        ///
        /// # Panics
        ///
        /// Panics if the value does not fit.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use fixed::types::extra::{U2, U4};
        #[doc = concat!("use fixed::", $s_fixed, ";")]
        #[doc = concat!("type FixA = ", $s_fixed, "<U2>;")]
        #[doc = concat!("type FixB = ", $s_fixed, "<U4>;")]
        /// const A: FixA = FixA::unwrapped_from_str("3.5");
        /// const B: FixB = FixB::const_from_fixed(A);
        /// assert_eq!(B, 3.5);
        /// ```
        ///
        /// The following would fail to compile because of overflow.
        ///
        /// ```rust,compile_fail
        /// use fixed::types::extra::{U2, U4};
        #[doc = concat!("use fixed::", $s_fixed, ";")]
        #[doc = concat!("const _OVERFLOW: ", $s_fixed, "<U4> = ", $s_fixed, "::const_from_fixed(", $s_fixed, "::<U2>::MAX);")]
        /// ```
        ///
        /// [`from_num`]: Self::from_num
        /// [`unwrapped_from_num`]: Self::unwrapped_from_num
        #[inline]
        #[track_caller]
        #[must_use]
        pub const fn const_from_fixed<SrcFrac: $LeEqU>(src: $Fixed<SrcFrac>) -> $Fixed<Frac> {
            let shift_left = Frac::I32 - SrcFrac::I32;
            let nbits = $Inner::BITS as i32;
            let src_bits = src.to_bits();
            let bits = if shift_left <= -nbits {
                src_bits >> (nbits / 2) >> (nbits / 2)
            } else if shift_left <= 0 {
                src_bits >> -shift_left
            } else if shift_left >= nbits {
                if src_bits != 0 {
                    panic!("overflow");
                }
                0
            } else {
                let shifted = src_bits << shift_left;
                if (shifted >> shift_left) != src_bits {
                    panic!("overflow");
                }
                shifted
            };
            Self::from_bits(bits)
        }

        /// Creates a fixed-point number from the underlying integer type
        #[doc = concat!("[`", $s_inner, "`].")]
        /// Usable in constant context.
        ///
        /// This is equivalent to the [`unwrapped_from_num`] method with
        #[doc = concat!("[`", $s_inner, "`]")]
        /// as its generic parameter, but can also be used in constant context.
        /// Unless required in constant context, use [`unwrapped_from_num`] or
        /// [`from_num`] instead.
        ///
        /// # Planned deprecation
        ///
        /// This method will be deprecated when the [`unwrapped_from_num`]
        /// method is usable in constant context.
        ///
        /// # Panics
        ///
        /// Panics if the value does not fit.
        ///
        /// # Examples
        ///
        /// ```rust
        #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, "};")]
        #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
        /// const FIVE: Fix = Fix::const_from_int(5);
        /// assert_eq!(FIVE, 5);
        /// ```
        ///
        /// The following would fail to compile because of overflow.
        ///
        /// ```rust,compile_fail
        #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, "};")]
        #[doc = concat!("const _OVERFLOW: ", $s_fixed, "<U4> = ", $s_fixed, "::const_from_int(", $s_inner, "::MAX);")]
        /// ```
        ///
        /// [`from_num`]: Self::from_num
        /// [`unwrapped_from_num`]: Self::unwrapped_from_num
        #[inline]
        #[track_caller]
        #[must_use]
        pub const fn const_from_int(src: $Inner) -> $Fixed<Frac> {
            Self::const_from_fixed($Fixed::<U0>::from_bits(src))
        }

        comment! {
            "Parses a fixed-point literal.

Rounding is to the nearest, with ties rounded to even.

This is similar to [`from_str`][Self::from_str] but accepts a prefix for setting
the radix, and ignores underscores, such that the parsing is more similar to
numeric literals in Rust code.

",
            if_signed_else_empty_str! {
                $Signedness;
                "The string can start with “`-`” for a negative number.

",
            },
            r#"Strings starting with “`0b`” are parsed as binary, strings
starting with “`0o`” are parsed as octal, and strings starting with “`0x`” are
parsed as hexadecimal.

Exponents are supported as well. For decimal, binary and octal numbers, the
separator “`e`” or “`E`” can be used to start an exponent, which is then
followed by an optional sign “`+`” or “`-`”, and then by a decimal integer which
is the exponent. For hexadecimal numbers, since “`e`” and “`E`” are hexadecimal
digits, the separator “`@`” has to be used instead. The separator “`@`” is
accepted for all radices. The parsed value is scaled by the radix to the power
of the exponent.

For binary, octal and hexadecimal, base-2 exponents are supported too, using the
separator “`p`” or “`P`”. The parsed value is scaled by 2 to the power of the
exponent. For example, for hexadecimal “`P8`” means ×2⁸, and is equivalent to
“`@2`” which means ×16².

# Panics

Panics if the number is not valid or overflows.

# Examples

```rust
use fixed::{types::extra::U4, "#, $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;

assert_eq!(Fix::lit("1.75"), 1.75);
assert_eq!(Fix::lit("1_.7_5_"), 1.75);
assert_eq!(Fix::lit("17.5e-1"), 1.75);
assert_eq!(Fix::lit("0_.017_5_e+0_2"), 1.75);
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"assert_eq!(Fix::lit("-01.75"), -1.75);
"#,
            },
            r#"
assert_eq!(Fix::lit("0b_1.1_1"), 1.75);
assert_eq!(Fix::lit("0b_111e-2"), 1.75);

assert_eq!(Fix::lit("0o_1.6"), 1.75);
assert_eq!(Fix::lit("0o_.16E1"), 1.75);

assert_eq!(Fix::lit("0x1.C"), 1.75);
assert_eq!(Fix::lit("0x0.1C@1"), 1.75);
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"assert_eq!(Fix::lit("-0x1.C"), -1.75);
"#,
            },
            r#"```

This method is useful to write constant fixed-point literals.

```rust
use fixed::{types::extra::U4, "#, $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
const ONE_AND_HALF: Fix = Fix::lit("1.5");
assert_eq!(ONE_AND_HALF, 1.5);
```
"#;
            #[inline]
            #[track_caller]
            #[must_use]
            pub const fn lit(src: &str) -> $Fixed<Frac> {
                match from_str::$Inner::lit(src, Self::FRAC_NBITS) {
                    Ok(s) => $Fixed::from_bits(s),
                    Err(e) => panic!("{}", e.lit_message()),
                }
            }
        }

        comment! {
            "Parses a string slice containing decimal digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. The separator “`e`”, “`E`” or “`@`”
can be used to start an exponent, which is then followed by an optional sign
“`+`” or “`-`”, and then by a decimal integer which is the exponent. The parsed
value is scaled by 10 to the power of the exponent.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
assert_eq!(Fix::from_str("1.75"), Ok(Fix::from_num(1.75)));
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"assert_eq!(Fix::from_str("-1.75"), Ok(Fix::from_num(-1.75)));
"#,
            },
            r#"assert_eq!(Fix::from_str("0.00625E+3"), Ok(Fix::from_num(6.25)));
assert_eq!(Fix::from_str("1.25e-1"), Ok(Fix::from_num(0.125)));
```
"#;
            #[inline]
            pub const fn from_str(src: &str) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::from_str_radix(src, 10, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing binary digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. The separator “`e`”, “`E`” or “`@`”
can be used to start an exponent, which is then followed by an optional sign
“`+`” or “`-`”, and then by a decimal integer which is the exponent. The parsed
value is scaled by the radix (2 for binary) to the power of the exponent.

Base-2 exponents are supported too, using the separator “`p`” or “`P`”. The
parsed value is scaled by 2 to the power of the exponent. For binary, since the
radix is 2, base-2 exponents are equivalent to the other form of exponent.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
// 1.11 in binary is 1.75
assert_eq!(Fix::from_str_binary("1.11"), Ok(Fix::from_num(1.75)));
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"assert_eq!(Fix::from_str_binary("-1.11"), Ok(Fix::from_num(-1.75)));
"#,
            },
            r#"
// 111.0101 in binary is 7.3125
assert_eq!(Fix::from_str_binary("1.110101e2"), Ok(Fix::from_num(7.3125)));
assert_eq!(Fix::from_str_binary("11101.01e-2"), Ok(Fix::from_num(7.3125)));
```
"#;
            #[inline]
            pub const fn from_str_binary(src: &str) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::from_str_radix(src, 2, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing octal digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. The separator “`e`”, “`E`” or “`@`”
can be used to start an exponent, which is then followed by an optional sign
“`+`” or “`-`”, and then by a decimal integer which is the exponent. The parsed
value is scaled by 8 to the power of the exponent.

Base-2 exponents are supported too, using the separator “`p`” or “`P`”. The
parsed value is scaled by 2 to the power of the exponent. For example, for octal
“`P6`” means ×2⁶, and is equivalent to “`E2`” which means ×8².

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
// 1.75 is 1.11 in binary, 1.6 in octal
let f = Fix::from_str_octal("1.6");
let check = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(f, Ok(check));
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"let neg = Fix::from_str_octal("-1.6");
assert_eq!(neg, Ok(-check));
"#,
            },
            r#"assert_eq!(Fix::from_str_octal("160e-2"), Ok(check));
```
"#;
            #[inline]
            pub const fn from_str_octal(src: &str) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::from_str_radix(src, 8, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing hexadecimal digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. Since “`e`” and “`E`” are valid
hexadecimal digits, they cannot be used as a separator to start an exponent, so
“`@`” is used instead. This is then followed by an optional sign “`+`” or “`-`”,
and then by a decimal integer which is the exponent. The parsed value is scaled
by 16 to the power of the exponent.

Base-2 exponents are supported too, using the separator “`p`” or “`P`”. The
parsed value is scaled by 2 to the power of the exponent. For example, for
hexadecimal “`P8`” means ×2⁸, and is equivalent to “`E2`” which means ×16².

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
// 1.C in hexadecimal is 1.75
assert_eq!(Fix::from_str_hex("1.C"), Ok(Fix::from_num(1.75)));
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"assert_eq!(Fix::from_str_hex("-1.C"), Ok(Fix::from_num(-1.75)));
"#,
            },
            r#"assert_eq!(Fix::from_str_hex("1C@-1"), Ok(Fix::from_num(1.75)));
assert_eq!(Fix::from_str_hex(".01C@+2"), Ok(Fix::from_num(1.75)));
```
"#;
            #[inline]
            pub const fn from_str_hex(src: &str) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::from_str_radix(src, 16, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing decimal digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str("9999"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str("-9999"), Ok(I8F8::MIN));
"#,
                r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str("9999"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str("-1"), Ok(U8F8::ZERO));
"#,
            ),
            "```
";
            #[inline]
            pub const fn saturating_from_str(src: &str) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::saturating_from_str_radix(src, 10, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing binary digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str_binary("101100111000"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str_binary("-101100111000"), Ok(I8F8::MIN));
"#,
                r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str_binary("101100111000"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str_binary("-1"), Ok(U8F8::ZERO));
"#,
            ),
            "```
";
            #[inline]
            pub const fn saturating_from_str_binary(
                src: &str,
            ) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::saturating_from_str_radix(src, 2, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing octal digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str_octal("7777"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str_octal("-7777"), Ok(I8F8::MIN));
"#,
                r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str_octal("7777"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str_octal("-1"), Ok(U8F8::ZERO));
"#,
            ),
            "```
";
            #[inline]
            pub const fn saturating_from_str_octal(
                src: &str,
            ) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::saturating_from_str_radix(src, 8, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Prases a string slice containing hexadecimal digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str_hex("FFFF"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str_hex("-FFFF"), Ok(I8F8::MIN));
"#,
                r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str_hex("FFFF"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str_hex("-1"), Ok(U8F8::ZERO));
"#,
            ),
            "```
";
            #[inline]
            pub const fn saturating_from_str_hex(
                src: &str,
            ) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::saturating_from_str_radix(src, 16, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing decimal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
// 9999.5 = 15.5 + 256 × n
assert_eq!(I8F8::wrapping_from_str("9999.5"), Ok(I8F8::from_num(15.5)));
assert_eq!(I8F8::wrapping_from_str("-9999.5"), Ok(I8F8::from_num(-15.5)));
"#,
                r#"use fixed::types::U8F8;
// 9999.5 = 15.5 + 256 × n
assert_eq!(U8F8::wrapping_from_str("9999.5"), Ok(U8F8::from_num(15.5)));
assert_eq!(U8F8::wrapping_from_str("-9999.5"), Ok(U8F8::from_num(240.5)));
"#,
            ),
            "```
";
            #[inline]
            pub const fn wrapping_from_str(src: &str) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::wrapping_from_str_radix(src, 10, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing binary digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(I8F8::wrapping_from_str_binary("101100111000.1"), Ok(check));
assert_eq!(I8F8::wrapping_from_str_binary("-101100111000.1"), Ok(-check));
"#,
                r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(U8F8::wrapping_from_str_binary("101100111000.1"), Ok(check));
assert_eq!(U8F8::wrapping_from_str_binary("-101100111000.1"), Ok(check.wrapping_neg()));
"#,
            ),
            "```
";
            #[inline]
            pub const fn wrapping_from_str_binary(
                src: &str,
            ) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::wrapping_from_str_radix(src, 2, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing octal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(I8F8::wrapping_from_str_octal("7165.4"), Ok(check));
assert_eq!(I8F8::wrapping_from_str_octal("-7165.4"), Ok(-check));
"#,
                r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(U8F8::wrapping_from_str_octal("7165.4"), Ok(check));
assert_eq!(U8F8::wrapping_from_str_octal("-7165.4"), Ok(check.wrapping_neg()));
"#,
            ),
            "```
";
            #[inline]
            pub const fn wrapping_from_str_octal(
                src: &str,
            ) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::wrapping_from_str_radix(src, 8, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing hexadecimal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0xFFE);
assert_eq!(I8F8::wrapping_from_str_hex("C0F.FE"), Ok(check));
assert_eq!(I8F8::wrapping_from_str_hex("-C0F.FE"), Ok(-check));
"#,
                r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0xFFE);
assert_eq!(U8F8::wrapping_from_str_hex("C0F.FE"), Ok(check));
assert_eq!(U8F8::wrapping_from_str_hex("-C0F.FE"), Ok(check.wrapping_neg()));
"#,
            ),
            "```
";
            #[inline]
            pub const fn wrapping_from_str_hex(src: &str) -> Result<$Fixed<Frac>, ParseFixedError> {
                match from_str::$Inner::wrapping_from_str_radix(src, 16, Self::FRAC_NBITS) {
                    Ok(bits) => Ok($Fixed::from_bits(bits)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing decimal digits to return a fixed-point number,
panicking on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
// 1.75 is 1.11 in binary
let f = Fix::unwrapped_from_str("1.75");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
use fixed::{types::extra::U4, "#, $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
let _error = Fix::unwrapped_from_str("1.75.");
```
"#;
            #[inline]
            #[track_caller]
            #[must_use]
            pub const fn unwrapped_from_str(src: &str) -> $Fixed<Frac> {
                match $Fixed::from_str(src) {
                    Ok(o) => o,
                    Err(e) => panic!("{}", e.message()),
                }
            }
        }

        comment! {
            "Parses a string slice containing binary digits to return a fixed-point number,
panicking on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
// 1.75 is 1.11 in binary
let f = Fix::unwrapped_from_str_binary("1.11");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
use fixed::{types::extra::U4, "#, $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
let _error = Fix::unwrapped_from_str_binary("1.2");
```
"#;
            #[inline]
            #[track_caller]
            #[must_use]
            pub const fn unwrapped_from_str_binary(src: &str) -> $Fixed<Frac> {
                match $Fixed::from_str_binary(src) {
                    Ok(o) => o,
                    Err(e) => panic!("{}", e.message()),
                }
            }
        }

        comment! {
            "Parses a string slice containing octal digits to return a fixed-point number,
panicking on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
// 1.75 is 1.11 in binary, 1.6 in octal
let f = Fix::unwrapped_from_str_octal("1.6");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
use fixed::{types::extra::U4, "#, $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
let _error = Fix::unwrapped_from_str_octal("1.8");
```
"#;
            #[inline]
            #[track_caller]
            #[must_use]
            pub const fn unwrapped_from_str_octal(src: &str) -> $Fixed<Frac> {
                match $Fixed::from_str_octal(src) {
                    Ok(o) => o,
                    Err(e) => panic!("{}", e.message()),
                }
            }
        }

        comment! {
            "Parses a string slice containing hexadecimal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
// 1.75 is 1.11 in binary, 1.C in hexadecimal
let f = Fix::unwrapped_from_str_hex("1.C");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
use fixed::{types::extra::U4, "#, $s_fixed, "};
type Fix = ", $s_fixed, r#"<U4>;
let _error = Fix::unwrapped_from_str_hex("1.G");
```
"#;
            #[inline]
            #[track_caller]
            #[must_use]
            pub const fn unwrapped_from_str_hex(src: &str) -> $Fixed<Frac> {
                match $Fixed::from_str_hex(src) {
                    Ok(o) => o,
                    Err(e) => panic!("{}", e.message()),
                }
            }
        }

        comment! {
            "Parses a string slice containing decimal digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
assert_eq!(I8F8::overflowing_from_str("99.5"), Ok((I8F8::from_num(99.5), false)));
// 9999.5 = 15.5 + 256 × n
assert_eq!(I8F8::overflowing_from_str("-9999.5"), Ok((I8F8::from_num(-15.5), true)));
"#,
                r#"use fixed::types::U8F8;
assert_eq!(U8F8::overflowing_from_str("99.5"), Ok((U8F8::from_num(99.5), false)));
// 9999.5 = 15.5 + 256 × n
assert_eq!(U8F8::overflowing_from_str("9999.5"), Ok((U8F8::from_num(15.5), true)));
"#,
            ),
            "```
";
            #[inline]
            pub const fn overflowing_from_str(
                src: &str,
            ) -> Result<($Fixed<Frac>, bool), ParseFixedError> {
                match from_str::$Inner::overflowing_from_str_radix(src, 10, Self::FRAC_NBITS) {
                    Ok((bits, overflow)) => Ok(($Fixed::from_bits(bits), overflow)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing binary digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(I8F8::overflowing_from_str_binary("111000.1"), Ok((check, false)));
assert_eq!(I8F8::overflowing_from_str_binary("-101100111000.1"), Ok((-check, true)));
"#,
                r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(U8F8::overflowing_from_str_binary("111000.1"), Ok((check, false)));
assert_eq!(U8F8::overflowing_from_str_binary("101100111000.1"), Ok((check, true)));
"#,
            ),
            "```
";
            #[inline]
            pub const fn overflowing_from_str_binary(
                src: &str,
            ) -> Result<($Fixed<Frac>, bool), ParseFixedError> {
                match from_str::$Inner::overflowing_from_str_radix(src, 2, Self::FRAC_NBITS) {
                    Ok((bits, overflow)) => Ok(($Fixed::from_bits(bits), overflow)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing octal digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(I8F8::overflowing_from_str_octal("165.4"), Ok((check, false)));
assert_eq!(I8F8::overflowing_from_str_octal("-7165.4"), Ok((-check, true)));
"#,
                r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(U8F8::overflowing_from_str_octal("165.4"), Ok((check, false)));
assert_eq!(U8F8::overflowing_from_str_octal("7165.4"), Ok((check, true)));
"#,
            ),
            "```
";
            #[inline]
            pub const fn overflowing_from_str_octal(
                src: &str,
            ) -> Result<($Fixed<Frac>, bool), ParseFixedError> {
                match from_str::$Inner::overflowing_from_str_radix(src, 8, Self::FRAC_NBITS) {
                    Ok((bits, overflow)) => Ok(($Fixed::from_bits(bits), overflow)),
                    Err(e) => Err(e),
                }
            }
        }

        comment! {
            "Parses a string slice containing hexadecimal digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
",
            if_signed_unsigned!(
                $Signedness,
                r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0xFFE);
assert_eq!(I8F8::overflowing_from_str_hex("F.FE"), Ok((check, false)));
assert_eq!(I8F8::overflowing_from_str_hex("-C0F.FE"), Ok((-check, true)));
"#,
                r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0xFFE);
assert_eq!(U8F8::overflowing_from_str_hex("F.FE"), Ok((check, false)));
assert_eq!(U8F8::overflowing_from_str_hex("C0F.FE"), Ok((check, true)));
"#,
            ),
            "```
";
            #[inline]
            pub const fn overflowing_from_str_hex(
                src: &str,
            ) -> Result<($Fixed<Frac>, bool), ParseFixedError> {
                match from_str::$Inner::overflowing_from_str_radix(src, 16, Self::FRAC_NBITS) {
                    Ok((bits, overflow)) => Ok(($Fixed::from_bits(bits), overflow)),
                    Err(e) => Err(e),
                }
            }
        }
    };
}
