# WHIR candidate: public geometry assessment

Updated 2026-11-01. This is metadata-only evaluation of a possible flattened
univariate-column adapter. No WHIR commitment, setup, private witness, proof,
verification or native transaction was executed. No suite adoption or security
approval follows from constructor acceptance or the numbers below.

## Source and executed scope

The temporary driver in `/tmp/algorand-whir-geometry` constructed actual
`p3_whir::ZkWhirConfig` from Plonky3 revision
`1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f`, with Rust 1.99.1. It used the node's
public Config-associated Challenger, Goldilocks Value and degree-five Challenge
extension. Offline lock generation and release builds succeeded with a separate
`/tmp/algorand-whir-geometry-target`; shared dependency versions/sources matched
the repository lock. No repository dependency declaration changed.

`geometry.txt` records public results; README and Cargo.lock describe the driver.
These temporary files are metadata evidence, not a committed prover or wire codec.
The grid used log_source_cells = 21/24/25/26, requested per-term targets
128/256/288/304/320, PoW 0, starting log inverse rate 3, default later-round rates,
Constant(4) folding, CapacityBound, ell_zk = 4 and mask log inverse rate = 1.

## Constructor and initial-claim checks

All 128/256 constructors succeeded and independently passed validation of two
initial claims. All 288/304/320 constructors rejected the zero-PoW budget before
that claim-validation step; no grinding was executed.

| log source cells | Two-claim coordinate at constructed 128/256, bits | Derived required PoW at 288/304/320 |
| --- | ---: | --- |
| 21 | 287.678071905 | 19/35/51 |
| 24 | 284.678071905 | 27/43/59 |
| 25 | 283.678071905 | 28/44/60 |
| 26 | 282.678071905 | 29/45/61 |

At 256, intermediate query counts are 88/44/29 at log 21 and 88/44/29/22 for
the other cases. Randomized terminal counts are 24/20/19/19; mask queries are
279/280/280/280. These are derived schedules, not end-to-end soundness guarantees.

## Dense oracle geometry

Each interleaved source row has 16 cells. The initial oracle uses 8-byte base-field
cells; later oracles use 40-byte extension cells. Default inverse-rate growth
means domain rows shrink more slowly than message length, causing substantial
later allocations despite folding.

| log source cells | Initial base codeword | First extension codeword |
| --- | ---: | ---: |
| 21 | 128 MiB | 320 MiB |
| 24 | 1 GiB | 2.5 GiB |
| 25 | 2 GiB | 5 GiB |
| 26 | 4 GiB | 10 GiB |

These are nominal individual dense matrices, not measured peak memory. Cached
P3 DenseStorage<Vec> conversion consumes the owned vector, and bit reversal can
operate in place; these counts assume no automatic second full DFT copy. Other
live oracles/messages, FFT tables, Merkle data, masks and scratch remain additional.

The 256-target schema-derived field payloads are 724,376/907,424/907,344/908,624
bytes in the same log-cell order. They assume 8-byte base and 40-byte extension
encodings and count evaluations, sumchecks, OOD answers, blinded reveals and
query rows. They exclude roots, paths, salts, framing/list lengths and the initial
commitment. They are **not proof sizes**, a complete upper bound or native-fit evidence.

## Security and integration gates remain open

The public first-intermediate far-word query diagnostics are approximately
84.903 bits at 128 and 169.806 bits at 256, below those requested targets. P3 labels
them Approximation, not conservative bounds. Full intermediate hiding reduction
is Unavailable. The ideal-IOP base-case RBR diagnostic is 0 bits/Conditional
(vacuous). Neither these diagnostics nor constructor success certify compiled
PCS security, privacy, Fiat-Shamir composition or the application's relation.

The primary [Zero-Knowledge IOPPs for Constrained Interleaved Codes](https://eprint.iacr.org/2026/391)
preprint provides honest-verifier zero-knowledge IOP-of-proximity constructions
and composable reduction results. It does not establish that this adapter,
implementation, field/parameter choice or Algorand protocol meets those properties.

A ZK PCS is not a complete AIR proof. A new univariate adapter and private joins
between outer constraints, column commitments and openings remain necessary.
There is no closed 8 GiB/5 MB profile, approved replacement, successful full-node
result or resolution of [the current feasibility gate](FEASIBILITY_RESULTS.md).
Every native consensus version remains disabled. Full relation/security analysis,
complete serialization and measured proving/verification resources must precede
any adoption decision; all six real-node scenarios remain in
[delivery status](END_TO_END_STATUS.md).
