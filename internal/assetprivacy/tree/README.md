# Native commitment tree candidate

This tagged package implements the append-only, depth-32 native commitment tree.
It remains inactive unless built with `algorand_native_privacy` and integrated
through a consensus version enabling `EnableNativeAssetPrivacy`. No current
version enables it. Transaction proof verification and policy authorization
must precede appends; this package does neither.

## Hash contract

Leaves are raw 64-byte note commitments. All hashes use SHA3-512. Concatenation
is unambiguous because every component after the ASCII prefix has a fixed size.

- Empty leaf: `Algorand/AssetPrivacy/Candidate1/MerkleEmpty/` followed by
  genesis (32 bytes), asset (8-byte big endian), policy (64 bytes).
- Internal node: `Algorand/AssetPrivacy/Candidate1/MerkleNode/` followed by
  genesis, asset, policy, level (1 byte), left (64 bytes), right (64 bytes).
- Level zero combines leaves; level 31 produces the root. The empty leaf is
  reserved and cannot be appended. Asset, genesis and policy must be nonzero.
- Empty subtrees recursively combine two identical empty children.

`Witness.Siblings[level]` is bottom-up. Bit `level` of its private `Index`
selects the right branch when set. `Leaf` is the raw commitment. `Root` and
`Count` identify the snapshot; indices must be less than count. Capacity is
exactly 2^32 leaves; the count is uint64 to represent a full tree.

## Persistence

All keys start with `aprv:t:` and the 8-byte big-endian asset identifier.
The prefix is separate from AVM boxes and the supply/nullifier namespaces.

| Record | Key suffix | Canonical value |
| --- | --- | --- |
| Metadata | ASCII `m` | version 1, genesis32, policy64, countBE8, root64 |
| Node | ASCII `n`, digest64 | version 1, level1, left64, right64 |

Metadata is 169 bytes; nodes are 130 bytes. Nondefault nodes must exist and
match their content hash and level. Wrong sizes, versions, domains and counts
fail closed. Empty subtrees need no node records. Node records are content
addressed and retained, so appends preserve nodes belonging to historical roots.
This does not define which historical roots native spending may accept.

## Usage and atomicity

`Open(KV, Domain)` loads a snapshot. `Append(commitment)` returns its index and
new root. `Witness(index)` reads the current membership path. `Root` and `Count`
return snapshot values. A changed metadata snapshot rejects operations as stale.
Reopen after another writer appends; callers must serialize KV operations.

`KV.Apply` must atomically apply the whole batch or roll it back on error.
Tree snapshots advance only after successful application. The COW adapter stages
all writes in a child, commits it on success, and discards it on error. A later
group failure still requires discarding that group's parent COW as usual.

Persistence includes 32 node writes plus metadata per append. Fees, storage
funding, pruning, accepted-root history, RPC witness privacy, recovery validation,
and transaction execution remain integration work. No tests have been added or
run, as requested; builds and vet provide only static verification.
