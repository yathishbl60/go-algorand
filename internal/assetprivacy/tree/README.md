# Native commitment tree candidate

This tagged package implements the append-only, depth-32 native commitment tree.
It remains inactive unless built with `algorand_native_privacy` and integrated
through a consensus version enabling `EnableNativeAssetPrivacy`. No current
version enables it. Transaction proof verification and policy authorization
must precede appends; this generic package does neither. Native notes use the
[notetree generation2 wrapper](../notetree/README.md) to enforce global creation
uniqueness and read-only native views; key/eligibility registries remain separate.

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
Accepted roots use the round-bounded history described below.

## Usage and atomicity

`Open(KV, Domain)` loads a snapshot. `Append(commitment)` returns its index and
new root. `Witness(index)` reads the current membership path. `Root` and `Count`
return snapshot values. A changed metadata snapshot rejects operations as stale.
Reopen after another writer appends; callers must serialize KV operations.

`KV.Apply` must atomically apply the whole batch or roll it back on error.
Tree snapshots advance only after successful application. The COW adapter stages
all writes in a child, commits it on success, and discards it on error. A later
group failure still requires discarding that group's parent COW as usual.

## Accepted-root history

`AppendAtRound(leaf, round, slots)` atomically stores the first pre-append root
and count for that round together with the append. Subsequent appends in the
same round preserve this first snapshot. This keeps a confirmed root usable
through a busy round. `AcceptRoot(root, count, currentRound, slots)` accepts the
current root/count or a stored snapshot with `currentRound - snapshotRound < slots`.
An unchanged current root remains accepted even after many idle rounds.

Slots come from the consensus parameter `NativeAssetPrivacyRootHistory`, never
from the transaction or local configuration. Every current consensus version
leaves this parameter zero and privacy disabled. Zero slots fail closed. Changing
the slot count in place is rejected; a future upgrade must define ring migration.
Native COW appends always use `AppendAtRound`, with the evaluator round and
consensus slot count. Generic `Append` is history-free and is not the native path.

The native key suffix `h` stores the newest pre-append snapshot; `r` plus an
8-byte big-endian slot stores each ring entry at `snapshotRound % slots`.
Both contain 185 bytes: the 169-byte metadata shape, roundBE8, slotsBE8.
The count must precede the current count; domains must match. Wrong versions,
lengths, future rounds, slot counts, aliased slots, missing newest entries and
inconsistent snapshot counts fail closed. Nonempty roots must have a valid
content-addressed top node. Expired entries may remain stored but cannot authorize
a root. All ring slots are checked; old snapshots and tree nodes are retained
until overwritten ring entries expire. Historical tree nodes are never pruned.

The first append in a round writes two history records plus 32 nodes and metadata;
later same-round appends write only the nodes and metadata. Fees, storage funding,
pruning, RPC witness privacy, recovery validation and transaction execution remain
integration work. No tests have been added or run, as requested; builds and vet
provide only static verification.
