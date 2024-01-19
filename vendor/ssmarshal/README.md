# ssmarshal

[![Crates.io](https://img.shields.io/crates/v/ssmarshal.svg?style=flat-square)](https://crates.io/crates/ssmarshal)

[Documentation](https://docs.rs/ssmarshal)

ssmarshal ("stupid simple marshaling") is a serde-based de/serialization
library. It is somewhat like [bincode](https://github.com/TyOverby/bincode),
but doesn't support String/Vec - this library is entirely zero-allocation, for
use in extremely limited `no_std` contexts. The key invariant is that the
encoding space for any value of a type is `<= size_of::<T>()`. This allows
easy reasoning about limited buffer sizes, and how much is always enough.

## Limitations

These sorts of types are not supported:

- Any non-core type (eg, `Vec`, `HashMap`)
- Slices
- Strings
- Types containing references or pointers
- Enums with more than 256 variants

All enums MUST be `#[repr(C)]` in order for the size invariant to be upheld.
Note that this excludes using `Option`, especially with `NonZero` types!
You can use this crate with non-`#[repr(C)]` enums, but you should thoroughly
test (I recommend quickcheck, see [the tests](https://gitlab.com/robigalia/ssmarshal/blob/master/tests/roundtrip.rs)
for an example) de/serializing values of that type to be assured the size
invariant holds for the type.

## Details of the format

The format is not incredibly compact, but doesn't add extra fluff, and is
quick to en/decode. 

- `bool` is serialized as a byte, 1 if true, 0 is false.
- the integer types are encoded in their little-endian form.
- f32 is bitcast to a u32 then encoded as a u32. likewise, f64 and u64.
- inhabited enums are serialized as 1 byte for the discriminant, and then the fields.
- structs are serialized as just their fields.
- the unit type and uninhabited enums are not serialized at all.
- tuples are serialized as the fields, in order.

There is no padding.

As you might see, this format is not self-describing. To successfully
deserialize a value, the exact layout must be known ahead-of-time.

## Alternatives

This is designed for doing IPC in a microkernel, with a stable ABI, not saving
to disk or transferring over the network. It may be useful for those cases,
although you'll likely want a format which can handle data evolution, like
Cap'n Proto.

If you care about truly minimizing encoding space, you might look into ASN.1
PER. 

If you need more features (for example, slices or references), but still don't
care about data evolution, I recommend using bincode.

## Testing

The `roundtrip` test suite exercises almost all of the functionality of
ssmarshal, with coverage over 95% (missing mostly some of the error cases for
invalid/unsupported types).

This library is extensively fuzz tested with `cargo-fuzz` (libFuzzer) before
every release. See the `fuzz` directory for the scripts used.
