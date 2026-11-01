# Candidate local output scanning

Updated 2026-11-01. Feature-tagged libgoal now provides a bounded local scanner
using [the typed RPC client](GO_QUERY_CLIENT.md). It recovers openings locally;
it does not establish balances, unspent status, spending authority or finality.
Every consensus version remains disabled. No scanner, encryption recovery,
HTTP query, proof, test or real-node scenario has run.

## Configure trusted scope and historical keys

Build with `algorand_native_privacy`. Obtain an already configured
`*client.NativePrivacyClient` through RestClient or libgoal.NativePrivacyQueries.
`NativePrivacyScanDomain` pins Network (`crypto.Digest` genesis hash), positive
Asset, nonzero 64-byte Policy, and Disclosures (the policy's 0–3 declaration mask).
These pins must come from the caller's trusted network/policy configuration.

`RestoreNativePrivacyViewingKey(seed, role, epoch)` restores a local ML-KEM768
handle from a uniformly random 64-byte d||z seed. Roles are the client's
NativePrivacyRecipientView (1), NativePrivacyAuditorView (3) and
NativePrivacyControllerView (4); epoch must be positive. Candidate public-key
admission is checked. Keep the input stable during restoration, then clear it.
Retain historical role/epoch keys to recover outputs encrypted before rotation.

`NewNativePrivacyScanner(client, domain, keys)` accepts 1–64 nonduplicate historical
key handles and clones the handle slice. The immutable underlying expanded keys
remain in Go-managed memory; no heap/stack zeroization guarantee is provided.
Only public output-stream requests are sent: no local account list, viewing key,
key epoch, owner secret or note opening is uploaded to discover matching records.

## One atomic recovery page

With configured scanner, ctx and apiclient package alias, this unexecuted caller
fragment requests one page and then its continuation:

```go
page, err := scanner.ScanPage(ctx, apiclient.NativePrivacyOutputOptions{Limit: 32})
if err != nil { return err }
// Persist recovered openings and page.Checkpoint together before continuing.
next, err := scanner.ResumePage(ctx, page.Checkpoint, 32)
```

`ScanPage(ctx, options)` accepts `client.NativePrivacyOutputOptions`: optional
Round, Start, Limit and Next. Limits/cursors follow [the Go query contract](GO_QUERY_CLIENT.md).
Each page checks `GenesisHash(ctx)` from `/versions`, retrieves Outputs, and
reads Snapshot at the returned page.Round to check the same live policy/disclosures.
Those requests are owner-agnostic. A page contains at most 64 records and tries at
most 64 keys per record: at most 4096 local recovery attempts, without a CPU/time quota.

Only `envelope.ErrCiphertext` is treated as a key miss. Authenticated but invalid
note/context data aborts the whole page. The first successful key per record wins;
overlapping viewing roles do not duplicate that record. A miss is ambiguous with
ciphertext corruption. Any error or observed cancellation returns a zero result
and no checkpoint advance; cancellation does not interrupt an in-progress decrypt.

`NativePrivacyScanPage` returns Notes (`[]*NativePrivacyRecoveredNote`) and
Checkpoint together. `PublicMetadata()` excludes value, account, owner, viewing
role and epoch. `RecoveredWith()` exposes the local role/epoch explicitly.
`Value()`, `Account()`, `Owner()`, `Nullifier()` and `ExportOpening()` are explicit
private accessors, never RPC inputs. Formatting/JSON/text serialization redact
recovered-note/key handles; JSON-marshaling Notes does not create a wallet backup.
Whole-page JSON encoding fails with `ErrNativePrivacyScanStorage`; redacted notes
are not backups. Export openings explicitly and commit encrypted notes with progress.
Clear exported plaintext bytes after use. A recovered note's `Destroy()` drops
that handle's fields; it does not erase earlier copies or guarantee Go heap wiping.

## Resume and durable storage

The V1 checkpoint embeds NativePrivacyScanDomain and exports version, network,
asset, policy, disclosures, key-set, round, root, count, next and optional next-token.
Its fixed byte arrays serialize as JSON number arrays, not base64. String/GoString
formatting redact the checkpoint, while JSON intentionally exposes its fields.
KeySet hashes sorted role/epoch/public-key identities with domain separation and
scope. It reveals wallet-correlating coverage metadata; never publish or upload it.

`ResumePage(ctx, checkpoint, limit)` checks local scope, coverage and cursor before
querying. Use ResumePage for durable continuation; raw ScanPage options cannot
check earlier key coverage. Changed keys require discarding the checkpoint and rescanning old ranges;
reordering the same keys does not change coverage. A terminal checkpoint returns
`ErrNativePrivacyScanComplete` without a latest-round fallback. To scan newer
outputs, deliberately begin a fresh ScanPage snapshot with Start=checkpoint.Next.
An unavailable/pruned cursor also fails; choosing a new round/rescan is explicit.

Persist recovered openings and checkpoint atomically in authenticated, encrypted
local storage; that storage/backup implementation is not provided. Never persist
advanced progress before its notes. Checkpoint validation detects scope/cursor
mismatches, but does not authenticate stored progress or prove prior scan coverage.
Tampered progress can skip outputs; a key-set digest is not an integrity signature.

## Trusted-RPC boundary

`GenesisHash` is an RPC assertion, not a finality proof. A note Merkle root does
not authenticate ciphertext metadata, and the client does not verify finality.
The configured node must supply complete, authentic outputs. An untrusted RPC can
withhold/change records and make recoverable notes appear to be key misses; safe
skipping, receipt authenticity and completeness cannot be inferred from this scan.
No recovered value is an independently verified balance or proof of spendability.

Standalone SDKs, spending/proving, receipts, backups and complete wallet recovery
remain in [delivery status](END_TO_END_STATUS.md). Tests and runtime evidence remain deferred.
