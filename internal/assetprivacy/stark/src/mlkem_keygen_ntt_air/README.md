# Private key-generation seed/noise and forward transforms

MlKemKeygenNttAir joins the seed/noise composition to six forward ML-KEM NTTs.
It has no public values or selected native verifier layout. Viewing registration
still rejects; matrix arithmetic and exact public-key matching remain required.

## Aliased roots and exact bindings

The first2304 columns alias the existing child root: private seed/rho/sigma bits
followed by six raw256-coefficient vectors. Child noise workspace occupies the
next5522 columns. Each transform reuses920 of those workspace columns after the
noise stage finishes. A separate1536-column tail holds all transformed vectors.
No independent noise witness is accepted by the trace constructor: its input is
only the private32-byte derivation seed.

The parent preserves raw roots and transformed roots across every row. Each NTT
frame binds its input to the matching raw vector and its output to that vector's
tail registers. The child is always the existing forward transform. Indices0..2
are secret vectors;3..5 are error vectors. Canonical arithmetic/ranges are checked
by the noise/NTT child constraints, rather than a host-only equality.

## Trusted program and dimensions

One4096-row seed/noise frame is followed by six1024-row NTT frames. These occupy
10240 rows; the remaining6144 rows of the16384-row matrix activate no child.
Global transitions preserve roots through all stages and padding. The570 public
preprocessing columns contain28 noise columns,514 reused NTT schedule columns
and seven4-column sets of parent active/first/last/transition flags.

There are9362 main columns and zero public values. Static raw main storage is
`16384*9362*8` bytes and preprocessing is `16384*570*8` bytes. These omit masking,
commitments, LDE/quotient and allocations; no practical resource claim follows.
Nested frame selectors also require degree/resource review in the full relation.

## APIs and writer boundaries

`new` fixes forward transforms and checks child dimensions. Compile-time checks
pin expected child widths/schedules. `wires` returns private seed/rho/sigma, raw
and transformed slices only for a correctly sized row; nothing is serialized.
`trace(seed)` delegates to the bounded crate-private direct writer.

The writer validates its full rectangle before writes, clears that rectangle,
writes the seed/noise child directly, then uses its actual root coefficients as
transform inputs. It writes each transform directly into shared scratch, records
its outputs and publishes constant roots. Cells outside the rectangle remain
unchanged. No second full child main matrix is needed. No writer has been run.

The [complete possession contract](../../../../../docs/native-asset-privacy/KEYGEN_POSSESSION.md)
still requires the rho-derived matrix, matrix-vector product plus error, canonical
public-key bytes, finalized registration context and real native verification.
No artifact entry, accepted viewing-key registration or success callback was added.

Only builds/static checks have run. No tests, vectors, traces, constraint
executions, setup, key pairs, proofs or node scenarios have been generated.
