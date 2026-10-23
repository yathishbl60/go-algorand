# Native asset privacy: end-to-end delivery status

Updated 2026-10-22. **Not end-to-end yet.** There is no native private transfer,
complete proof verifier or real-node demo. The full north-star scope remains;
completed components below do not substitute for it.

## Work-package evidence

| Package | Current state and evidence still needed |
| --- | --- |
| NAP-01 profile | Pinned candidate documented; complete relation, security argument, vectors and measurements missing |
| NAP-02 consensus specification | Draft obligations/accounting/candidate layouts; complete normative protocol and sibling specs changes missing |
| NAP-03 types/gates | Consensus flag added, false in every version; transaction/witness types and codecs missing |
| NAP-04 cryptography | Hiding backend, SHA3/note/ownership AIRs, integer gadget and local encryption implemented; full relation/prover/verifier missing |
| NAP-05 native state | COW supply access implemented; policy/key records, trees, nullifiers, roots, outputs and storage charges missing |
| NAP-06 verification | Missing bounded proof decoder, deterministic native verifier, work limits and cache integration |
| NAP-07 lifecycle | Policy checks and destruction guard implemented; native create/adopt/register/convert/spend/control operations missing |
| NAP-08 groups/pool/AVM | Missing final-group binding, pool admission/work accounting and gated AVM inspection |
| NAP-09 recovery | Existing KV path reused; privacy replay/restart/catchpoint/catchup behavior unverified |
| NAP-10 APIs | Public streams, witness queries, decoding, simulation and error contracts missing |
| NAP-11 SDK/wallet | Local note/encryption primitives exist; SDK, scanning, proving, receipts and encrypted recovery missing |
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
- [Proof relation and group-binding requirements](PROOF_RELATIONS.md).
- [Native accounting and destruction behavior](NATIVE_ACCOUNTING.md).
- [Local note API](../../internal/assetprivacy/note/README.md).
- [Envelope API](../../internal/assetprivacy/envelope/README.md).
- [Pinned Rust backend/build commands](../../internal/assetprivacy/stark/README.md).
- [Evidence bundle preparation](../../cmd/algoprivacybundle/README.md) and
  [integrity checking](../../cmd/algoprivacy/README.md), support tooling only.

## Verification limits

Go builds and static analysis passed for the modified packages with the feature
enabled; native default builds also passed. Rust feature-enabled compilation and
Clippy passed. SHA3/note/ownership AIRs compile; no proofs have been executed.
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
