# Scoped Owner artifact-bound proof regression

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy \
  --test scoped_owner_registry --release --locked --offline -- --test-threads=1
```

This serial integration test runs actual Owner preprocessing, a scoped NAPK7
registry with one real key, independent pin loading, fresh private proof generation,
canonical serialization, mandatory prover self-verification and artifact-bound
verification. Public fixture secrets are test data, not a production RNG strategy.
Private proof masks still come from the published OS-seeded prover path.

The absent Unshield kind cannot bind. Malformed flags/roots/dimensions/kind/order,
markers/count/lengths, an empty scope, wrong pins and duplicate encoder entries
must reject. Malformed files are re-pinned when exercising canonical semantics.
Changed owner/group values, proof bytes, relation asset and preprocessing root
must not authenticate the original proof. A changed pinned root also invalidates
the original public setup. Every unsupported record is an explicit absence,
never a dummy preprocessing key.

The serialized proof must remain below the4,900,000-byte proof cap. This is not
a complete signed-transaction size measurement, Go/Rust ABI conformance test,
native account authorization check, ledger transition or node scenario. Those
remain separate integration obligations. No complete privacy security claim follows.
