# Native transaction integration boundaries

Updated 2026-10-30. The candidate body codec, wire metadata and stateless admission
hooks, evaluator dispatch and child-COW application are implemented. No native
transaction, proof or real-node scenario has run; activation remains disabled.

## Wire fields and disabled behavior

`protocol.NativeAssetPrivacyTx` is `aprv`. Transaction.NativeAssetPrivacy is the
bounded `napb` byte field in the signed body; SignedTxn.NativeAssetPrivacyProof is
the bounded `napw` witness outside TXID/group IDs: NAPW for NAPT, NAPR for admin
registration, empty for other NAPL. Signed/block encoding commits them. Neither
field carries local registry files, pins, budgets or private note openings.

The metadata fields exist in both builds. Default builds reject the type and
nonempty privacy fields. Tagged builds require explicit execution configuration before
native body decoding; ordinary transaction types reject orphan bodies/witnesses.
All current consensus versions keep the flag false, suite catalogs nil and
native decoder bounds zero. No artifact or suite approval has been added.

The [body contract](NATIVE_TRANSACTION_BODY.md) and
[nativebody API](../../internal/assetprivacy/nativebody/README.md) define exact
bytes and operation arities. Decode scans into fixed two-slot staging storage,
checks complete framing and only then copies returned input/output data. Every
returned ciphertext is owned. NAPT permits shielded mode4 only; other modes/tag3
shield fail. [NAPL1](LIFECYCLE_INGRESS.md) accepts Adopt/Assign/Freeze and admin Register2, roles5..7 only.

## Stateless signed admission

`txnGroupBatchPrep` calls checkNativePrivacyGroup before ordinary well-formedness
and signature dispatch. Native admission decodes the body, validates its suite
through the complete current consensus catalog, and requires top-level PQsig with
nonempty signature bytes. Classical, multisig and LogicSig alternatives fail.
Admission also requires an off-Edwards-curve claimed authorizer. ValidateEnvelope
checks enabled scheme/identity; normal PQ signature verification follows. AuthAddr/rekey
reconciliation remains a ledger obligation, not a result of this envelope check.

Binding recomputes the final ordinary group and SHA3 statement from all bodies,
target position, genesis and suite. NAPL Register requires exact NAPR framing and
enabled Falcon1024; other NAPL requires empty witness. NAPT checks exact NAPW binding.
The payload cap is min(MaxBundleBytes,MaxAllocationBytes), and the complete frame
must also fit MaxTxnBytesPerBlock. This is framing/admission, not STARK checking.
[Body-derived ledger plans](NATIVE_TRANSACTION_PROJECTION.md) now select and check
actual proofs through the linked verifier; [native execution](NATIVE_EXECUTION.md) now applies the verified result.

## Cache and commitment identity

The nonempty-witness signature cache includes exact length and SHA3-512 digest
for NAPW/NAPR as owned metadata. Empty-witness lifecycle conservatively misses it.
No caller-owned witness slice is retained. TXID commits the native body; cache
context includes the consensus version and
ordinary signature authorization material. Replacing proof bytes cannot reuse an
entry for different witness bytes without a hash collision.

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

Only tagged enabled consensus parameters with a nonempty catalog of at most16
entries contribute decoder ceilings. The body ceiling is10465 bytes. The proof
frame ceiling includes per-suite min(MaxBundleBytes+137,512MiB+137,blockBytes)
and, when Falcon1024 enabled, min(8+Falcon1024MaxSignatureSize,blockBytes) for NAPR.
These global ceilings aggregate versions; they authenticate neither suite nor proof.

Current catalog admission separately validates every record. A future enabled
version can enlarge global decoding bounds even while processing older chains;
current-protocol validation and network/admission limits remain mandatory.
The proof-field ceiling excludes body/PQ/msgpack overhead and is not a complete
signed transaction, group, proposal or block allowance. Exact encoded size limits
must still hold. TX gossip remains5000000 bytes; existing frame/proposal/block
limits have not been raised to accommodate the candidate proof configuration.

Coarse accepted-block proof counting and proof/byte fee formulas now exist; their
capacities/prices remain zero and uncalibrated. Failed-attempt/peer scheduling and
aggregate memory metering remain missing. Large proof/trace estimates remain
unresolved; configuration alone cannot establish practical feasibility.

## Remaining native delivery

The dispatcher now checks current PQ consent and proofs, applies public conversion
and native state atomically, and meters creator storage in one child COW. Native
inner/admin operations requiring independent PQ consent fail. NAPL adoption,
assignment, freeze and admin possession registration route atomically; viewing/spending
registration, shield, other modes, AVM/API/SDK/wallet
and six real-node scenarios remain incomplete. See [execution](NATIVE_EXECUTION.md).

Tests, codec round trips, vectors, proofs, setup, node execution and performance
measurements remain deferred/unexecuted. Compilation/source checks cannot prove
historical compatibility, authorization, soundness, privacy or end-to-end delivery.
`python3 scripts/native_privacy_limits.py` checks authored file/function limits.
