# Privacy policy lifecycle rules

Added 2026-10-22 as an independent component of the native privacy implementation.
This package implements the non-cryptographic rules already fixed in the
[north-star plan](../../../docs/native-asset-privacy/README.md).
Gated native ledger helpers use these rules; authenticated policy-adoption
transaction ingress remains incomplete.

## API contract

| API | Responsibility |
| --- | --- |
| `Policy.Validate()` | Reject empty/unknown modes, unknown capability/disclosure flags, and clawback without declared controller disclosure |
| `Modes.Allows(mode)` | Accept exactly one known mode in a valid allowed-mode set |
| `CheckCreation(enabled, proposed, capabilities)` | Validate policy and require an exact match with the new asset's declared authority capabilities |
| `CheckAdoption(enabled, proposed, existing)` | Additionally require an existing manager, no previous policy, and continued public mode |
| `CheckUpdate(enabled, before, after)` | Keep modes/disclosures immutable and permit only capability relinquishment |

All lifecycle entry points return `ErrDisabled` first when disabled. Pass the
consensus-derived feature gate; a node-local preference cannot enable these
rules. All files additionally require the `algorand_native_privacy` build tag.

Values are passed by value and never mutate external state. There are no maps,
unbounded slices, I/O, dependencies or cryptographic callbacks in this package.
Named sentinel errors support `errors.Is`; their strings are not protocol codes.

`CanFreeze` and `CanClawback` describe retained powers. At adoption they must
exactly match the existing asset's nonzero freeze/clawback authorities. A later
update can remove either power but cannot restore it. Mandatory disclosure
declarations remain unchanged after relinquishment, including controller
disclosure: removing a power cannot retract information already revealed.

Auditor disclosure alone does not authorize recovery. Controller disclosure is
required when clawback exists. Ciphertext correctness and key permissions remain
obligations of the reviewed cryptographic profile.

## Integration responsibilities

These are semantic checks, not transaction authorization or a verifier:

1. Load current asset/policy state from the group's ordered ledger snapshot.
   Propagate lookup failures; do not turn errors into an absent policy.
2. Derive capabilities from `basics.AssetParams.Freeze` and `Clawback`.
   Derive manager presence from `Manager`, and previous adoption from persistent
   privacy state. Never accept these facts from the transaction sender.
3. Verify the applicable manager/controller authorization, including the reviewed
   PQ requirements. `ExistingAsset.Managed` proves only presence, not ownership.
4. Validate policy semantics with this package. At key rotation, synchronize
   retained powers with the actual resulting asset authority records.
5. Check proof, key epochs, recovery material, registration and funding under
   the eventual frozen specification, then commit all changes atomically.

Initial adoption may not reset holdings, alter total supply, or privately move
another holder's funds. Those ledger invariants are separate from this package.
Public-only mode sets are structurally valid; a policy containing no private
mode grants no permission to transfer privately.

The Go bit values are internal semantic representations. Do not serialize them
as consensus or SDK fields before NAP-02 freezes the canonical wire format.
No authority addresses, keys, key epochs or encrypted data are stored here.

## Build and static checks

```sh
go build -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/policy
go vet -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/policy
```

Both commands passed on 2026-10-22. Default-build inspection excludes all five
Go files. New functions are at most 18 lines; all eight files added in this
increment are at most 100 lines. The 25 earlier feature files are unchanged.

Tests remain deferred by user instruction. Required later coverage includes
unknown bits/modes, all authority relinquishment transitions, restoration
rejection, adoption twice, absent manager, missing public mode, disclosure
changes, disabled gating and current-ledger integration. Compilation and static
analysis do not prove those behaviors.

The crypto prerequisite assessment is in
[CRYPTO_CANDIDATES.md](../../../docs/native-asset-privacy/CRYPTO_CANDIDATES.md);
the draft statement obligations are in
[PROOF_RELATIONS.md](../../../docs/native-asset-privacy/PROOF_RELATIONS.md).
