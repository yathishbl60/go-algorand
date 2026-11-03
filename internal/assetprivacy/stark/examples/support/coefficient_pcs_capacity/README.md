# Explicit full-dimension PCS capacity run

This optional heavy example commits a synthetic public matrix of 65,536 rows
and 3,384 relation columns, opens two OOD points with 192 FRI queries, and
checks the result using the original compact stock PCS verifier.

From the repository root, run explicitly:

```sh
cargo run --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --example coefficient_pcs_capacity \
  --features native-privacy-compact --release --locked
```

On macOS, prefix that command with `/usr/bin/time -l` to measure peak RSS;
on Linux, use `/usr/bin/time -v`. Build time is included unless built first.
The matrix is deterministic public data, but masking and commitment salts use
fresh OS entropy. The example writes no private artifacts and accepts no
smaller geometry or reduced security settings.

Limits count base-field scalars: 64 tile columns, 67,108,864 tile values,
444,858,368 coefficient values, 500,000,000 evaluation values, 67,880 claim
values, and 651,648 queried values. These are individual buffer bounds, not
an aggregate resident-memory guarantee.

The equivalent precursor run measured 169.650 seconds and 2,545.844 MiB peak
RSS on the development host. Timing and memory vary by host. This validates
full-dimension PCS capacity; it does not establish the complete semantic
prover's 8 GiB bound or native transaction acceptance. Final output preserves
`complete_semantic_prover=0 native_transaction_accepted=0` for that reason.
