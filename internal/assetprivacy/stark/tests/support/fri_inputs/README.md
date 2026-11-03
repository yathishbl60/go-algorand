# Coefficient-backed FRI pipeline regression

Run with the pinned Rust toolchain:

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test compact_fri_inputs --release --locked --offline -- --test-threads=1
```

This uses actual masked512-row source data: one relation column and ten random
codeword columns. `CoefficientMatrix` exposes honest8192-row bit-reversed LDE
values on the canonical `Value32::GENERATOR` coset. Typed `InputMmcs` commits
without a complete expanded matrix and supplies the actual generic FRI inputs.

The test evaluates all eleven polynomials at a transcript-derived extension
point, binds their claims through the stock PCS transcript, derives the batching
challenge and computes the stock weighted quotient word. It runs actual
`prove_fri` and `verify_fri` with published compact parameters: D10,192 queries,
blowup8 and maximum8-ary folds. The full field-degree check excludes base-domain
opening points. The polynomial degree domain has1024 points; the LDE has8192.

For an independent compatibility check, the verifier replays the actual FRI
commit/query phases and authenticates all input leaves with the stock hiding
Merkle MMCS directly, in addition to the stock FRI verifier's own input checks.

The valid argument passes both checks. An altered input leaf fails stock Merkle
authentication and FRI verification. An altered folding sibling leaves input
authentication intact and fails FRI verification. A changed public transcript
context and a false out-of-domain claimed evaluation also reject.

All setup salts and masks use fresh OS entropy. The test deliberately materializes
its small eleven-column reduction matrix. It is a backend integration regression,
not a complete bounded PCS, production hiding wrapper, native transaction test,
whole-prover8GiB measurement or security approval. No native profile is selected.
