# Pinned matrix extension-degree fallback

`p3-matrix-d10.patch` targets Plonky3 revision
`1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f`, `matrix/src/lib.rs`.
It is the reviewable diff mirrored in the [vendored dependency](../../vendor/p3-matrix/UPSTREAM.md).
The workspace selects that pinned local crate. No Cargo checkout is modified.

The pinned `Matrix::rowwise_packed_dot_product` calls a packed kernel with an
eight-coordinate return array. Real D10 proving reaches that array's out-of-range
slice and panics. Compiling field arithmetic does not reveal this runtime limit.
The patch adds nine lines: for degrees above eight, accumulate the generic
packed extension-times-base product. Existing smaller-degree code is retained.
No field dimension is misreported; no proof check or masking parameter is removed.

An isolated temporary copy and then the repository-vendored crate verified three
actual Fp31/D10 note proofs and one compact Keccak proof. Altered proofs rejected.
See [bounded results](../../../../docs/native-asset-privacy/COMPACT_PRIMITIVE_RESULTS.md).
This is runtime evidence for these primitives, not an independent backend audit.

The user approved vendoring with size-limit exceptions for third-party code.
The upstream library has ten source files totaling
8694 lines after this change, including existing inline tests; all ten exceed
100 lines. The user's handwritten file/function limits remain enforced here.
Only the upstream files/inherited functions use that explicit exception.
No automated tests or upstream tests were added/run for this proposal.
