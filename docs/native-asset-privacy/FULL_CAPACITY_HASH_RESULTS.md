# Full-capacity compact hash result

Updated 2026-11-01. Actual standalone primitive proving at maximum supported
capacity. **Not** a full private transaction, selected profile, native acceptance
or proof that the complete local prover meets its8GiB limit.

## Executed configuration and outcome

The [compact Keccak AIR](../../internal/assetprivacy/stark/src/compact_keccak_air/README.md)
processed1365 independent permutations,32760 active round rows and8 dummy rows.
Total height32768; raw width1920; trusted preprocessing11; symbolic degree5.
The raw trace occupied251658240 bytes. No private-pad or lookup columns were
included. Inputs alternated two public SHA3/SHAKE permutation fixtures; fresh
OS-backed masks/salts were used. Dense allocation dimensions do not rely on
those inputs being secret or distinct; this is not a worst-case CPU benchmark.

The worker used actual repository-vendored P3, the optional Fp31/D10 arithmetic,
192 queries, blowup8, binary FRI, ten random codewords and ten salt coordinates.
Its hash/transcript domains remain validation-specific, not native-profile IDs.
Setup, trace generation, proving and verification ran in the same worker.

| Observation | Result |
| --- | ---: |
| Actual proof verification |Passed |
| Altered proof opening |Rejected |
| Quotient chunks |16 |
| Canonical-layout byte counter |3519753 |
| Wall time |68.013 seconds |
| Sampled peak RSS |5636.000MiB =5.50390625GiB |

The byte counter follows actual proof metadata with four-byte base values;
no artifact-bound native codec serialization occurred. Fresh query randomness
affects size. No private trace, raw proof or secret seed was persisted.

## Bounds and interpretation

The supervisor disabled core dumps and imposed300-second CPU/wall limits plus
a7GiB sampled RSS stop threshold. RSS was sampled approximately every quarter
second; this does not give an exact peak or a hard kernel memory limit.
The host had64GiB physical RAM. The observed process footprint fits below8GiB;
the run is not evidence of operation on a physical8GiB machine under pressure.

At10/20/30/40/50/61seconds the sampled RSS was approximately4245/4292/4330/
4330/4330/5560MiB. The worker completed normally within both stop bounds.
The current static standalone prover retained all of this one AIR's commitment
data. No streaming adapter was used.

This resolves one material uncertainty: a real proof for the largest hash-table
capacity can execute below8GiB RSS. It does **not** account for private pads,
lookup auxiliaries, complete envelope tables, all required roles, their public
programs, global quotient, regeneration or final joint proof assembly.
The complete pipeline still requires the [multipass allocation design](STREAMING_PROVER_DRAFT.md)
and a complete resource-constrained statement. Stock batch proving retains all
instances and cannot inherit this one-component bound by summing proofs.

No automated test files/suites were added/run. Scratch worker and supervisor:
`/tmp/algorand-compact-feasibility`, mode `--full-hash`. This temporary worker is
not a maintained test deliverable. [Small primitive evidence](COMPACT_PRIMITIVE_RESULTS.md)
and [delivery status](DELIVERY_PROGRESS.md) retain the remaining boundaries.
