# Coefficient-backed hiding PCS

For full-width quotient evaluation use the explicit `StripQuotient` scheduler;
see [its API and resource boundary](strip_quotient/README.md). The ordinary batch
prover retains its whole-domain default. This does not select a native profile.

This candidate backend is available under `native-privacy-compact` through
`coefficient_pcs::prover_config(Limits)` or `CoefficientPcs::new(Limits)`.
It does not change default/native backend selection or activate a protocol profile.
It implements the pinned `Pcs` and `UnivariateStarkPcs` interfaces and returns the
existing compact proof type. Verification delegates to the existing stock verifier.
Commitments stream column tiles into the compatible input Merkle commitment.
The private prover state retains coefficients rather than a full trace LDE.

## Input domains and supported geometry

For a direct `Pcs::commit`, supply an N-row rectangular trace with an **extended
2N-row domain**. This matches the stock hiding PCS and batch prover interface:

```rust,ignore
let domain = pcs.natural_domain_for_degree(2 * n);
let (commitment, data) = pcs.commit([(domain, trace)])?;
```

N must be a power of two from 512 through 65,536. A batch has 1..=16 matrices
of equal height; widths may differ. Separate opening rounds may have different
heights. The shift must be nonzero. Internally, mask interpolation uses the
N-row domain with the same shift. The resulting polynomial has 2N coefficient
rows, relation-width + 10 columns, and a canonical `GENERATOR * H_(16N)` LDE.
The lower-level `MaskedTrace::from_trace` instead accepts the original N domain.

An opening has 1..=4 rounds and at least one point for every matrix. Zero and
extension-field points are supported; base-field points inside the committed
LDE domain are rejected before inversion. Repeated and translated points count
as separate disclosures. The conservative hiding requirement is
`N >= 2 * (192 + 10 * number_of_points)` for every matrix.

Public preprocessing commitments and preprocessing opening indexes return
`Error::Unsupported`. Public periodic columns use the bounded stock table path.
Randomization commitments receive extended 2N domains and directly sample
uniform 2N-by-20 coefficients; they do not apply trace masking a second time.
Quotient preparation supports 2..=16 power-of-two, equal-height, disjoint chunks.

## Limits and allocation assumptions

All limits must be nonzero. Scalar counts use **base-field scalar units**;
an extension-field element consumes ten units. `tile_columns` is a column count.

| Field | Guarded payload |
| --- | --- |
| `max_tile_values` | One base-field LDE column tile |
| `max_coefficient_values` | Total masked coefficient cells per commitment |
| `max_evaluation_values` | Evaluation/import/quotient payloads and opening vectors |
| `max_claim_values` | All masked columns at every point, times ten |
| `max_query_values` | All masked widths times 192 query rows |

For opening reduction, let S be the sum of distinct LDE heights and M the
largest height. The checked extension-vector payload is `10 * (S + 3M)`:
retained reductions, one compressed vector, differences, and inversion output.
The pinned batch inversion uses its output for prefix products. Column tiles
and extension weights have separate checks. Arithmetic uses checked products
and sums before private mask entropy or opening lease consumption.

These are selected payload guards, **not an aggregate RAM or 8 GiB guarantee**.
Caller inputs, retained commitments, simultaneous proofs, FFT/FRI allocations,
Merkle hashes/salts/proofs, vector capacities, allocator overhead, and thread
stacks can coexist. Choose limits for the host and measure the complete prover.
Trait methods without a fallible return type reject evaluation or periodic-table
limit violations by panicking; fallible preparation/opening methods return errors.

## Entropy, imports, and retries

Construction seeds private cryptographic RNG state from OS entropy. Each regular
commitment forks private mask randomness; input Merkle salts have independent
private RNG state. There is no public predictable-mask RNG constructor.

`ProverData` clones share immutable private coefficients and one opening lease.
The entire opening request is validated before any lease changes. Duplicate,
already-used, or invalid requests leave all leases unchanged. A valid request
consumes all leases atomically **before** producing claims. Subsequent failures
do not restore them. Retry by recommitting original inputs with fresh masks;
cloning the prover data does not permit another opening. Concurrent requests
cannot consume the same data twice.

`commit_ldes` requires freshly masked, bit-reversed LDEs from
`get_quotient_ldes`. Import reverses rows, performs the canonical coset inverse
DFT, checks the degree bound, and preserves those existing masks. It does not
add entropy. Degree checks cannot prove randomness. Recommitting copies of an
already-opened masked LDE violates this API precondition; leases identify shared
prover state, not polynomial contents across separate imports. Regenerate the
quotient masks for each new proof. Treat coefficients, evaluation buffers, and
prover data as secrets; this module does not establish zeroization guarantees.
