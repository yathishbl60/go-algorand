# Native asset privacy: end-to-end delivery status

Updated 2026-10-25. **Not end-to-end yet.** There is no native private transfer,
complete proof verifier or real-node demo. The full north-star scope remains;
completed components below do not substitute for it.

## Work-package evidence

| Package | Current state and evidence still needed |
| --- | --- |
| NAP-01 profile | Pinned candidate documented; complete relation, security argument, vectors and measurements missing |
| NAP-02 consensus specification | Draft obligations/accounting/candidate layouts; complete normative protocol and sibling specs changes missing |
| NAP-03 types/gates | Consensus flag added, false in every version; transaction/witness types and codecs missing |
| NAP-04 cryptography | Note/conservation, complete envelope encryption, recipient current-key/eligibility membership and assigned disclosure constraints implemented; ordinary two-input eligibility/conservation/group constraints implemented; recovery/remainder constraints implemented; output marker freshness, other arities, runtime proofs and native verifier missing |
| NAP-05 native state | COW supply, policy/key/control epochs, current-key and effective-freeze roots, spent markers, tree/root history and atomic outputs implemented; storage charges and runtime recovery evidence missing |
| NAP-06 verification | Bounded opaque proof frame, exact main/output vectors and native state/body-context planning implemented; embedded STARK decoder, deterministic native verifier, work limits and cache integration missing |
| NAP-07 lifecycle | Policy lifecycle, bounded viewing-key admission, destruction guard and public holding freeze persistence implemented; PQ administrative ingress and authenticated native transactions missing |
| NAP-08 groups/pool/AVM | Group digest API and transfer public-input layout implemented; native verifier wiring, pool accounting and gated AVM inspection missing |
| NAP-09 recovery | Existing KV path reused; privacy replay/restart/catchpoint/catchup behavior unverified |
| NAP-10 APIs | Bounded internal output pages exist; REST streams, witness queries, decoding, simulation and error contracts missing |
| NAP-11 SDK/wallet | Local note/encryption/output-recovery primitives exist; SDK, scanning, proving, receipts and wallet backup/recovery missing |
| NAP-12 demo | All six real-node scenarios missing |
| NAP-13 validation | Tests intentionally deferred; adversarial/benchmark/recovery/upgrade evidence missing |
| NAP-14 release | Coordinated specs/node/SDK/indexer package and external review evidence missing |

## Current authorization and defaults

Necessary existing-file integration edits are authorized. The user also waived a
supplied cryptographer-reviewed profile as an implementation prerequisite. Work
continues using a pinned, explicitly unreviewed candidate; no security approval or
no-defects claim is made. Generated lockfiles may exceed the 100-line limit.
Other new authored files remain at most 100 lines and new functions at most 20.

Go uses `algorand_native_privacy`; Rust uses opt-in `native-privacy`. No consensus
version, including `ConsensusFuture`, sets `EnableNativeAssetPrivacy` to true.
The native destruction hook is inert under historical/default rules and requires
private-supply validation when the consensus flag is enabled. It does not provide
private transaction execution or privacy-state cleanup.

## Useful entry points

- [Candidate profile and outstanding crypto obligations](CANDIDATE_PROFILE.md).
- [Implemented proof components and their limits](PROOF_COMPONENTS.md).
- [Developer and agent implementation guide](DEVELOPER_GUIDE.md).
- [Native tree](../../internal/assetprivacy/tree/README.md), [spent markers](../../internal/assetprivacy/nullifier/README.md), and [group context](../../internal/assetprivacy/statement/README.md).
- [Proof relation and group-binding requirements](PROOF_RELATIONS.md).
- [Exact output statements and composition obligations](PROOF_PLANNING.md).
- [Ordinary input spending/conservation and native planning](SPENDING_COMPOSITION.md).
- [Recovery/remainder relations and authority boundaries](RECOVERY.md).
- [Unresolved output-nullifier freshness gate](NULLIFIER_FRESHNESS.md).
- [Native accounting and destruction behavior](NATIVE_ACCOUNTING.md).
- [Canonical policy records and lifecycle ordering](POLICY_RECORDS.md).
- [Current-key registry and native atomic updates](KEY_AUTHORIZATION.md).
- [Key roles, epoch persistence and caller obligations](../../internal/assetprivacy/keys/README.md).
- [Current eligibility snapshots, atomic hooks and private recipient binding](ELIGIBILITY.md).
- [Bounded outer witness framing](../../internal/assetprivacy/witness/README.md).
- [Local note API](../../internal/assetprivacy/note/README.md).
- [Envelope API](../../internal/assetprivacy/envelope/README.md).
- [Bounded matrix relation and matching key-admission rule](MATRIX_SAMPLING.md).
- [Encrypted output stream and local recovery](../../internal/assetprivacy/outputs/README.md).
- [Pinned Rust backend/build commands](../../internal/assetprivacy/stark/README.md).
- [Evidence bundle preparation](../../cmd/algoprivacybundle/README.md) and
  [integrity checking](../../cmd/algoprivacy/README.md), support tooling only.

## Verification limits

Go builds and static analysis passed for the modified packages with the feature
enabled; native default builds also passed. Rust feature-enabled compilation and
Clippy passed. Ownership/nullifier/membership AIRs compile; no proofs have been executed.
Membership alone has a ~641 MB base matrix; streaming assembly reduces its
static matrix overlap from~1.28 GB to~671 MB. Transfer assembly also releases
each component before generating the next. AES-256 uses 78,388 field columns per
block; [resource implications](CRYPTO_RESOURCES.md) remain unresolved. No tests or proofs ran.
[Complete single-envelope constraints](../../internal/assetprivacy/stark/src/envelope_air/README.md)
now bind KEM/AAD/HKDF/GCM to one canonical opening/commitment. [Assigned disclosures](../../internal/assetprivacy/disclosure/README.md) and their envelope constraints now exist; native spending verification/execution remains unfinished. [Recipient binding](../../internal/assetprivacy/stark/src/authorized_recipient_air/README.md) now joins current viewing/spending records.
Formatting and authored file/function limits were checked.
These are not proof correctness, privacy, anonymity, interoperability or protocol
validation. No tests have been added or run, as requested.

## Completion evidence still required

The six scenarios in [the north-star plan](README.md) must run with actual local
proofs and ordinary node validation: all privacy modes; existing-asset adoption;
private payroll and selective receipts; atomic swaps and rejected bad proofs;
freeze/partial-clawback/auditor separation; node/wallet restoration and upgrades.
Replay and catchup must reproduce the same consensus state. The present build
results do not establish any of those scenarios. Specification, SDK and indexer
sibling checkouts were absent when inspected; cross-repository delivery remains.
