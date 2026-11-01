# Candidate encrypted scan archives

Updated 2026-11-01. Feature-tagged libgoal can seal one recovered scan page and
its checkpoint together, restore it offline and publish an immutable archive file.
Build with `algorand_native_privacy`; every consensus version remains disabled.
No encryption, key generation, filesystem recovery, test or node demo has run.
This is source integration, not a reviewed full-wallet backup format.

## Keys and local API

`GenerateNativePrivacyArchiveKey()` obtains an OS-random AES-256 key.
`RestoreNativePrivacyArchiveKey(bytes)` requires exactly 32 bytes and rejects the
all-zero key; it is not a password-to-key interface. `NativePrivacyArchiveKey` redacts ordinary
formatting, JSON and text. `key.Export()` explicitly returns owned secret bytes
for a separate protected key backup. Clear exported buffers after use. Losing the
archive key can make its archived openings unrecoverable; never store it beside
its ciphertext without an independently protected key-storage mechanism.

| API | Behavior |
| --- | --- |
| `scanner.SealScanPage(key, page)` | Validate and encrypt one page into owned bytes |
| `scanner.OpenScanPage(key, encoded)` | Authenticate, validate and restore one page |
| `scanner.SaveScanPage(path, key, page)` | Seal, then create one immutable archive file |
| `scanner.LoadScanPage(path, key)` | Read a bounded regular archive and open it |
| `OpenNativePrivacyScanArchive(key, domain, encoded)` | Restore offline without RPC/viewing keys |
| `LoadNativePrivacyScanArchive(path, key, domain)` | Load and restore offline |
| `RekeyNativePrivacyScanArchive(oldKey, newKey, domain, encoded)` | Re-encrypt a validated archive |
| `RekeyNativePrivacyScanArchiveFile(oldPath, newPath, oldKey, newKey, domain)` | Re-encrypt into a new immutable file |

Offline calls require the caller's trusted NativePrivacyScanDomain. They make no
RPC request, discover no viewing key and grant no spending authority. Re-encryption
preserves the old checkpoint/key coverage and generates a fresh nonce. A scanner
can open old coverage after viewing-key rotation, but ResumePage still rejects
that old coverage; new viewing keys require rescanning. SealScanPage validates
current scanner coverage. Keep input pages/bytes stable throughout each call.

## V1 authenticated framing

[Exact binary offsets and validation rules](SCAN_ARCHIVE_FORMAT.md) support future SDK implementations.

[Archive crypto](../../libgoal/assetprivacy_archive_crypto.go) uses standard-library
AES-256-GCM via `cipher.NewGCMWithRandomNonce`: a random 96-bit nonce is prepended,
with 28 bytes total nonce/tag overhead. Outer magic is six bytes `NAPSC\x01`.
Every V1 archive is exactly 40,549 bytes, including pages with no recovered notes.
Plaintext is 40,515 bytes: a 258-byte checkpoint, one note-count byte and 64 fixed
629-byte slots. Count is at most 64; unused slots must be zero. Fixed padding
prevents file length from revealing matched-note count. Filenames, file-set/page
counts, access patterns and other caller-controlled metadata remain observable.

AAD binds `Algorand/AssetPrivacy/ScanArchive/V1/`, magic and the trusted network,
asset, policy and disclosure mask. KeySet is inside the authenticated plaintext,
not AAD, so historical archives can be restored without their old viewing keys.
After authentication, decoding checks scope/checkpoint/cursor consistency,
canonical openings and commitments, role/epoch/disclosures, ordered unique note
indices and record bounds. Errors yield no restored page or advanced checkpoint.

The standard-library nonce rule permits no more than 2^32 encryptions per key.
Callers must manage that lifetime limit across processes/restores, including
rekeys; no persistent usage counter is provided. Key material, expanded cipher
schedules and decoded openings can have Go-managed copies. Buffer clearing does
not guarantee heap/stack zeroization or erase earlier exported copies.

## File publication and durability

[Saving](../../libgoal/assetprivacy_archive_file_save.go) seals before touching the
filesystem, creates a 0600 temporary file in the existing parent, writes/syncs/
closes it, then publishes by hard link without overwriting any existing entry,
including a symlink. The parent directory is synced after publication. Filesystems
without the necessary hard-link operation fail; there is no overwrite fallback.
Use one new path per page to retain historical recovered notes and progress.
The parent directory/path must be caller-controlled and trusted.

If `errors.Is(err, ErrNativePrivacyArchiveDurability)`, publication happened but
directory durability is unconfirmed. Reconcile the existing destination; do not
assume nothing was written or blindly overwrite it. Temporary-name cleanup is
best effort and not separately directory-synced; encrypted orphan names may
remain after failure/crash. This interface supplies no journal or latest pointer.

## Backup and trust limits

Archives contain recovered private openings, public output identity, local
role/epoch and checkpoint. They omit viewing seeds, spending secrets, the archive
key and complete wallet state. Keep those separately recoverable under an actual
wallet backup policy. Recovered handles remain redacted; explicit opening access
and destruction follow [local scanning](LOCAL_SCANNING.md).

Authentication detects changes to one archive under its key/scope. It does not
detect rollback to an authentic older archive, deletion of other archive files,
or reordered history, and cannot determine which archive is latest. No anti-rollback
anchor, archive-set manifest or wallet-wide transaction is implemented.

The [scanner's trusted-RPC boundary](LOCAL_SCANNING.md) still applies. Restoring
notes proves neither consensus finality nor unspent state, spending ownership,
a balance or receipt authenticity. Full wallet/SDK delivery and six real-node
scenarios remain in [delivery status](END_TO_END_STATUS.md). Tests, vectors,
cryptographic execution and filesystem recovery evidence remain deferred.
