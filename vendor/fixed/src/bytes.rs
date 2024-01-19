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

// Kept trimmed: no underscores at beginning or end of slice
#[derive(Clone, Copy, Debug)]
pub struct DigitsUnds<'a> {
    bytes: &'a [u8],
    digits: usize,
}

impl<'a> DigitsUnds<'a> {
    pub const EMPTY: DigitsUnds<'a> = DigitsUnds::new(&[]);

    pub const fn new(bytes: &'a [u8]) -> DigitsUnds<'a> {
        let mut digits = 0;
        let mut leading_unds = 0;
        let mut trailing_unds = 0;
        let mut rem_bytes = bytes;
        while let Some((&byte, rem)) = rem_bytes.split_first() {
            rem_bytes = rem;

            if byte == b'_' {
                trailing_unds += 1;
            } else {
                if digits == 0 {
                    leading_unds = trailing_unds;
                }
                digits += 1;
                trailing_unds = 0;
            }
        }
        let without_trailing_unds = bytes.split_at(bytes.len() - trailing_unds).0;
        let without_leading_unds = without_trailing_unds.split_at(leading_unds).1;
        DigitsUnds {
            bytes: without_leading_unds,
            digits,
        }
    }

    #[inline]
    pub const fn len(self) -> usize {
        self.digits
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.digits == 0
    }

    pub const fn split_at(self, mid: usize) -> (DigitsUnds<'a>, DigitsUnds<'a>) {
        let mut remaining_digits = mid;
        let mut unds = 0;
        let mut rem_bytes = self.bytes;
        while let Some((&byte, rem)) = rem_bytes.split_first() {
            rem_bytes = rem;

            if byte == b'_' {
                unds += 1;
            } else {
                remaining_digits -= 1;
                if remaining_digits == 0 {
                    break;
                }
            }
        }
        if remaining_digits > 0 {
            panic!("index out of bounds");
        }
        let first = DigitsUnds {
            bytes: self.bytes.split_at(mid + unds).0,
            digits: mid,
        };

        // skip over underscores between first part and last part
        while let Some((&b'_', rem)) = rem_bytes.split_first() {
            rem_bytes = rem;
        }
        (
            first,
            DigitsUnds {
                bytes: rem_bytes,
                digits: self.digits - mid,
            },
        )
    }

    #[inline]
    pub const fn split_first(self) -> Option<(u8, DigitsUnds<'a>)> {
        let Some((&first, mut rem_bytes)) = self.bytes.split_first() else {
            return None;
        };

        // first byte is never underscore
        debug_assert!(first != b'_');

        // skip over underscores between first digit and last part
        while let Some((&b'_', rem)) = rem_bytes.split_first() {
            rem_bytes = rem;
        }
        Some((
            first,
            DigitsUnds {
                bytes: rem_bytes,
                digits: self.digits - 1,
            },
        ))
    }

    #[inline]
    const fn split_last(self) -> Option<(u8, DigitsUnds<'a>)> {
        let Some((&last, mut rem_bytes)) = self.bytes.split_last() else {
            return None;
        };

        // last byte is never underscore
        debug_assert!(last != b'_');

        // skip over underscores between first part and last digit
        while let Some((&b'_', rem)) = rem_bytes.split_last() {
            rem_bytes = rem;
        }
        Some((
            last,
            DigitsUnds {
                bytes: rem_bytes,
                digits: self.digits - 1,
            },
        ))
    }

    const fn split_leading_zeros(self) -> (usize, DigitsUnds<'a>) {
        let mut zeros = 0;
        let mut rem = self;
        while let Some((b'0', rest)) = rem.split_first() {
            zeros += 1;
            rem = rest;
        }
        (zeros, rem)
    }

    const fn split_trailing_zeros(self) -> (usize, DigitsUnds<'a>) {
        let mut zeros = 0;
        let mut rem = self;
        while let Some((b'0', rest)) = rem.split_last() {
            zeros += 1;
            rem = rest;
        }
        (zeros, rem)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DigitsExp<'a> {
    leading_zeros: usize,
    part1: DigitsUnds<'a>,
    part2: DigitsUnds<'a>,
    trailing_zeros: usize,
}

impl<'a> DigitsExp<'a> {
    const EMPTY: DigitsExp<'a> = DigitsExp {
        leading_zeros: 0,
        part1: DigitsUnds::EMPTY,
        part2: DigitsUnds::EMPTY,
        trailing_zeros: 0,
    };

    const fn new1(digits: DigitsUnds<'a>) -> DigitsExp<'a> {
        let (leading_zeros, rest) = digits.split_leading_zeros();
        let (trailing_zeros, rest) = rest.split_trailing_zeros();
        DigitsExp {
            leading_zeros,
            part1: rest,
            part2: DigitsUnds::EMPTY,
            trailing_zeros,
        }
    }

    const fn new2(digits1: DigitsUnds<'a>, digits2: DigitsUnds<'a>) -> DigitsExp<'a> {
        let (mut leading_zeros, mut digits1) = digits1.split_leading_zeros();
        let digits2 = if digits1.is_empty() {
            let (more_leading_zeros, new_digits1) = digits2.split_leading_zeros();
            leading_zeros += more_leading_zeros;
            digits1 = new_digits1;
            DigitsUnds::EMPTY
        } else {
            digits2
        };
        let (mut trailing_zeros, digits2) = digits2.split_trailing_zeros();
        if digits2.is_empty() {
            let (more_trailing_zeros, new_digits1) = digits1.split_trailing_zeros();
            trailing_zeros += more_trailing_zeros;
            digits1 = new_digits1;
        }
        DigitsExp {
            leading_zeros,
            part1: digits1,
            part2: digits2,
            trailing_zeros,
        }
    }

    // exp.unsigned_abs() must fit in usize, and results must have lengths that fit in usize
    pub const fn new_int_frac(
        int: DigitsUnds<'a>,
        frac: DigitsUnds<'a>,
        exp: i32,
    ) -> Option<(DigitsExp<'a>, DigitsExp<'a>)> {
        if int.len() > usize::MAX - frac.len() {
            return None;
        }
        let abs_exp = exp.unsigned_abs() as usize;
        if abs_exp as u32 != exp.unsigned_abs() {
            return None;
        }

        let (mut int, mut frac) = if exp == 0 {
            (DigitsExp::new1(int), DigitsExp::new1(frac))
        } else if exp < 0 {
            if let Some(extra_zeros) = abs_exp.checked_sub(int.len()) {
                let mut frac = DigitsExp::new2(int, frac);
                frac.trailing_zeros = 0;
                if extra_zeros > usize::MAX - frac.len() {
                    return None;
                }
                frac.leading_zeros += extra_zeros;
                (DigitsExp::EMPTY, frac)
            } else {
                let int = int.split_at(int.len() - abs_exp);
                (DigitsExp::new1(int.0), DigitsExp::new2(int.1, frac))
            }
        } else {
            // exp > 0
            if let Some(extra_zeros) = abs_exp.checked_sub(frac.len()) {
                let mut int = DigitsExp::new2(int, frac);
                int.leading_zeros = 0;
                if extra_zeros > usize::MAX - int.len() {
                    return None;
                }
                int.trailing_zeros += extra_zeros;
                (int, DigitsExp::EMPTY)
            } else {
                let frac = frac.split_at(abs_exp);
                (DigitsExp::new2(int, frac.0), DigitsExp::new1(frac.1))
            }
        };
        int.leading_zeros = 0;
        if int.part1.is_empty() && int.part2.is_empty() {
            int.trailing_zeros = 0;
        }
        frac.trailing_zeros = 0;
        if frac.part2.is_empty() && frac.part1.is_empty() {
            frac.leading_zeros = 0;
        }
        Some((int, frac))
    }

    #[inline]
    pub const fn len(self) -> usize {
        self.leading_zeros + self.part1.len() + self.part2.len() + self.trailing_zeros
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub const fn split_at(self, mut mid: usize) -> (DigitsExp<'a>, DigitsExp<'a>) {
        let mut first = DigitsExp::EMPTY;
        let mut last = self;
        if mid == 0 {
            return (first, last);
        }

        if mid < self.leading_zeros {
            (first.leading_zeros, last.leading_zeros) = (mid, self.leading_zeros - mid);
            return (first, last);
        }

        (first.leading_zeros, last.leading_zeros) = (self.leading_zeros, 0);
        mid -= self.leading_zeros;
        if mid == 0 {
            return (first, last);
        }

        if mid < self.part1.len() {
            (first.part1, last.part1) = self.part1.split_at(mid);
            return (first, last);
        }

        first.part1 = self.part1;
        last.part1 = self.part2;
        last.part2 = DigitsUnds::EMPTY;
        mid -= self.part1.len();
        if mid == 0 {
            return (first, last);
        }

        if mid < self.part2.len() {
            (first.part2, last.part1) = self.part2.split_at(mid);
            return (first, last);
        }

        first.part2 = self.part2;
        last.leading_zeros = self.trailing_zeros;
        last.part1 = DigitsUnds::EMPTY;
        last.trailing_zeros = 0;
        mid -= self.part2.len();
        if mid == 0 {
            return (first, last);
        }

        if mid < self.trailing_zeros {
            (first.trailing_zeros, last.leading_zeros) = (mid, self.trailing_zeros - mid);
            return (first, last);
        }

        (first.trailing_zeros, last.leading_zeros) = (self.trailing_zeros, 0);
        mid -= self.trailing_zeros;
        if mid == 0 {
            return (first, last);
        }

        panic!("index out of bounds");
    }

    // no automatic renormalization done after split_first
    #[inline]
    pub const fn split_first(self) -> Option<(u8, DigitsExp<'a>)> {
        if self.leading_zeros > 0 {
            return Some((
                b'0',
                DigitsExp {
                    leading_zeros: self.leading_zeros - 1,
                    ..self
                },
            ));
        }
        if let Some((first, rest)) = self.part1.split_first() {
            return Some((
                first,
                DigitsExp {
                    part1: rest,
                    ..self
                },
            ));
        }
        if let Some((first, rest)) = self.part2.split_first() {
            return Some((
                first,
                DigitsExp {
                    part2: rest,
                    ..self
                },
            ));
        }
        if self.trailing_zeros > 0 {
            return Some((
                b'0',
                DigitsExp {
                    trailing_zeros: self.trailing_zeros - 1,
                    ..self
                },
            ));
        }
        None
    }
}
