# Native transaction integration boundaries

Updated 2026-10-29. The candidate body codec, wire metadata and stateless admission
hooks are implemented. Native evaluator dispatch and atomic execution remain
unfinished; no native transaction, proof or real-node scenario has run.

## Wire fields and disabled behavior

`protocol.NativeAssetPrivacyTx` is `aprv`. Transaction.NativeAssetPrivacy is the
bounded `napb` byte field in the signed body; SignedTxn.NativeAssetPrivacyProof is
the bounded `napw` NAPW frame outside TXID/group IDs. The witness remains inside
the encoded signed transaction and block commitment. Neither field carries local
registry files, pins, budgets or private note openings.

The metadata fields exist in both builds. Default builds reject the type and
nonempty privacy fields. Tagged builds require EnableNativeAssetPrivacy before
native body decoding; ordinary transaction types reject orphan bodies/witnesses.
All current consensus versions keep the flag false, suite catalogs nil and
native decoder bounds zero. No artifact or suite approval has been added.

The [body contract](NATIVE_TRANSACTION_BODY.md) and
[nativebody API](../../internal/assetprivacy/nativebody/README.md) define exact
bytes and operation arities. Decode scans into fixed two-slot staging storage,
checks complete framing and only then copies returned input/output data. Every
returned ciphertext is owned. The codec permits shielded mode4 only; other modes,
tag3 shield and unsupported arities fail rather than selecting weaker relations.

## Stateless signed admission

`txnGroupBatchPrep` calls checkNativePrivacyGroup before ordinary well-formedness
and signature dispatch. Native admission decodes the body, validates its suite
through the complete current consensus catalog, and requires top-level PQsig with
nonempty signature bytes. Classical, multisig and LogicSig alternatives fail.
ValidateEnvelope checks an enabled scheme and claimed authorizer identity; normal
PQ signature verification still runs afterward. Current account AuthAddr/rekey
reconciliation remains a ledger obligation, not a result of this envelope check.

Binding recomputes the final ordinary group and SHA3 statement from all bodies,
target position, genesis and suite. witness.Check validates NAPW magic/version,
suite/statement, declared size and exact full length without copying its payload.
The payload cap is min(MaxBundleBytes,MaxAllocationBytes), and the complete frame
must also fit MaxTxnBytesPerBlock. This is framing/admission, not STARK checking.
[Body-derived ledger plans](NATIVE_TRANSACTION_PROJECTION.md) now select and check
actual proofs through the linked verifier; evaluator dispatch remains unfinished.

## Cache and commitment identity

The existing verification cache now includes witness length and SHA3-512 digest
as owned value metadata. It retains no caller-owned witness slice. TXID still
commits the native body, while cache context includes the consensus version and
ordinary signature authorization material. Replacing proof bytes cannot reuse an
entry for different witness bytes without a hash collision.

This does not turn signature-cache success into privacy proof acceptance. Every
execution must build current expected statements and recheck current policy,
authority, modes, eligibility/keys, roots, nullifiers, public holdings and funding.
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
frame ceiling is the maximum per-entry min(MaxBundleBytes+137,512MiB+137,
MaxTxnBytesPerBlock). These global parser ceilings aggregate supported versions;
they do not approve malformed catalog entries or authenticate a suite.

Current catalog admission separately validates every record. A future enabled
version can enlarge global decoding bounds even while processing older chains;
current-protocol validation and network/admission limits remain mandatory.
The proof-field ceiling excludes body/PQ/msgpack overhead and is not a complete
signed transaction, group, proposal or block allowance. Exact encoded size limits
must still hold. TX gossip remains5000000 bytes; existing frame/proposal/block
limits have not been raised to accommodate the candidate proof configuration.

No measured node work budget, verification queue policy, per-peer proof admission,
fee schedule or aggregate memory meter exists. The candidate's large proof/trace
resource estimates remain unresolved. Enabling a flag or adding a catalog entry
does not establish practical transport or validation feasibility.

## Remaining native delivery

Finish evaluator dispatch through current-state proof checking, enforce public
conversion rules and storage funding, and atomically apply holdings, private
supply, markers, commitments and encrypted outputs in one child COW. Native inner
transactions/AVM inspection, all privacy modes, shield, lifecycle administration,
API/SDK/wallet and the six real-node scenarios remain incomplete.

Tests, codec round trips, vectors, proofs, setup, node execution and performance
measurements remain deferred/unexecuted. Compilation/source checks cannot prove
historical compatibility, authorization, soundness, privacy or end-to-end delivery.
`python3 scripts/native_privacy_limits.py` checks authored file/function limits.
See [delivery status](END_TO_END_STATUS.md) for the full remaining scope.
