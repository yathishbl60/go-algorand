# Compact field and hash primitive results

Updated 2026-11-01. Actual bounded primitive execution, **not** a complete private
transaction, selected suite, approved security profile or8GiB full-prover result.
No automated test files/suites were added. Default/native configuration remains
Goldilocks/quintic. Optional `native-privacy-compact` exports arithmetic only.

## Implemented components

- [field31](../../internal/assetprivacy/field31/README.md): scalar four-byte base
  representation, D10 extension and canonical serialization. The proof crate
  retains its unsafe-code prohibition. The separate arithmetic crate permits
  only the required unsafe trait-method declarations, with safe reducing bodies.
- [note byte AIR](NOTE_BYTES_RESULTS.md):11 raw columns,9 preprocessing columns,
  degree3. Pads and authenticated private byte/word lookups remain unimplemented.
- [compact Keccak](../../internal/assetprivacy/stark/src/compact_keccak_air/README.md):
  1920 raw columns,11 trusted preprocessing columns, degree5. Its100 input words
  are reconstructed expressions, rather than extra persistent columns.

The hash component constrains independent Keccak-f permutations. No sponge,
private input/output lookup, cSHAKE profile, note commitment, key authorization or
whole transaction binding is installed. Correct rounds alone do not prove those.

## Runtime dependency finding

Unmodified pinned P3 compiles the optional field but panics during a real D10
proof's PCS opening compression. `PackedField::coeffwise_dot_product` returns
eight coordinates; Matrix's rowwise caller requests ten. Quotient evaluation and
field arithmetic are not the failing path.

The [nine-line fallback](../../internal/assetprivacy/stark/patches/p3-matrix-d10.patch)
uses a generic packed mixed-field dot product only for degree>8, leaving the
existing fast path intact. Initial validation used an isolated temporary copy;
after the user approved the third-party size exception, the exact change was
[vendored with provenance](../../internal/assetprivacy/vendor/p3-matrix/UPSTREAM.md).
All four proofs then verified against the repository copy. Cargo's pinned cache
was not modified. This compatibility fix establishes no protocol security claim.

## Bounded Fp31/D10 worker

A disposable worker used192 queries, blowup8, binary FRI, ten hiding codewords,
ten salt coordinates, SHA3-512 commitments/transcript and fresh OS entropy.
Its validation-specific transcript/hash domains are not a native proof profile.
The supervisor capped CPU/wall at300 seconds and sampled RSS against2GiB,
disabled core dumps and redacted expected constraint-panic contents.

The first patched run verified one/three/four-note proofs in1.400 seconds with
72.984MiB observed peak RSS. The expanded run added the compact hash and took
2.640 seconds with145.500MiB observed peak RSS. The repository-vendor rerun took
2.449 seconds and observed150.703MiB. Samples are not exact peaks.

| Expanded-run relation | Rows | Width / preprocessing | Degree | Chunks | Layout byte counter |
| --- | ---: | ---: | ---: | ---: | ---: |
| One canonical note |512 |11 /9 |3 |8 |796177 |
| Three canonical notes |2048 |11 /9 |3 |8 |1052153 |
| Four canonical notes |2048 |11 /9 |3 |8 |1047801 |
| Two Keccak-f permutations |512 |1920 /11 |5 |16 |2617041 |

These counters follow actual proof metadata using a four-byte adaptation of the
existing writer layout. They are not artifact-bound serialization measurements.
All four proofs verified; altered proof openings rejected. Note proofs additionally
rejected a different policy's verifier key. No payloads/traces were persisted.

## Arithmetic, representation and rejection checks

100 public samples matched independent integer/convolution multiplication for
base/extension elements and verified inverses. Four-byte canonical encoding and
the coherent2^28/2^27 roots passed. The full-degree helper rejected base, degree2
and degree5 examples, accepted a degree10 element and gave Cauchy-basis rank10.
That check is necessary groundwork for private pads, not a complete hiding proof.

All note checks in [the note result](NOTE_BYTES_RESULTS.md) repeated under Fp31.
All48 hash rounds and9600 input/output word checks matched independently generated
public Python Keccak states previously checked against standard-library SHA3/SHAKE.
Five corrupted hash traces rejected: a non-Boolean bit, wrong parity, a replaced
continuation row, nonzero dummy data and a replaced final-round row.

No Fp31 native codec/configuration, private bus, complete envelope, global quotient,
streaming adapter or native proof acceptance follows. The conditional
[whole-wire budget](COMPACT_WIRE_BUDGET.md) and [memory draft](STREAMING_PROVER_DRAFT.md)
still need complete construction and measurement. Scratch worker path:
`/tmp/algorand-compact-feasibility`; it is not a maintained test deliverable.
