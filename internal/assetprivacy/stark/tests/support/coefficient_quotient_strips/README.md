# Strip quotient regression

Run serially from the repository root:

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test coefficient_quotient_strips --release --locked -- --test-threads=1
```

The test builds complete `OwnedNotesAir` batch proofs for one note at 512 rows
and two notes at 1024 rows. Every proof is verified with the original compact
stock configuration and its coefficient PCS configuration. The actual
STARK degree validator must accept extended height 17 as original height 16
and reject extended height 18 using this backend's advertised bounds.
The AIR includes 1,988 main columns, all 57 periodic
columns, and its actual lookup permutation.

For each case, one run compares every quotient value from eight strips with
the original full-domain evaluator over the same committed private matrices
and sampled challenges. A second run limits the evaluation budget to
3,000,000 or 6,000,000 base-field values. This admits the strips and quotient
chunk LDEs while rejecting the wide full-domain trace evaluation.
The preflight budget includes main and permutation matrices, three copies of
periodic data, four selectors, strip quotient values, and the full quotient
output, with extension elements counted as ten base-field values.

The wrapper requires exactly eight evaluator calls, each at twice the original
trace height, and requires serial instance scheduling. Invalid geometry,
preprocessing, and insufficient evaluation budget must reject before any
callback runs. No mutex or new dependency is used.
Callbacks returning one too few or one too many strip values must reject with
`Shape`; an evaluator's `Allocation` error must propagate after its first call.

An optional ignored case uses two notes at 65,536 rows with the full worker's
limits: 64 tile columns, 67,108,864 tile values, 261,881,856 coefficient values,
400,000,000 evaluation values, 2,000,000 claim values, and 5,000,000 query values.
It verifies the full proof with the stock configuration and rejects a different
one-note program at the same height. The small test limits remain unchanged.
Run this heavy case only under an external process monitor that enforces an
8 GiB process-tree RSS cap and a time limit; these scalar limits do not guarantee
an RSS bound. After arranging that monitor, use this exact command:

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test coefficient_quotient_strips --release --locked -- \
  --ignored --exact strip_quotient_owned_notes_full_height_stock_verification \
  --test-threads=1
```

These are backend and OwnedNotes regressions. They do not establish the full
recipient semantic prover's memory bound, private-value native execution,
protocol activation, or completion of the six private-network scenarios.
