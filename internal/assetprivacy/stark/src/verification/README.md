# Per-AIR candidate proof codec

This module is gated by `native-privacy`. It supplies typed relation/registry
selection, `Verifier`, `DecodeLimits`, `Error`, and backend type re-exports. See the
[trust/API contract](../../../../../docs/native-asset-privacy/VERIFICATION_BOUNDARY.md).
This candidate format is not an activated consensus encoding or a complete native
proof bundle. No serialization round trip or proof verification has been executed.

## Scalar and collection encoding

All integers here use **little endian**, independently of the outer NAPW frame.
There are no serde/postcard encodings, native `usize` encodings, padding bytes,
embedded keys, public-input vectors, wire-advertised limits or arbitrary type tags.

| Type | Encoding |
| --- | --- |
| `F` | canonical Goldilocks representative, `u64`; reject words >= `0xffffffff00000001` |
| `E` | five consecutive `F` coefficients in the backend's `1,X,...,X^4` basis |
| `D` | eight raw `u64` SHA3 digest words; these are **not** field representatives |
| `L<T>` | `u32` element count followed by the ordered encodings of those elements |
| `C` | exactly one `D`; cap height is fixed at zero, with no length prefix |
| `Z` | one `F` whose value must be zero |

Nested lists repeat `L` framing at every level. No optional-field tag is encoded:
the trusted hiding configuration and AIR row-access shape select field presence.
Absent next-row openings consume no bytes; an empty list is not their replacement.

## Complete field order

| Order | Contents |
| --- | --- |
| 1 | eight bytes `4e 41 50 53 54 4b 01 00` (`NAPSTK`, version 1, reserved zero) |
| 2 | 64-byte trusted artifact ID |
| 3 | one byte: hiding-extended degree bits, exactly `log2(base rows)+1` |
| 4 | main trace commitment `C` |
| 5 | quotient-batch commitment `C` |
| 6 | hiding random-polynomial commitment `C` |
| 7 | main local opening `L<E>`, exactly AIR width |
| 8 | main next opening `L<E>`, exactly AIR width, only if AIR reads next row |
| 9 | preprocessed local opening `L<E>`, exactly preprocessing width |
| 10 | preprocessed next `L<E>`, same width, only if AIR reads next preprocessing row |
| 11 | quotient chunk openings `L<L<E>>`, five `E` per chunk |
| 12 | random-polynomial opening `L<E>`, exactly five `E` |
| 13 | wrapper random-codeword openings `L<L<L<L<E>>>>` (round/matrix/point/value) |
| 14 | FRI batch grind witness `Z` |
| 15 | FRI phase commitments `L<C>` |
| 16 | FRI phase grind witnesses `L<Z>` |
| 17 | input batch openings `L<InputBatch>` |
| 18 | FRI phase openings `L<Phase>` |
| 19 | final FRI polynomial `L<E>` |
| 20 | FRI query grind witness `Z` |
| 21 | STARK out-of-domain grind witness `Z` |

`InputBatch` is query/matrix/column values `L<L<L<F>>>`, then `MultiProof`.
`Phase` is query/sibling values `L<L<E>>`, then `MultiProof`.
`MultiProof` is query/matrix/salt words `L<L<L<F>>>`, followed by frontier sibling
digests `L<D>` in the pinned backend's pruned-path order. Extension-MMCS proofs
retain base-field salts; opening values themselves have the types above.

Each ordinary blinded matrix has five random codewords. The preprocessing batch
has zero wrapper random-codeword values per point. Hiding-MMCS salt vectors have
five base words per matrix/query. The pinned backend checks those shapes, input
batch order, all 192 queries, binary-fold rounds, multiproofs and final length one.
The decoder checks bounded lengths; it does not treat a decoded object as valid.
Quotient chunk count is derived by the backend from the trusted AIR, not chosen
by the wire. The local structural guard also caps it at 128.

## Artifact selection and limits

The artifact ID is SHA3-512 under
`Algorand/AssetPrivacy/Candidate1/VerifierArtifactV2/`. Its ordered preimage is the
pinned backend/configuration label, compiled source64, profile64, registry digest64, full relation64,
seven `u32` dimensions/parameters, fixed parameter bytes, two row-access bytes,
then the trusted preprocessing root's eight little-endian words. The exact label,
dimension order and constants are in [metadata.rs](metadata.rs). Changing the
backend, configuration, codec or relation revision requires a different identity.

The caller supplies that trusted identity/key selection before parsing. Equality
with the wire ID cannot establish provenance, authorize a suite, select a weaker
AIR or replace public-input binding. The native bridge, approved artifact files/pins
and adversarial/runtime validation remain unfinished. [Exact typed bundles](../../../../../docs/native-asset-privacy/PROOF_BUNDLES.md)
now compose the current main/output proof set; native transport/acceptance remain.

Decoder allocation admission uses canonical charges from a fixed64-bit model,
independent of host `size_of`: Value8, Challenge40, digest64, Vec header24,
Cap24, input opening72 and FRI phase opening72. Child element storage is charged
separately from each stored Vec header. These units are deterministic admission
accounting, not exact heap usage; allocator/backend overhead remains outside them.
Checked charge overflow or exhausted quota returns `MemoryLimit`. An actual
fallible reservation failure returns `AllocationFailure`, a local resource failure
that must never be treated as deterministic invalid-proof evidence.
