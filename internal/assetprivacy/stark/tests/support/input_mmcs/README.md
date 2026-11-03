# Hybrid input-MMCS integration regression

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test compact_input_mmcs --release --locked --offline -- --test-threads=1
```

The typed streamed path commits actual Arc-owned masked matrices and returns
honest generic matrix metadata. One/two/sixteen-matrix batches are compared with
independent dense coset FFTs in stock bit-reversed order. Single and multiproofs
must verify through both the hybrid implementation and an independently constructed
pinned stock hiding-MMCS verifier. InputMmcs salts always use its OS-seeded API;
only public synthetic masking fixtures have deterministic test seeds.

Queries cover original ordering, duplicates, empty sets, adjacent leaves and a
complete1024-leaf single-matrix fixture. That complete public fixture exposes
every actual salt row, allowing an independent stock tree to be rebuilt over
the exact salted rows. Its cap and pruned-subset frontier must match byte-for-byte
without an insecure test-seeding hook. Full-tree queries need no boundary siblings.

Corrupted row values, salts, boundary digests, omitted/extra digests, query order,
caps and dimensions must reject. Constructor shape/budget errors, invalid queries
and aggregate output limits reject through the fallible API. Cloned commitments
must advance the shared private salt stream rather than repeat commitments.

The generic dense fallback separately commits64/32-row mixed-height matrices,
opens their correctly reduced row indices and verifies with the stock verifier.
This protects existing MMCS behavior while the typed fast path is integrated.

This is backend commitment/opening compatibility, not a complete FRI/PCS proof,
an asserted HidingMmcs marker, a private-data disclosure-budget argument, native
transaction coverage or a whole-prover8GiB measurement.
