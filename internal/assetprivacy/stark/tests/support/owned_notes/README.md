# Compact ownership proof-pipeline regressions

Run with the pinned Rust toolchain and cached dependencies:

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test compact_owned_notes --release --locked --offline -- --test-threads=1
```

This executes actual setup, proof generation and verification using the published
OS-seeded compact configuration. Fixtures cover one/two notes, padded height,
amount endpoints, altered openings, changed network/asset/policy/group, malformed
witnesses, inverse/program-relay corruption and three indexed private-link swaps.
The link swaps pass ordinary local constraints. In release they generate nonzero
lookup terminals and the verifier must reject. Debug P3 instead checks global
lookup balance before proof publication; the test requires rejection there.

Execution is serial to avoid overlapping the six proof generations. These are
small ownership-component pipeline tests, not native-node or whole-transaction
end-to-end coverage. They establish no anonymity, ZK or complete security claim.
