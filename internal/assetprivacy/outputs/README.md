# Native encrypted output stream

Opt-in Go package `outputs` persists public encrypted notes for future scanning.
It is compiled only with `algorand_native_privacy`; no consensus version enables
the feature. There is no accepted private transaction or registered REST route.

`Open(kv, domain, disclosures)` loads a snapshot using the trusted immutable
policy declaration. `Record.Encode`/`Decode` enforce canonical fixed fields and
exact envelope presence. Recipient data is mandatory; auditor/controller data
is present exactly when declared, even after a corresponding power is dropped.
Only length/version framing is checked here; encryption proofs remain required.

The header is 250 bytes: version1, genesis32, assetBE8, policy64, indexBE4,
roundBE8, transactionID32, transactionNonce32, outputOrdinalBE4, commitment64,
disclosureFlags1. Role-ordered envelopes follow: recipient, optional auditor,
optional controller, each exactly 1629 bytes. Encodings are 1879, 3508 or 5137
bytes. Decode bounds the whole input before allocating owned envelope copies.

No owner, account, recipient public key or key epoch is serialized. A wallet
tries its privately known role/epoch keys. `record.Context(role, epoch)` rebuilds
the exact envelope context; `record.Recover(key, role, epoch)` authenticates the
envelope, decodes the note and checks asset/policy/commitment locally. The note
is secret material. Never return it from node RPC or include it in logs.
Recovery does not establish confirmation, membership, spending ownership or an
unspent note. Recipient key anonymity remains an unreviewed profile obligation.

Keys use `aprv:o:`||assetBE8 followed by `m` for metadata or `e`||indexBE4 for a
record. Metadata is 121 bytes: version1, full domain104, countBE8, lastRoundBE8.
Only absent metadata denotes the empty stream. Nonempty metadata requires a
count of 1 through 2^32 and a matching final record. Domain/index/role mismatches,
unknown flags, missing referenced records and future-to-head rounds are rejected.
These are local consistency checks, not an exhaustive database integrity audit.

`CheckAppend`/`Append` require the exact next index, nondecreasing round, matching
domain/declarations, a current snapshot and an unused record key. A serialized KV
must copy writes and atomically apply the record/metadata batch or roll it back.
All records remain immutable; retention/pruning and storage prices are pending.

`Page(start, limit)` accepts 1–64 records and start at most snapshot count. The
returned `Next`/`Count` are uint64 so the terminal cursor represents 2^32.
Start=count returns an empty page. Missing records fail the page, never skip it.
The 64-record limit bounds this component's allocation; it is not a finalized
consensus or REST contract. Reopen the store after external writes.

Native `AppendAssetPrivacyOutput` derives the domain/declaration from current
policy, requires record round=current round and matching tree/stream counts,
then appends the tree leaf/root history and stream record in one child COW.
Any error discards both. Future transaction handlers must use this coupled path.
The older standalone commitment helper is a primitive and does not populate the
stream. Mixing those paths is rejected when counts no longer match.

The caller still verifies complete proofs, all role/key/epoch bindings, current
eligibility, group/transaction binding, supply/nullifiers, authority, fees and
storage funding. Query helper `AssetPrivacyOutputPage` checks the native domain,
counts and current round. REST, SDK, scanning cursors, witness queries, replay,
catchpoints and real-node scenarios remain pending.

Feature-enabled builds/vet and default native builds passed. Formatting and
authored file/function limits passed. No tests, encryption recovery runs, proof
evaluation or replay checks were executed; tests remain deferred by the user.
