# Candidate native record inventory

This package maintains a per-asset doubly linked inventory of canonical native
data keys. It is enabled only by `algorand_native_privacy`. It supplies no account
authority, retirement permission, proof result or asset-destruction authorization.
All current consensus flags remain false and suite catalogs empty.

## Exact state

Keys begin with ASCII `aprv:inventory:` and assetBE8. Suffix `m` selects metadata;
suffix `e` plus64 raw digest bytes selects an entry. The complete metadata key is
24 bytes; an entry key is88 bytes. These keys are reserved for the private ledger
adapter and cannot be written through ordinary metered native KV operations.

Metadata is145 bytes: version1, itemCountBE8, dataBytesBE8, head64 and tail64.
Data totals include each original data key and value, excluding inventory and
the creator counter. Empty metadata requires zero totals/head/tail. More than one
item requires distinct head/tail IDs; a singleton requires equal IDs. Counts and byte arithmetic are checked.

An entry is141 bytes plus its original key: version1, keyLengthBE2, valueBytesBE8,
previous-present1/previous64, next-present1/next64, then the exact key bytes.
Presence bytes must be0/1; an absent link has zero ID bytes. Present zero IDs are
valid. Keys must be canonical native data keys for the same nonzero asset, bounded
to256 bytes. Inventory keys and creator counters are not data entries.

The ID is SHA3-512 of ASCII `Algorand/AssetPrivacy/Candidate1/InventoryKey/`,
keyLengthBE2 and exact data-key bytes. Reads recompute it. A distinct key cannot
replace an existing entry sharing its ID. This ID is an indexing mechanism, not
a cryptographic authorization statement or an independently approved artifact.

## Planning and ledger obligations

`Plan(Getter,asset,Change)` returns owned writes without applying them. The caller
must provide authoritative old/new presence and lengths. An absent inventory can
initialize only with an explicitly allowed first policy insertion. Missing
metadata on later mutations, missing/mismatched entries and inconsistent local
links reject. The node reads the actual old KV value before supplying a Change.

Insertions append at the tail; overwrites preserve position; deletions unlink
their immediate neighbors. No tombstone holes or per-mutation full scan is used.
A plan writes at most four inventory records. Deletion verifies reciprocal links.
This is local integrity checking, not an exhaustive audit of an imported chain.

`Read` returns a copied metadata record and explicit presence. `First` rejects
missing inventory and validates the head, its data presence/length and singleton
byte total. Empty inventory returns no first entry. These internal point-read
interfaces do not define an RPC or permit deletion.

The ledger adapter charges original data, inventory metadata, entries and link
changes to the authoritative creator. It checks funding for the final combined
obligation before publishing inventory changes. The enclosing native operation
and group must discard their COW on failure. Getter access must be serialized;
it must not change while a plan is being assembled. Input lengths are not proofs.

## Delivery limits

This inventory is a prerequisite for bounded authenticated retirement/cleanup.
The destruction guard remains in place. Cleanup, refunds, private-only final
reconciliation and runtime replay/catchpoint evidence remain unfinished.
Unapproved pre-inventory experimental state has no implicit migration path.

Builds, vet and static checks establish source properties only. No tests, vectors,
state-transition runs, node execution or cryptographic proofs have been performed.
