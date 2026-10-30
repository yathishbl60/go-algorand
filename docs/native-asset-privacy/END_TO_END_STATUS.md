# Native asset privacy: end-to-end delivery status

Updated 2026-10-30. **Not end-to-end yet.** Native evaluator proof routing and
atomic application now exist as inactive source integration. No accepted private
transaction, runtime proof or real-node demo has been demonstrated.

## Work-package evidence

| Package | Current state and evidence still needed |
| --- | --- |
| NAP-01 profile | Pinned candidate documented; complete relation, security argument, vectors and measurements missing |
| NAP-02 consensus specification | Draft obligations/accounting/candidate layouts; complete normative protocol and sibling specs changes missing |
| NAP-03 types/gates | Flag false, suite catalog nil and native decoder bounds zero in every current version; NAPT/NAPL1 body codecs, raw transaction and signed-witness metadata implemented; compatibility/runtime vectors missing |
| NAP-04 cryptography | Note/conservation, complete envelope encryption, recipient current-key/eligibility membership and assigned disclosure constraints implemented; ordinary two-input eligibility/conservation/group constraints implemented; recovery/remainder constraints implemented; commitment-bound V2 markers/global creation uniqueness implemented; native Shield3/0-input2-output source routing added; other arities and runtime acceptance missing |
| NAP-05 native state | COW supply, policy/key/control epochs, current-key and effective-freeze roots, spent markers, tree/root history and atomic outputs implemented; creator storage obligations implemented; JSON MBR reporting implemented; cleanup/refunds and runtime recovery evidence missing |
| NAP-06 verification | Exact bundles, ABI2 bridge, current-consensus suites and deterministic decoder charges implemented; stateless PQ-envelope/group/witness admission and witness-sensitive cache added; evaluator proof routing, typed NAPR admin possession, coarse block work and proof/byte fees implemented; approved entries/artifacts, resource calibration and runtime acceptance missing |
| NAP-07 lifecycle | Policy lifecycle, bounded viewing-key admission, destruction guard and public holding freeze persistence implemented; current PQ/admin guards, NAPL1 adoption/assignment/native-freeze and child-COW execution implemented; admin Falcon, recipient owner and viewing kind22 possession registration source-integrated; exact public-key generation constraints and Shield3 source routing implemented; approved artifacts, runtime acceptance, creation and complete administration missing |
| NAP-08 groups/pool/AVM | Group binding, signed framing and native verifier helper wiring implemented; native dispatch and accepted-block proof counting implemented; failed-attempt/pool scheduling and gated AVM inspection missing |
| NAP-09 recovery | Existing KV path reused; privacy replay/restart/catchpoint/catchup behavior unverified |
| NAP-10 APIs | Bounded internal output pages and JSON native MBR reporting exist; REST streams, witness queries, decoding, simulation and error contracts missing |
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
version, including `ConsensusFuture`, sets `EnableNativeAssetPrivacy` to true;
NativeAssetPrivacySuites is nil everywhere. No suite/artifact approval exists.
All execution/storage prices and block proof capacity are zero. Local artifacts
are configured before replay; no availability or approval is established. Native
state cleanup, funding/refund lifecycle and upgrade evidence remain incomplete.

## Useful entry points

- [Candidate profile and outstanding crypto obligations](CANDIDATE_PROFILE.md).
- [Implemented proof components and their limits](PROOF_COMPONENTS.md).
- [Developer and agent implementation guide](DEVELOPER_GUIDE.md).
- [Native tree](../../internal/assetprivacy/tree/README.md), [spent markers](../../internal/assetprivacy/nullifier/README.md), and [group context](../../internal/assetprivacy/statement/README.md).
- [Proof relation and group-binding requirements](PROOF_RELATIONS.md).
- [Exact output statements and composition obligations](PROOF_PLANNING.md).
- [Ordinary input spending/conservation and native planning](SPENDING_COMPOSITION.md).
- [Recovery/remainder relations and authority boundaries](RECOVERY.md).
- [V2 output-nullifier freshness correction and generation2 state](NULLIFIER_FRESHNESS.md).
- [Native accounting](NATIVE_ACCOUNTING.md), [execution](NATIVE_EXECUTION.md), [shield](NATIVE_SHIELD.md) and [storage](NATIVE_STORAGE.md).
- [Canonical policy records](POLICY_RECORDS.md) and [lifecycle ingress, admin possession and remaining viewing work](LIFECYCLE_INGRESS.md).
- [Current-key registry and native atomic updates](KEY_AUTHORIZATION.md).
- [Key roles, epoch persistence and caller obligations](../../internal/assetprivacy/keys/README.md).
- [Viewing-key possession registration and exact public-key relation](VIEWING_REGISTRATION.md).
- [Current eligibility snapshots, atomic hooks and private recipient binding](ELIGIBILITY.md).
- [Bounded outer witness framing](../../internal/assetprivacy/witness/README.md).
- [Rust verification boundary, trusted artifacts and bounded codec](VERIFICATION_BOUNDARY.md).
- [Typed relations, source fingerprints and pinned artifact registries](ARTIFACT_SELECTION.md).
- [Native Go/Rust verification bridge, C ABI and trusted request](NATIVE_VERIFIER_BRIDGE.md).
- [Current consensus suite catalog, references and retention](NATIVE_SUITE_CATALOG.md).
- [Canonical native body contract](NATIVE_TRANSACTION_BODY.md) and [partial integration boundaries](NATIVE_TRANSACTION_INTEGRATION.md).
- [Exact main/output bundles and typed Rust verification](PROOF_BUNDLES.md).
- [Local note API](../../internal/assetprivacy/note/README.md).
- [Envelope API](../../internal/assetprivacy/envelope/README.md).
- [Bounded matrix relation and matching key-admission rule](MATRIX_SAMPLING.md).
- [Encrypted output stream and local recovery](../../internal/assetprivacy/outputs/README.md).
- [Pinned Rust backend/build commands](../../internal/assetprivacy/stark/README.md).
- [Evidence bundle preparation](../../cmd/algoprivacybundle/README.md) and
  [integrity checking](../../cmd/algoprivacy/README.md), support tooling only.

## Verification limits

Go builds and static analysis passed for the modified packages with the feature
enabled; native default builds also passed. New shield Rust workspace checks,
Clippy, release FFI and linked algod builds passed. No proofs have been executed.
Membership alone has a ~641 MB base matrix; streaming assembly reduces its
static matrix overlap from~1.28 GB to~671 MB. Transfer assembly also releases
each component before generating the next. AES-256 uses 78,388 field columns per
block; [resource implications](CRYPTO_RESOURCES.md) remain unresolved. No tests or proofs ran.
[Complete single-envelope constraints](../../internal/assetprivacy/stark/src/envelope_air/README.md)
now bind KEM/AAD/HKDF/GCM to one canonical opening/commitment. [Assigned disclosures](../../internal/assetprivacy/disclosure/README.md) and their envelope constraints now exist; native proof routing/application exist; runtime evidence is missing. [Recipient binding](../../internal/assetprivacy/stark/src/authorized_recipient_air/README.md) now joins current viewing/spending records.
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
