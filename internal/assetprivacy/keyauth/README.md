# Current public-key authorization registry

This `algorand_native_privacy` package authenticates the current public key
record for each complete `keys.Domain`. It persists public material only. It
does not authorize registration, verify possession or execute transactions.
Native callers must derive and authorize the exact before/after records from
current policy, issuer/account authority and the serialized native key store.

`LeafHash(record)` is SHA3-512 of the 50 ASCII bytes
`Algorand/AssetPrivacy/Candidate1/KeyAuthorization/` followed by
`keys.Encode(record)`. ML-KEM viewing records encode to 1338 bytes, making a
1388-byte leaf message. Other key roles retain their canonical role-specific
record lengths. The leaf includes network, asset, policy, account, role, epoch,
creation round and the exact public key; it is not merely a key-byte digest.

## Storage and current roots

The separate namespace is `aprv:authkeys:`. A private KV wrapper prefixes every
tree read/write with it; the wrapped tree's original `aprv:t:` keys are never
shared with the note commitment tree. Node hashing retains the exact
network/asset/policy-bound `tree.Domain.NodeHash` contract. The caller must keep
current key roots and note commitment/historical roots in distinct protocol roles.

Index keys are `aprv:authkeys:i` followed by the full 137-byte key domain:
network32, assetBE8, policy64, account32 and role1. An index value is exactly
214 bytes: version1 at0, domain137 at1..138, indexBE4 at138..142, leaf64 at142..206
and epochBE8 at206..214. Unknown versions, lengths, swapped domains or zero
epochs fail closed. An existing domain retains its index when the key rotates.
Only a genuinely new domain appends a leaf and increases the registry count.

A rotation replaces the existing current leaf through the distinct
[mutable-tree capability](../tree/MUTABLE.md); it never appends history as another
accepted current key. No historical key/root acceptance API exists here.
The native key store remains responsible for immutable historical public records.
An old account/capability becoming ineligible is a separate policy/control check;
membership in this registry does not restore relinquished authority.

## APIs and atomicity

- `Open(tree.KV, tree.Domain)` validates current tree metadata.
- `Snapshot()` returns `(Root, count, error)` from freshly read metadata.
  `Root` is a struct containing `Digest tree.Hash`; it cannot be assigned as a
  bare note/history hash without explicitly unwrapping `Root.Digest`.
- `Update(after, before keys.Record, exists bool, round uint64)` persists one
  preauthorized change. `after.Created` must equal round; first epoch is1 and
  before must be zero. Rotations require the same domain, exactly next epoch
  without overflow and nondecreasing creation round, plus the exact indexed old
  leaf/epoch. All records pass canonical key validation and domain checks.
- `Witness(record, round)` returns the current `tree.Witness` only if the full
  canonical record, index leaf/epoch, current path/root and creation round agree.

A prior native key record with no corresponding index fails closed. Callers
must pass authoritative `before/exists` values; this package does not silently
migrate legacy records or infer missing registrations. An index conflicting with
the supplied existence flag is rejected. Corrupt present metadata, missing referenced nodes and stale snapshots propagate
errors. Absent tree metadata denotes the deterministic empty registry; it cannot
authenticate an existing indexed key or silently migrate prior key history.

Update stages tree and index writes in an overlay and calls the parent KV's
`Apply` exactly once. The parent must serialize access and apply the batch
atomically, rolling back on failure. The store holds no mutable tree snapshot:
failed parent publication cannot leave advanced in-memory registry state. The
surrounding native group COW must still discard both ordinary key persistence
and registry changes if any group step fails. Never publish these separately.

Witness indices, account identities, epochs and key bytes are intended as
private proof inputs where the complete relation requires that privacy; current
registry roots come from authoritative native state. This package itself neither
constructs a proof nor prevents callers from disclosing their in-memory witness.

Build/vet, formatting and source review provide static checks only. No tests,
crypto vectors, trace evaluation, proofs, node execution, recovery or adversarial
runs were performed. Default builds expose no API and no consensus version is
enabled by this package. Native protocol acceptance remains unfinished.
