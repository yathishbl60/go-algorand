# Candidate scan-archive binary format V1

All offsets are zero-based and end-exclusive. Integers are unsigned big-endian;
arrays are raw bytes. This private local format is feature-tagged, unreviewed and
has no runtime interoperability evidence. It changes no transaction or consensus encoding.

## Encrypted frame

Every frame is exactly 40,549 bytes. AES-256-GCM authenticates the full padded
plaintext using a fresh96-bit random nonce; its full 128-bit tag is retained.

| Byte interval | Field |
| --- | --- |
| 0–6 | ASCII `NAPSC` followed by byte 1 |
| 6–18 | Random nonce |
| 18–40533 | Encrypted 40,515-byte plaintext |
| 40533–40549 | Authentication tag |

AAD is the exact concatenation of ASCII `Algorand/AssetPrivacy/ScanArchive/V1/`,
the six magic/version bytes, expected network32, assetBE8, policy64 and
one disclosure-mask byte. Domain values come from trusted caller configuration,
not an unauthenticated header. There is no password derivation or caller nonce.
KeySet is encrypted/authenticated inside the payload and deliberately excluded
from AAD, permitting restoration without the original viewing-key set.

## Padded plaintext

| Byte interval | Field |
| --- | --- |
| 0–1 | Checkpoint version 1 |
| 1–33 | Genesis/network hash |
| 33–41 | Positive asset ID |
| 41–105 | Nonzero policy identity |
| 105–106 | Disclosure mask 0..3 |
| 106–170 | Nonzero local key-set coverage identity |
| 170–178 | Pinned snapshot round |
| 178–242 | Nonzero pinned note-tree root |
| 242–250 | Snapshot output count, at most 2^32 |
| 250–258 | Next output index, at most count |
| 258–259 | Number of recovered notes, at most 64 |
| 259–40515 | Sixty-four 629-byte note slots |

Only the first note-count slots contain records. Every byte of all remaining
slots must be zero. A zero-note page still has the complete padded size.
Nonterminal output cursors are reconstructed from round/asset/policy/root/count/
next using the existing canonical [cursor](../../internal/assetprivacy/query/cursor.go).
Terminal next=count has no cursor, including terminal 2^32.

## Each occupied note slot

| Relative byte interval | Field |
| --- | --- |
| 0–4 | Note index |
| 4–12 | Output inclusion round |
| 12–44 | Transaction ID |
| 44–108 | Note commitment |
| 108–109 | Public viewing registration role 1, 3 or 4 |
| 109–117 | Positive viewing epoch |
| 117–629 | Canonical 512-byte [private opening](../../internal/assetprivacy/note/README.md) |

Indices strictly increase and are below checkpoint next/count. Inclusion round
cannot exceed the snapshot round. Auditor/controller roles require their declared
disclosure bit. Opening asset/policy/network and recomputed commitment must match
the checkpoint and record. These are consistency checks, not chain evidence.

Sealing requires current scanner key coverage. Restoring preserves the original
coverage identity even when the caller now has different viewing keys; ResumePage
separately refuses mismatched coverage. Rekeying changes only encryption and nonce.

Exact length/magic are checked before decryption. Authentication precedes all
plaintext interpretation. Unknown versions, malformed scope/records, wrong
padding, duplicates and inconsistent bounds fail with no restored page.
No frame carries viewing seeds, spending secrets, archive key or a latest pointer.

See [archive APIs and durability](ENCRYPTED_SCAN_ARCHIVES.md) for key lifetime,
create-only publication, trusted paths and the absence of archive-set rollback
protection. Tests, encryption, filesystem recovery and vectors remain deferred.
