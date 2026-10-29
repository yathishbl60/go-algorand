# Candidate native evaluator execution

Updated 2026-10-29. `aprv` routes current PQ consent, value proofs or explicit
public lifecycle operations, and child-COW application. Every protocol remains
disabled, unpriced and without an approved suite/artifact. No node/proof has run.

## Explicit configuration gate

The Go privacy tag is required; value proofs also require the linked Rust backend.
NativeAssetPrivacyExecutionConfigured requires the flag and all five values below.
Every current version leaves them zero; metadata alone cannot enable execution.

| Consensus field | Required value and meaning |
| --- | --- |
| NativeAssetPrivacyFlatMinBalance | Positive creator charge per native KV item |
| NativeAssetPrivacyByteMinBalance | Positive creator charge per full KV key/value byte |
| NativeAssetPrivacyMaxProofChecksPerBlock | Positive accepted-block component budget |
| NativeAssetPrivacyProofFeeFactor | At least1000000 Micros per proof component |
| NativeAssetPrivacyByteFeeFactor | Positive Micros per canonical SignedTxn byte |

These are draft obligations, not selected safe prices/capacities. Approved pins,
retained artifacts, backend compatibility and normative specification changes
remain separate prerequisites. Arbitrary nonzero values establish no readiness.
StartEvaluator requires nonzero root history and valid catalog. After activation
it forbids disabling privacy, changing history length or dropping/changing any old
suite record. Default builds reject enabled protocols. This supplies no migration.

## Authorization and atomic dispatch

BlockEvaluator.transaction checks actual top-level PQ authorization against the
sender's current group-local AuthAddr, falling back to Sender when unset. The
authorizer must satisfy IsPQCompliant off-Edwards-curve checking. Classical,
multisig and LogicSig alternatives fail. A cache hit cannot skip current PQ verify.
Privacy-policy ASA configuration, freezing and public issuer clawback also require
this consent. Ordinary public transfers keep their existing authorization, subject
to policy modes. Native/admin inner operations needing independent PQ consent fail.

The native case rejects unconfigured execution and inner calls. It creates a
child of the current transaction/group COW, bounds the complete signed group with
Msgsize estimates and checks pooled fees before either branch:

- NAPL1 [lifecycle](LIFECYCLE_INGRESS.md) checks exact syntax, empty witness, suite,
  final group and current principal. Adopt writes policy plus initial assignments;
  Assign updates disclosure targets; Freeze updates native control and eligibility.
  No P3 proof, registry read or proof-unit reservation occurs. Registration2 fails.
- NAPT1 value operations derive the exact current policy/root/marker/key/eligibility
  plan, reserve proof-component units, load selected registry bytes and verify all
  main/output-role proofs. Only then apply public holdings, private supply, spent
  markers and complete encrypted outputs/commitments.

The child commits only after every required step succeeds. Failed native/group
COWs retain neither tentative state nor proof reservations. Ordinary fee/rekey
changes remain subject to the surrounding transaction/group rollback contract.
Adoption preserves holdings/supply and rejects prior private-supply state. Its
required initial controller must come from the existing ASA clawback authority.
Native freeze changes the marker only: an existing public freeze still contributes
OR to effective freeze; without a holding, marker/default rules apply.

Normal public debits/credits require existing opted-in, unfrozen holdings and
checked uint64 arithmetic. Authorized recovery may bypass public-credit freeze,
but still needs opt-in and cannot debit a public holding. Positive conversions
require public mode. Private-only final reconciliation is still incomplete.

## Proof work and fee construction

NAPT Body.ProofChecks counts one main plus every mandatory output-role proof:1..7.
Each is one coarse worst-profile unit. Parent-chain COW accounting enforces the
configured block cap; commits merge counts, failed COWs discard them. This is not
measured time/heap or per-peer/failed-attempt scheduling. NAPL counts zero checks.

SignedTxn.FeeFactor adds proofFactor*checks + byteFactor*canonicalSignedTxnLength
to ordinary transaction/PQ fees, using saturating arithmetic and fee pooling.
Micros are fixed point:1000000 is one ordinary minimum-fee multiplier. Byte cost
includes complete witness/PQ encoding; lifecycle still pays full signed-byte cost.
No practical price or aggregate memory meter has been established.

**Fix a conservative upper-bound fee before finalizing a value group and proving.**
P3 proof and PQ signature lengths can vary. Reserve approved encoding maxima first;
actual byte pricing is checked against that fixed fee. Changing Fee after proving
changes TXID, group and statement, invalidating the proof. Lifecycle has no proof,
but its PQ signature/group must also cover the final fee. SDK builders are pending.

## Local artifacts, storage and remaining work

Tagged Ledger initialization installs a directory provider before tracker replay;
MakeFull uses RootGenesisDir/native-privacy/registries. Filenames are the complete
lowercase registry-digest hex plus `.napk`. Reads are bounded to954 bytes, regular
file/no-follow checked and digest verified. Preserve historical replay artifacts.
Provider bytes grant no approval; local backend/resource/availability failures
must not become invalid-proof evidence. See [provider](../../internal/assetprivacy/artifactstore/README.md).

[Creator obligations](NATIVE_STORAGE.md) charge every native KV mutation in its
COW, including ordinary asset hooks. [JSON MBR](ACCOUNT_REPORTING.md) includes them.
Key-possession registration, shield/all modes/arities, authority synchronization,
cleanup/refunds, AVM inspection, admission scheduling, complete APIs, SDK/wallet
and all six real-node scenarios remain unfinished. See [delivery status](END_TO_END_STATUS.md).
Runtime correctness, rollback/restart, crypto security and resource feasibility
remain unproven. No tests, vectors, proofs, preprocessing or node runs occurred.
