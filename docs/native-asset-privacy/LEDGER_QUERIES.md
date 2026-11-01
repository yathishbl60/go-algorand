# Candidate ledger privacy queries

Updated 2026-11-01. The opt-in ledger exposes owned public state, bounded encrypted
output pages and public note membership witnesses at one explicitly selected
round. [REST routing](REST_QUERIES.md) now exists; SDK methods remain pending. All consensus activation flags
remain false and suite catalogs empty. No runtime query, proof or node has run.

## Methods and the round contract

| Ledger method | Result |
| --- | --- |
| LookupLatestNativeAssetPrivacy(asset) | Snapshot with its selected latest Round |
| LookupNativeAssetPrivacy(round,asset) | Snapshot at that exact round |
| LookupNativeAssetPrivacyOutputs(round,asset,start,limit) | OutputPage at that round |
| LookupNativeAssetPrivacyOutputCursor(cursor,limit) | Next page of the same snapshot |
| LookupNativeAssetPrivacyWitness(round,asset,index) | Public NoteWitness at that round |

Every wrapper holds trackerMu.RLock across its complete operation and calls
account trackers directly. It never recursively calls a Ledger wrapper that
acquires the same lock. Creator, ASA resource, policy, supply, retirement, closure,
inventory and output/tree data all use the selected round. The block header at
that round selects the consensus feature gate and suite catalog. No endpoint,
caller pin or record may approve a suite implicitly.

The tracker lock prevents new blocks entering these reads; database commits can
still advance retention. Each lookup must return data valid at the pinned round
or an error. If retention advances past that round mid-operation, the whole result
is discarded. Never substitute latest data or return a partial successful page.
Archival block retention does not imply historical account/KV availability.
Header, tracker retention and database errors propagate for later API mapping.

## Snapshot states

A Snapshot includes Round, Asset, Exists, Creator, Manager, Total, creator holding
presence/amount, Policy, PolicyID, aggregate Private, Retired, Closure and Inventory.
All decoded arrays, structs and pointers are owned by the returned value; mutable
tracker buffers never escape. Query helpers expose no private opening or key.
The internal KV adapter rejects every Apply attempt with query.ErrReadOnly.

An ordinary live ASA has Exists=true and no privacy records. A live privacy ASA
requires canonical policy and nonempty inventory metadata, a matching network/
asset identity and private aggregate no greater than Total. A missing aggregate
record represents zero. Retirement must match the current manager, policy/suite,
creation ordering, zero private supply and full creator holding. A pending closure
with a live ASA is rejected: the final pair cannot persist halfway through.

After final destruction, Exists=false with closure and nonempty inventory, and
without policy, supply or retirement. Creator, Manager, Total and PolicyID come
from the closure. A fully cleaned or unknown missing ASA returns query.ErrNotFound.
Malformed or conflicting records fail. Inventory metadata is decoded; this query
does not walk the complete inventory or prove every retained key exists.
Current authorizer lookup and actual signed consent remain separate obligations.

## Output pages and continuation

Limit is1..64; start is uint64 and must not exceed the selected count. Zero or
oversized limits reject before tracker reads or output allocation. Count and Next
can represent the full2^32 capacity. Output metadata and generation2 note-tree
counts must agree; the output stream's last round cannot exceed the query round.
Records are checked for exact index, domain, disclosures, framing and ownership
of returned ciphertext buffers. Missing/gapped records reject the entire page.

The page includes its Round, Asset, PolicyID, Root, Count, Next and encrypted
Records. HasMore() compares Next<Count; NextCursor() returns an empty string at
the end. For a nonterminal page it encodes the strict public continuation fields:
version1, roundBE8, assetBE8, policy64, root64, countBE8 and nextBE8. The161 bytes
use canonical unpadded base64url, exactly215 characters. Unknown version, wrong
length, alternate encoding, invalid asset/policy or Next>Count or Count>capacity rejects.
No field is a secret or a signature; the ledger rechecks every field against the
loaded snapshot. Root/count/policy/asset/round mismatch returns query.ErrCursor.

Use the latest snapshot's Round for the first output request, then its cursor for
continuation. Keep the same round until the scan completes. If pruned, restart at
an available round and deduplicate by asset/policy/output index. The cursor is
not authorization, a spending witness, permanent retention or an authenticated
receipt. A page does not include per-record Merkle proofs.

## Witnesses, cleanup and delivery limits

The public witness query checks the note-tree creation mapping and Merkle path,
then matches its leaf to the output record at the same index. It returns the
selected Round/Asset/PolicyID plus fixed-depth tree.Witness. This proves no secret
ownership or current eligibility, and its root may expire for later spending.
Every actual transaction must recheck accepted roots and current native state.

Output pages and witnesses reject retired or closed assets with query.ErrUnavailable;
cleanup can remove their historical records. Historical queries work only while
that exact state remains available. Wallets still need encrypted backups, old
viewing epochs, receipts and retained block/indexer data for long-term restoration.

[Authorization/key/eligibility ledger queries](AUTHORIZATION_QUERIES.md) also exist. REST runtime/error evidence,
transaction decoding/simulation, SDK/wallet callers and all six demos remain in
[the delivery checklist](END_TO_END_STATUS.md). Builds, vet, linking, formatting
and source limits establish compilation properties. Tests, retention/race cases,
codec vectors, runtime queries and end-to-end validation remain deferred.
