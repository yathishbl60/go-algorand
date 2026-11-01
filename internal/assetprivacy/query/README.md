# Public native privacy query records

The algorand_native_privacy build tag gates these source-only query helpers.
Read(reader,config) reads fixed policy, aggregate private supply, retirement,
closure and inventory metadata. Reader and Config must describe one authoritative
round. Config supplies network, asset, round, creator/manager/total, ASA existence
and creator holding presence/amount. The package does not acquire a ledger lock,
approve a suite, expose arbitrary KV data or authorize a transaction.

Snapshot contains owned decoded public records. Ordinary ASAs have no privacy
records. A privacy policy requires nonempty inventory and valid contextual
identity/supply. Retirement additionally matches manager/policy/suite, creation
ordering and zero-private/full-creator reconciliation. Committed live ASAs cannot
have a pending closure. A destroyed ASA requires closure/inventory and absence of
policy/supply/retirement; its creator, manager, total and policy ID come from the
closure. Missing and fully cleaned assets return ErrNotFound. Conflicts reject.
Inventory metadata is not a complete inventory integrity walk.

OutputPage adds selected round/asset/policy/root to outputs.Page. CheckPage bounds
limit1..64 and start<=2^32 before any ledger reads. CheckOutputState rejects
retired/closed assets, and ordinary ASAs without a privacy policy. NoteWitness
contains a public tree.Witness with its selected domain/round metadata. It grants
no ownership, current eligibility or authorization. Root acceptance can expire.

Cursor encodes only public version1/roundBE8/assetBE8/policy64/root64/countBE8/
nextBE8 fields. Its161 bytes use exact215-character unpadded base64url. DecodeCursor
checks exact length, strict encoding, canonical re-encoding, version, nonzero
asset/policy, Next<=Count and Count<=2^32 before returning copied fields.
CheckSnapshot matches them to independently loaded state; it is not a signature.
OutputPage.HasMore reports Next<Count. NextCursor validates the page and returns
empty at completion, or a canonical cursor for continuation. Root words retain
all64-byte values; an all-zero hash is not rejected as a placeholder.

The [ledger contract](../../../docs/native-asset-privacy/LEDGER_QUERIES.md) covers
locking, historical retention, suite gates and cursor behavior. The ledger rejects
writes through its read adapter and discards partial results on storage failures.
REST routing exists for the earlier query methods; SDK, live queries, tests, vectors, proofs and node execution remain pending.
Every consensus version remains disabled with no approved suite catalog.

AuthorizationSnapshot, KeySnapshot and EligibilitySnapshot expose only public
current registry/assignment/effective-freeze data. Registration membership does
not imply live issuer assignment; missing records imply no membership permission.
ReadOnly[T] adapts a Getter to each store's typed rejecting Apply method.
The [authorization query contract](../../../docs/native-asset-privacy/AUTHORIZATION_QUERIES.md)
describes role flags, current roots, retained views and freeze-source reconciliation.
