# Separate mutable-tree capability

The existing `Tree` API remains append-only. `OpenMutable(KV, Domain)` returns a
separate `Mutable` capability embedding a tree snapshot. It is intended for
current key authorization state; do not hand it the note commitment KV namespace.
All code remains behind `algorand_native_privacy`.

`Replace(index, before, after)` requires an existing in-range index, a current
snapshot and an authenticated path whose leaf equals `before` exactly. It rejects
the domain's reserved empty leaf as `after`. The same index and count remain.
It constructs all 32 content-addressed path nodes with the existing level/domain
hash contract and includes unchanged-count/new-root metadata in one Apply batch.
The in-memory root advances only after successful atomic KV publication.

Stale snapshots, missing/corrupt nodes, domain/metadata errors and Apply failures
are propagated. The KV implementation must serialize operations and atomically
apply or roll back each batch, matching the append-only tree's existing contract.
No historical-root acceptance, authorization, deletion or index reuse is added.
The caller must reopen after other writers change the current root/count.

The [current key registry](../keyauth/README.md) places this capability behind a
separate KV namespace and stages it together with its stable domain-index update.
This keeps key rotation separate from commitment append/history semantics.

Source checks and build/vet only; no tests or runtime cryptographic checks ran.
