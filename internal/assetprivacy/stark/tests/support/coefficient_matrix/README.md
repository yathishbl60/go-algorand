# Coefficient matrix backend differential regression

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test compact_coefficient_matrix --release --locked --offline -- --test-threads=1
```

This integration target compares the logical coefficient-backed LDE with a
complete, independent pinned coset FFT and stock bit-reversed row permutation.
Public fixtures use512/1024 original rows, different shifts and relation widths,
including all ten random codeword columns. Production masks require fresh private
entropy; deterministic fixture seeds are deliberately public test data.

Direct rows/cells and generic P3 Matrix access must match the dense evaluations,
with the full target height rather than coefficient height. Borrowed, owned and
Arc-owned sources are exercised; the original Arc is dropped before row access.
Safe generic row/slice/get, wrapping rows and horizontally packed rows exercise
the stock subsequence defaults without introducing unsafe test code. These calls
also detect circular Matrix fallback behavior.

FFT gathering must preserve out-of-order queries and duplicates for tile widths
1/3/larger-than-matrix. Exact tile/output budgets pass; one-cell-short budgets,
invalid indices/columns and invalid geometries reject. Empty queries retain their
documented semantics. The concrete DFT interface does not permit a counting probe,
so this test makes no measured claim about work performed on rejection paths.

This verifies matrix/query compatibility, not complete FRI/PCS proof generation,
native transactions, cryptographic security or the whole-prover8GiB gate.
