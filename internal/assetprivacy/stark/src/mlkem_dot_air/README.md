# Private ML-KEM NTT dot products

`MlKemDotAir` constrains four length-three dot products of NTT polynomials.
All inputs, intermediate values and outputs are private; there are no public
values. This opt-in `native-privacy` component does not sample matrices/noise,
establish key authority or prove complete encapsulation.

`DotInputs` contains a 3x3 `matrix`, three `public_polynomials` and three
`r_hat` polynomials, each represented by 256 canonical u16 coefficients.
The name `public_polynomials` follows the ML-KEM key format; the values stay
private in this AIR. No Debug or serialization implementation is supplied.

Outputs 0–2 are `sum_term matrix[term][output]*r_hat[term]`, implementing the
transposed matrix multiplication used by K-PKE encryption. Output 3 is
`sum_term public_polynomials[term]*r_hat[term]`. Every product uses all 128
quadratic NTT factors, with the fixed per-pair gamma. Scalar coefficient-wise
multiplication would be a different operation and is not substituted here.

The fixed program has 1536 active rows: four dots, three terms each, and 128
adjacent coefficient pairs per term. A pair reuses the existing product AIR's
five bounded modular multiplications and two modular additions. The running
accumulator adds both product coefficients through canonical inputs/outputs
and Boolean carries. Its 256 scalar registers start at zero; each update changes
only the selected pair. At a dot's final row, the prospective updated register
vector equals that dot's constant output polynomial, then the next row resets
the whole accumulator to zero. Padding has no selected pair and preserves zero.

All global polynomials remain constant across rows. Trusted operation flags
select the matrix column or key polynomial and the intended r_hat operand.
Trusted pair flags bind both operands to their exact global coefficients.
Every matrix/key/r_hat coefficient is selected and canonically range checked.
Accumulator bounds follow from zero initialization and canonical updates;
each final output coefficient is therefore in 0–3328. No integer equation can
wrap Goldilocks: scalar products are below 11.1 million and sums below 6657.

The layout has 6161 main columns:

| Columns | Meaning |
|---|---|
|0..2304|Nine matrix polynomials, row-major|
|2304..3072|Three private key polynomials|
|3072..3840|Three private r_hat polynomials|
|3840..4864|Four private output polynomials|
|4864..6059|Shared product scratch; its output-vector region holds the accumulator|
|6059..6159|Two canonical accumulator inputs and two canonical next values|
|6159..6161|Two Boolean accumulator carries|

There are 2048 rows and 145 preprocessed columns: 128 pair selectors, 12 operation
selectors, gamma, and four dot-end flags. The source maximum degree is 4 including
transition selectors; parent frame gating can increase it. Trusted setup uses
base log-height 11, extended to 12 by the candidate hiding configuration. Never
accept proof-chosen operation flags or preprocessing commitments. The enclosing
relation fixes the intended sources of the private polynomial coefficients.

`values(input)` computes the same canonical results without field matrices.
`trace(input)` builds a private witness using the crate-private direct writer.
The writer checks exact column/row bounds with checked arithmetic and validates
all inputs and prepares all outputs before writing. It overwrites exactly its
2048-row component rectangle and preserves surrounding columns/rows. A parent
can therefore alias the global core and reuse scratch across other phases.
Accessors expose matrix, key, r_hat and output slices for direct private bindings.

Raw storage is 100,941,824 main bytes plus 2,375,680 preprocessing bytes, excluding
hiding, commitments and prover allocations. The writer retains only a 38,912-byte
fixed field prefix and small polynomial temporaries, not a second full trace.
These are static counts, not measured performance or protocol limits.

Source inspection, builds, strict Clippy, formatting and authored size checks
only. No tests, traces, constraint evaluations, vectors or proofs ran. The parent
still must bind matrix/noise derivation, forward/inverse transforms, message and
ciphertext encoding, KEM derivation, key registry state and native authorization.
