# Coefficient-backed hiding PCS

Recorded 2026-11-03. The optional compact backend now has a complete candidate
PCS interface: fresh masked commitments, tiled opening reduction, real FRI,
quotient imports and STARK randomization. Native/default selection is unchanged.
This is backend evidence, not a complete private-value transaction prover.

## Compatibility and disclosure checks

The implementation uses Fp31/D10,192 queries, blowup8, maximum8-ary folding,
ten random codewords and independent stock Merkle salts. It returns the exact
stock hiding-PCS proof type. Verification delegates to the unchanged stock
verifier; no alternative acceptance rule or reduced security setting is added.

Private masks and salt streams start from OS entropy. Prover-data clones share
one opening lease. All requests are validated before atomically consuming their
leases, so duplicate/shared-state or hiding-budget rejection cannot partly
consume an unrelated commitment. A failure after that boundary requires fresh
commitments. Importing masked quotient LDEs requires fresh masks from the honest
prover; a dense matrix cannot establish entropy provenance or prevent deliberate
separate re-imports. This is an explicit trusted-prover precondition.

The regular commit API takes N trace rows and a2N domain, matching pinned
batch-STARK. Mixed heights within a commitment and public preprocessing are
explicitly unsupported; separate rounds may have different heights. The current
universal direction uses verifier-derived periodic columns.

Opening reduction retains one codeword per height, one compressed codeword and
one point's denominator/inverse pair. Column tiles never materialize the complete
wide LDE. Claimed evaluations share extension-field powers across columns.
Quotient imports validate the high-degree zero tail, then copy into a smaller
allocation instead of retaining full-LDE vector capacity. Caller limits count
base scalars, including all ten coordinates of extension values; they are not a
whole-process memory guarantee. See the [API guide](../../internal/assetprivacy/stark/src/coefficient_pcs/README.md).

## Repeatable proof-pipeline regression

Replace the target in [the regression command](INTEGRATION_TESTS.md) with
`--test coefficient_pcs`. The eight-target serial release run passed; its PCS
target took **6.50s**.
Strict release Clippy and authored file/function limits also passed.

The target covers multiple matrices,512/1024-row commitments in separate
rounds, repeated/translated points, claim order and ten hidden codeword claims.
The stock verifier accepts actual proofs and rejects modified context, claims,
hidden claims and commitment order. Hiding-budget rejection leaves the
transcript and leases usable; six points at512 rows pass and seven reject.
Duplicate clones cannot consume an unrelated fresh lease, and concurrent callers
sharing data get exactly one successful opening. Resource limits and unsupported
preprocessing reject before disclosure.

Actual one-note/512-row and two-note/1024-row ownership AIRs pass through the
candidate PCS, including private lookup/permutation, quotient and randomization
commitments. Their complete batch proofs verify using the original stock compact
configuration; changed verifier-owned programs reject. This demonstrates the
PCS integration for those relations, without spending or recipient encryption.

## Full-dimension synthetic PCS run

A bounded release worker committed65536×3384 synthetic relation cells, stored
131072×3394 masked coefficients, opened two extension points and192 authenticated
queries, and passed unchanged stock PCS verification. It exited0 in **169.650s**,
with **2545.844MiB sampled peak RSS** (~2.486GiB), under300s CPU/wall and7GiB
sampled-RSS stops. FFT tiles were64 columns on the1048576-row LDE. The host has
64GiB physical RAM; sampling can miss brief peaks. No secret seed, coefficient
dump or proof artifact was retained. This executes the candidate's complete PCS
path, without AIR semantics, quotient scheduling or the complete transaction.
The [optional capacity example](../../internal/assetprivacy/stark/examples/support/coefficient_pcs_capacity/README.md)
reproduces the geometry and verification path; its limits are not a RAM monitor.

## Remaining delivery gates

Whole-prover8GiB and5MB signed-transaction gates remain open. The current generic
batch prover still requests a full wide quotient-domain matrix; production
stripwise scheduling and full semantic composition remain required. Bounded
full-dimension PCS measurements are separate from complete semantic proving.
No compact native proof codec/profile is approved or selected, no private value
transaction is accepted, and the six real-node privacy scenarios remain0/6.
See [delivery status](DELIVERY_PROGRESS.md).

With fresh fixture/FFI/node builds for this source fingerprint, native Owner
registration, bad-proof rollback and peer restart passed again in8.34s. Its
NAPW was4,159,963 bytes, SignedTxn4,163,438 bytes and group4,166,691 bytes.
This remains a public-only policy registration flow with no private value moved.

An intermediate run caught a stale archive in a cached Go executable: the
constant archive marker's length check compiled away, leaving unchanged linked
object contents. The build marker now retains runtime data. An ordinary rebuild
embeds the current Rust reference and the node regression passes; source-pin
checks remain mandatory. The mismatched intermediate node rejected with503.
