# Staged private K-PKE arithmetic

`MlKemPkeAir` constrains the complete ML-KEM-768 arithmetic pipeline given
private key polynomials, matrix, noise and message bits. It has exactly 544
public little-endian u16 ciphertext limbs, encoding 1088 bytes. It does not
derive the matrix/noise, authenticate a key, or establish a complete KEM proof.
The enclosing relation must supply those missing bindings. The module is
compiled only with the opt-in `native-privacy` feature.

The relation constrains three forward NTTs of noise 0–2 into r_hat; four NTT
dot products; four inverse NTTs; error/message addition; and complete ciphertext
compression/packing. The first three dots use A[term][output]*r_hat[term],
preserving the transpose required by encryption. The fourth uses t[term].
Errors are noise 3–5 for u and noise 6 for v. Boolean message bits decompress to
0/1665 only in v. Two canonical additions per coefficient enforce modular sums.

Root matrix, t, r_hat, dot outputs, seven noise polynomials, message bits and
final u/v remain constant across all 16384 rows. NTT input/output bindings,
dot input/output aliases and selected canonical additions connect the stages.
An explicit raw-output carry constraint includes the inverse frame's final row
and all but the final addition row. This prevents changing an inverse result
between its transform check and its selected error addition.

| Root rows (half-open) | Stage |
|---|---|
|0..3072|Three 1024-row forward NTT frames|
|3072..5120|One 2048-row four-dot-product frame|
|5120..14336|Four spans of 2048 inverse rows then 256 addition rows|
|14336..16384|One 2048-row ciphertext compression/packing frame|

All child row boundaries, active filters, preprocessing slices and public ranges
come from the trusted fixed schedule. Only the codec consumes public values.
No private polynomial, noise, message, key or intermediate is exposed to join
stages. No cross-AIR lookup terminal or independent proof receipt connects them.

| Root columns (half-open) | Meaning |
|---|---|
|0..2304|Nine matrix polynomials|
|2304..3072|Three encapsulation-key polynomials t|
|3072..3840|Three forward NTT results r_hat|
|3840..4864|Four NTT dot outputs|
|4864..6161|Reused NTT/dot/addition workspace|
|6161..7953|Seven input noise polynomials|
|7953..8209|Private Boolean message bits|
|8209..9233|Four final u/v polynomials, physically shared with codec inputs|
|9233..9328|Codec scratch and public-word decompositions|

`wires(row)` exposes private matrix, key-polynomial, noise, message and output
slices for direct enclosing constraints. All arithmetic operands/results are
canonical through their selected NTT/product/add/compression checks. The compact
NTT's fixed first-stage coverage and dot accumulator invariants remain required.
Compile-time shape checks guard the child layouts and fixed frame heights.
Maximum source composition degree is 5 before quotient accounting.

`trace(&EncryptionWitness)` returns a `PkeWitness` with secret columns and public
ciphertext values. Before the large allocation it checks canonical inputs and
message shape, recomputes/compares r_hat, u/v and ciphertext using the shared
host arithmetic. Direct writers populate the destination without keeping full
child matrices. Those host checks are construction checks, not proof validation.
`public_values(ciphertext)` supplies the verifier's exact body-to-field mapping.
A crate-private `write_into(input, values, (stride, column, first_row))` performs
the same preparation before writing exactly its 16384-row frame. Bounds and
row shape are checked before mutation; every component cell is overwritten and
all other rows/columns are preserved. No second full PKE matrix is allocated.

There are 9328 main columns and 1967 preprocessed columns at 16384 rows:
1,222,639,616 raw main bytes plus 257,818,624 preprocessing bytes, before hiding,
commitments, allocator and prover temporaries. No memory/time measurements ran;
this layout is not an established practical or production resource budget.
Trusted setup uses base log-height 14, extended to 15 by the hiding configuration.

The [bounded KEM parent](../mlkem_encapsulation_air/README.md) now binds all nine
matrices to key rho/coordinates, all seven noise samples to derived coins/nonces,
message bits to m, and t to canonical key bytes. Shared-secret/KDF/AAD,
the committed note, role/epoch authority, group binding and native validation
remain separate obligations. Do not accept this arithmetic component alone.

Builds, strict Clippy, formatting, size checks and source reviews only. No tests,
vectors, trace generation, constraint evaluation or proofs have run. Every
consensus version remains disabled and no native acceptance path is added.
