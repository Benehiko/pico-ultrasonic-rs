fixed-macro
===========

[![Build](https://github.com/aldanor/fixed-macro/workflows/CI/badge.svg)](https://github.com/aldanor/fixed-macro/actions?query=branch%3Amaster)
[![Latest Version](https://img.shields.io/crates/v/fixed-macro.svg)](https://crates.io/crates/fixed-macro)
[![Documentation](https://docs.rs/fixed-macro/badge.svg)](https://docs.rs/fixed-macro)
[![Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This library provides [`fixed!`][fm-fixed], a proc-macro that allows
easily creating fixed-point constants for all of the fixed-point types provided in
[`fixed`][fixed] crate.

```toml
[dependencies]
fixed-macro = "1.1"
```

*Compiler support: rustc 1.61+.*

[fixed]: https://docs.rs/fixed
[fixed-types]: https://docs.rs/fixed/latest/fixed/types/index.html
[fm-fixed]: https://docs.rs/fixed-macro/latest/fixed_macro/macro.fixed.html
[fm-types]: https://docs.rs/fixed-macro/latest/fixed_macro/types/index.html

## Details

- The syntax of the macro is as follows:
  
  ```rust
  fixed!(<value>: <type>)
  ```
  
  where `<value>` is an integer literal or a float literal, and `<type>` is either of the 
  form `I<i>F<f>` or `U<i>F<f>`, matching one of the type aliases provided in
  [`fixed::types`][fixed-types]. Note in particular that `<value>` has to be a literal and
  not an arbitrary arithmetic expression, and that `<type>` is considered a special identifier,
  so that it doesn't have to be imported first.

- Create a fixed-point constant which is parsed at compile time (the same syntax for int
  and float literals is supported as in Rust itself, including underscores and scientific
  notation):

  ```rust
  use fixed_macro::fixed;
  use fixed::types::U8F8;

  let x1 = fixed!(-1.23: I32F32);         // float literal (note, the type is not in scope)
  const X2: U8F8 = fixed!(1.2: U8F8);     // can be used to initialize const values
  let x3 = fixed!(123: U8F8);             // decimal integers work as well
  let x4 = fixed!(0x7B: U8F8);            // and hex/oct/bin integers too
  let x5 = fixed!(1_234.567_890: I32F32); // underscores are ignored, same as in rustc
  let x7 = fixed!(0.12e+01: U8F8);        // scientific notation is also supported
  ```
    
- For each type alias from [`fixed::types`][fixed-types], there is a macro with a matching
  name in [`fixed_macro::types`][fm-types] which you can use without specifying the type name:
  
  ```rust
  use fixed_macro::types::I16F48;
  
  let a1 = I16F48!(-1.23);
  ```
  
  Both the macro and the type can happily coexist in the same scope:
  
  ```rust
  use fixed::types::I16F48;
  use fixed_macro::types::I16F48;
  
  const B1: I16F48 = I16F48!(1.23e-2);
  ```
  
  You can choose to import both under different (or same) user-defined names:
  
  ```rust
  use fixed::types::{I16F48 as Decimal};
  use fixed_macro::types::{I16F48 as dec};
  
  let c1 = dec!(12_345);
  const C2: Decimal = dec!(-0.123_456);
  ```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
