# `ssmarshal` changelog

This project follows semantic versioning.

## v1.0.0 (2017-05-13)

- [added] `cargo-fuzz` based fuzz testing
- [added] random datatype-based testing
- [removed] afl.rs based fuzz testing
- [changed] expanded quickcheck-based testing to more interesting types
- [changed] Ported to serde 1.0
- [removed] Support for serializing values which can't be deserialized (slices etc)
- [fixed] Documented the high-level details of the format and limitations.

## v0.0.1 (2017-01-03)

- First release.
