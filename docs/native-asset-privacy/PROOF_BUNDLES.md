# Exact main/output proof bundles

Updated 2026-10-25. Native spend/recovery snapshots now derive immutable bundle
plans. Go and Rust implement the same candidate framing; Rust has typed dispatch
that requires actual backend verification of every selected component. No bundle
round trip, cross-language conformance check or proof execution has run. Native
transactions and approved artifacts/pins remain unfinished; the [native bridge](NATIVE_VERIFIER_BRIDGE.md) links.

## Expected proof set

`AssetPrivacySpendPlan.Bundle()` and `AssetPrivacyRecoveryPlan.Bundle()` derive
the expected descriptors/vectors from the existing current-state plans. Generic
`proofbundle.Spend`/`Recovery` require trusted plan provenance. They compare each
output's domain, commitment and ordinal with the main statement and require the
correct recipient kind followed by exactly its planned auditor/controller roles.
Output metadata/vectors and native Domain/Suite/Group are value snapshots; plan getters copy.

The required order is one main proof, then output 0's recipient/auditor/controller,
then output 1's roles. Optional roles follow immutable native policy, never a wire
choice. Controller without auditor is allowed when that is the policy. Maximum
count is seven, including the main proof. Current supported main relations are:

| Main kind | Outputs | Required recipient relation |
| --- | --- | --- |
| Transfer | 2 | ordinary current-key/current-eligibility recipients |
| Unshield | 0 | none |
| Partial private recovery | 2 | recovered receipt 0; original-owner remainder 1 |
| Full private recovery | 1 | recovered receipt 0 |
| Partial public recovery | 1 | original-owner remainder 0 |
| Full public recovery | 0 | none |

Shield and other arities still need native main plans and bundle integration.
Zero-knowledge ownership/eligibility/conservation constraints and issuer authority
checks retain their existing separate obligations; framing does not establish them.

## Candidate framing

All bundle integers use **big endian**; the nested NAPSTK codec uses little endian.
The header is ASCII `NAPB`, byte version `1`, then `u16` proof count (seven bytes).
Each entry is `kind:u8`, `output:u32`, `role:u8`, `proofLength:u32`, then exactly
that many bytes of one [NAPSTK proof](../../internal/assetprivacy/stark/src/verification/README.md).
The descriptor/length header is ten bytes. No public vector, key, work limit,
profile selector, statement digest or optional-role selector is carried here.

| Kind byte | Meaning |
| --- | --- |
| 1, 2 | transfer, unshield main |
| 4, 5, 6, 7 | partial-private, full-private, partial-public, full-public recovery main |
| 17 | ordinary recipient |
| 18 | assigned auditor/controller disclosure |
| 19 | original-owner retained recipient |
| 20 | recovered receipt with current recipient keys |

Main uses output `0xffffffff`, role `0`. Output ordinals are `0` and `1`; roles
are recipient `1`, auditor `2`, controller `3`. Kind `3` is not accepted. Missing,
extra, duplicated, reordered, mismatched or trailing entries fail. Same-sized
vectors do not permit substituting a weaker recipient relation. The outer
[NAPW frame](../../internal/assetprivacy/witness/README.md) remains separately bound
to the trusted profile and recomputed finalized-body group statement.

## Rust checking and developer integration

Construct `verification::bundle::Expected` with `transfer`, `unshield`, `recovery`,
`recipient`, `disclosure`, `retained` or `recovered`. Each constructor requires the
corresponding concrete `Verifier<AIR>` type and exact native public vector. The
private dispatcher has six concrete verifier variants; it accepts no user-supplied
success callback. All selected artifacts must share one profile and registry ID.

`encode_bundle(expected, proof_slices, limits)` frames existing per-AIR proofs.
It checks descriptors, vector dimensions, composition and artifact headers;
serialization itself does not verify proofs. `verify_bundle(bytes, expected,
bundle_limits, proof_limits)` first checks the complete framing and all selected
artifact IDs/degrees before any cryptographic verification. It then invokes every
actual per-AIR verifier sequentially. One rejection rejects the entire bundle.

Preflight also requires every output vector's role/ordinal and public commitment
to match its descriptor and the main vector's ordered commitments. The caller
still must derive all vectors, keys and required roles from authoritative native
state and bodies; neither bytes nor a generic library type establishes provenance.
No bundle result authenticates issuer signatures, checks fees or writes state.

## Bounds and outstanding evidence

Both codecs require explicit positive local limits: at most 128 MiB per proof,
512 MiB per bundle, and 512 MiB owned codec allocation requests. These are guard
ceilings, not approved node budgets. Go `Check` scans framing; `Decode` preflights
the whole bundle before allocating independent proof copies. Rust borrows bundle
slices and decodes/verifies one component at a time. Per-proof decoder limits are
separate; backend copies, CPU, allocator overhead and caller buffers remain outside
codec accounting. [Native execution](NATIVE_EXECUTION.md) adds coarse accepted-block proof counts
and proof/byte fee formulas; failed-attempt scheduling and aggregate memory metering remain open.

At the current widths, seven main-trace query payloads for a fully disclosed
transfer alone total 236,752,896 bytes (~225.8 MiB), before other openings/paths.
This is source arithmetic, not a proof measurement; resource feasibility remains
unresolved. See [the verification boundary](VERIFICATION_BOUNDARY.md) and
[complete delivery checklist](END_TO_END_STATUS.md). Features remain disabled.
