# Optional scalar Fp31/D10 support

This arithmetic crate is available to the proof crate only through its optional
`native-privacy-compact` feature. It selects no prover configuration, native
codec, artifact suite or consensus activation. Existing default/native privacy
configuration remains Goldilocks with its existing quintic extension.

`Value32` delegates prime-field arithmetic to the pinned BabyBear implementation
for p=2013265921, while using scalar `Packing=Self`. Canonical serialization is
four-byte little-endian, not BabyBear's internal Montgomery representation.
Human-readable Serde uses the canonical u32. Decoding rejects values >=p.

`Challenge32` is the power basis modulo X^10-11. The generic pinned binomial
multiplier accumulates into its output; this crate zeros that output first.
Frobenius, inversion and extension packing use the pinned generic field code.
The algebra identifier explicitly binds p, degree, polynomial and power basis.
`has_full_extension_degree` rejects the proper degree2/5 subfields, including
the base field. Checking only non-base membership is insufficient for a proposed
ten-coordinate private-pad basis. No challenge sampler/private bus is installed.

Public [metadata certificates](../../../docs/native-asset-privacy/COMPACT_FIELD_METADATA.json)
record irreducibility, complete order factorization, primitive generator2+X,
DTH_ROOT1403701133 and coherent2^28 root17094607X^5. These are mathematical
metadata checks, not independent implementation or field-security approval.
The [candidate assessment](../../../docs/native-asset-privacy/COMPACT_PROFILE_ASSESSMENT.md)
describes the unresolved complete proof, wire and8GiB peak-memory requirements.

## Unsafe-syntax policy

The proof crate retains `forbid(unsafe_code)`. This separate arithmetic crate
uses `forbid(unsafe_op_in_unsafe_fn)`. Its only authored unsafe syntax is the
ten `QuotientMap::from_canonical_unchecked` method declarations required by P3.
Every body calls the safe reducing constructor; there are no unsafe blocks,
pointer casts or unchecked memory operations. P3 supplies usize/isize mappings
through its blanket implementations. Dependency implementations have their own
audited safety boundaries; this policy does not claim dependencies contain no unsafe code.

No automated tests were added. Native integration, reviewed security parameters,
profile selection and complete proving feasibility remain separate work.
Actual bounded arithmetic and primitive proofs are recorded in the
[runtime evidence](../../../docs/native-asset-privacy/COMPACT_PRIMITIVE_RESULTS.md).
