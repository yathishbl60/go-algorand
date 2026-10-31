# Native asset privacy implementation progress

Updated 2026-10-31. [End-to-end status](END_TO_END_STATUS.md) is the canonical
completion checklist. The full native feature remains in progress.

## Current user instructions

- Necessary existing-file integration edits are authorized; the earlier
  new-files-only restriction has been superseded.
- New functions: at most 20 physical lines, including declaration and braces.
- New handwritten files: at most 100 lines. Generated dependency lockfiles may
  exceed 100 lines, explicitly authorized by the user.
- Keep all functionality disabled by default.
- Do not write or run tests yet; builds and static analysis are permitted.
- Proceed without a supplied cryptographer-reviewed profile. Keep the selected
  candidate and review status explicit; do not claim defect-free security.
- Commit verified increments and continue toward the full implementation.

## Delivered components

1. Profile evidence packaging and integrity checks. These hash supplied files;
   they do not authenticate reviewers or approve cryptography.
2. Native COW supply, policy/key/control epochs, spent markers, commitment tree,
   root history and atomic encrypted output streams. Public holding changes
   preserve freeze state and atomically publish current eligibility. Native destruction
   now follows [authenticated retirement and bounded cleanup](RETIREMENT_CLEANUP.md), using the [complete mutation inventory](NATIVE_INVENTORY.md). [NAPL1 lifecycle](LIFECYCLE_INGRESS.md)
   adopts policy/assignments, controls freeze and verifies all role possession. [Native creation](NATIVE_CREATION.md) declares policy at birth and converts private-only creator inventory. [Authority synchronization](AUTHORITY_SYNCHRONIZATION.md) couples configuration and final key readiness; artifacts/pricing remain.
3. Local canonical 512-byte notes, SHA3 commitments/nullifiers/ownership, fixed
   ML-KEM/HKDF/AES-GCM envelopes, role/epoch output recovery and bounded key
   admission. Wallet scanning, SDK integration and runtime acceptance remain.
4. [Native body codec and signed PQ/group/witness admission](NATIVE_TRANSACTION_INTEGRATION.md),
   witness-sensitive cache identity, [exact typed bundles](PROOF_BUNDLES.md),
   [ABI2 bridge](NATIVE_VERIFIER_BRIDGE.md) and [suite selection](NATIVE_SUITE_CATALOG.md) exist; catalogs/artifacts remain empty.
5. Pinned Plonky3 hiding configuration and exact uint64 conservation. Note,
   ownership, nullifier and depth-32 membership constraints compose into fixed
   transfer/shield/unshield arities. [Ordinary staged spending](SPENDING_COMPOSITION.md)
   joins eligibility/group conservation. [Native Shield3](NATIVE_SHIELD.md) is source-routed;
   [recovery](RECOVERY.md) preserves frozen-owner remainders. Runtime evidence remains.
6. SHA3/SHAKE, private HMAC/HKDF, full typed GCM and staged GCM constraints.
   Staged GCM covers all AES/GHASH blocks and the final tag in one AIR. Resource
   feasibility and complete envelope/note/disclosure bindings remain unverified.
7. Canonical ML-KEM coefficient arithmetic, compact NTT, products, packing,
   private noise and bounded matrix sampling. Matrix sampling composes SHAKE128
   and first-accepted rejection through shared columns, with matching native
   viewing-key admission. Private canonical-key hashing derives shared secret
   and coins. Deterministic host KEM constructs complete encryption witnesses.
8. Staged private K-PKE arithmetic now joins forward transforms, all matrix/key
   dot products, inverse transforms, errors/message and public ciphertext packing.
   [Full bounded encapsulation](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/README.md)
   now binds canonical key hashing, all nine matrices, seven noise samples and
   K-PKE to the same private m/K/coins. Compact window/squeeze layouts reduce
   component widths.
9. [Complete single-envelope AIR](../../internal/assetprivacy/stark/src/envelope_air/README.md)
   connects the same canonical key/Ckem/shared secret through AAD/HKDF/GCM to
   one canonical opening and its public commitment. All operations fit within
   65,536 rows using bounded frame writers and shared scratch. Enclosing
   recipient/disclosure and ordinary spending/group constraints exist; native
   viewing registration possession now routes through kind22; runtime proof evidence remains.
10. [Current-key authorization registry](KEY_AUTHORIZATION.md) replaces a stable
    account/role leaf on rotation and atomically couples it to native key history
    and pointers. Typed current roots and exact-record witnesses are available.
    Private viewing-record constraints and current membership match the canonical
    hash; current-consensus proof checking exists, runtime provenance evidence remains.
11. [Recipient output authorization](../../internal/assetprivacy/stark/src/authorized_recipient_air/README.md)
    joins complete envelope/note encryption to current viewing and spending
    membership for the same private account, independent epochs and note owner.
    Staged membership and compact matrix frames preserve every operation while
    keeping the full recipient schedule within 65,536 rows. [Current eligibility](ELIGIBILITY.md)
    now binds the same private account. Complete lifecycle ingress and runtime evidence remain.
12. [Assigned disclosures](../../internal/assetprivacy/disclosure/README.md) persist
    prospective auditor/controller targets and bind complete encrypted openings
    to their current registered viewing keys. [Exact output-role plans](PROOF_PLANNING.md)
    derive expected vectors from native state. Main planning also checks native
    roots/nullifiers and finalized body context. No proof execution has run.

Go uses `algorand_native_privacy`; Rust's optional `native-privacy` feature is
empty by default. No consensus version enables privacy or configures a suite. See [the candidate
profile](CANDIDATE_PROFILE.md) and [developer guide](DEVELOPER_GUIDE.md).
Detailed code history is preserved in Git; component READMEs define current
interfaces, exact layouts, caller obligations and source-only checks.

## Verification scope

Feature-enabled Go builds/vet and default native builds have passed. The Rust
workspace feature/default checks, Clippy, release FFI and linked algod builds
passed. Authored size/link checks pass; source reviews do not replace
cryptographic vectors, constraint evaluations, proofs, node execution or replay.
None of those runtime checks, tests or the six demo scenarios have run. No
independent cryptographic security review has been supplied or completed.

## Next work

Validate viewing registration, authority synchronization, creation and cleanup when authorized; complete other modes and private-only final reconciliation.
Populate/approve catalog entries and actual artifacts; preserve live-asset references.
Source execution now routes through the bridge with body/current-authority checks.
Establish resource feasibility, calibrated work limits
and runtime evidence for [V2 markers](NULLIFIER_FRESHNESS.md) before activation. Then finish
API/SDK/wallet/CLI, replay/recovery and the six real-node scenarios. Maintain the
full [north-star work-package scope](IMPLEMENTATION.md); the present components
do not substitute for that outcome. Tests remain deferred by user instruction.
