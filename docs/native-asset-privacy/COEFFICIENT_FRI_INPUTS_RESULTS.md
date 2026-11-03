# Coefficient-backed FRI inputs and openings

Recorded 2026-11-03. The optional compact backend now supports honest virtual
evaluation matrices, streamed input multiproofs and a real stock-verified FRI
regression. A complete bounded PCS and native compact suite are not selected.

## Implementation boundary

`CoefficientMatrix` owns/borrows masked coefficients, including Arc ownership.
Its logical dimensions are the FULL bit-reversed LDE, not coefficient storage.
Cells/rows return actual Horner evaluations. Batched queries FFT each bounded
column tile once and preserve supplied query order/duplicates. Invalid indices
and aggregate scalar budgets reject before FFT work. No placeholder values exist.

P3 Matrix requires an unchecked trait-method signature. A separate tiny
`matrix-adapter` crate supplies that one declaration with an entirely safe,
bounds/width-checked body and no unsafe operations/blocks. The proof crate retains
`forbid(unsafe_code)`. Generic P3 row, slice, packed and wrapping access are tested.

`InputMmcs` has two real storage branches. Generic commits delegate to stock,
including mixed heights. Typed coefficient commits use streamed commitments and
bounded query gathering. Minimal binary pruned paths match stock frontier order.
Both verifier methods delegate to stock. Clones share monotonic OS-seeded RNG
streams; seed draws are locked, while FFT/hashing run after releasing the lock.
No `HidingMmcs` marker or PCS one-opening security claim is made yet.

Pinned FRI is vendored at1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f. Four production
type/import edits let three input-opening paths accept `impl Matrix<Val>` while
retaining the six named generic parameters. Two existing upstream tests need
typed empty slices. No proof, transcript or verifier body changed. The10327
upstream Rust lines, licenses, original manifest, exact patches and SHA3 provenance
are retained/fingerprinted; third-party size exceptions are explicitly authorized.
Lockfile changes add the local adapter/FRI source without dependency version updates.

## Repeatable regressions

Seven Rust integration targets pass in one serial release run. New targets:

| Target | Evidence |
|---|---|
| compact_coefficient_matrix | Dense FFT equality, real generic Matrix access, Arc/borrow ownership, ordered duplicate queries, budgets;0.03s |
| compact_input_mmcs |1/2/16 matrices, stock cap/frontier reconstruction, single/multi proof verification, corruption, mixed-height fallback;0.04s |
| compact_fri_inputs | Actual coefficient-backed commitments→generic prove_fri→stock verify_fri;0.04s |

The FRI test uses D10,192 queries, blowup8, maximum8-ary folding, eleven columns
and8192 LDE rows. Real PCS transcript helpers bind context, commitments, claims
and batching challenge. Stock verification authenticates actual input openings
and the FRI relation. Independent transcript replay checks the same query indices.
Changed input leaves fail Merkle+FRI checks; changed FRI siblings retain input
authentication but fail FRI; changed context/claimed evaluations also reject.
Its small reduction is deliberately dense test code, not the production PCS.

Replace the target in [the regression command](INTEGRATION_TESTS.md) with any
target above. These are backend integration tests, not private-value node scenarios.

## Full trace-dimension commitment plus queries

A bounded release worker used65536×3384 synthetic relation cells, masked
coefficients131072×3394, a1048576-row commitment and64-column FFT tiles. It
opened192 queries, including duplicate/edge indices. Stock verification accepted
the multiproof with2316 boundary hashes. Three complete rows independently
matched Horner evaluation; a changed index rejected.

The worker exited0 in **108.943s**, with **3053.250MiB sampled peak RSS**
(~2.982GiB), under300s CPU/wall and7GiB sampled-RSS stops. The host has64GiB;
sampling can miss brief peaks. No full LDE, private user data, seed or proof
artifact was retained. This is not a physical8GiB-host run or a complete prover.

Remaining work includes production opening reduction, hiding/disclosure guards,
quotient orchestration, full semantic AIR composition, exact compact native codecs
and approved profiles. Whole-prover8GiB/5MB gates and six privacy scenarios remain
open in [delivery status](DELIVERY_PROGRESS.md).

After rebuilding the fixture, FFI and nodes for the new source fingerprint, the
native Owner acceptance/cryptographic rollback/peer-restart regression passed
again in9.22s. Its SignedTxn measured4,156,332 bytes and group4,159,583 bytes.
All shipped consensus versions and their suite catalogs remain disabled/empty.
