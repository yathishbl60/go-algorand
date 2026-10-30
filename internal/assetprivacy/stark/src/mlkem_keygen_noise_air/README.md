# Private key-generation seed/noise composition

MlKemKeygenNoiseAir joins one fixed seed-expansion component and six existing
MlKemNoiseAir components. It has no public values and accepts no native operation.
All seven frames share one reusable private workspace and trusted stage schedule.

The root registers contain768 Boolean seed bits followed by six256-coefficient
vectors. Each frame binds its outputs to those constant registers. Noise nonces
are fixed0..5 and every child receives the same derived sigma. Coefficient bounds
come from the complete CBD child constraints, not just host witness generation.

The circuit has4096 rows,7826 main columns and28 preprocessing columns. Seven
512-row frames occupy the first3584 rows; no child is active in the final512.
Global transitions preserve proved roots through padding. `new` checks child
widths/heights before permitting construction. Schedule flags are not peer inputs.

`wires` exposes exact private seed/rho/sigma and polynomial slices. `trace` uses
the crate-private bounded writer. That writer validates its entire destination,
computes fixed auxiliary values, clears only its rectangle, writes each child
and installs global registers. It preserves outside data and avoids a second
full child matrix. No trace/writer has been run or benchmarked.

Matrix sampling, transforms, dot/error arithmetic and exact public-key packing
remain separate required stages. Viewing registration still rejects. No artifact
layout or callback can turn this partial relation into accepted possession.
See [the full contract](../../../../../docs/native-asset-privacy/KEYGEN_POSSESSION.md).

No tests, vectors, traces, setup, proofs or key-pair generation occurred. Static
checks provide no full-key possession, privacy, interoperability or runtime proof.
