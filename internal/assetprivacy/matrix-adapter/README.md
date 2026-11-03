# Checked P3 matrix bridge

This small crate adapts safe `CheckedRows<T>` sources to pinned P3's `Matrix<T>`.
It is separate so the proof crate retains `#![forbid(unsafe_code)]`.

The pinned Matrix trait's default unchecked methods are circular unless an
implementation overrides at least one unchecked row method. The bridge therefore
contains exactly one required `unsafe fn row_unchecked` trait declaration. Its
body uses only safe row bounds, checked source access and an exact width assertion.
It contains no unsafe block, pointer cast, raw memory access or unchecked operation.
`unsafe_op_in_unsafe_fn` is forbidden. Invalid access panics rather than reading
outside storage. The inherited P3 trait retains its original caller contracts.

`SafeMatrix::new(source)` rejects overflowing logical dimensions. `inner()` and
`into_inner()` allow typed fast paths without private-data cloning. Safe `get`
checks dimensions before delegating to optional `checked_get`; default cell access
uses a checked owned row. No Clone, Debug or serialization is supplied.

Sources remain responsible for mathematically correct dimensions and values.
This bridge establishes no proof authenticity, opening budget or resource ceiling.
