# Compact masked storage and quotient differential evidence

Recorded 2026-11-02. `compact_storage` is optional backend groundwork, not a
selected PCS or a complete transaction prover. Consensus remains disabled.

## Correctness comparisons

A bounded temporary release worker compared `MaskedTrace` with the pinned stock
masking/interpolation sequence. Original heights512/1024/4096 and trace coset
shifts1/11 produced identical full coefficient matrices, including all ten random
codeword columns. Deterministic public RNG seeds were used only for differential
fixtures, never for a published/private proof configuration.

For each case, natural-order tiles0..1,1..9,9..17 and17..27 on a shifted16N domain
equaled full stock coset FFT output. Insufficient tile budgets were rejected.
Evaluating back on the original shifted2N domain recovered every even source row.
Empty/out-of-range tiles and undersized evaluation domains were rejected.

`QuotientStrips` was used with actual pinned `batch_stark::prover::quotient_values`.
Eight2N cosets merged by `j+8*r` exactly matched full16N quotient evaluation for
N32/128/512 and shifts3/11. The fixture exercises first/last/transition selectors,
next-row access, nonlinear constraints and a full-period public program column.
The original N-row trace domain remained the selector domain in every strip.

The combined worker exited0 in0.975s with18.016MiB sampled peak RSS, under300s
CPU/wall and2GiB sampled-RSS stops. It proves equality for these cases, not all
possible fields, AIRs, lookups or complete prover executions.

## Full trace-dimension storage run

A separate bounded release worker used **65536 original rows×3384 relation
columns**, the provisional universal raw width before extra bridges/relays.
It consumed fresh OS entropy, constructed stock-interleaved masked coefficients
with131072 rows×3394 columns, recovered all source even rows in the first/last
relation columns, then evaluated a64-column tile on a1048576-row shifted coset.

It exited0 in **4.475s**, with **2805.641MiB sampled peak RSS** (~2.740GiB),
under300s CPU/wall and7GiB sampled-RSS stops. The host has64GiB physical RAM;
sampling may miss transient peaks. This is not a run on an8GiB physical machine.
The full expanded matrix was never allocated. It was a storage/tile run, not
full-width quotient constraints, Merkle commitment, FRI or a complete proof.

The source trace was a public synthetic matrix; no private user material,
coefficient dump, seed or proof bytes were retained. No native transaction was
accepted and no full-prover8GiB or5MB transport gate is closed by this run.

## Review findings and next boundary

Review found a generic matrix-to-owned conversion that cloned each complete tile.
The implementation now requires an owned row-major DFT result and returns it
directly, avoiding approximately1.7GiB extra overlap for a full-width2N strip.
Target height uses checked shifts/multiplication before allocation. User-authored
files/functions retain100/20 physical-line limits.

The repeatable `compact_storage` release integration test now passes in0.12s;
see [commands and scope](INTEGRATION_TESTS.md). The caller's `max_values` bounds
one output tile only, not FFT scratch or all live
buffers. Stock-compatible independent salts, streamed Merkle construction, input
matrix FRI hooks, claimed openings/reduction and stripwise orchestration still
need implementation and full differential proofs. The stock verifier, transcript
and mathematical construction should remain compatible. See the
[universal plan](UNIVERSAL_PROOF_PLAN.md) and [delivery gates](DELIVERY_PROGRESS.md).
