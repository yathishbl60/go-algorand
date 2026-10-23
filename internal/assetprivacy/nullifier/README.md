# Native spent-marker candidate

Built only with `algorand_native_privacy`; native access also requires the inactive
consensus flag. No current consensus version enables it.

A marker key is ASCII `aprv:n:`, an 8-byte big-endian asset ID, then the complete
64-byte nullifier. Asset ID zero is invalid. Every hash output is permitted,
including all zero; absence is represented by KV presence, never a sentinel hash.
The only valid stored value is the single version byte `1`. Malformed records
return an error and must not be interpreted as unspent.

`roundCowState.HasAssetPrivacyNullifier` checks the flag before I/O, verifies that
the ASA exists, then reads current group-local KV state. The boolean is meaningful
only when the error is nil. `ConsumeAssetPrivacyNullifier` rejects an existing
marker and records consumption in the current transaction/group COW.

Later members of a group see earlier consumption through ordinary COW lookup.
The caller must validate the complete proof and current authorization, account
for storage funding, and commit all note/supply/public-balance changes together.
Any later failure requires discarding the transaction/group COW. There is no
early-publication or marker-deletion API.

The Rust `NullifiedNoteAir` binds the public marker to the exact serial in a
privately owned note using the candidate SHA3-512 network/asset/policy domain.
`MemberNoteAir` additionally constrains membership of that same note. Neither
component replaces current-ledger freshness checks or the future controller path.
Holder spending and controller recovery must eventually consume the same marker.

These accessors have no native privacy-transaction callers yet. Builds and static
analysis pass; no tests, proofs, replay or recovery scenarios have been executed.
