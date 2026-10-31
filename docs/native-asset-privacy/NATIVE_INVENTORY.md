# Candidate inventory for bounded native cleanup

Updated 2026-10-31. Every native data mutation now has disabled source integration
with a creator-funded per-asset inventory. [Authenticated retirement and bounded
cleanup](RETIREMENT_CLEANUP.md) now consume it before final destruction/storage release.
Every consensus flag remains false, catalogs empty and execution/storage prices
zero. No inventory transition, proof or node scenario has run.

## Completeness and mutation boundary

The evaluator provides native point reads rather than an asset-wide iterator.
Cleanup cannot rely on a caller's unverified list of records: omissions could
orphan state or release storage funding early. The central native storage meter
now maintains an inventory whenever it inserts, overwrites or deletes data.

The inventory starts with the asset's first policy insertion. Later writes require
existing metadata and an entry matching the exact old key/presence/value length.
It covers policy, supply, nullifiers, permanent creation markers, encrypted output
streams, note trees, key/control history and authorization/eligibility state.
Unknown or malformed namespaces fail as before. Inventory records and creator
counters are protected private bookkeeping, excluded from recursive indexing.

Entries form a doubly linked list in insertion order. Overwrite retains position;
deletion reconnects neighbors and removes the entry. Head/tail, item count and
original-data byte totals update with checked arithmetic. Exact key digests,
reciprocal deletion links and actual head-data lengths are checked. No caller
can select another asset's keys through this internal planner.

An absent index can initialize only at the first canonical policy insertion.
Unapproved experimental state without an index fails closed on later mutations;
there is no implicit partial backfill, approved migration or historical reference.

## Storage and atomicity

[The exact codec and API](../../internal/assetprivacy/inventory/README.md) define
145-byte metadata and141-byte-plus-key entries, with at most256 key bytes.
Metadata keys are24 bytes and entry keys88 bytes. Original data plus every
inventory key/value and creator counter contribute to the same native minimum
balance obligation. This increases logical state costs; safe numeric prices and
physical resource measurements are still outstanding.

For a data mutation, the adapter plans the inventory writes, calculates the
combined creator obligation from authoritative old values and checks final
funding. It then stages the counter/inventory and data in the same execution COW.
Later operation/group failure discards all of them. At most four inventory
records change per data mutation; no full asset scan or growing hole scan occurs.

`Read`, `First` and bounded `Keys` are internal interfaces for cleanup.
They confer no manager/PQ authorization, supply reconciliation or refund rights.
The inventory is ordinary authenticated ledger state, not a separate verifier or
proof-success shortcut. It does not expose private note openings or keys.

## Destruction integration and remaining evidence

Destruction rejects remaining native state. Retirement now requires current PQ
manager authority, zero private supply and full creator holding, then blocks new
activity. Bounded cleanup preserves fixed authentication records until the final
ordinary destruction wrapper removes them and the empty inventory. Only complete
cleanup releases its storage obligation. Private-only final reconciliation still
needs its atomic destruction proof path; ordinary withdrawals remain prohibited.

Build/static evidence does not prove those transitions. No tests, vectors,
runtime inventory walks, cleanup, replay, crash recovery, catchpoints or node
scenarios have run. The complete [delivery checklist](END_TO_END_STATUS.md) remains
active, including APIs, SDK/wallet, all modes and the six real-node demos.
