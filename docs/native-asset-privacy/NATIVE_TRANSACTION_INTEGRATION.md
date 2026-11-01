# Native transaction integration boundaries

Updated 2026-11-01. The candidate body codec, wire metadata and stateless admission
hooks, evaluator dispatch and child-COW application are implemented. Activation
remains disabled; no native transaction, proof or real-node scenario has run.

## Wire fields and disabled behavior

`protocol.NativeAssetPrivacyTx` is `aprv`. Transaction.NativeAssetPrivacy is bounded
`napb`; SignedTxn.NativeAssetPrivacyProof is bounded `napw` outside TXID/group IDs:
NAPW for NAPT, NAPR for Register, otherwise empty. Block encoding commits both;
neither contains registry files, pins, budgets or private note openings.

The metadata fields exist in both builds. Default builds reject the type and
nonempty privacy fields. Tagged builds require explicit execution configuration before
native body decoding; only [ordinary ASA creation with Create5](NATIVE_CREATION.md) admits another body placement, with empty witness.
All current consensus versions keep the flag false, suite catalogs nil and
native decoder bounds zero. No artifact or suite approval has been added.

The [body codec](NATIVE_TRANSACTION_BODY.md) checks exact framing in fixed two-slot staging before copying owned outputs.
NAPT supports Shielded4 and [Confidential2 ordinary/recovery operations](CONFIDENTIAL_MODE.md); Shield3 has 0 inputs and 2 outputs.
Public1 uses ordinary ASA transfers. [NAPL1](LIFECYCLE_INGRESS.md) includes lifecycle registration, creation, retirement and cleanup.

## Stateless signed admission

`txnGroupBatchPrep` calls checkNativePrivacyGroup before ordinary well-formedness
and signature dispatch. It decodes the body, checks the complete current suite
catalog and requires nonempty top-level PQsig and off-Edwards-curve authorizer.
Classical/multisig/LogicSig fail. ValidateEnvelope checks enabled scheme/identity;
normal PQ verification follows. Ledger execution reconciles AuthAddr/rekey.

Binding recomputes the final ordinary group and SHA3 statement from all bodies,
target position, genesis and suite. NAPL Register requires exact NAPR framing and
enabled Falcon1024; other NAPL requires empty witness. NAPT checks exact NAPW binding.
The payload cap is min(MaxBundleBytes,MaxAllocationBytes), and the complete frame
must also fit MaxTxnBytesPerBlock. This is framing/admission, not STARK checking.
[Body-derived ledger plans](NATIVE_TRANSACTION_PROJECTION.md) now select and check
actual proofs through the linked verifier; [native execution](NATIVE_EXECUTION.md) now applies the verified result.

## Cache and commitment identity

The signature cache owns exact NAPW/NAPR length and SHA3-512 digest metadata;
empty-witness lifecycle conservatively misses it. TXID commits the native body.
Cache context includes consensus version and ordinary authorization material;
different witness bytes cannot reuse an entry without a hash collision.

This does not turn signature-cache success into privacy proof acceptance. Every
value execution rebuilds current statements/state; lifecycle rechecks principal,
policy/suite/epoch and actual new-key possession for Register before any writes.
Local backend/resource failures must never populate invalid-proof cache entries.
Generated array decoders append privacy fields after all historical positions;
canonical map keys are distinct from existing application fields. Source order
and generation idempotence were checked; runtime round trips remain deferred.
Regenerate through `make data/transactions/msgp_gen.go` so strict append-only
array normalization runs; direct generator output alone is incomplete.
Full SignedTxnInBlock encoding and the existing signed-transaction block leaf
commitment include witness bytes; proof substitution cannot preserve that leaf.

## Decoder and network resource limits

Only tagged enabled consensus parameters with 1–16 catalog entries contribute
decoder ceilings: body 10593 bytes; NAPW min(MaxBundleBytes+137,512MiB+137,blockBytes);
NAPR min(8+Falcon1024MaxSignatureSize,blockBytes) when Falcon1024 enabled.
These ceilings aggregate versions and authenticate neither suite nor proof.

Current catalog admission validates every record. Future versions can enlarge
global bounds while processing older chains; current-protocol limits still apply.
Proof-field ceilings exclude body/PQ/msgpack overhead; exact signed-transaction,
group, proposal and block limits still hold. TX gossip remains 5000000 bytes;
existing frame/proposal/block limits have not been raised for candidate proofs.

Coarse accepted-block proof counting and proof/byte fees exist, with zero,
uncalibrated capacities/prices. [Local single-bundle admission/retries](LOCAL_VERIFIER_ADMISSION.md) exist.
Per-peer/failed-attempt scheduling, aggregate memory and proof feasibility remain unresolved.

## Pending and simulation wire propagation

In `daemon/algod/api/server/v2/handlers.go`, PreEncodedTxInfo carries SignedTxn;
pending lists do too. PreEncodedSimulateRequest decodes groups of SignedTxn, and
`utils.go` convertSimulationRequest copies them unchanged. ConvertInnerTxn and
convertTxnResult retain the signed object on response. Thus raw napb/napw already
propagate through JSON/msgpack without new field-copy handlers. Generated models
use a raw transaction map/RawMessage, not typed native semantic interpretation.
Simulator.check uses ordinary group verification; its evaluator runs the same
native consent/proof path. Empty-signature/fix-signer options do not supply native
PQ consent or valid proofs; synthetic native signers reject before mutation.
MaxSimulateBytes remains 1,000,000, potentially below a future proof bundle. Existing group fee usage includes native factors;
[Native work diagnostics](SIMULATION_WORK.md) retain reservations/calls after rollback; typed semantic decoding remains pending.

## Remaining native delivery

The dispatcher checks current PQ consent/proofs and applies conversion, native
state and creator storage in one child COW. Viewing/admin/owner registration,
ordinary shielded/confidential operations, recovery and supported-arity final
reconciliation and [gated AVM inspection](AVM_INSPECTION.md) have source integration.
General note-count closure, full SDK/wallet and six real-node scenarios remain incomplete; see [execution](NATIVE_EXECUTION.md).

Tests, round trips, vectors, proofs, setup, node execution and measurements remain
deferred. Build/static checks do not establish compatibility, security or delivery.
