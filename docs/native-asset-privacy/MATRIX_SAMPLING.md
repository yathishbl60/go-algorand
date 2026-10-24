# Candidate matrix sampling and key admission

Updated 2026-10-24. Disabled implementation components; no accepted native proof.
This records a concrete candidate protocol choice, not a security approval.

The candidate fixes a 16-block SHAKE128 prefix for each ML-KEM-768 matrix entry:
2688 bytes, containing 1792 contiguous 12-bit candidates. Keep the first 256
values strictly below 3329 in order. Never pad, skip accepted values, resample
the seed or substitute a different polynomial on exhaustion.

The [Go envelope admission checker](../../internal/assetprivacy/envelope/README.md)
requires all nine entries to finish within this bound. It checks the canonical
1184-byte key and all 768 public coefficients before the bounded matrix checks.
Each entry uses the final 32 key bytes followed by the exact two coordinate
bytes. All coordinates 0–2 are covered. Standard ML-KEM parsing runs afterward,
so its internal expansion cannot require a larger prefix for an admitted key.
Native viewing-key record validation inherits this check.

This is an additional candidate admission condition. Otherwise standard-valid
keys are rejected if a prefix exhausts; no exhausting seed was demonstrated and
coverage of all standard keys was not proven. Standard key generation and seed
restoration remain unchanged and do not silently retry. A wallet must explicitly
check candidate admission before registration/use. Changing the bound later
requires coordinated profile/consensus compatibility work, not a proof option.

The [SHAKE128 AIR](../../internal/assetprivacy/stark/src/shake128_matrix_air/README.md)
constrains padding, complete Keccak state continuation and every squeezed byte.
The [sampler AIR](../../internal/assetprivacy/stark/src/mlkem_sample_air/README.md)
constrains comparison, first-accepted ordering, output selection and completion.
The [composed matrix AIR](../../internal/assetprivacy/stark/src/mlkem_matrix_air/README.md)
uses the same private prefix columns for both and trusted fixed preprocessing.
`MlKemMatrixAir::candidate(first, second)` selects the matching 16-block bound.
The [compact successor](../../internal/assetprivacy/stark/src/mlkem_stream_matrix_air/README.md)
streams each squeeze and candidate window using 4872 columns and 4096 rows.
[Bounded full KEM composition](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/README.md)
now binds the actual key/rho, all nine coordinates and polynomial operations.

Go's A[i,j] appends j then i. K-PKE encryption consumes A-transpose; the parent
must preserve both conventions when connecting private products and vector sums.
No seed, matrix coefficient, candidate, cursor or consumed recipient key is made
public for composition. No cross-AIR private lookup bus is used.

The candidate matrix AIR is 24,932 columns by 2048 rows plus 1808 preprocessed
columns. Raw matrices occupy 408,485,888 main and 29,622,272 preprocessing bytes
per entry, before hiding/prover overhead. Direct writers allocate the enclosing
trace once and only a Keccak temporary. Full KEM feasibility remains unmeasured.

Source reviews, tagged/default builds, vet, Clippy, formatting, local links and
file/function limits are the current checks. No crypto vectors, traces, tests,
constraint evaluations, proofs, node execution or admission-runtime checks ran.
None of those build results proves cross-language equivalence or security.
