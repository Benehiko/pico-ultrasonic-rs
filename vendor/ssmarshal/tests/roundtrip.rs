// Copyright (c) 2017 The Robigalia Project Developers Licensed under the Apache License, Version
// 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. All files in the project
// carrying such notice may not be copied, modified, or distributed except according to those
// terms.

#![feature(proc_macro, plugin)]
#![plugin(quickcheck_macros)]
#![allow(non_snake_case)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate quickcheck;

extern crate ssmarshal;

use serde::{Serialize};
use serde::de::DeserializeOwned;

use ssmarshal::{serialize, deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct U8(u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct U16(u16);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct U32(u32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct U64(u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct USize(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct I8(i8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct I16(i16);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct I32(i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct I64(i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ISize(isize);

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
struct F32(f32);

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
struct F64(f64);

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
struct Simple {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: f32,
    g: u8,
    h: f64,
}

impl quickcheck::Arbitrary for Simple {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Simple {
        Simple {
            a: g.gen(),
            b: g.gen(),
            c: g.gen(),
            d: g.gen(),
            e: g.gen(),
            f: g.gen(),
            g: g.gen(),
            h: g.gen(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
struct Unit;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
struct Complex {
    a: Simple,
    e: Unit,
    b: Simple,
    c: [u8; 7],
    d: (),
    f: [Unit; 3],
}

impl quickcheck::Arbitrary for Complex {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Complex {
        Complex {
            a: Simple::arbitrary(g),
            b: Simple::arbitrary(g),
            c: g.gen(),
            d: (),
            e: Unit,
            f: [Unit; 3],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
enum ComplexEnum {
    A,
    B(Simple),
    C(u8, u16),
    D(isize),
    E {
        foo: Simple
    },
    F {
        bar: Complex,
        baz: Simple,
        qux: char
    }
}

impl quickcheck::Arbitrary for ComplexEnum {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> ComplexEnum {
        match g.gen_range(0, 6) {
            0 => {
                ComplexEnum::A
            },
            1 => {
                ComplexEnum::B(Simple::arbitrary(g))
            },
            2 => {
                ComplexEnum::C(g.gen(), g.gen())
            },
            3 => {
                ComplexEnum::D(g.gen())
            },
            4 => {
                ComplexEnum::E { foo: Simple::arbitrary(g) }
            },
            5 => {
                ComplexEnum::F { bar: Complex::arbitrary(g), baz: Simple::arbitrary(g), qux: g.gen() }
            },
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
struct TupleStruct(u8, u64, Simple, Option<ComplexEnum>);

impl quickcheck::Arbitrary for TupleStruct {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> TupleStruct {
        TupleStruct(g.gen(), g.gen(), quickcheck::Arbitrary::arbitrary(g), quickcheck::Arbitrary::arbitrary(g))
    }
}

fn rt_val<T: Serialize + DeserializeOwned + PartialEq+std::fmt::Debug>(val: &T) -> bool {
    let mut buf = vec![0; std::mem::size_of::<T>()];
    serialize(&mut buf, val).unwrap();
    let new_val: T = deserialize(&buf).unwrap().0;
    println!("\n\nOld: {:?}\nNew: {:?}", val, new_val);
    val == &new_val
}

#[quickcheck]
fn rt_simple(val: Simple) -> bool {
    rt_val(&val)
}

#[quickcheck]
fn rt_complex(val: Complex) -> bool {
    rt_val(&val)
}

#[quickcheck]
fn rt_complexenum(val: ComplexEnum) -> bool {
    rt_val(&val)
}

#[quickcheck]
fn rt_tuplestruct(val: TupleStruct) -> bool {
    rt_val(&val)
}

macro_rules! rt {
    ($(($name:ident, $t:ty)),*) => {
        $(#[quickcheck] fn $name(val: $t) -> bool {
            rt_val::<$t>(&val)
        })*
    }
}

macro_rules! rt_wrap {
    ($(($name:ident, $t:ty, $n:ident)),*) => {
        $(#[quickcheck] fn $name(val: $t) -> bool {
            rt_val::<$n>(&$n(val))
        })*
    }
}

rt! {
    (rt_i8,  i8),
    (rt_i16, i16),
    (rt_i32, i32),
    (rt_i64, i64),
    (rt_u8,  u8),
    (rt_u16, u16),
    (rt_u32, u32),
    (rt_u64, u64),
    (rt_usize, usize),
    (rt_isize, isize),
    (rt_f32, f32),
    (rt_f64, f64),
    (rt_tp1, (u8, u8, u8)),
    (rt_tp2, (u8, i16, u8)),
    (rt_tp3, (u8, i16, u8)),
    (rt_tp4, (usize, i16, isize)),
    (rt_char, char)
}

rt_wrap! {
    (rt_I8,  i8 , I8),
    (rt_I16, i16, I16),
    (rt_I32, i32, I32),
    (rt_I64, i64, I64),
    (rt_U8,  u8 , U8),
    (rt_U16, u16, U16),
    (rt_U32, u32, U32),
    (rt_U64, u64, U64),
    (rt_F32, f32, F32),
    (rt_F64, f64, F64)
}
