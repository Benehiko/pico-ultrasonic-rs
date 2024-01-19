//! Macros allowing to create constants for each available fixed-point type.

#![no_std]

// 8-bit signed
#[macro_export]
/// Macro to create [I8F0](https://docs.rs/fixed/latest/fixed/types/type.I0F8.html) constants.
macro_rules! I8F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I8F0) } }
#[macro_export]
/// Macro to create [I7F1](https://docs.rs/fixed/latest/fixed/types/type.I7F1.html) constants.
macro_rules! I7F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I7F1) } }
#[macro_export]
/// Macro to create [I6F2](https://docs.rs/fixed/latest/fixed/types/type.I6F2.html) constants.
macro_rules! I6F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I6F2) } }
#[macro_export]
/// Macro to create [I5F3](https://docs.rs/fixed/latest/fixed/types/type.I5F3.html) constants.
macro_rules! I5F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I5F3) } }
#[macro_export]
/// Macro to create [I4F4](https://docs.rs/fixed/latest/fixed/types/type.I4F4.html) constants.
macro_rules! I4F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I4F4) } }
#[macro_export]
/// Macro to create [I3F5](https://docs.rs/fixed/latest/fixed/types/type.I3F5.html) constants.
macro_rules! I3F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I3F5) } }
#[macro_export]
/// Macro to create [I2F6](https://docs.rs/fixed/latest/fixed/types/type.I2F6.html) constants.
macro_rules! I2F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I2F6) } }
#[macro_export]
/// Macro to create [I1F7](https://docs.rs/fixed/latest/fixed/types/type.I1F7.html) constants.
macro_rules! I1F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I1F7) } }
#[macro_export]
/// Macro to create [I0F8](https://docs.rs/fixed/latest/fixed/types/type.I0F8.html) constants.
macro_rules! I0F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I0F8) } }

// 16-bit signed
#[macro_export]
/// Macro to create [I16F0](https://docs.rs/fixed/latest/fixed/types/type.I16F0.html) constants.
macro_rules! I16F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I16F0) } }
#[macro_export]
/// Macro to create [I15F1](https://docs.rs/fixed/latest/fixed/types/type.I15F1.html) constants.
macro_rules! I15F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I15F1) } }
#[macro_export]
/// Macro to create [I14F2](https://docs.rs/fixed/latest/fixed/types/type.I14F2.html) constants.
macro_rules! I14F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I14F2) } }
#[macro_export]
/// Macro to create [I13F3](https://docs.rs/fixed/latest/fixed/types/type.I13F3.html) constants.
macro_rules! I13F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I13F3) } }
#[macro_export]
/// Macro to create [I12F4](https://docs.rs/fixed/latest/fixed/types/type.I12F4.html) constants.
macro_rules! I12F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I12F4) } }
#[macro_export]
/// Macro to create [I11F5](https://docs.rs/fixed/latest/fixed/types/type.I11F5.html) constants.
macro_rules! I11F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I11F5) } }
#[macro_export]
/// Macro to create [I10F6](https://docs.rs/fixed/latest/fixed/types/type.I10F6.html) constants.
macro_rules! I10F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I10F6) } }
#[macro_export]
/// Macro to create [I9F7](https://docs.rs/fixed/latest/fixed/types/type.I9F7.html) constants.
macro_rules! I9F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I9F7) } }
#[macro_export]
/// Macro to create [I8F8](https://docs.rs/fixed/latest/fixed/types/type.I8F8.html) constants.
macro_rules! I8F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I8F8) } }
#[macro_export]
/// Macro to create [I7F9](https://docs.rs/fixed/latest/fixed/types/type.I7F9.html) constants.
macro_rules! I7F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I7F9) } }
#[macro_export]
/// Macro to create [I6F10](https://docs.rs/fixed/latest/fixed/types/type.I6F10.html) constants.
macro_rules! I6F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I6F10) } }
#[macro_export]
/// Macro to create [I5F11](https://docs.rs/fixed/latest/fixed/types/type.I5F11.html) constants.
macro_rules! I5F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I5F11) } }
#[macro_export]
/// Macro to create [I4F12](https://docs.rs/fixed/latest/fixed/types/type.I4F12.html) constants.
macro_rules! I4F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I4F12) } }
#[macro_export]
/// Macro to create [I3F13](https://docs.rs/fixed/latest/fixed/types/type.I3F13.html) constants.
macro_rules! I3F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I3F13) } }
#[macro_export]
/// Macro to create [I2F14](https://docs.rs/fixed/latest/fixed/types/type.I2F14.html) constants.
macro_rules! I2F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I2F14) } }
#[macro_export]
/// Macro to create [I1F15](https://docs.rs/fixed/latest/fixed/types/type.I1F15.html) constants.
macro_rules! I1F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I1F15) } }
#[macro_export]
/// Macro to create [I0F16](https://docs.rs/fixed/latest/fixed/types/type.I0F16.html) constants.
macro_rules! I0F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I0F16) } }

// 32-bit signed
#[macro_export]
/// Macro to create [I32F0](https://docs.rs/fixed/latest/fixed/types/type.I32F0.html) constants.
macro_rules! I32F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I32F0) } }
#[macro_export]
/// Macro to create [I31F1](https://docs.rs/fixed/latest/fixed/types/type.I31F1.html) constants.
macro_rules! I31F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I31F1) } }
#[macro_export]
/// Macro to create [I30F2](https://docs.rs/fixed/latest/fixed/types/type.I30F2.html) constants.
macro_rules! I30F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I30F2) } }
#[macro_export]
/// Macro to create [I29F3](https://docs.rs/fixed/latest/fixed/types/type.I29F3.html) constants.
macro_rules! I29F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I29F3) } }
#[macro_export]
/// Macro to create [I28F4](https://docs.rs/fixed/latest/fixed/types/type.I28F4.html) constants.
macro_rules! I28F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I28F4) } }
#[macro_export]
/// Macro to create [I27F5](https://docs.rs/fixed/latest/fixed/types/type.I27F5.html) constants.
macro_rules! I27F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I27F5) } }
#[macro_export]
/// Macro to create [I26F6](https://docs.rs/fixed/latest/fixed/types/type.I26F6.html) constants.
macro_rules! I26F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I26F6) } }
#[macro_export]
/// Macro to create [I25F7](https://docs.rs/fixed/latest/fixed/types/type.I25F7.html) constants.
macro_rules! I25F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I25F7) } }
#[macro_export]
/// Macro to create [I24F8](https://docs.rs/fixed/latest/fixed/types/type.I24F8.html) constants.
macro_rules! I24F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I24F8) } }
#[macro_export]
/// Macro to create [I23F9](https://docs.rs/fixed/latest/fixed/types/type.I23F9.html) constants.
macro_rules! I23F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I23F9) } }
#[macro_export]
/// Macro to create [I22F10](https://docs.rs/fixed/latest/fixed/types/type.I22F10.html) constants.
macro_rules! I22F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I22F10) } }
#[macro_export]
/// Macro to create [I21F11](https://docs.rs/fixed/latest/fixed/types/type.I21F11.html) constants.
macro_rules! I21F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I21F11) } }
#[macro_export]
/// Macro to create [I20F12](https://docs.rs/fixed/latest/fixed/types/type.I20F12.html) constants.
macro_rules! I20F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I20F12) } }
#[macro_export]
/// Macro to create [I19F13](https://docs.rs/fixed/latest/fixed/types/type.I19F13.html) constants.
macro_rules! I19F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I19F13) } }
#[macro_export]
/// Macro to create [I18F14](https://docs.rs/fixed/latest/fixed/types/type.I18F14.html) constants.
macro_rules! I18F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I18F14) } }
#[macro_export]
/// Macro to create [I17F15](https://docs.rs/fixed/latest/fixed/types/type.I17F15.html) constants.
macro_rules! I17F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I17F15) } }
#[macro_export]
/// Macro to create [I16F16](https://docs.rs/fixed/latest/fixed/types/type.I16F16.html) constants.
macro_rules! I16F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I16F16) } }
#[macro_export]
/// Macro to create [I15F17](https://docs.rs/fixed/latest/fixed/types/type.I15F17.html) constants.
macro_rules! I15F17 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I15F17) } }
#[macro_export]
/// Macro to create [I14F18](https://docs.rs/fixed/latest/fixed/types/type.I14F18.html) constants.
macro_rules! I14F18 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I14F18) } }
#[macro_export]
/// Macro to create [I13F19](https://docs.rs/fixed/latest/fixed/types/type.I13F19.html) constants.
macro_rules! I13F19 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I13F19) } }
#[macro_export]
/// Macro to create [I12F20](https://docs.rs/fixed/latest/fixed/types/type.I12F20.html) constants.
macro_rules! I12F20 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I12F20) } }
#[macro_export]
/// Macro to create [I11F21](https://docs.rs/fixed/latest/fixed/types/type.I11F21.html) constants.
macro_rules! I11F21 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I11F21) } }
#[macro_export]
/// Macro to create [I10F22](https://docs.rs/fixed/latest/fixed/types/type.I10F22.html) constants.
macro_rules! I10F22 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I10F22) } }
#[macro_export]
/// Macro to create [I9F23](https://docs.rs/fixed/latest/fixed/types/type.I9F23.html) constants.
macro_rules! I9F23 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I9F23) } }
#[macro_export]
/// Macro to create [I8F24](https://docs.rs/fixed/latest/fixed/types/type.I8F24.html) constants.
macro_rules! I8F24 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I8F24) } }
#[macro_export]
/// Macro to create [I7F25](https://docs.rs/fixed/latest/fixed/types/type.I7F25.html) constants.
macro_rules! I7F25 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I7F25) } }
#[macro_export]
/// Macro to create [I6F26](https://docs.rs/fixed/latest/fixed/types/type.I6F26.html) constants.
macro_rules! I6F26 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I6F26) } }
#[macro_export]
/// Macro to create [I5F27](https://docs.rs/fixed/latest/fixed/types/type.I5F27.html) constants.
macro_rules! I5F27 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I5F27) } }
#[macro_export]
/// Macro to create [I4F28](https://docs.rs/fixed/latest/fixed/types/type.I4F28.html) constants.
macro_rules! I4F28 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I4F28) } }
#[macro_export]
/// Macro to create [I3F29](https://docs.rs/fixed/latest/fixed/types/type.I3F29.html) constants.
macro_rules! I3F29 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I3F29) } }
#[macro_export]
/// Macro to create [I2F30](https://docs.rs/fixed/latest/fixed/types/type.I2F30.html) constants.
macro_rules! I2F30 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I2F30) } }
#[macro_export]
/// Macro to create [I1F31](https://docs.rs/fixed/latest/fixed/types/type.I1F31.html) constants.
macro_rules! I1F31 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I1F31) } }
#[macro_export]
/// Macro to create [I0F32](https://docs.rs/fixed/latest/fixed/types/type.I0F32.html) constants.
macro_rules! I0F32 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I0F32) } }

// 64-bit signed
#[macro_export]
/// Macro to create [I64F0](https://docs.rs/fixed/latest/fixed/types/type.I64F0.html) constants.
macro_rules! I64F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I64F0) } }
#[macro_export]
/// Macro to create [I63F1](https://docs.rs/fixed/latest/fixed/types/type.I63F1.html) constants.
macro_rules! I63F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I63F1) } }
#[macro_export]
/// Macro to create [I62F2](https://docs.rs/fixed/latest/fixed/types/type.I62F2.html) constants.
macro_rules! I62F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I62F2) } }
#[macro_export]
/// Macro to create [I61F3](https://docs.rs/fixed/latest/fixed/types/type.I61F3.html) constants.
macro_rules! I61F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I61F3) } }
#[macro_export]
/// Macro to create [I60F4](https://docs.rs/fixed/latest/fixed/types/type.I60F4.html) constants.
macro_rules! I60F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I60F4) } }
#[macro_export]
/// Macro to create [I59F5](https://docs.rs/fixed/latest/fixed/types/type.I59F5.html) constants.
macro_rules! I59F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I59F5) } }
#[macro_export]
/// Macro to create [I58F6](https://docs.rs/fixed/latest/fixed/types/type.I58F6.html) constants.
macro_rules! I58F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I58F6) } }
#[macro_export]
/// Macro to create [I57F7](https://docs.rs/fixed/latest/fixed/types/type.I57F7.html) constants.
macro_rules! I57F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I57F7) } }
#[macro_export]
/// Macro to create [I56F8](https://docs.rs/fixed/latest/fixed/types/type.I56F8.html) constants.
macro_rules! I56F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I56F8) } }
#[macro_export]
/// Macro to create [I55F9](https://docs.rs/fixed/latest/fixed/types/type.I55F9.html) constants.
macro_rules! I55F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I55F9) } }
#[macro_export]
/// Macro to create [I54F10](https://docs.rs/fixed/latest/fixed/types/type.I54F10.html) constants.
macro_rules! I54F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I54F10) } }
#[macro_export]
/// Macro to create [I53F11](https://docs.rs/fixed/latest/fixed/types/type.I53F11.html) constants.
macro_rules! I53F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I53F11) } }
#[macro_export]
/// Macro to create [I52F12](https://docs.rs/fixed/latest/fixed/types/type.I52F12.html) constants.
macro_rules! I52F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I52F12) } }
#[macro_export]
/// Macro to create [I51F13](https://docs.rs/fixed/latest/fixed/types/type.I51F13.html) constants.
macro_rules! I51F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I51F13) } }
#[macro_export]
/// Macro to create [I50F14](https://docs.rs/fixed/latest/fixed/types/type.I50F14.html) constants.
macro_rules! I50F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I50F14) } }
#[macro_export]
/// Macro to create [I49F15](https://docs.rs/fixed/latest/fixed/types/type.I49F15.html) constants.
macro_rules! I49F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I49F15) } }
#[macro_export]
/// Macro to create [I48F16](https://docs.rs/fixed/latest/fixed/types/type.I48F16.html) constants.
macro_rules! I48F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I48F16) } }
#[macro_export]
/// Macro to create [I47F17](https://docs.rs/fixed/latest/fixed/types/type.I47F17.html) constants.
macro_rules! I47F17 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I47F17) } }
#[macro_export]
/// Macro to create [I46F18](https://docs.rs/fixed/latest/fixed/types/type.I46F18.html) constants.
macro_rules! I46F18 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I46F18) } }
#[macro_export]
/// Macro to create [I45F19](https://docs.rs/fixed/latest/fixed/types/type.I45F19.html) constants.
macro_rules! I45F19 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I45F19) } }
#[macro_export]
/// Macro to create [I44F20](https://docs.rs/fixed/latest/fixed/types/type.I44F20.html) constants.
macro_rules! I44F20 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I44F20) } }
#[macro_export]
/// Macro to create [I43F21](https://docs.rs/fixed/latest/fixed/types/type.I43F21.html) constants.
macro_rules! I43F21 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I43F21) } }
#[macro_export]
/// Macro to create [I42F22](https://docs.rs/fixed/latest/fixed/types/type.I42F22.html) constants.
macro_rules! I42F22 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I42F22) } }
#[macro_export]
/// Macro to create [I41F23](https://docs.rs/fixed/latest/fixed/types/type.I41F23.html) constants.
macro_rules! I41F23 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I41F23) } }
#[macro_export]
/// Macro to create [I40F24](https://docs.rs/fixed/latest/fixed/types/type.I40F24.html) constants.
macro_rules! I40F24 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I40F24) } }
#[macro_export]
/// Macro to create [I39F25](https://docs.rs/fixed/latest/fixed/types/type.I39F25.html) constants.
macro_rules! I39F25 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I39F25) } }
#[macro_export]
/// Macro to create [I38F26](https://docs.rs/fixed/latest/fixed/types/type.I38F26.html) constants.
macro_rules! I38F26 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I38F26) } }
#[macro_export]
/// Macro to create [I37F27](https://docs.rs/fixed/latest/fixed/types/type.I37F27.html) constants.
macro_rules! I37F27 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I37F27) } }
#[macro_export]
/// Macro to create [I36F28](https://docs.rs/fixed/latest/fixed/types/type.I36F28.html) constants.
macro_rules! I36F28 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I36F28) } }
#[macro_export]
/// Macro to create [I35F29](https://docs.rs/fixed/latest/fixed/types/type.I35F29.html) constants.
macro_rules! I35F29 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I35F29) } }
#[macro_export]
/// Macro to create [I34F30](https://docs.rs/fixed/latest/fixed/types/type.I34F30.html) constants.
macro_rules! I34F30 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I34F30) } }
#[macro_export]
/// Macro to create [I33F31](https://docs.rs/fixed/latest/fixed/types/type.I33F31.html) constants.
macro_rules! I33F31 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I33F31) } }
#[macro_export]
/// Macro to create [I32F32](https://docs.rs/fixed/latest/fixed/types/type.I32F32.html) constants.
macro_rules! I32F32 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I32F32) } }
#[macro_export]
/// Macro to create [I31F33](https://docs.rs/fixed/latest/fixed/types/type.I31F33.html) constants.
macro_rules! I31F33 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I31F33) } }
#[macro_export]
/// Macro to create [I30F34](https://docs.rs/fixed/latest/fixed/types/type.I30F34.html) constants.
macro_rules! I30F34 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I30F34) } }
#[macro_export]
/// Macro to create [I29F35](https://docs.rs/fixed/latest/fixed/types/type.I29F35.html) constants.
macro_rules! I29F35 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I29F35) } }
#[macro_export]
/// Macro to create [I28F36](https://docs.rs/fixed/latest/fixed/types/type.I28F36.html) constants.
macro_rules! I28F36 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I28F36) } }
#[macro_export]
/// Macro to create [I27F37](https://docs.rs/fixed/latest/fixed/types/type.I27F37.html) constants.
macro_rules! I27F37 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I27F37) } }
#[macro_export]
/// Macro to create [I26F38](https://docs.rs/fixed/latest/fixed/types/type.I26F38.html) constants.
macro_rules! I26F38 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I26F38) } }
#[macro_export]
/// Macro to create [I25F39](https://docs.rs/fixed/latest/fixed/types/type.I25F39.html) constants.
macro_rules! I25F39 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I25F39) } }
#[macro_export]
/// Macro to create [I24F40](https://docs.rs/fixed/latest/fixed/types/type.I24F40.html) constants.
macro_rules! I24F40 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I24F40) } }
#[macro_export]
/// Macro to create [I23F41](https://docs.rs/fixed/latest/fixed/types/type.I23F41.html) constants.
macro_rules! I23F41 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I23F41) } }
#[macro_export]
/// Macro to create [I22F42](https://docs.rs/fixed/latest/fixed/types/type.I22F42.html) constants.
macro_rules! I22F42 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I22F42) } }
#[macro_export]
/// Macro to create [I21F43](https://docs.rs/fixed/latest/fixed/types/type.I21F43.html) constants.
macro_rules! I21F43 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I21F43) } }
#[macro_export]
/// Macro to create [I20F44](https://docs.rs/fixed/latest/fixed/types/type.I20F44.html) constants.
macro_rules! I20F44 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I20F44) } }
#[macro_export]
/// Macro to create [I19F45](https://docs.rs/fixed/latest/fixed/types/type.I19F45.html) constants.
macro_rules! I19F45 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I19F45) } }
#[macro_export]
/// Macro to create [I18F46](https://docs.rs/fixed/latest/fixed/types/type.I18F46.html) constants.
macro_rules! I18F46 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I18F46) } }
#[macro_export]
/// Macro to create [I17F47](https://docs.rs/fixed/latest/fixed/types/type.I17F47.html) constants.
macro_rules! I17F47 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I17F47) } }
#[macro_export]
/// Macro to create [I16F48](https://docs.rs/fixed/latest/fixed/types/type.I16F48.html) constants.
macro_rules! I16F48 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I16F48) } }
#[macro_export]
/// Macro to create [I15F49](https://docs.rs/fixed/latest/fixed/types/type.I15F49.html) constants.
macro_rules! I15F49 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I15F49) } }
#[macro_export]
/// Macro to create [I14F50](https://docs.rs/fixed/latest/fixed/types/type.I14F50.html) constants.
macro_rules! I14F50 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I14F50) } }
#[macro_export]
/// Macro to create [I13F51](https://docs.rs/fixed/latest/fixed/types/type.I13F51.html) constants.
macro_rules! I13F51 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I13F51) } }
#[macro_export]
/// Macro to create [I12F52](https://docs.rs/fixed/latest/fixed/types/type.I12F52.html) constants.
macro_rules! I12F52 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I12F52) } }
#[macro_export]
/// Macro to create [I11F53](https://docs.rs/fixed/latest/fixed/types/type.I11F53.html) constants.
macro_rules! I11F53 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I11F53) } }
#[macro_export]
/// Macro to create [I10F54](https://docs.rs/fixed/latest/fixed/types/type.I10F54.html) constants.
macro_rules! I10F54 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I10F54) } }
#[macro_export]
/// Macro to create [I9F55](https://docs.rs/fixed/latest/fixed/types/type.I9F55.html) constants.
macro_rules! I9F55 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I9F55) } }
#[macro_export]
/// Macro to create [I8F56](https://docs.rs/fixed/latest/fixed/types/type.I8F56.html) constants.
macro_rules! I8F56 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I8F56) } }
#[macro_export]
/// Macro to create [I7F57](https://docs.rs/fixed/latest/fixed/types/type.I7F57.html) constants.
macro_rules! I7F57 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I7F57) } }
#[macro_export]
/// Macro to create [I6F58](https://docs.rs/fixed/latest/fixed/types/type.I6F58.html) constants.
macro_rules! I6F58 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I6F58) } }
#[macro_export]
/// Macro to create [I5F59](https://docs.rs/fixed/latest/fixed/types/type.I5F59.html) constants.
macro_rules! I5F59 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I5F59) } }
#[macro_export]
/// Macro to create [I4F60](https://docs.rs/fixed/latest/fixed/types/type.I4F60.html) constants.
macro_rules! I4F60 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I4F60) } }
#[macro_export]
/// Macro to create [I3F61](https://docs.rs/fixed/latest/fixed/types/type.I3F61.html) constants.
macro_rules! I3F61 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I3F61) } }
#[macro_export]
/// Macro to create [I2F62](https://docs.rs/fixed/latest/fixed/types/type.I2F62.html) constants.
macro_rules! I2F62 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I2F62) } }
#[macro_export]
/// Macro to create [I1F63](https://docs.rs/fixed/latest/fixed/types/type.I1F63.html) constants.
macro_rules! I1F63 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I1F63) } }
#[macro_export]
/// Macro to create [I0F64](https://docs.rs/fixed/latest/fixed/types/type.I0F64.html) constants.
macro_rules! I0F64 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I0F64) } }

// 128-bit signed
#[macro_export]
/// Macro to create [I128F0](https://docs.rs/fixed/latest/fixed/types/type.I128F0.html) constants.
macro_rules! I128F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I128F0) } }
#[macro_export]
/// Macro to create [I127F1](https://docs.rs/fixed/latest/fixed/types/type.I127F1.html) constants.
macro_rules! I127F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I127F1) } }
#[macro_export]
/// Macro to create [I126F2](https://docs.rs/fixed/latest/fixed/types/type.I126F2.html) constants.
macro_rules! I126F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I126F2) } }
#[macro_export]
/// Macro to create [I125F3](https://docs.rs/fixed/latest/fixed/types/type.I125F3.html) constants.
macro_rules! I125F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I125F3) } }
#[macro_export]
/// Macro to create [I124F4](https://docs.rs/fixed/latest/fixed/types/type.I124F4.html) constants.
macro_rules! I124F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I124F4) } }
#[macro_export]
/// Macro to create [I123F5](https://docs.rs/fixed/latest/fixed/types/type.I123F5.html) constants.
macro_rules! I123F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I123F5) } }
#[macro_export]
/// Macro to create [I122F6](https://docs.rs/fixed/latest/fixed/types/type.I122F6.html) constants.
macro_rules! I122F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I122F6) } }
#[macro_export]
/// Macro to create [I121F7](https://docs.rs/fixed/latest/fixed/types/type.I121F7.html) constants.
macro_rules! I121F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I121F7) } }
#[macro_export]
/// Macro to create [I120F8](https://docs.rs/fixed/latest/fixed/types/type.I120F8.html) constants.
macro_rules! I120F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I120F8) } }
#[macro_export]
/// Macro to create [I119F9](https://docs.rs/fixed/latest/fixed/types/type.I119F9.html) constants.
macro_rules! I119F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I119F9) } }
#[macro_export]
/// Macro to create [I118F10](https://docs.rs/fixed/latest/fixed/types/type.I118F10.html) constants.
macro_rules! I118F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I118F10) } }
#[macro_export]
/// Macro to create [I117F11](https://docs.rs/fixed/latest/fixed/types/type.I117F11.html) constants.
macro_rules! I117F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I117F11) } }
#[macro_export]
/// Macro to create [I116F12](https://docs.rs/fixed/latest/fixed/types/type.I116F12.html) constants.
macro_rules! I116F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I116F12) } }
#[macro_export]
/// Macro to create [I115F13](https://docs.rs/fixed/latest/fixed/types/type.I115F13.html) constants.
macro_rules! I115F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I115F13) } }
#[macro_export]
/// Macro to create [I114F14](https://docs.rs/fixed/latest/fixed/types/type.I114F14.html) constants.
macro_rules! I114F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I114F14) } }
#[macro_export]
/// Macro to create [I113F15](https://docs.rs/fixed/latest/fixed/types/type.I113F15.html) constants.
macro_rules! I113F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I113F15) } }
#[macro_export]
/// Macro to create [I112F16](https://docs.rs/fixed/latest/fixed/types/type.I112F16.html) constants.
macro_rules! I112F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I112F16) } }
#[macro_export]
/// Macro to create [I111F17](https://docs.rs/fixed/latest/fixed/types/type.I111F17.html) constants.
macro_rules! I111F17 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I111F17) } }
#[macro_export]
/// Macro to create [I110F18](https://docs.rs/fixed/latest/fixed/types/type.I110F18.html) constants.
macro_rules! I110F18 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I110F18) } }
#[macro_export]
/// Macro to create [I109F19](https://docs.rs/fixed/latest/fixed/types/type.I109F19.html) constants.
macro_rules! I109F19 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I109F19) } }
#[macro_export]
/// Macro to create [I108F20](https://docs.rs/fixed/latest/fixed/types/type.I108F20.html) constants.
macro_rules! I108F20 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I108F20) } }
#[macro_export]
/// Macro to create [I107F21](https://docs.rs/fixed/latest/fixed/types/type.I107F21.html) constants.
macro_rules! I107F21 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I107F21) } }
#[macro_export]
/// Macro to create [I106F22](https://docs.rs/fixed/latest/fixed/types/type.I106F22.html) constants.
macro_rules! I106F22 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I106F22) } }
#[macro_export]
/// Macro to create [I105F23](https://docs.rs/fixed/latest/fixed/types/type.I105F23.html) constants.
macro_rules! I105F23 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I105F23) } }
#[macro_export]
/// Macro to create [I104F24](https://docs.rs/fixed/latest/fixed/types/type.I104F24.html) constants.
macro_rules! I104F24 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I104F24) } }
#[macro_export]
/// Macro to create [I103F25](https://docs.rs/fixed/latest/fixed/types/type.I103F25.html) constants.
macro_rules! I103F25 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I103F25) } }
#[macro_export]
/// Macro to create [I102F26](https://docs.rs/fixed/latest/fixed/types/type.I102F26.html) constants.
macro_rules! I102F26 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I102F26) } }
#[macro_export]
/// Macro to create [I101F27](https://docs.rs/fixed/latest/fixed/types/type.I101F27.html) constants.
macro_rules! I101F27 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I101F27) } }
#[macro_export]
/// Macro to create [I100F28](https://docs.rs/fixed/latest/fixed/types/type.I100F28.html) constants.
macro_rules! I100F28 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I100F28) } }
#[macro_export]
/// Macro to create [I99F29](https://docs.rs/fixed/latest/fixed/types/type.I99F29.html) constants.
macro_rules! I99F29 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I99F29) } }
#[macro_export]
/// Macro to create [I98F30](https://docs.rs/fixed/latest/fixed/types/type.I98F30.html) constants.
macro_rules! I98F30 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I98F30) } }
#[macro_export]
/// Macro to create [I97F31](https://docs.rs/fixed/latest/fixed/types/type.I97F31.html) constants.
macro_rules! I97F31 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I97F31) } }
#[macro_export]
/// Macro to create [I96F32](https://docs.rs/fixed/latest/fixed/types/type.I96F32.html) constants.
macro_rules! I96F32 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I96F32) } }
#[macro_export]
/// Macro to create [I95F33](https://docs.rs/fixed/latest/fixed/types/type.I95F33.html) constants.
macro_rules! I95F33 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I95F33) } }
#[macro_export]
/// Macro to create [I94F34](https://docs.rs/fixed/latest/fixed/types/type.I94F34.html) constants.
macro_rules! I94F34 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I94F34) } }
#[macro_export]
/// Macro to create [I93F35](https://docs.rs/fixed/latest/fixed/types/type.I93F35.html) constants.
macro_rules! I93F35 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I93F35) } }
#[macro_export]
/// Macro to create [I92F36](https://docs.rs/fixed/latest/fixed/types/type.I92F36.html) constants.
macro_rules! I92F36 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I92F36) } }
#[macro_export]
/// Macro to create [I91F37](https://docs.rs/fixed/latest/fixed/types/type.I91F37.html) constants.
macro_rules! I91F37 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I91F37) } }
#[macro_export]
/// Macro to create [I90F38](https://docs.rs/fixed/latest/fixed/types/type.I90F38.html) constants.
macro_rules! I90F38 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I90F38) } }
#[macro_export]
/// Macro to create [I89F39](https://docs.rs/fixed/latest/fixed/types/type.I89F39.html) constants.
macro_rules! I89F39 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I89F39) } }
#[macro_export]
/// Macro to create [I88F40](https://docs.rs/fixed/latest/fixed/types/type.I88F40.html) constants.
macro_rules! I88F40 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I88F40) } }
#[macro_export]
/// Macro to create [I87F41](https://docs.rs/fixed/latest/fixed/types/type.I87F41.html) constants.
macro_rules! I87F41 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I87F41) } }
#[macro_export]
/// Macro to create [I86F42](https://docs.rs/fixed/latest/fixed/types/type.I86F42.html) constants.
macro_rules! I86F42 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I86F42) } }
#[macro_export]
/// Macro to create [I85F43](https://docs.rs/fixed/latest/fixed/types/type.I85F43.html) constants.
macro_rules! I85F43 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I85F43) } }
#[macro_export]
/// Macro to create [I84F44](https://docs.rs/fixed/latest/fixed/types/type.I84F44.html) constants.
macro_rules! I84F44 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I84F44) } }
#[macro_export]
/// Macro to create [I83F45](https://docs.rs/fixed/latest/fixed/types/type.I83F45.html) constants.
macro_rules! I83F45 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I83F45) } }
#[macro_export]
/// Macro to create [I82F46](https://docs.rs/fixed/latest/fixed/types/type.I82F46.html) constants.
macro_rules! I82F46 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I82F46) } }
#[macro_export]
/// Macro to create [I81F47](https://docs.rs/fixed/latest/fixed/types/type.I81F47.html) constants.
macro_rules! I81F47 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I81F47) } }
#[macro_export]
/// Macro to create [I80F48](https://docs.rs/fixed/latest/fixed/types/type.I80F48.html) constants.
macro_rules! I80F48 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I80F48) } }
#[macro_export]
/// Macro to create [I79F49](https://docs.rs/fixed/latest/fixed/types/type.I79F49.html) constants.
macro_rules! I79F49 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I79F49) } }
#[macro_export]
/// Macro to create [I78F50](https://docs.rs/fixed/latest/fixed/types/type.I78F50.html) constants.
macro_rules! I78F50 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I78F50) } }
#[macro_export]
/// Macro to create [I77F51](https://docs.rs/fixed/latest/fixed/types/type.I77F51.html) constants.
macro_rules! I77F51 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I77F51) } }
#[macro_export]
/// Macro to create [I76F52](https://docs.rs/fixed/latest/fixed/types/type.I76F52.html) constants.
macro_rules! I76F52 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I76F52) } }
#[macro_export]
/// Macro to create [I75F53](https://docs.rs/fixed/latest/fixed/types/type.I75F53.html) constants.
macro_rules! I75F53 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I75F53) } }
#[macro_export]
/// Macro to create [I74F54](https://docs.rs/fixed/latest/fixed/types/type.I74F54.html) constants.
macro_rules! I74F54 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I74F54) } }
#[macro_export]
/// Macro to create [I73F55](https://docs.rs/fixed/latest/fixed/types/type.I73F55.html) constants.
macro_rules! I73F55 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I73F55) } }
#[macro_export]
/// Macro to create [I72F56](https://docs.rs/fixed/latest/fixed/types/type.I72F56.html) constants.
macro_rules! I72F56 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I72F56) } }
#[macro_export]
/// Macro to create [I71F57](https://docs.rs/fixed/latest/fixed/types/type.I71F57.html) constants.
macro_rules! I71F57 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I71F57) } }
#[macro_export]
/// Macro to create [I70F58](https://docs.rs/fixed/latest/fixed/types/type.I70F58.html) constants.
macro_rules! I70F58 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I70F58) } }
#[macro_export]
/// Macro to create [I69F59](https://docs.rs/fixed/latest/fixed/types/type.I69F59.html) constants.
macro_rules! I69F59 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I69F59) } }
#[macro_export]
/// Macro to create [I68F60](https://docs.rs/fixed/latest/fixed/types/type.I68F60.html) constants.
macro_rules! I68F60 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I68F60) } }
#[macro_export]
/// Macro to create [I67F61](https://docs.rs/fixed/latest/fixed/types/type.I67F61.html) constants.
macro_rules! I67F61 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I67F61) } }
#[macro_export]
/// Macro to create [I66F62](https://docs.rs/fixed/latest/fixed/types/type.I66F62.html) constants.
macro_rules! I66F62 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I66F62) } }
#[macro_export]
/// Macro to create [I65F63](https://docs.rs/fixed/latest/fixed/types/type.I65F63.html) constants.
macro_rules! I65F63 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I65F63) } }
#[macro_export]
/// Macro to create [I64F64](https://docs.rs/fixed/latest/fixed/types/type.I64F64.html) constants.
macro_rules! I64F64 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I64F64) } }
#[macro_export]
/// Macro to create [I63F65](https://docs.rs/fixed/latest/fixed/types/type.I63F65.html) constants.
macro_rules! I63F65 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I63F65) } }
#[macro_export]
/// Macro to create [I62F66](https://docs.rs/fixed/latest/fixed/types/type.I62F66.html) constants.
macro_rules! I62F66 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I62F66) } }
#[macro_export]
/// Macro to create [I61F67](https://docs.rs/fixed/latest/fixed/types/type.I61F67.html) constants.
macro_rules! I61F67 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I61F67) } }
#[macro_export]
/// Macro to create [I60F68](https://docs.rs/fixed/latest/fixed/types/type.I60F68.html) constants.
macro_rules! I60F68 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I60F68) } }
#[macro_export]
/// Macro to create [I59F69](https://docs.rs/fixed/latest/fixed/types/type.I59F69.html) constants.
macro_rules! I59F69 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I59F69) } }
#[macro_export]
/// Macro to create [I58F70](https://docs.rs/fixed/latest/fixed/types/type.I58F70.html) constants.
macro_rules! I58F70 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I58F70) } }
#[macro_export]
/// Macro to create [I57F71](https://docs.rs/fixed/latest/fixed/types/type.I57F71.html) constants.
macro_rules! I57F71 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I57F71) } }
#[macro_export]
/// Macro to create [I56F72](https://docs.rs/fixed/latest/fixed/types/type.I56F72.html) constants.
macro_rules! I56F72 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I56F72) } }
#[macro_export]
/// Macro to create [I55F73](https://docs.rs/fixed/latest/fixed/types/type.I55F73.html) constants.
macro_rules! I55F73 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I55F73) } }
#[macro_export]
/// Macro to create [I54F74](https://docs.rs/fixed/latest/fixed/types/type.I54F74.html) constants.
macro_rules! I54F74 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I54F74) } }
#[macro_export]
/// Macro to create [I53F75](https://docs.rs/fixed/latest/fixed/types/type.I53F75.html) constants.
macro_rules! I53F75 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I53F75) } }
#[macro_export]
/// Macro to create [I52F76](https://docs.rs/fixed/latest/fixed/types/type.I52F76.html) constants.
macro_rules! I52F76 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I52F76) } }
#[macro_export]
/// Macro to create [I51F77](https://docs.rs/fixed/latest/fixed/types/type.I51F77.html) constants.
macro_rules! I51F77 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I51F77) } }
#[macro_export]
/// Macro to create [I50F78](https://docs.rs/fixed/latest/fixed/types/type.I50F78.html) constants.
macro_rules! I50F78 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I50F78) } }
#[macro_export]
/// Macro to create [I49F79](https://docs.rs/fixed/latest/fixed/types/type.I49F79.html) constants.
macro_rules! I49F79 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I49F79) } }
#[macro_export]
/// Macro to create [I48F80](https://docs.rs/fixed/latest/fixed/types/type.I48F80.html) constants.
macro_rules! I48F80 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I48F80) } }
#[macro_export]
/// Macro to create [I47F81](https://docs.rs/fixed/latest/fixed/types/type.I47F81.html) constants.
macro_rules! I47F81 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I47F81) } }
#[macro_export]
/// Macro to create [I46F82](https://docs.rs/fixed/latest/fixed/types/type.I46F82.html) constants.
macro_rules! I46F82 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I46F82) } }
#[macro_export]
/// Macro to create [I45F83](https://docs.rs/fixed/latest/fixed/types/type.I45F83.html) constants.
macro_rules! I45F83 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I45F83) } }
#[macro_export]
/// Macro to create [I44F84](https://docs.rs/fixed/latest/fixed/types/type.I44F84.html) constants.
macro_rules! I44F84 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I44F84) } }
#[macro_export]
/// Macro to create [I43F85](https://docs.rs/fixed/latest/fixed/types/type.I43F85.html) constants.
macro_rules! I43F85 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I43F85) } }
#[macro_export]
/// Macro to create [I42F86](https://docs.rs/fixed/latest/fixed/types/type.I42F86.html) constants.
macro_rules! I42F86 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I42F86) } }
#[macro_export]
/// Macro to create [I41F87](https://docs.rs/fixed/latest/fixed/types/type.I41F87.html) constants.
macro_rules! I41F87 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I41F87) } }
#[macro_export]
/// Macro to create [I40F88](https://docs.rs/fixed/latest/fixed/types/type.I40F88.html) constants.
macro_rules! I40F88 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I40F88) } }
#[macro_export]
/// Macro to create [I39F89](https://docs.rs/fixed/latest/fixed/types/type.I39F89.html) constants.
macro_rules! I39F89 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I39F89) } }
#[macro_export]
/// Macro to create [I38F90](https://docs.rs/fixed/latest/fixed/types/type.I38F90.html) constants.
macro_rules! I38F90 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I38F90) } }
#[macro_export]
/// Macro to create [I37F91](https://docs.rs/fixed/latest/fixed/types/type.I37F91.html) constants.
macro_rules! I37F91 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I37F91) } }
#[macro_export]
/// Macro to create [I36F92](https://docs.rs/fixed/latest/fixed/types/type.I36F92.html) constants.
macro_rules! I36F92 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I36F92) } }
#[macro_export]
/// Macro to create [I35F93](https://docs.rs/fixed/latest/fixed/types/type.I35F93.html) constants.
macro_rules! I35F93 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I35F93) } }
#[macro_export]
/// Macro to create [I34F94](https://docs.rs/fixed/latest/fixed/types/type.I34F94.html) constants.
macro_rules! I34F94 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I34F94) } }
#[macro_export]
/// Macro to create [I33F95](https://docs.rs/fixed/latest/fixed/types/type.I33F95.html) constants.
macro_rules! I33F95 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I33F95) } }
#[macro_export]
/// Macro to create [I32F96](https://docs.rs/fixed/latest/fixed/types/type.I32F96.html) constants.
macro_rules! I32F96 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I32F96) } }
#[macro_export]
/// Macro to create [I31F97](https://docs.rs/fixed/latest/fixed/types/type.I31F97.html) constants.
macro_rules! I31F97 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I31F97) } }
#[macro_export]
/// Macro to create [I30F98](https://docs.rs/fixed/latest/fixed/types/type.I30F98.html) constants.
macro_rules! I30F98 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I30F98) } }
#[macro_export]
/// Macro to create [I29F99](https://docs.rs/fixed/latest/fixed/types/type.I29F99.html) constants.
macro_rules! I29F99 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I29F99) } }
#[macro_export]
/// Macro to create [I28F100](https://docs.rs/fixed/latest/fixed/types/type.I28F100.html) constants.
macro_rules! I28F100 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I28F100) } }
#[macro_export]
/// Macro to create [I27F101](https://docs.rs/fixed/latest/fixed/types/type.I27F101.html) constants.
macro_rules! I27F101 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I27F101) } }
#[macro_export]
/// Macro to create [I26F102](https://docs.rs/fixed/latest/fixed/types/type.I26F102.html) constants.
macro_rules! I26F102 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I26F102) } }
#[macro_export]
/// Macro to create [I25F103](https://docs.rs/fixed/latest/fixed/types/type.I25F103.html) constants.
macro_rules! I25F103 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I25F103) } }
#[macro_export]
/// Macro to create [I24F104](https://docs.rs/fixed/latest/fixed/types/type.I24F104.html) constants.
macro_rules! I24F104 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I24F104) } }
#[macro_export]
/// Macro to create [I23F105](https://docs.rs/fixed/latest/fixed/types/type.I23F105.html) constants.
macro_rules! I23F105 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I23F105) } }
#[macro_export]
/// Macro to create [I22F106](https://docs.rs/fixed/latest/fixed/types/type.I22F106.html) constants.
macro_rules! I22F106 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I22F106) } }
#[macro_export]
/// Macro to create [I21F107](https://docs.rs/fixed/latest/fixed/types/type.I21F107.html) constants.
macro_rules! I21F107 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I21F107) } }
#[macro_export]
/// Macro to create [I20F108](https://docs.rs/fixed/latest/fixed/types/type.I20F108.html) constants.
macro_rules! I20F108 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I20F108) } }
#[macro_export]
/// Macro to create [I19F109](https://docs.rs/fixed/latest/fixed/types/type.I19F109.html) constants.
macro_rules! I19F109 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I19F109) } }
#[macro_export]
/// Macro to create [I18F110](https://docs.rs/fixed/latest/fixed/types/type.I18F110.html) constants.
macro_rules! I18F110 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I18F110) } }
#[macro_export]
/// Macro to create [I17F111](https://docs.rs/fixed/latest/fixed/types/type.I17F111.html) constants.
macro_rules! I17F111 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I17F111) } }
#[macro_export]
/// Macro to create [I16F112](https://docs.rs/fixed/latest/fixed/types/type.I16F112.html) constants.
macro_rules! I16F112 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I16F112) } }
#[macro_export]
/// Macro to create [I15F113](https://docs.rs/fixed/latest/fixed/types/type.I15F113.html) constants.
macro_rules! I15F113 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I15F113) } }
#[macro_export]
/// Macro to create [I14F114](https://docs.rs/fixed/latest/fixed/types/type.I14F114.html) constants.
macro_rules! I14F114 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I14F114) } }
#[macro_export]
/// Macro to create [I13F115](https://docs.rs/fixed/latest/fixed/types/type.I13F115.html) constants.
macro_rules! I13F115 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I13F115) } }
#[macro_export]
/// Macro to create [I12F116](https://docs.rs/fixed/latest/fixed/types/type.I12F116.html) constants.
macro_rules! I12F116 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I12F116) } }
#[macro_export]
/// Macro to create [I11F117](https://docs.rs/fixed/latest/fixed/types/type.I11F117.html) constants.
macro_rules! I11F117 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I11F117) } }
#[macro_export]
/// Macro to create [I10F118](https://docs.rs/fixed/latest/fixed/types/type.I10F118.html) constants.
macro_rules! I10F118 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I10F118) } }
#[macro_export]
/// Macro to create [I9F119](https://docs.rs/fixed/latest/fixed/types/type.I9F119.html) constants.
macro_rules! I9F119 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I9F119) } }
#[macro_export]
/// Macro to create [I8F120](https://docs.rs/fixed/latest/fixed/types/type.I8F120.html) constants.
macro_rules! I8F120 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I8F120) } }
#[macro_export]
/// Macro to create [I7F121](https://docs.rs/fixed/latest/fixed/types/type.I7F121.html) constants.
macro_rules! I7F121 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I7F121) } }
#[macro_export]
/// Macro to create [I6F122](https://docs.rs/fixed/latest/fixed/types/type.I6F122.html) constants.
macro_rules! I6F122 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I6F122) } }
#[macro_export]
/// Macro to create [I5F123](https://docs.rs/fixed/latest/fixed/types/type.I5F123.html) constants.
macro_rules! I5F123 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I5F123) } }
#[macro_export]
/// Macro to create [I4F124](https://docs.rs/fixed/latest/fixed/types/type.I4F124.html) constants.
macro_rules! I4F124 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I4F124) } }
#[macro_export]
/// Macro to create [I3F125](https://docs.rs/fixed/latest/fixed/types/type.I3F125.html) constants.
macro_rules! I3F125 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I3F125) } }
#[macro_export]
/// Macro to create [I2F126](https://docs.rs/fixed/latest/fixed/types/type.I2F126.html) constants.
macro_rules! I2F126 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I2F126) } }
#[macro_export]
/// Macro to create [I1F127](https://docs.rs/fixed/latest/fixed/types/type.I1F127.html) constants.
macro_rules! I1F127 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I1F127) } }
#[macro_export]
/// Macro to create [I0F128](https://docs.rs/fixed/latest/fixed/types/type.I0F128.html) constants.
macro_rules! I0F128 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: I0F128) } }

// 8-bit unsigned
#[macro_export]
/// Macro to create [U8F0](https://docs.rs/fixed/latest/fixed/types/type.U8F0.html) constants.
macro_rules! U8F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U8F0) } }
#[macro_export]
/// Macro to create [U7F1](https://docs.rs/fixed/latest/fixed/types/type.U7F1.html) constants.
macro_rules! U7F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U7F1) } }
#[macro_export]
/// Macro to create [U6F2](https://docs.rs/fixed/latest/fixed/types/type.U6F2.html) constants.
macro_rules! U6F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U6F2) } }
#[macro_export]
/// Macro to create [U5F3](https://docs.rs/fixed/latest/fixed/types/type.U5F3.html) constants.
macro_rules! U5F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U5F3) } }
#[macro_export]
/// Macro to create [U4F4](https://docs.rs/fixed/latest/fixed/types/type.U4F4.html) constants.
macro_rules! U4F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U4F4) } }
#[macro_export]
/// Macro to create [U3F5](https://docs.rs/fixed/latest/fixed/types/type.U3F5.html) constants.
macro_rules! U3F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U3F5) } }
#[macro_export]
/// Macro to create [U2F6](https://docs.rs/fixed/latest/fixed/types/type.U2F6.html) constants.
macro_rules! U2F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U2F6) } }
#[macro_export]
/// Macro to create [U1F7](https://docs.rs/fixed/latest/fixed/types/type.U1F7.html) constants.
macro_rules! U1F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U1F7) } }
#[macro_export]
/// Macro to create [U0F8](https://docs.rs/fixed/latest/fixed/types/type.U0F8.html) constants.
macro_rules! U0F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U0F8) } }

// 16-bit unsigned
#[macro_export]
/// Macro to create [U16F0](https://docs.rs/fixed/latest/fixed/types/type.U16F0.html) constants.
macro_rules! U16F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U16F0) } }
#[macro_export]
/// Macro to create [U15F1](https://docs.rs/fixed/latest/fixed/types/type.U15F1.html) constants.
macro_rules! U15F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U15F1) } }
#[macro_export]
/// Macro to create [U14F2](https://docs.rs/fixed/latest/fixed/types/type.U14F2.html) constants.
macro_rules! U14F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U14F2) } }
#[macro_export]
/// Macro to create [U13F3](https://docs.rs/fixed/latest/fixed/types/type.U13F3.html) constants.
macro_rules! U13F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U13F3) } }
#[macro_export]
/// Macro to create [U12F4](https://docs.rs/fixed/latest/fixed/types/type.U12F4.html) constants.
macro_rules! U12F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U12F4) } }
#[macro_export]
/// Macro to create [U11F5](https://docs.rs/fixed/latest/fixed/types/type.U11F5.html) constants.
macro_rules! U11F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U11F5) } }
#[macro_export]
/// Macro to create [U10F6](https://docs.rs/fixed/latest/fixed/types/type.U10F6.html) constants.
macro_rules! U10F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U10F6) } }
#[macro_export]
/// Macro to create [U9F7](https://docs.rs/fixed/latest/fixed/types/type.U9F7.html) constants.
macro_rules! U9F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U9F7) } }
#[macro_export]
/// Macro to create [U8F8](https://docs.rs/fixed/latest/fixed/types/type.U8F8.html) constants.
macro_rules! U8F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U8F8) } }
#[macro_export]
/// Macro to create [U7F9](https://docs.rs/fixed/latest/fixed/types/type.U7F9.html) constants.
macro_rules! U7F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U7F9) } }
#[macro_export]
/// Macro to create [U6F10](https://docs.rs/fixed/latest/fixed/types/type.U6F10.html) constants.
macro_rules! U6F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U6F10) } }
#[macro_export]
/// Macro to create [U5F11](https://docs.rs/fixed/latest/fixed/types/type.U5F11.html) constants.
macro_rules! U5F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U5F11) } }
#[macro_export]
/// Macro to create [U4F12](https://docs.rs/fixed/latest/fixed/types/type.U4F12.html) constants.
macro_rules! U4F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U4F12) } }
#[macro_export]
/// Macro to create [U3F13](https://docs.rs/fixed/latest/fixed/types/type.U3F13.html) constants.
macro_rules! U3F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U3F13) } }
#[macro_export]
/// Macro to create [U2F14](https://docs.rs/fixed/latest/fixed/types/type.U2F14.html) constants.
macro_rules! U2F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U2F14) } }
#[macro_export]
/// Macro to create [U1F15](https://docs.rs/fixed/latest/fixed/types/type.U1F15.html) constants.
macro_rules! U1F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U1F15) } }
#[macro_export]
/// Macro to create [U0F16](https://docs.rs/fixed/latest/fixed/types/type.U0F16.html) constants.
macro_rules! U0F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U0F16) } }

// 32-bit unsigned
#[macro_export]
/// Macro to create [U32F0](https://docs.rs/fixed/latest/fixed/types/type.U32F0.html) constants.
macro_rules! U32F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U32F0) } }
#[macro_export]
/// Macro to create [U31F1](https://docs.rs/fixed/latest/fixed/types/type.U31F1.html) constants.
macro_rules! U31F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U31F1) } }
#[macro_export]
/// Macro to create [U30F2](https://docs.rs/fixed/latest/fixed/types/type.U30F2.html) constants.
macro_rules! U30F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U30F2) } }
#[macro_export]
/// Macro to create [U29F3](https://docs.rs/fixed/latest/fixed/types/type.U29F3.html) constants.
macro_rules! U29F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U29F3) } }
#[macro_export]
/// Macro to create [U28F4](https://docs.rs/fixed/latest/fixed/types/type.U28F4.html) constants.
macro_rules! U28F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U28F4) } }
#[macro_export]
/// Macro to create [U27F5](https://docs.rs/fixed/latest/fixed/types/type.U27F5.html) constants.
macro_rules! U27F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U27F5) } }
#[macro_export]
/// Macro to create [U26F6](https://docs.rs/fixed/latest/fixed/types/type.U26F6.html) constants.
macro_rules! U26F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U26F6) } }
#[macro_export]
/// Macro to create [U25F7](https://docs.rs/fixed/latest/fixed/types/type.U25F7.html) constants.
macro_rules! U25F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U25F7) } }
#[macro_export]
/// Macro to create [U24F8](https://docs.rs/fixed/latest/fixed/types/type.U24F8.html) constants.
macro_rules! U24F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U24F8) } }
#[macro_export]
/// Macro to create [U23F9](https://docs.rs/fixed/latest/fixed/types/type.U23F9.html) constants.
macro_rules! U23F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U23F9) } }
#[macro_export]
/// Macro to create [U22F10](https://docs.rs/fixed/latest/fixed/types/type.U22F10.html) constants.
macro_rules! U22F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U22F10) } }
#[macro_export]
/// Macro to create [U21F11](https://docs.rs/fixed/latest/fixed/types/type.U21F11.html) constants.
macro_rules! U21F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U21F11) } }
#[macro_export]
/// Macro to create [U20F12](https://docs.rs/fixed/latest/fixed/types/type.U20F12.html) constants.
macro_rules! U20F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U20F12) } }
#[macro_export]
/// Macro to create [U19F13](https://docs.rs/fixed/latest/fixed/types/type.U19F13.html) constants.
macro_rules! U19F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U19F13) } }
#[macro_export]
/// Macro to create [U18F14](https://docs.rs/fixed/latest/fixed/types/type.U18F14.html) constants.
macro_rules! U18F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U18F14) } }
#[macro_export]
/// Macro to create [U17F15](https://docs.rs/fixed/latest/fixed/types/type.U17F15.html) constants.
macro_rules! U17F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U17F15) } }
#[macro_export]
/// Macro to create [U16F16](https://docs.rs/fixed/latest/fixed/types/type.U16F16.html) constants.
macro_rules! U16F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U16F16) } }
#[macro_export]
/// Macro to create [U15F17](https://docs.rs/fixed/latest/fixed/types/type.U15F17.html) constants.
macro_rules! U15F17 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U15F17) } }
#[macro_export]
/// Macro to create [U14F18](https://docs.rs/fixed/latest/fixed/types/type.U14F18.html) constants.
macro_rules! U14F18 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U14F18) } }
#[macro_export]
/// Macro to create [U13F19](https://docs.rs/fixed/latest/fixed/types/type.U13F19.html) constants.
macro_rules! U13F19 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U13F19) } }
#[macro_export]
/// Macro to create [U12F20](https://docs.rs/fixed/latest/fixed/types/type.U12F20.html) constants.
macro_rules! U12F20 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U12F20) } }
#[macro_export]
/// Macro to create [U11F21](https://docs.rs/fixed/latest/fixed/types/type.U11F21.html) constants.
macro_rules! U11F21 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U11F21) } }
#[macro_export]
/// Macro to create [U10F22](https://docs.rs/fixed/latest/fixed/types/type.U10F22.html) constants.
macro_rules! U10F22 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U10F22) } }
#[macro_export]
/// Macro to create [U9F23](https://docs.rs/fixed/latest/fixed/types/type.U9F23.html) constants.
macro_rules! U9F23 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U9F23) } }
#[macro_export]
/// Macro to create [U8F24](https://docs.rs/fixed/latest/fixed/types/type.U8F24.html) constants.
macro_rules! U8F24 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U8F24) } }
#[macro_export]
/// Macro to create [U7F25](https://docs.rs/fixed/latest/fixed/types/type.U7F25.html) constants.
macro_rules! U7F25 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U7F25) } }
#[macro_export]
/// Macro to create [U6F26](https://docs.rs/fixed/latest/fixed/types/type.U6F26.html) constants.
macro_rules! U6F26 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U6F26) } }
#[macro_export]
/// Macro to create [U5F27](https://docs.rs/fixed/latest/fixed/types/type.U5F27.html) constants.
macro_rules! U5F27 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U5F27) } }
#[macro_export]
/// Macro to create [U4F28](https://docs.rs/fixed/latest/fixed/types/type.U4F28.html) constants.
macro_rules! U4F28 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U4F28) } }
#[macro_export]
/// Macro to create [U3F29](https://docs.rs/fixed/latest/fixed/types/type.U3F29.html) constants.
macro_rules! U3F29 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U3F29) } }
#[macro_export]
/// Macro to create [U2F30](https://docs.rs/fixed/latest/fixed/types/type.U2F30.html) constants.
macro_rules! U2F30 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U2F30) } }
#[macro_export]
/// Macro to create [U1F31](https://docs.rs/fixed/latest/fixed/types/type.U1F31.html) constants.
macro_rules! U1F31 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U1F31) } }
#[macro_export]
/// Macro to create [U0F32](https://docs.rs/fixed/latest/fixed/types/type.U0F32.html) constants.
macro_rules! U0F32 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U0F32) } }

// 64-bit unsigned
#[macro_export]
/// Macro to create [U64F0](https://docs.rs/fixed/latest/fixed/types/type.U64F0.html) constants.
macro_rules! U64F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U64F0) } }
#[macro_export]
/// Macro to create [U63F1](https://docs.rs/fixed/latest/fixed/types/type.U63F1.html) constants.
macro_rules! U63F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U63F1) } }
#[macro_export]
/// Macro to create [U62F2](https://docs.rs/fixed/latest/fixed/types/type.U62F2.html) constants.
macro_rules! U62F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U62F2) } }
#[macro_export]
/// Macro to create [U61F3](https://docs.rs/fixed/latest/fixed/types/type.U61F3.html) constants.
macro_rules! U61F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U61F3) } }
#[macro_export]
/// Macro to create [U60F4](https://docs.rs/fixed/latest/fixed/types/type.U60F4.html) constants.
macro_rules! U60F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U60F4) } }
#[macro_export]
/// Macro to create [U59F5](https://docs.rs/fixed/latest/fixed/types/type.U59F5.html) constants.
macro_rules! U59F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U59F5) } }
#[macro_export]
/// Macro to create [U58F6](https://docs.rs/fixed/latest/fixed/types/type.U58F6.html) constants.
macro_rules! U58F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U58F6) } }
#[macro_export]
/// Macro to create [U57F7](https://docs.rs/fixed/latest/fixed/types/type.U57F7.html) constants.
macro_rules! U57F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U57F7) } }
#[macro_export]
/// Macro to create [U56F8](https://docs.rs/fixed/latest/fixed/types/type.U56F8.html) constants.
macro_rules! U56F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U56F8) } }
#[macro_export]
/// Macro to create [U55F9](https://docs.rs/fixed/latest/fixed/types/type.U55F9.html) constants.
macro_rules! U55F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U55F9) } }
#[macro_export]
/// Macro to create [U54F10](https://docs.rs/fixed/latest/fixed/types/type.U54F10.html) constants.
macro_rules! U54F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U54F10) } }
#[macro_export]
/// Macro to create [U53F11](https://docs.rs/fixed/latest/fixed/types/type.U53F11.html) constants.
macro_rules! U53F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U53F11) } }
#[macro_export]
/// Macro to create [U52F12](https://docs.rs/fixed/latest/fixed/types/type.U52F12.html) constants.
macro_rules! U52F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U52F12) } }
#[macro_export]
/// Macro to create [U51F13](https://docs.rs/fixed/latest/fixed/types/type.U51F13.html) constants.
macro_rules! U51F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U51F13) } }
#[macro_export]
/// Macro to create [U50F14](https://docs.rs/fixed/latest/fixed/types/type.U50F14.html) constants.
macro_rules! U50F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U50F14) } }
#[macro_export]
/// Macro to create [U49F15](https://docs.rs/fixed/latest/fixed/types/type.U49F15.html) constants.
macro_rules! U49F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U49F15) } }
#[macro_export]
/// Macro to create [U48F16](https://docs.rs/fixed/latest/fixed/types/type.U48F16.html) constants.
macro_rules! U48F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U48F16) } }
#[macro_export]
/// Macro to create [U47F17](https://docs.rs/fixed/latest/fixed/types/type.U47F17.html) constants.
macro_rules! U47F17 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U47F17) } }
#[macro_export]
/// Macro to create [U46F18](https://docs.rs/fixed/latest/fixed/types/type.U46F18.html) constants.
macro_rules! U46F18 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U46F18) } }
#[macro_export]
/// Macro to create [U45F19](https://docs.rs/fixed/latest/fixed/types/type.U45F19.html) constants.
macro_rules! U45F19 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U45F19) } }
#[macro_export]
/// Macro to create [U44F20](https://docs.rs/fixed/latest/fixed/types/type.U44F20.html) constants.
macro_rules! U44F20 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U44F20) } }
#[macro_export]
/// Macro to create [U43F21](https://docs.rs/fixed/latest/fixed/types/type.U43F21.html) constants.
macro_rules! U43F21 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U43F21) } }
#[macro_export]
/// Macro to create [U42F22](https://docs.rs/fixed/latest/fixed/types/type.U42F22.html) constants.
macro_rules! U42F22 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U42F22) } }
#[macro_export]
/// Macro to create [U41F23](https://docs.rs/fixed/latest/fixed/types/type.U41F23.html) constants.
macro_rules! U41F23 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U41F23) } }
#[macro_export]
/// Macro to create [U40F24](https://docs.rs/fixed/latest/fixed/types/type.U40F24.html) constants.
macro_rules! U40F24 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U40F24) } }
#[macro_export]
/// Macro to create [U39F25](https://docs.rs/fixed/latest/fixed/types/type.U39F25.html) constants.
macro_rules! U39F25 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U39F25) } }
#[macro_export]
/// Macro to create [U38F26](https://docs.rs/fixed/latest/fixed/types/type.U38F26.html) constants.
macro_rules! U38F26 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U38F26) } }
#[macro_export]
/// Macro to create [U37F27](https://docs.rs/fixed/latest/fixed/types/type.U37F27.html) constants.
macro_rules! U37F27 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U37F27) } }
#[macro_export]
/// Macro to create [U36F28](https://docs.rs/fixed/latest/fixed/types/type.U36F28.html) constants.
macro_rules! U36F28 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U36F28) } }
#[macro_export]
/// Macro to create [U35F29](https://docs.rs/fixed/latest/fixed/types/type.U35F29.html) constants.
macro_rules! U35F29 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U35F29) } }
#[macro_export]
/// Macro to create [U34F30](https://docs.rs/fixed/latest/fixed/types/type.U34F30.html) constants.
macro_rules! U34F30 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U34F30) } }
#[macro_export]
/// Macro to create [U33F31](https://docs.rs/fixed/latest/fixed/types/type.U33F31.html) constants.
macro_rules! U33F31 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U33F31) } }
#[macro_export]
/// Macro to create [U32F32](https://docs.rs/fixed/latest/fixed/types/type.U32F32.html) constants.
macro_rules! U32F32 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U32F32) } }
#[macro_export]
/// Macro to create [U31F33](https://docs.rs/fixed/latest/fixed/types/type.U31F33.html) constants.
macro_rules! U31F33 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U31F33) } }
#[macro_export]
/// Macro to create [U30F34](https://docs.rs/fixed/latest/fixed/types/type.U30F34.html) constants.
macro_rules! U30F34 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U30F34) } }
#[macro_export]
/// Macro to create [U29F35](https://docs.rs/fixed/latest/fixed/types/type.U29F35.html) constants.
macro_rules! U29F35 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U29F35) } }
#[macro_export]
/// Macro to create [U28F36](https://docs.rs/fixed/latest/fixed/types/type.U28F36.html) constants.
macro_rules! U28F36 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U28F36) } }
#[macro_export]
/// Macro to create [U27F37](https://docs.rs/fixed/latest/fixed/types/type.U27F37.html) constants.
macro_rules! U27F37 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U27F37) } }
#[macro_export]
/// Macro to create [U26F38](https://docs.rs/fixed/latest/fixed/types/type.U26F38.html) constants.
macro_rules! U26F38 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U26F38) } }
#[macro_export]
/// Macro to create [U25F39](https://docs.rs/fixed/latest/fixed/types/type.U25F39.html) constants.
macro_rules! U25F39 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U25F39) } }
#[macro_export]
/// Macro to create [U24F40](https://docs.rs/fixed/latest/fixed/types/type.U24F40.html) constants.
macro_rules! U24F40 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U24F40) } }
#[macro_export]
/// Macro to create [U23F41](https://docs.rs/fixed/latest/fixed/types/type.U23F41.html) constants.
macro_rules! U23F41 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U23F41) } }
#[macro_export]
/// Macro to create [U22F42](https://docs.rs/fixed/latest/fixed/types/type.U22F42.html) constants.
macro_rules! U22F42 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U22F42) } }
#[macro_export]
/// Macro to create [U21F43](https://docs.rs/fixed/latest/fixed/types/type.U21F43.html) constants.
macro_rules! U21F43 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U21F43) } }
#[macro_export]
/// Macro to create [U20F44](https://docs.rs/fixed/latest/fixed/types/type.U20F44.html) constants.
macro_rules! U20F44 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U20F44) } }
#[macro_export]
/// Macro to create [U19F45](https://docs.rs/fixed/latest/fixed/types/type.U19F45.html) constants.
macro_rules! U19F45 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U19F45) } }
#[macro_export]
/// Macro to create [U18F46](https://docs.rs/fixed/latest/fixed/types/type.U18F46.html) constants.
macro_rules! U18F46 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U18F46) } }
#[macro_export]
/// Macro to create [U17F47](https://docs.rs/fixed/latest/fixed/types/type.U17F47.html) constants.
macro_rules! U17F47 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U17F47) } }
#[macro_export]
/// Macro to create [U16F48](https://docs.rs/fixed/latest/fixed/types/type.U16F48.html) constants.
macro_rules! U16F48 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U16F48) } }
#[macro_export]
/// Macro to create [U15F49](https://docs.rs/fixed/latest/fixed/types/type.U15F49.html) constants.
macro_rules! U15F49 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U15F49) } }
#[macro_export]
/// Macro to create [U14F50](https://docs.rs/fixed/latest/fixed/types/type.U14F50.html) constants.
macro_rules! U14F50 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U14F50) } }
#[macro_export]
/// Macro to create [U13F51](https://docs.rs/fixed/latest/fixed/types/type.U13F51.html) constants.
macro_rules! U13F51 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U13F51) } }
#[macro_export]
/// Macro to create [U12F52](https://docs.rs/fixed/latest/fixed/types/type.U12F52.html) constants.
macro_rules! U12F52 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U12F52) } }
#[macro_export]
/// Macro to create [U11F53](https://docs.rs/fixed/latest/fixed/types/type.U11F53.html) constants.
macro_rules! U11F53 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U11F53) } }
#[macro_export]
/// Macro to create [U10F54](https://docs.rs/fixed/latest/fixed/types/type.U10F54.html) constants.
macro_rules! U10F54 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U10F54) } }
#[macro_export]
/// Macro to create [U9F55](https://docs.rs/fixed/latest/fixed/types/type.U9F55.html) constants.
macro_rules! U9F55 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U9F55) } }
#[macro_export]
/// Macro to create [U8F56](https://docs.rs/fixed/latest/fixed/types/type.U8F56.html) constants.
macro_rules! U8F56 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U8F56) } }
#[macro_export]
/// Macro to create [U7F57](https://docs.rs/fixed/latest/fixed/types/type.U7F57.html) constants.
macro_rules! U7F57 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U7F57) } }
#[macro_export]
/// Macro to create [U6F58](https://docs.rs/fixed/latest/fixed/types/type.U6F58.html) constants.
macro_rules! U6F58 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U6F58) } }
#[macro_export]
/// Macro to create [U5F59](https://docs.rs/fixed/latest/fixed/types/type.U5F59.html) constants.
macro_rules! U5F59 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U5F59) } }
#[macro_export]
/// Macro to create [U4F60](https://docs.rs/fixed/latest/fixed/types/type.U4F60.html) constants.
macro_rules! U4F60 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U4F60) } }
#[macro_export]
/// Macro to create [U3F61](https://docs.rs/fixed/latest/fixed/types/type.U3F61.html) constants.
macro_rules! U3F61 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U3F61) } }
#[macro_export]
/// Macro to create [U2F62](https://docs.rs/fixed/latest/fixed/types/type.U2F62.html) constants.
macro_rules! U2F62 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U2F62) } }
#[macro_export]
/// Macro to create [U1F63](https://docs.rs/fixed/latest/fixed/types/type.U1F63.html) constants.
macro_rules! U1F63 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U1F63) } }
#[macro_export]
/// Macro to create [U0F64](https://docs.rs/fixed/latest/fixed/types/type.U0F64.html) constants.
macro_rules! U0F64 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U0F64) } }

// 128-bit unsigned
#[macro_export]
/// Macro to create [U128F0](https://docs.rs/fixed/latest/fixed/types/type.U128F0.html) constants.
macro_rules! U128F0 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U128F0) } }
#[macro_export]
/// Macro to create [U127F1](https://docs.rs/fixed/latest/fixed/types/type.U127F1.html) constants.
macro_rules! U127F1 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U127F1) } }
#[macro_export]
/// Macro to create [U126F2](https://docs.rs/fixed/latest/fixed/types/type.U126F2.html) constants.
macro_rules! U126F2 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U126F2) } }
#[macro_export]
/// Macro to create [U125F3](https://docs.rs/fixed/latest/fixed/types/type.U125F3.html) constants.
macro_rules! U125F3 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U125F3) } }
#[macro_export]
/// Macro to create [U124F4](https://docs.rs/fixed/latest/fixed/types/type.U124F4.html) constants.
macro_rules! U124F4 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U124F4) } }
#[macro_export]
/// Macro to create [U123F5](https://docs.rs/fixed/latest/fixed/types/type.U123F5.html) constants.
macro_rules! U123F5 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U123F5) } }
#[macro_export]
/// Macro to create [U122F6](https://docs.rs/fixed/latest/fixed/types/type.U122F6.html) constants.
macro_rules! U122F6 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U122F6) } }
#[macro_export]
/// Macro to create [U121F7](https://docs.rs/fixed/latest/fixed/types/type.U121F7.html) constants.
macro_rules! U121F7 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U121F7) } }
#[macro_export]
/// Macro to create [U120F8](https://docs.rs/fixed/latest/fixed/types/type.U120F8.html) constants.
macro_rules! U120F8 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U120F8) } }
#[macro_export]
/// Macro to create [U119F9](https://docs.rs/fixed/latest/fixed/types/type.U119F9.html) constants.
macro_rules! U119F9 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U119F9) } }
#[macro_export]
/// Macro to create [U118F10](https://docs.rs/fixed/latest/fixed/types/type.U118F10.html) constants.
macro_rules! U118F10 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U118F10) } }
#[macro_export]
/// Macro to create [U117F11](https://docs.rs/fixed/latest/fixed/types/type.U117F11.html) constants.
macro_rules! U117F11 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U117F11) } }
#[macro_export]
/// Macro to create [U116F12](https://docs.rs/fixed/latest/fixed/types/type.U116F12.html) constants.
macro_rules! U116F12 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U116F12) } }
#[macro_export]
/// Macro to create [U115F13](https://docs.rs/fixed/latest/fixed/types/type.U115F13.html) constants.
macro_rules! U115F13 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U115F13) } }
#[macro_export]
/// Macro to create [U114F14](https://docs.rs/fixed/latest/fixed/types/type.U114F14.html) constants.
macro_rules! U114F14 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U114F14) } }
#[macro_export]
/// Macro to create [U113F15](https://docs.rs/fixed/latest/fixed/types/type.U113F15.html) constants.
macro_rules! U113F15 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U113F15) } }
#[macro_export]
/// Macro to create [U112F16](https://docs.rs/fixed/latest/fixed/types/type.U112F16.html) constants.
macro_rules! U112F16 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U112F16) } }
#[macro_export]
/// Macro to create [U111F17](https://docs.rs/fixed/latest/fixed/types/type.U111F17.html) constants.
macro_rules! U111F17 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U111F17) } }
#[macro_export]
/// Macro to create [U110F18](https://docs.rs/fixed/latest/fixed/types/type.U110F18.html) constants.
macro_rules! U110F18 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U110F18) } }
#[macro_export]
/// Macro to create [U109F19](https://docs.rs/fixed/latest/fixed/types/type.U109F19.html) constants.
macro_rules! U109F19 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U109F19) } }
#[macro_export]
/// Macro to create [U108F20](https://docs.rs/fixed/latest/fixed/types/type.U108F20.html) constants.
macro_rules! U108F20 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U108F20) } }
#[macro_export]
/// Macro to create [U107F21](https://docs.rs/fixed/latest/fixed/types/type.U107F21.html) constants.
macro_rules! U107F21 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U107F21) } }
#[macro_export]
/// Macro to create [U106F22](https://docs.rs/fixed/latest/fixed/types/type.U106F22.html) constants.
macro_rules! U106F22 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U106F22) } }
#[macro_export]
/// Macro to create [U105F23](https://docs.rs/fixed/latest/fixed/types/type.U105F23.html) constants.
macro_rules! U105F23 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U105F23) } }
#[macro_export]
/// Macro to create [U104F24](https://docs.rs/fixed/latest/fixed/types/type.U104F24.html) constants.
macro_rules! U104F24 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U104F24) } }
#[macro_export]
/// Macro to create [U103F25](https://docs.rs/fixed/latest/fixed/types/type.U103F25.html) constants.
macro_rules! U103F25 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U103F25) } }
#[macro_export]
/// Macro to create [U102F26](https://docs.rs/fixed/latest/fixed/types/type.U102F26.html) constants.
macro_rules! U102F26 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U102F26) } }
#[macro_export]
/// Macro to create [U101F27](https://docs.rs/fixed/latest/fixed/types/type.U101F27.html) constants.
macro_rules! U101F27 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U101F27) } }
#[macro_export]
/// Macro to create [U100F28](https://docs.rs/fixed/latest/fixed/types/type.U100F28.html) constants.
macro_rules! U100F28 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U100F28) } }
#[macro_export]
/// Macro to create [U99F29](https://docs.rs/fixed/latest/fixed/types/type.U99F29.html) constants.
macro_rules! U99F29 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U99F29) } }
#[macro_export]
/// Macro to create [U98F30](https://docs.rs/fixed/latest/fixed/types/type.U98F30.html) constants.
macro_rules! U98F30 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U98F30) } }
#[macro_export]
/// Macro to create [U97F31](https://docs.rs/fixed/latest/fixed/types/type.U97F31.html) constants.
macro_rules! U97F31 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U97F31) } }
#[macro_export]
/// Macro to create [U96F32](https://docs.rs/fixed/latest/fixed/types/type.U96F32.html) constants.
macro_rules! U96F32 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U96F32) } }
#[macro_export]
/// Macro to create [U95F33](https://docs.rs/fixed/latest/fixed/types/type.U95F33.html) constants.
macro_rules! U95F33 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U95F33) } }
#[macro_export]
/// Macro to create [U94F34](https://docs.rs/fixed/latest/fixed/types/type.U94F34.html) constants.
macro_rules! U94F34 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U94F34) } }
#[macro_export]
/// Macro to create [U93F35](https://docs.rs/fixed/latest/fixed/types/type.U93F35.html) constants.
macro_rules! U93F35 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U93F35) } }
#[macro_export]
/// Macro to create [U92F36](https://docs.rs/fixed/latest/fixed/types/type.U92F36.html) constants.
macro_rules! U92F36 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U92F36) } }
#[macro_export]
/// Macro to create [U91F37](https://docs.rs/fixed/latest/fixed/types/type.U91F37.html) constants.
macro_rules! U91F37 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U91F37) } }
#[macro_export]
/// Macro to create [U90F38](https://docs.rs/fixed/latest/fixed/types/type.U90F38.html) constants.
macro_rules! U90F38 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U90F38) } }
#[macro_export]
/// Macro to create [U89F39](https://docs.rs/fixed/latest/fixed/types/type.U89F39.html) constants.
macro_rules! U89F39 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U89F39) } }
#[macro_export]
/// Macro to create [U88F40](https://docs.rs/fixed/latest/fixed/types/type.U88F40.html) constants.
macro_rules! U88F40 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U88F40) } }
#[macro_export]
/// Macro to create [U87F41](https://docs.rs/fixed/latest/fixed/types/type.U87F41.html) constants.
macro_rules! U87F41 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U87F41) } }
#[macro_export]
/// Macro to create [U86F42](https://docs.rs/fixed/latest/fixed/types/type.U86F42.html) constants.
macro_rules! U86F42 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U86F42) } }
#[macro_export]
/// Macro to create [U85F43](https://docs.rs/fixed/latest/fixed/types/type.U85F43.html) constants.
macro_rules! U85F43 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U85F43) } }
#[macro_export]
/// Macro to create [U84F44](https://docs.rs/fixed/latest/fixed/types/type.U84F44.html) constants.
macro_rules! U84F44 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U84F44) } }
#[macro_export]
/// Macro to create [U83F45](https://docs.rs/fixed/latest/fixed/types/type.U83F45.html) constants.
macro_rules! U83F45 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U83F45) } }
#[macro_export]
/// Macro to create [U82F46](https://docs.rs/fixed/latest/fixed/types/type.U82F46.html) constants.
macro_rules! U82F46 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U82F46) } }
#[macro_export]
/// Macro to create [U81F47](https://docs.rs/fixed/latest/fixed/types/type.U81F47.html) constants.
macro_rules! U81F47 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U81F47) } }
#[macro_export]
/// Macro to create [U80F48](https://docs.rs/fixed/latest/fixed/types/type.U80F48.html) constants.
macro_rules! U80F48 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U80F48) } }
#[macro_export]
/// Macro to create [U79F49](https://docs.rs/fixed/latest/fixed/types/type.U79F49.html) constants.
macro_rules! U79F49 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U79F49) } }
#[macro_export]
/// Macro to create [U78F50](https://docs.rs/fixed/latest/fixed/types/type.U78F50.html) constants.
macro_rules! U78F50 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U78F50) } }
#[macro_export]
/// Macro to create [U77F51](https://docs.rs/fixed/latest/fixed/types/type.U77F51.html) constants.
macro_rules! U77F51 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U77F51) } }
#[macro_export]
/// Macro to create [U76F52](https://docs.rs/fixed/latest/fixed/types/type.U76F52.html) constants.
macro_rules! U76F52 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U76F52) } }
#[macro_export]
/// Macro to create [U75F53](https://docs.rs/fixed/latest/fixed/types/type.U75F53.html) constants.
macro_rules! U75F53 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U75F53) } }
#[macro_export]
/// Macro to create [U74F54](https://docs.rs/fixed/latest/fixed/types/type.U74F54.html) constants.
macro_rules! U74F54 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U74F54) } }
#[macro_export]
/// Macro to create [U73F55](https://docs.rs/fixed/latest/fixed/types/type.U73F55.html) constants.
macro_rules! U73F55 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U73F55) } }
#[macro_export]
/// Macro to create [U72F56](https://docs.rs/fixed/latest/fixed/types/type.U72F56.html) constants.
macro_rules! U72F56 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U72F56) } }
#[macro_export]
/// Macro to create [U71F57](https://docs.rs/fixed/latest/fixed/types/type.U71F57.html) constants.
macro_rules! U71F57 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U71F57) } }
#[macro_export]
/// Macro to create [U70F58](https://docs.rs/fixed/latest/fixed/types/type.U70F58.html) constants.
macro_rules! U70F58 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U70F58) } }
#[macro_export]
/// Macro to create [U69F59](https://docs.rs/fixed/latest/fixed/types/type.U69F59.html) constants.
macro_rules! U69F59 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U69F59) } }
#[macro_export]
/// Macro to create [U68F60](https://docs.rs/fixed/latest/fixed/types/type.U68F60.html) constants.
macro_rules! U68F60 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U68F60) } }
#[macro_export]
/// Macro to create [U67F61](https://docs.rs/fixed/latest/fixed/types/type.U67F61.html) constants.
macro_rules! U67F61 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U67F61) } }
#[macro_export]
/// Macro to create [U66F62](https://docs.rs/fixed/latest/fixed/types/type.U66F62.html) constants.
macro_rules! U66F62 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U66F62) } }
#[macro_export]
/// Macro to create [U65F63](https://docs.rs/fixed/latest/fixed/types/type.U65F63.html) constants.
macro_rules! U65F63 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U65F63) } }
#[macro_export]
/// Macro to create [U64F64](https://docs.rs/fixed/latest/fixed/types/type.U64F64.html) constants.
macro_rules! U64F64 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U64F64) } }
#[macro_export]
/// Macro to create [U63F65](https://docs.rs/fixed/latest/fixed/types/type.U63F65.html) constants.
macro_rules! U63F65 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U63F65) } }
#[macro_export]
/// Macro to create [U62F66](https://docs.rs/fixed/latest/fixed/types/type.U62F66.html) constants.
macro_rules! U62F66 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U62F66) } }
#[macro_export]
/// Macro to create [U61F67](https://docs.rs/fixed/latest/fixed/types/type.U61F67.html) constants.
macro_rules! U61F67 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U61F67) } }
#[macro_export]
/// Macro to create [U60F68](https://docs.rs/fixed/latest/fixed/types/type.U60F68.html) constants.
macro_rules! U60F68 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U60F68) } }
#[macro_export]
/// Macro to create [U59F69](https://docs.rs/fixed/latest/fixed/types/type.U59F69.html) constants.
macro_rules! U59F69 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U59F69) } }
#[macro_export]
/// Macro to create [U58F70](https://docs.rs/fixed/latest/fixed/types/type.U58F70.html) constants.
macro_rules! U58F70 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U58F70) } }
#[macro_export]
/// Macro to create [U57F71](https://docs.rs/fixed/latest/fixed/types/type.U57F71.html) constants.
macro_rules! U57F71 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U57F71) } }
#[macro_export]
/// Macro to create [U56F72](https://docs.rs/fixed/latest/fixed/types/type.U56F72.html) constants.
macro_rules! U56F72 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U56F72) } }
#[macro_export]
/// Macro to create [U55F73](https://docs.rs/fixed/latest/fixed/types/type.U55F73.html) constants.
macro_rules! U55F73 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U55F73) } }
#[macro_export]
/// Macro to create [U54F74](https://docs.rs/fixed/latest/fixed/types/type.U54F74.html) constants.
macro_rules! U54F74 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U54F74) } }
#[macro_export]
/// Macro to create [U53F75](https://docs.rs/fixed/latest/fixed/types/type.U53F75.html) constants.
macro_rules! U53F75 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U53F75) } }
#[macro_export]
/// Macro to create [U52F76](https://docs.rs/fixed/latest/fixed/types/type.U52F76.html) constants.
macro_rules! U52F76 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U52F76) } }
#[macro_export]
/// Macro to create [U51F77](https://docs.rs/fixed/latest/fixed/types/type.U51F77.html) constants.
macro_rules! U51F77 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U51F77) } }
#[macro_export]
/// Macro to create [U50F78](https://docs.rs/fixed/latest/fixed/types/type.U50F78.html) constants.
macro_rules! U50F78 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U50F78) } }
#[macro_export]
/// Macro to create [U49F79](https://docs.rs/fixed/latest/fixed/types/type.U49F79.html) constants.
macro_rules! U49F79 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U49F79) } }
#[macro_export]
/// Macro to create [U48F80](https://docs.rs/fixed/latest/fixed/types/type.U48F80.html) constants.
macro_rules! U48F80 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U48F80) } }
#[macro_export]
/// Macro to create [U47F81](https://docs.rs/fixed/latest/fixed/types/type.U47F81.html) constants.
macro_rules! U47F81 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U47F81) } }
#[macro_export]
/// Macro to create [U46F82](https://docs.rs/fixed/latest/fixed/types/type.U46F82.html) constants.
macro_rules! U46F82 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U46F82) } }
#[macro_export]
/// Macro to create [U45F83](https://docs.rs/fixed/latest/fixed/types/type.U45F83.html) constants.
macro_rules! U45F83 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U45F83) } }
#[macro_export]
/// Macro to create [U44F84](https://docs.rs/fixed/latest/fixed/types/type.U44F84.html) constants.
macro_rules! U44F84 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U44F84) } }
#[macro_export]
/// Macro to create [U43F85](https://docs.rs/fixed/latest/fixed/types/type.U43F85.html) constants.
macro_rules! U43F85 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U43F85) } }
#[macro_export]
/// Macro to create [U42F86](https://docs.rs/fixed/latest/fixed/types/type.U42F86.html) constants.
macro_rules! U42F86 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U42F86) } }
#[macro_export]
/// Macro to create [U41F87](https://docs.rs/fixed/latest/fixed/types/type.U41F87.html) constants.
macro_rules! U41F87 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U41F87) } }
#[macro_export]
/// Macro to create [U40F88](https://docs.rs/fixed/latest/fixed/types/type.U40F88.html) constants.
macro_rules! U40F88 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U40F88) } }
#[macro_export]
/// Macro to create [U39F89](https://docs.rs/fixed/latest/fixed/types/type.U39F89.html) constants.
macro_rules! U39F89 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U39F89) } }
#[macro_export]
/// Macro to create [U38F90](https://docs.rs/fixed/latest/fixed/types/type.U38F90.html) constants.
macro_rules! U38F90 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U38F90) } }
#[macro_export]
/// Macro to create [U37F91](https://docs.rs/fixed/latest/fixed/types/type.U37F91.html) constants.
macro_rules! U37F91 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U37F91) } }
#[macro_export]
/// Macro to create [U36F92](https://docs.rs/fixed/latest/fixed/types/type.U36F92.html) constants.
macro_rules! U36F92 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U36F92) } }
#[macro_export]
/// Macro to create [U35F93](https://docs.rs/fixed/latest/fixed/types/type.U35F93.html) constants.
macro_rules! U35F93 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U35F93) } }
#[macro_export]
/// Macro to create [U34F94](https://docs.rs/fixed/latest/fixed/types/type.U34F94.html) constants.
macro_rules! U34F94 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U34F94) } }
#[macro_export]
/// Macro to create [U33F95](https://docs.rs/fixed/latest/fixed/types/type.U33F95.html) constants.
macro_rules! U33F95 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U33F95) } }
#[macro_export]
/// Macro to create [U32F96](https://docs.rs/fixed/latest/fixed/types/type.U32F96.html) constants.
macro_rules! U32F96 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U32F96) } }
#[macro_export]
/// Macro to create [U31F97](https://docs.rs/fixed/latest/fixed/types/type.U31F97.html) constants.
macro_rules! U31F97 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U31F97) } }
#[macro_export]
/// Macro to create [U30F98](https://docs.rs/fixed/latest/fixed/types/type.U30F98.html) constants.
macro_rules! U30F98 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U30F98) } }
#[macro_export]
/// Macro to create [U29F99](https://docs.rs/fixed/latest/fixed/types/type.U29F99.html) constants.
macro_rules! U29F99 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U29F99) } }
#[macro_export]
/// Macro to create [U28F100](https://docs.rs/fixed/latest/fixed/types/type.U28F100.html) constants.
macro_rules! U28F100 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U28F100) } }
#[macro_export]
/// Macro to create [U27F101](https://docs.rs/fixed/latest/fixed/types/type.U27F101.html) constants.
macro_rules! U27F101 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U27F101) } }
#[macro_export]
/// Macro to create [U26F102](https://docs.rs/fixed/latest/fixed/types/type.U26F102.html) constants.
macro_rules! U26F102 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U26F102) } }
#[macro_export]
/// Macro to create [U25F103](https://docs.rs/fixed/latest/fixed/types/type.U25F103.html) constants.
macro_rules! U25F103 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U25F103) } }
#[macro_export]
/// Macro to create [U24F104](https://docs.rs/fixed/latest/fixed/types/type.U24F104.html) constants.
macro_rules! U24F104 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U24F104) } }
#[macro_export]
/// Macro to create [U23F105](https://docs.rs/fixed/latest/fixed/types/type.U23F105.html) constants.
macro_rules! U23F105 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U23F105) } }
#[macro_export]
/// Macro to create [U22F106](https://docs.rs/fixed/latest/fixed/types/type.U22F106.html) constants.
macro_rules! U22F106 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U22F106) } }
#[macro_export]
/// Macro to create [U21F107](https://docs.rs/fixed/latest/fixed/types/type.U21F107.html) constants.
macro_rules! U21F107 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U21F107) } }
#[macro_export]
/// Macro to create [U20F108](https://docs.rs/fixed/latest/fixed/types/type.U20F108.html) constants.
macro_rules! U20F108 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U20F108) } }
#[macro_export]
/// Macro to create [U19F109](https://docs.rs/fixed/latest/fixed/types/type.U19F109.html) constants.
macro_rules! U19F109 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U19F109) } }
#[macro_export]
/// Macro to create [U18F110](https://docs.rs/fixed/latest/fixed/types/type.U18F110.html) constants.
macro_rules! U18F110 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U18F110) } }
#[macro_export]
/// Macro to create [U17F111](https://docs.rs/fixed/latest/fixed/types/type.U17F111.html) constants.
macro_rules! U17F111 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U17F111) } }
#[macro_export]
/// Macro to create [U16F112](https://docs.rs/fixed/latest/fixed/types/type.U16F112.html) constants.
macro_rules! U16F112 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U16F112) } }
#[macro_export]
/// Macro to create [U15F113](https://docs.rs/fixed/latest/fixed/types/type.U15F113.html) constants.
macro_rules! U15F113 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U15F113) } }
#[macro_export]
/// Macro to create [U14F114](https://docs.rs/fixed/latest/fixed/types/type.U14F114.html) constants.
macro_rules! U14F114 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U14F114) } }
#[macro_export]
/// Macro to create [U13F115](https://docs.rs/fixed/latest/fixed/types/type.U13F115.html) constants.
macro_rules! U13F115 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U13F115) } }
#[macro_export]
/// Macro to create [U12F116](https://docs.rs/fixed/latest/fixed/types/type.U12F116.html) constants.
macro_rules! U12F116 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U12F116) } }
#[macro_export]
/// Macro to create [U11F117](https://docs.rs/fixed/latest/fixed/types/type.U11F117.html) constants.
macro_rules! U11F117 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U11F117) } }
#[macro_export]
/// Macro to create [U10F118](https://docs.rs/fixed/latest/fixed/types/type.U10F118.html) constants.
macro_rules! U10F118 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U10F118) } }
#[macro_export]
/// Macro to create [U9F119](https://docs.rs/fixed/latest/fixed/types/type.U9F119.html) constants.
macro_rules! U9F119 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U9F119) } }
#[macro_export]
/// Macro to create [U8F120](https://docs.rs/fixed/latest/fixed/types/type.U8F120.html) constants.
macro_rules! U8F120 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U8F120) } }
#[macro_export]
/// Macro to create [U7F121](https://docs.rs/fixed/latest/fixed/types/type.U7F121.html) constants.
macro_rules! U7F121 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U7F121) } }
#[macro_export]
/// Macro to create [U6F122](https://docs.rs/fixed/latest/fixed/types/type.U6F122.html) constants.
macro_rules! U6F122 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U6F122) } }
#[macro_export]
/// Macro to create [U5F123](https://docs.rs/fixed/latest/fixed/types/type.U5F123.html) constants.
macro_rules! U5F123 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U5F123) } }
#[macro_export]
/// Macro to create [U4F124](https://docs.rs/fixed/latest/fixed/types/type.U4F124.html) constants.
macro_rules! U4F124 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U4F124) } }
#[macro_export]
/// Macro to create [U3F125](https://docs.rs/fixed/latest/fixed/types/type.U3F125.html) constants.
macro_rules! U3F125 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U3F125) } }
#[macro_export]
/// Macro to create [U2F126](https://docs.rs/fixed/latest/fixed/types/type.U2F126.html) constants.
macro_rules! U2F126 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U2F126) } }
#[macro_export]
/// Macro to create [U1F127](https://docs.rs/fixed/latest/fixed/types/type.U1F127.html) constants.
macro_rules! U1F127 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U1F127) } }
#[macro_export]
/// Macro to create [U0F128](https://docs.rs/fixed/latest/fixed/types/type.U0F128.html) constants.
macro_rules! U0F128 { ($($a:tt)+) => { ::fixed_macro::fixed!($($a)+: U0F128) } }
