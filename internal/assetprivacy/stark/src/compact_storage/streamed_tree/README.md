# Streamed compact binary commitments

`StreamedTree` commits one to sixteen equal-height masked-coefficient matrices
on one caller-fixed coset, using bounded column FFT tiles. It retains independent
salts and digest layers, not the expanded matrices. It is an experimental storage
primitive; no native suite, PCS, FRI or proof selection changes.

`commit(inputs,domain,tile_columns,max_tile_values,dft,rng)` accepts immutable
`MaskedTrace<Value32>` references in the commitment's matrix order. Inputs must
have the same coefficient height and coherent canonical roots. It uses scalar
`Radix2Dit<Value32>`. Tiles are natural-order FFT output; their rows are mapped
to the bit-reversed indices committed by the pinned FRI backend.

The caller supplies fresh private cryptographic entropy. Every matrix receives
ten independent field salts per committed row, drawn matrix-major then row-major,
matching the pinned hiding MMCS. Salts are stored; no public seed, recomputation
shortcut, shared salt or derived salt is introduced.

For each committed row the exact leaf sequence is:
`matrix0 values || salt0 || matrix1 values || salt1 || ...`.
The pinned `SerializingHasher<CompactHash<1>>` targets eight u64 digest words.
Value32 emits canonical four-byte little-endian encodings; the u64 stream packs
pairs of fields and zero-pads four bytes ONLY when the complete sequence has an
odd field count. Tiles and matrix boundaries introduce no padding or delimiters.
Shared hash helpers retain the existing domain/tag and little-endian digest words.
Binary node compression uses `CompactHash<2>` on left then right digest words.

`cap()` returns the stock-compatible root-only Merkle cap. `path(index)` returns
bottom-up binary siblings and `salts(index)` the independent per-matrix salt rows.
Indices are the committed bit-reversed row indices. The caller supplies the actual
opened matrix values; stock MMCS verification must check them against the cap.
Invalid query indices return None. No private state implements Clone, Debug or
serialization. Public typed hash access supports independent stock comparisons.

`max_tile_values` caps one FFT tile, not aggregate memory, hash states, retained
salts, digest layers, scratch or all proving phases. Checked geometry/allocation
failures return None. This primitive does not enforce one-opening disclosures,
zeroization, FRI hiding budgets or a complete8GiB prover bound. Mixed-height
injections, nonbinary trees and nonzero cap heights are deliberately unsupported.
