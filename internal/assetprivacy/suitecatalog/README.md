# Consensus-pinned native privacy suite selection

`suitecatalog` is available only under `algorand_native_privacy`. It selects
explicit metadata from the current ConsensusParams.NativeAssetPrivacySuites list.
There is no default suite, fallback profile, implicit approval or proof-selected
registry key. Every current protocol version leaves the list nil and the native
privacy feature disabled. Adding metadata alone cannot enable protocol behavior.

`Lookup(proto,id)` and `Validate(proto)` first check EnableNativeAssetPrivacy and
return policy.ErrDisabled before inspecting catalog content when it is false.
Enabled catalogs contain at most16 records. A fixed16-element array snapshots the
entire bounded list before validation; caller-owned slices are never retained.
Keep the original list stable during the call. Returned Suite values contain only
private value fields, so later caller-buffer edits cannot alter the selection.

Every record must have a nonzero64-byte ID, Reference and RegistryDigest. IDs
must be unique. ProofKinds must be a nonzero subset of bits1..14,17..22. All records
are validated before lookup selects a match: a malformed later entry cannot hide
behind a valid earlier prefix. No record is skipped. Malformed/oversized lists
return ErrCatalog. An enabled empty list passes Validate; Lookup returns
ErrUnknown. Missing IDs also return ErrUnknown after whole-list validation.
Lookup and Validate perform no artifact hashing, file/network access, backend
calls, preprocessing generation or cryptographic proof verification.

Record bounds use the same exported proofbridge.Limits.Validate implementation
as verifier construction: bundle bytes7..512MiB, proof bytes73..128MiB and no
larger than the bundle cap, allocation bytes1..512MiB. These are structural/local
safety ceilings, not measured proof costs, consensus pricing or activation evidence.
Actual configured bounds must come from trusted protocol/suite configuration.

`Suite.Pin()` returns artifactregistry.Pin by value: ID maps to Profile,
Reference maps to the existing Implementation wire field, and RegistryDigest maps
to Digest. Reference is a consensus-selected compatibility reference. Exact
reference/relation identity remains checked by the Rust verifier; the wire
field name does not authorize caller-selected implementations. `Suite.Limits()`
returns the three selected budget values by value. Neither getter changes state.

`Suite.Verifier(registryBytes)` revalidates the record and requires its ProofKinds
to exactly match the pinned registry, then calls proofbridge.NewVerifier. It checks
independently pinned digest/profile/reference and linked backend ABI/compatibility.
The bytes may come from an untrusted source but must match the selected pins.
The input buffer must remain stable while NewVerifier copies it. The zero Suite
fails with ErrCatalog; a missing backend fails closed with ErrUnavailable. There
is no callback or alternate success path. The catalog never generates approvals,
artifacts or placeholder pins and never treats a digest read from the same file
as independent authority.

A Suite is a snapshot, not permanent transaction authorization. Native code must
select again from current protocol and authoritative policy suite when rebuilding
expected statements, and invalidate stale verification/cache state after relevant
upgrades or preceding group changes. Required signatures, mode/authority checks,
current roots/nullifiers, public balances, funding and atomic execution remain
separate. Successful catalog validation alone proves none of them.

`Suite.CheckPolicy` permits a partial scope only for exactlyPublic mode; any
private mode requires all twenty kinds. Evaluator reads/writes and snapshot queries
enforce it. [Scope integration evidence](../../../docs/native-asset-privacy/SCOPED_SUITE_REGISTRIES.md)
does not establish full native privacy delivery or artifact approval.
