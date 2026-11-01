# Whole-transaction privacy feasibility gate

Updated 2026-11-01. The [serial Owner result](SERIAL_OWNER_RESULTS.md) improves one
primitive. No complete native private transaction has been accepted, and the
complete requested workflow has not passed its resource/transport gate. Current
Shield cannot fit existing transport; no approved suite or activation exists.

## Necessary query bytes before private construction

For each current layout, `(W + P + 5) × 192 × 8` counts only the main and
preprocessing query rows plus five hiding columns. Duplicate queries retain rows;
pruned Merkle paths do not remove these values. It omits quotient/random batches,
OOD evaluations, salts, paths, framing, PQ signatures and transaction bodies.

| Current relation or complete Shield set | Necessary bytes |
| --- | ---: |
| Serial Owner registration | 3024384 |
| Shield main | 11756544 |
| One authorized recipient | 41393664 |
| One assigned disclosure | 40005120 |
| Shield main + two recipients | 94543872 |
| Shield main + two recipients + four disclosures | 254564352 |

The base Shield set needs both recipients. One mandatory auditor/controller role
adds two envelopes/proofs; both roles add four. Omitting those proofs changes the
required relation. Current transaction gossip caps at5000000 bytes, so even the
necessary bytes rule out these complete Shield sets without running a prover.

[Layout::minimum_query_bytes and minimum_bundle_query_bytes](../../internal/assetprivacy/stark/src/verification/selection/encoding_floor.rs)
expose these lower bounds. Bundle preflight checks count/overflow, not legal
operation ordering or authorization. `ProofBudget` rejects a configured wire or
allocation budget below the floor before the private builder. A metadata-only
worker returned the stated Owner/base-Shield/full-role totals, accepted the
Owner requirements at5MB, rejected wire/allocation budgets one byte below its
floor, and rejected empty/eight-layout lists. That run built no setup, trace or
proof and did not exercise artifact-bound proving, native acceptance or peak-RAM enforcement.

## A joint bit core is still a large dense matrix

For one hypothetical joint main width W=1920, current hiding/blowup gives
`main_LDE_bytes = 128 × N × (W + 5)`. This is only one matrix:

| Rows N | W1920 main LDE, bytes | GiB | Max W for a 4GiB main LDE alone |
| --- | ---: | ---: | ---: |
| 65536 | 16148070400 | 15.0390625 | 507 |
| 131072 | 32296140800 | 30.078125 | 251 |
| 262144 | 64592281600 | 60.15625 | 123 |

The required local prover target is 8GiB peak RAM on an ordinary developer laptop.
It covers the complete pipeline: setup/preprocessing, FFTs, auxiliaries, assembly
and maximum mandatory roles. A closed complete-candidate budget is still missing.
The 1920-column example covers only one state/parity core, not that full pipeline.
Four GiB is an illustrative half-budget, not an approved deployment ceiling or
proof of 8GiB peak feasibility. Quotient, Merkle, FRI, temporaries and allocator
overshoot remain additional. More rows do not reduce query-cell width cost.

## Compression and batching boundaries

Packing persistent Boolean roots into16-bit private limbs is useful groundwork:
carry limbs across rows and constrain each active child decomposition exactly.
Packing recipient/envelope Boolean prefixes alone would still leave width18629,
raw main-query cells28614144 bytes and main LDE156313321472 bytes at65536 rows.
ML-KEM coefficient state and large encryption scratch remain; this is no full fix.

Current hiding PCS adds private random rows to each column. Constant original
bits therefore do not imply compressible masked openings. Skipping openings or
reusing masks requires a different justified construction. The privately selected
recipient key/account/owner/epochs must not be promoted into public preprocessing.

Pinned batch-STARK shares commitments/FRI but still opens every matrix column:
query bytes retain the sum of widths, and its prover currently clones traces.
Batching already-complete AIRs can amortize overhead; it cannot erase private
joins. Its optional per-AIR lookup terminals are public. A single private flush
with terminal `1/(alpha+t)` exposes t from public alpha and terminal, illustrating
why an unmasked private-join terminal is unacceptable without a hiding argument.
Pinned multi-STARK explicitly provides no zero knowledge; its packed Boolean/
subfield backends are not a drop-in privacy replacement.

## Required next design, before another implementation expansion

Specify one complete joint Shield statement: two recipient envelopes and every
required 0/2/4 additional role envelope, private value/key/epoch/account joins,
canonical ML-KEM coefficient state, hashing, encryption and conservation.
Choose a narrow representation with explicit range/Boolean/copy constraints;
retain all privacy roles and current cryptographic security parameters.
Shield is the first milestone; the final 8GiB/wire gate also covers viewing-key
registration, spends, recovery and all roles, with finite full-pipeline/PQ bounds.
Typed artifacts, preprocessing, codecs and native verifier routing must match.
No such complete replacement design or accepted native result is established.

## Pinned source basis

P3 revision1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f was read from the local pinned
checkout; browser retrieval of GitHub was unavailable during the review.
See [hiding PCS](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/fri/src/hiding_pcs.rs),
[batch proving](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/batch-stark/src/prover.rs),
[lookup terminal fields](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/batch-stark/src/proof.rs),
and the [non-hiding multi-STARK contract](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/multi-stark/src/contract/mod.rs).
[Serial design bounds](SERIAL_OWNER_LAYOUT.md) and [delivery gates](DELIVERY_PROGRESS.md)
separate source/primitive evidence from the missing complete native result.
