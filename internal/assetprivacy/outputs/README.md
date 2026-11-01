# Native encrypted output stream

Opt-in Go package `outputs` persists public encrypted notes for local scanning.
It is compiled only with `algorand_native_privacy`; no consensus version enables
the feature. Ledger/REST pages and a libgoal scanner have source integration;
no accepted private transaction or runtime scan has been demonstrated.

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
Records are immutable while live. Authorized retirement/closed-asset cleanup can
remove them through the coupled native inventory path. Retention/runtime recovery
and calibrated storage prices remain unverified.

`Page(start, limit)` accepts 1–64 records and start at most snapshot count. The
returned `Next`/`Count` are uint64 so the terminal cursor represents 2^32.
Start=count returns an empty page. Missing records fail the page, never skip it.
The 64-record limit bounds this component's allocation and the current REST page
contract; it is not a calibrated execution-resource budget. Reopen after writes.

Native `AppendAssetPrivacyOutput` derives the domain/declaration from current
policy, requires record round=current round and matching tree/stream counts,
then appends the tree leaf/root history and stream record in one child COW.
Any error discards both. Native transaction execution uses this coupled path.
The older standalone commitment helper is a primitive and does not populate the
stream. Mixing those paths is rejected when counts no longer match.

The caller still verifies complete proofs, all role/key/epoch bindings, current
eligibility, group/transaction binding, supply/nullifiers, authority, fees and
storage funding. Query helper `AssetPrivacyOutputPage` checks the native domain,
counts and current round. [REST queries](../../../docs/native-asset-privacy/REST_QUERIES.md),
round-pinned cursors, note witnesses and
[local libgoal scanning](../../../docs/native-asset-privacy/LOCAL_SCANNING.md) now exist
as source. Full SDK/wallet delivery, replay/catchpoints and real-node scenarios
remain pending. A note Merkle root does not authenticate output ciphertext
metadata; the local scanner requires a trusted, complete RPC output source and
grants no balance, receipt-finality, unspent-state or spending-authority claim.

Feature-enabled builds/vet and default native builds passed. Formatting and
authored file/function limits passed. No tests, encryption recovery runs, proof
evaluation or replay checks were executed; tests remain deferred by the user.
