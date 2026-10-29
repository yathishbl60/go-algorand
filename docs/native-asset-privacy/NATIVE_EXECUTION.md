# Candidate native evaluator execution

Updated 2026-10-29. The `aprv` evaluator case now routes through current-authority
PQ verification, native proof checking and child-COW application. This is source
integration, not an activated or runtime-validated feature. Every protocol remains
disabled, unpriced and without an approved suite/artifact. No node/proof has run.

## Explicit configuration gate

The Go privacy tag and linked Rust backend are necessary, but insufficient.
NativeAssetPrivacyExecutionConfigured additionally requires the consensus flag
and all five values below; every current version leaves these values zero:

| Consensus field | Required value and meaning |
| --- | --- |
| NativeAssetPrivacyFlatMinBalance | Positive creator charge per native KV item |
| NativeAssetPrivacyByteMinBalance | Positive creator charge per full KV key/value byte |
| NativeAssetPrivacyMaxProofChecksPerBlock | Positive accepted-block component budget |
| NativeAssetPrivacyProofFeeFactor | At least 1000000 Micros per proof component |
| NativeAssetPrivacyByteFeeFactor | Positive Micros per canonical SignedTxn byte |

These are draft configuration obligations, not selected safe prices/capacities.
Approved catalog pins, retained public artifacts, backend compatibility and
normative specification changes remain separate prerequisites. Adding metadata
or choosing arbitrary nonzero values does not establish deployment readiness.

StartEvaluator also requires a nonzero root-history length and valid catalog.
After activation it forbids disabling privacy, changing that history length or
dropping/changing any old suite record; adding suites is allowed. Default builds
reject enabled protocols. This retains layouts/pins, not artifacts or a migration.

## Authorization and ordered execution

BlockEvaluator.transaction checks actual top-level PQ authorization against the
sender's current group-local account AuthAddr, falling back to Sender only when
that address is unset. The authorizer must be off the Edwards curve under
IsPQCompliant. Classical/multisig/LogicSig alternatives fail. PQsig.Verify checks
the body; a generic signature-cache hit cannot skip current authorization.
Privacy-policy ASA configuration, freezing and public issuer clawback also require
this current PQ consent. Ordinary public holder transfers keep their existing
authorization, subject to allowed modes. Native/admin inner transactions requiring
independent PQ consent are rejected; sender equality alone grants no authority.

The native case rejects unconfigured execution and inner calls. It creates a
child of the current transaction/group COW and performs these steps:

1. Bound the complete signed group using conservative Msgsize estimates before
   exact fee marshaling. Keep existing group/body/protocol byte limits.
2. Check pooled group fees before artifact lookup or Rust proof work.
3. Derive the exact body/current-state plan, including current policy, roots,
   distinct fresh markers, key/eligibility roots and retained recovery authority.
4. Reserve the body's proof-component count on that child COW.
5. Select current consensus pins, obtain local registry bytes, and verify every
   required main/output-role proof through the real optional Rust backend.
6. Apply public holdings, private supply, spent markers and complete encrypted
   outputs/commitments. Commit the child only after every required step succeeds.

Normal public debits/credits require existing opted-in, unfrozen holdings and
checked uint64 arithmetic. Authorized recovery may bypass public-credit freeze,
while still requiring opt-in; recovery cannot debit a public holding. Positive
public conversions require the policy's public mode. Failed native/group COWs
retain no tentative state or proof-count reservation. Ordinary fee/rekey effects
remain subject to the surrounding transaction/group rollback contract.

## Proof work and fee construction

Body.ProofChecks counts one main plus every mandatory output-role proof: 1..7.
Each counts as one coarse worst-profile unit. COW parent-chain accounting enforces
the configured block cap; successful commits merge counts and failed COWs discard
them. This is not measured time/heap or a per-peer/failed-attempt scheduling budget.

SignedTxn.FeeFactor adds proofFactor*checks + byteFactor*canonicalSignedTxnLength
to ordinary transaction/PQ fees, using saturating arithmetic and normal fee pooling.
Micros are fixed point: 1000000 means one ordinary minimum-fee multiplier. The
byte term includes the exact NAPW frame and actual PQ signature encoding, not just
the native body. No practical price or aggregate memory meter has been established.

**Fix a conservative upper-bound fee before finalizing the group and proving.**
P3 proof and PQ signature lengths can vary. Use approved proof/signature encoding
maxima to reserve enough pooled fee first; actual signed-byte pricing is then
checked against that fixed fee. Changing Fee after proof generation changes TXID,
group IDs and statement binding, invalidating the proof. SDK fee/proof builders
and reproducible cost measurements remain pending; this document supplies none.

## Local artifacts, storage and remaining work

Tagged Ledger initialization installs a local directory provider before tracker
replay; MakeFull uses RootGenesisDir/native-privacy/registries. Filenames are the
complete lowercase registry-digest hex plus `.napk`. Reads are bounded to 954 bytes,
regular-file/no-follow checked and digest verified. Keep historical artifacts for
replay. Provider bytes grant no approval; local availability/backend/resource
failures must never become invalid-proof evidence. See [provider](../../internal/assetprivacy/artifactstore/README.md).

[Creator storage obligations](NATIVE_STORAGE.md) are charged with every native KV
mutation, including incidental ordinary asset hooks, in the same COW. JSON MBR
reporting includes these obligations. Safe cleanup/refunds, policy ingress, private-only
final reconciliation, shield/all modes/arities, AVM type support, admission queues,
SDK/wallet and six real-node scenarios remain unfinished. Runtime correctness,
rollback/restart, cryptographic security and resource feasibility remain unproven.
No tests, vectors, proofs, preprocessing, provider reads or node runs occurred.
