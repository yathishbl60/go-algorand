# Candidate current-key authorization registry

Updated 2026-10-25. Disabled native persistence and private record constraints
exist, including current-record membership and recipient envelope bindings.
Eligibility, disclosures, full spending and authenticated native execution remain
incomplete. A registry entry alone grants no authority.

## Current state and rotation

The registry domain is genesis32, assetBE8 and immutable policy64. Each full
key domain adds account32 and role1. The first record allocates the next uint32
tree index; a rotation replaces that account-and-role leaf at the same index.
The current root therefore authenticates the latest epoch, not every historical
key. Historical key records remain available for recovery and auditing.

All registry keys start with `aprv:authkeys:`. Tree metadata/content-addressed
nodes are namespaced separately from note commitments. Node hashing reuses the
domain-bound depth-32 Merkle contract. The index key adds `i` and the complete
137-byte key domain; its 214-byte value is version1, domain137, indexBE4,
leaf64 and epochBE8. Domain/epoch/shape mismatches fail closed.

A leaf is SHA3-512 of the 50-byte ASCII domain
`Algorand/AssetPrivacy/Candidate1/KeyAuthorization/` followed by the exact
canonical [keys.Encode record](../../internal/assetprivacy/keys/README.md).
For an ML-KEM viewing key this hashes 1388 bytes. Account, role, epoch,
creation round and every key byte are committed. Unknown/syntactically invalid
records and candidate matrix-exhaustion failures are rejected by native encoding.

New records require epoch1 and an absent index. Rotations require the exact
previous domain/epoch/leaf, epoch+1 without overflow, and nondecreasing creation
round. Existing history without an index, or an index without prior history,
fails closed. There is no implicit migration of earlier experimental state.

## Atomic native interfaces

`keyauth.Open(KV, domain)` provides `Snapshot`, `Update` and exact-current
`Witness`. It stages all path nodes, metadata and index writes, then invokes
the parent atomic Apply once. Failed updates retain no advanced snapshot.
KV operations must be serialized; concurrent unchecked writers are unsupported.

Native `OpenAssetPrivacyKeys` and `OpenAssetPrivacyKeyAuthorization` expose
read-only views. Mutations go through
`PutAssetPrivacyKey`, which wraps history, current pointer and registry
updates in one child COW. Any error discards the complete change. Policy/domain,
role eligibility and creation round checks precede publication. The caller must
still authenticate account/manager control, key possession, suite, fees and
storage funding; these internal methods are not RPC authorization boundaries.

`AssetPrivacyKeyAuthorizationRoot` returns `keyauth.Root { Digest }` and count
from current group-local state. This struct requires explicit unwrapping at the
proof statement boundary and cannot be passed directly as a note-root hash.
Never admit an authorization root through note-root history. Verification must
recheck the current registry after every preceding group member and invalidate
stale cached authorization results.

`AssetPrivacyKeyAuthorizationWitness(record)` checks the same current native
record before returning its path. This is an internal local proving interface;
it does not define an account-query RPC. Proofs must keep selected account,
index, epoch, viewing key and path private, including wallet query behavior.

## Proof integration

[ViewKeyAir](../../internal/assetprivacy/stark/src/view_key_air/README.md) commits
the same viewing-key record, with canonical coefficients and nonzero account/
epoch. Constructor roles are key roles1/3/4, corresponding to envelope roles
1/2/3. Its leaf and all account/key/epoch/creation wires remain private.

[RegisteredKeyAir](../../internal/assetprivacy/stark/src/registered_key_air/README.md)
now joins a private canonical record to staged depth-32 membership under the
current root. [Recipient authorization](../../internal/assetprivacy/stark/src/authorized_recipient_air/README.md)
joins viewing and spending membership to the same envelope account/key/epoch and
note owner. Their epoch streams stay independent. New outputs select the current
owner; old-note spending still proves that note's committed secret owner.
[Auditor/controller assignments](../../internal/assetprivacy/disclosure/README.md)
and their encryption constraints now exist; current freeze/recovery authority and
eligibility still require complete native proof/state bindings. Registry membership alone does not
designate a policy auditor or restore a relinquished issuer power.

Storage pricing, migration, replay/restart/catchpoint evidence, verifier work
limits and complete native execution remain unfinished. Private proof/wallet
resource feasibility remains unmeasured. Source reviews, feature/default builds,
vet, strict Clippy, formatting and size checks are the current evidence.
No tests, traces, vectors, constraint evaluations or proofs have run.
