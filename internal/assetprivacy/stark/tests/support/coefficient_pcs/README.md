# Coefficient PCS regression

Run the serial backend regression from the repository root:

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test coefficient_pcs --release --locked -- --test-threads=1
```

The test commits masked coefficients, opens commitments, and verifies the
returned proof with the original compact stock PCS. It covers mixed commitment
heights, multiple matrices, repeated and translated opening points, public and
random-codeword claim ordering, mutations, resource rejection, single-use
opening leases, shared clones, and simultaneous opening attempts.

The complete `OwnedNotesAir` cases use one note at 512 rows and two notes at
1024 rows. They build actual batch STARKs and verify them with the original
stock configuration. The conversion between batch-proof types moves identical
public fields; it performs no serialization or proof transformation.

These are proof-backend regressions. They do not demonstrate native private
asset transaction execution, consensus activation, the full recipient AIR,
the 8 GiB full-prover target, or the six private-network scenarios.
