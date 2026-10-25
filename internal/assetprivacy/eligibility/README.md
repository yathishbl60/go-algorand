# Current effective-freeze authentication

Opt-in Go package `eligibility` authenticates explicit account-specific snapshots
of effective frozen state. It grants no account consent, issuer powers or proof
acceptance. The native evaluator must derive the boolean from authoritative
holding/default/control state; callers must never copy it from a transaction or
proof. Current registration/consent and complete private proof relations remain
separate obligations. Native transaction ingress remains unfinished.

## Canonical state

Domain is network32, assetBE8, immutable policy64 and account32. All components
must be nonzero. Record is exactly154 bytes: version1, domain136, revisionBE8,
UpdatedBE8, Frozen1. Frozen is0 or1; revision is nonzero. Revision is independent
of raw control-marker and key epochs. Updated records the last effective-status
change or first explicit initialization, not every harmless native state write.

LeafHash is SHA3-512 of the exact45-byte ASCII
`Algorand/AssetPrivacy/Candidate1/Eligibility/` followed by Encode(record154),
a199-byte message. It differs from key-authorization and note commitment hash
domains. Merkle nodes use the existing domain-bound depth32 tree contract.
Root is a distinct typed struct; never admit it through note-root history or
substitute a key-authorization root. Only the current group-local root applies.

All state keys start with `aprv:eligibility:`. Tree metadata/content-addressed
node keys add their normal `aprv:t:` suffixes. The account index key adds `i`
and the complete136-byte domain. Its exact158-byte value is indexBE4 followed
by canonical Record154. It stores the current effective state and stable index
in one value; there is no independent stale record pointer to reconcile.
No current eligibility record, index or frozen status can be deleted/reset.
This package does not infer migration or enumerate existing holdings.

## Interfaces and mutation contract

`Open(tree.KV,tree.Domain)` validates domain and current tree metadata.
`Snapshot()` returns typed Root and count. `Current(account,round)` validates
metadata even for absent accounts, then authenticates any stored record/index
under the current tree. Future Updated values, malformed encodings/domain swaps,
missing tree paths and indexed-leaf mismatches fail closed.

`Reconcile(account,frozen,round)` is a preauthorized native projection. On first
explicit initialization it appends revision1. A changed effective status advances
revision by exactly1 without overflow and replaces the leaf at the stable index.
Time cannot move backwards. If frozen is unchanged, it first checks the exact
current path and returns the existing record with no write or revision advance.
An unchanged record at the maximum revision is readable; a change then fails.

Changed writes stage every tree node, root/count metadata and the combined
index/record, then invoke the parent KV Apply exactly once. KV access must be
serialized and Apply must commit atomically or roll back every write on error.
The Store retains no advanced Tree snapshot after failures. There is no public
administrative signature or arbitrary snapshot-writing endpoint here.

`Witness(record,round)` requires the exact current record/revision and verifies
its leaf against the current tree. Old revisions and altered domain/account/
round/status records fail. Index, account, leaf/path, revision and Updated must
remain private in the complete ZK relation and local witness-query workflow.
A public account-specific query can itself reveal routing information.

The native integration must initialize recipients from current state during
registration and recompute after every relevant holding/control mutation in the
same outer COW. Existing public holdings need no implicit scan: registration
must supply the explicit initialized record before private receipt/spending.
Removing public holdings must preserve applicable control state; key rotations
must not invalidate the secret owner already committed in old notes. This
snapshot alone establishes neither account consent nor proof verification.
[Native hooks and recipient constraints](../../../docs/native-asset-privacy/ELIGIBILITY.md)
now implement those publication/account bindings; input/spend work remains.
A wholly absent mapping cannot distinguish initialization from out-of-band
deletion/corruption; this is no account nonmembership or tree-wide integrity scan.

Builds, vet and source/physical-limit checks are the available verification.
No tests, cryptographic vectors, traces, constraint evaluations, proofs or node
demonstrations have run. Feature defaults and consensus activation remain off.
