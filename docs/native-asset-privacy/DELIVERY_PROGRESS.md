# Native privacy delivery gates and risks

Updated 2026-11-02 after reviewing code volume and feasibility. The earlier 45%
figure was a subjective work-package score. It is withdrawn as a delivery metric:
there is not enough runtime evidence for a reliable overall completion percentage.
The complete [north-star scope](IMPLEMENTATION.md) remains the intended end state.

## Intended end state

Native Algorand asset privacy: existing/new ASAs support the planned privacy modes,
with consensus checking proofs, commitments, nullifiers, supply and authorization.
The scope also includes opt-in issuer/auditor controls, SDK/wallet flows and six
real-node scenarios. It is a protocol extension plus a cryptographic system and
client tooling, rather than a single isolated application feature.
The user selected an 8 GiB peak-RAM ceiling for the complete local proving
pipeline, including preprocessing, FFTs, auxiliary proofs and assembly.

## Authoritative progress

| Delivery gate | Evidence now |
| --- | --- |
| Transaction/state/crypto/client source integration | Substantial inactive code exists |
| Builds, static analysis and selected generation checks | Passed for completed slices |
| Owner primitive proofs | Earlier and [serial-layout bounded runs](SERIAL_OWNER_RESULTS.md) verified |
| Compact field/note/hash primitives | [Bounded Fp31 proofs](COMPACT_PRIMITIVE_RESULTS.md) verified with the vendored compatibility fix; no complete profile |
| Private ownership composition | [One/two-note joint proofs](OWNED_NOTES_RESULTS.md) verified; mismatched private links rejected; complete spending absent |
| Compact coefficient storage | [Full trace-dimension storage/tile run](COMPACT_STORAGE_RESULTS.md) completed at2.740GiB sampled RSS; complete PCS/prover absent |
| Largest hash-table capacity | [1365-permutation proof](FULL_CAPACITY_HASH_RESULTS.md) verified in68s, sampled peak5.504GiB; excludes complete statement |
| Accepted end-to-end native private transactions | None demonstrated |
| Six real-node scenarios | 0/6 |
| Automated regression tests | [Ownership pipeline, storage differential and existing two-node ASA regression](INTEGRATION_TESTS.md) pass; native privacy node coverage pending |
| Approved suite/artifacts and protocol activation | None; all current versions disabled |

Git changes from the foundations parent through commit 2e8fa9c14 total 73,807 net
added lines: 57,909 source/scripts, 11,337 documentation, 2,942 schemas/configuration,
and 1,619 generated/lockfile lines. This inventory excludes subsequent local-prover
and layout work and does not establish implementation quality or completeness.

## Material rework risks

The recipient relation currently declares 65536 rows×24869 columns. At eight
bytes per Goldilocks element its main trace alone is 13,038,518,272 bytes
(~12.14 GiB), before preprocessing, hiding expansion, LDE/quotient matrices and
other prover allocations. These are source counts, not measured memory usage.
Practical proving and network-proof sizes remain unresolved.

Custom AIR constraints, proving schedules, artifact/proof formats and resource
assumptions have the highest redesign risk. Ledger/API/client scaffolding may
be reusable, but changes in the cryptographic architecture can affect it too.
No claim that all current code will ship is justified by compilation alone.

## Next feasibility gate

The bounded Owner primitive passed. Next resolve the failed layout/transport gate,
then run one complete native private transaction through ordinary node validation
on an isolated local network.
Record proof size, time and peak memory; exercise invalid-proof rejection and
atomic rollback. This must use the actual protocol path and cryptography.
Expand other features only after the critical path has credible evidence.
The [full workload](PROOF_WORKLOAD.md) and [WHIR assessment](WHIR_ASSESSMENT.md)
now quantify design costs. The historical [private-join draft](PRIVATE_JOIN_DRAFT.md)
is superseded as a direction by [one universal AIR](UNIVERSAL_PROOF_PLAN.md),
which preserves existing SHA3 contracts and avoids cross-AIR terminal pads.
The [compact FRI assessment](COMPACT_PROFILE_ASSESSMENT.md) records joint ownership
proofs, a conditional sub-5MB wire budget and an unmeasured allocation draft.
Its complete 8 GiB/5 MB gates remain open; current profile/defaults are unchanged.

Bounded proof/setup and isolated-node execution are now authorized. The first
Owner primitive passed; static transport/memory bounds block the full candidate.
Runtime outcomes may require redesign or removal of speculative implementation.
The detailed [status ledger](END_TO_END_STATUS.md) records remaining source gaps.
