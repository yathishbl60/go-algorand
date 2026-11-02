# Streamed compact commitment differential regression

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test compact_streamed_tree --release --locked --offline -- --test-threads=1
```

This backend integration test compares `StreamedTree` with the pinned stock
`MerkleTreeHidingMmcs`. The oracle materializes complete masked LDE matrices,
applies the stock bit-reversed row permutation and uses the same public test salt
RNG seed. Production masking/salting must use fresh private cryptographic entropy.

Five input batches cover one/two/sixteen matrices, odd/even widths, odd combined
field streams and two odd-width salted matrices whose combined stream is even.
Each batch uses tile widths1/3/64. Caps, bottom-up paths and per-matrix salts must
match exactly at five nontrivial MMCS indices. The stock verifier must accept
the streamed paths/salts and reject altered values/salts/siblings, missing proof
elements, wrong row widths and changed indices. Out-of-range queries and malformed
commitment geometries also reject, including mixed coefficient heights.

This is a differential commitment/opening regression, not a complete FRI/PCS
proof, native transaction, node scenario or8GiB whole-prover measurement.
