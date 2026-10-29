# Native consensus and ledger accounting integration

Added 2026-10-22 after the user authorized edits to existing consensus,
transaction and ledger integration files. The 20-line new-function limit,
100-line new-file limit, disabled-by-default behavior and no-tests instruction
remain in effect.

This increment connects aggregate private-supply accounting to `roundCowState`.
It does not implement native private transfers, approve a proof system, complete
NAP-05 or satisfy the end-to-end release gates.

## Consensus boundary

[config/consensus.go](../../config/consensus.go) now declares
`ConsensusParams.EnableNativeAssetPrivacy`. All defined consensus versions,
including `ConsensusFuture`, leave it false. No historical version is activated.

Native supply accessors exist only when compiled with `algorand_native_privacy`.
Each public accessor reaches the gated getter before accessing asset/KV state.
When the consensus flag is false, the getter returns `ErrDisabled` before I/O.
The compilation tag alone cannot enable ledger access. A successful build does
not authorize protocol activation or establish that the complete feature exists.

This parameter needs a matching specification-repository change before release;
the local document records the draft behavior, not a completed cross-repo update.

## Native ledger entry points

| COW method | Behavior |
| --- | --- |
| `GetAssetPrivacySupply` | Read authoritative ASA total and current private aggregate; return presence separately; reject an over-total record |
| `PutAssetPrivacySupply` | Validate both current and replacement supply, then add a KV delta |
| `IncreaseAssetPrivateSupply` | Check the shield arithmetic and write the resulting aggregate |
| `DecreaseAssetPrivateSupply` | Check the unshield arithmetic and write the resulting aggregate |
| `CheckAssetPrivacyDestruction` | Require zero private supply/full creator holding and absence of native supply/policy records until authenticated cleanup exists |
| `DeleteAssetPrivacySupply` | Refuse deletion while value remains; remove only an empty aggregate record |

The methods read the current COW and its parents, so ordered group changes are
visible. They use existing `kvGet`, `kvPut`, `kvDel` and `StateDelta.KvMods`.
A failed lookup, decode or arithmetic check returns before the mutation. The
caller must discard the transaction/group COW if a later operation fails.

Native `AssetConfig` destruction now calls the read-only supply guard before
mutating asset records. Existing native supply/policy records now block deletion
even after full value reconciliation, preserving creator storage attribution.
[Native execution](NATIVE_EXECUTION.md) verifies current consent/proofs and applies
public/private changes. Deleting a supply record alone grants no destruction right.

## Storage and recovery path

The [accounting contract](../../internal/assetprivacy/accounting/README.md)
defines the provisional fixed-size key and value. Native keys occupy a namespace
different from AVM boxes; no box interface or box storage counters are reused.

Existing [COW delta propagation](../../ledger/eval/cow.go) includes KV changes.
[KV storage](../../ledger/store/trackerdb/generickv/accounts_writer.go) accepts
opaque keys. Reusing these paths is the implementation basis for persistence;
it is not evidence that restart, catchpoints or every backend have passed
privacy-specific validation. Those checks remain in NAP-09 and NAP-13.

This aggregate record is only one part of native privacy state. Policy/key
records, commitment trees, nullifiers, accepted roots, historical output streams,
[creator storage metering](NATIVE_STORAGE.md) now share the COW. Approved pricing
and authenticated cleanup/refunds remain separate work.

## Validation and remaining work

Run both compilation modes and static analysis:

```sh
go build -mod=readonly -tags algorand_native_privacy ./config ./ledger/eval ./internal/assetprivacy/...
go build -mod=readonly ./config ./ledger/eval
go vet -mod=readonly -tags algorand_native_privacy ./config ./ledger/eval ./internal/assetprivacy/...
```

Tagged native accounting/config/apply/evaluator compilation, default native builds
and static analysis passed. The destruction hook returns immediately when the
consensus flag is false. If enabled without the build tag or a ledger supplying
the required checker, it rejects destruction. The guard does not remove private
state; storage cleanup remains part of the complete lifecycle implementation.

The aggregate state codec remains explicit, bounded and inactive. Subsequent
wire/evaluator integration is documented separately. Tests remain deferred.
The complete [candidate proof relation](CANDIDATE_PROFILE.md), native
transaction pipeline, coordinated specification/SDK changes and real-node demo
are still required. Do not label this accounting integration end-to-end privacy.
