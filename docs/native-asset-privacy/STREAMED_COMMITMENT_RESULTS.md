# Streamed compact Merkle commitments

Recorded 2026-11-02. Optional `compact_storage::StreamedTree` commits masked
coefficient matrices through bounded FFT tiles. It is not a selected PCS or a
complete8GiB transaction prover. Native/default backend selection is unchanged.

## Exact compatibility contract

The component supports1..16 equal-height matrices, binary trees and cap height0.
It commits bit-reversed evaluation rows, preserving stable matrix order. Each
matrix receives its own independently drawn ten-element salt at every row;
draw order matches the stock matrix-major/row-major generator. No salts are shared.

Pinned `SerializingHasher<CompactHash<1>>` serializes canonical Fp31 cells as four
little-endian bytes, groups the combined stream into u64 words and pads four
zero bytes only when the COMPLETE combined row has an odd field count. Padding
must not occur separately per tile or matrix. Parent compression and all domains
reuse the existing compact hash primitive. `compact::CompactHash` is exported
for compatibility oracles; the fixed configuration still selects tags1/2/3.

The implementation retains one SHA3 state per row, independent matrix salts,
binary digest layers and one FFT tile. It never retains the full expanded matrix.
`cap`, `path` and `salts` expose compatible query data. Callers must supply actual
opened values; the component does not enforce PCS one-opening/hiding budgets.
Its tile scalar cap does not bound all live allocations or total process RSS.

## Repeatable differential regression

The release `compact_streamed_tree` integration target passed in0.07s. It made
15 commitments across five batch cases and tile widths1/3/64. Raw widths1,2,
mixed1+2,1+1 and sixteen cycling1..3 cover odd/even padding, matrix boundaries,
stable ordering and bit reversal. It compared exact stock caps/paths/salts at
75 queries and verified streamed openings with pinned stock MMCS verification.
525 changed-value/salt/path/shape/index cases rejected.

Empty/oversized batches, zero tile width, insufficient tile budgets, short domains
and mixed coefficient heights reject. These comparisons validate the tested
commitment construction, not an entire STARK, anonymity or cryptographic profile.

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test compact_streamed_tree --release --locked --offline -- --test-threads=1
```

## Full trace-dimension run

A bounded release worker constructed65536×3384 synthetic relation cells, fresh
masked coefficients131072×3394, and a complete1048576-row commitment through
64-column FFT tiles. Every original even row was recovered in the first/last
relation columns. For bit-reversed row913571, a direct Horner evaluation of ALL
3394 coefficient columns supplied an opening accepted by the stock MMCS verifier;
the same proof at a changed index rejected. No complete LDE was materialized.

The run exited0 in **81.645s** with **3053.734MiB sampled peak RSS** (~2.982GiB),
under300s CPU/wall and7GiB sampled-RSS stops. An earlier cap-only run completed
in80.550s/3053.688MiB. The physical host has64GiB; sampling may miss brief peaks.
This does not demonstrate operation on an8GiB physical machine. No private user
material, raw matrices, random seed or proof artifacts were retained in the repo.

The full transaction still needs claimed openings/reduction, FRI input storage,
single-use hiding guards, quotient orchestration and all semantic AIR regions.
Mixed-height batches and complete PCS multiproofs are not implemented here.
Its memory/time measurements cannot close whole-prover/transport gates. See
[universal plan](UNIVERSAL_PROOF_PLAN.md) and [delivery status](DELIVERY_PROGRESS.md).

After rebuilding the Rust fixture, FFI and nodes for the new source fingerprint,
the full native Owner rejection/acceptance/peer-restart regression passed again
in9.54s. Its SignedTxn measured4,156,335 bytes and group4,159,585 bytes. All four
Rust integration targets also pass. No current consensus flag or suite was enabled.
