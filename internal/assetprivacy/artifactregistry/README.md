# Independently pinned preprocessing registry reader

`artifactregistry` is opt-in Go code under `algorand_native_privacy`. It reads an
exact candidate registry of twenty physical capability slots. It does not
approve a cryptographic suite, authenticate a proof or enable native acceptance.
There is no default approved registry or fallback to proof-advertised metadata.

`Load(encoded,Pin{Profile,Implementation,Digest})` requires three nonzero64-byte
pins supplied independently by trusted suite configuration. The file, transaction,
proof header, RPC caller or evidence manifest must not choose those pins. The
native boundary must additionally ensure the profile is allowed by its consensus
configuration and matches the authoritative policy suite. A local profile evidence
Manifest verifies supplied file integrity; it is not a preprocessing-key authority.

Load first checks pins and exact size, then copies a fixed1774-byte snapshot. It
checks header/profile/implementation/digest and every compiled layout before
allocating/publishing the returned Registry. No input buffer is retained. The
registry contains fixed value arrays; Profile(), Implementation(), Digest(),
Entries() and Entry(kind) return copies. Unknown kinds are absent. A zero-value
Registry is not a loaded registry. Callers must not concurrently mutate the input
while Load copies it; subsequent buffer mutation does not affect the result.

## Exact candidate binary layout

The file is exactly1774 bytes, with no trailing bytes, padding, native integer
sizes, aliases or optional records. The header occupies134 bytes:

| Byte interval, end excluded | Encoding |
| --- | --- |
| 0..4 | ASCII `NAPK` |
| 4 | Version7 |
| 5..69 | Profile64, identical to independent Pin.Profile |
| 69..133 | Selected reference64, identical to legacy Pin.Implementation |
| 133 | Count20 |

Exactly twenty82-byte records follow in order kinds1..14,17..22. Each
record contains kind1, rowsu32LE, mainWidthu32LE, preprocessingWidthu32LE,
publicCountu32LE, flags1 and eight rawu64LE root words (64 bytes). Present flags
are3: bit0 main-next and bit1 preprocessing-next. Absent flags are0 with root0.
Root words are SHA3 commitment digest words, not Goldilocks field cells. They
are never reduced as field representatives. Present roots must be nonzero; absent
slots retain exact dimensions. Empty scopes and inconsistent flags/roots reject.
Missing capabilities cannot bind proofs. Independent pins/provenance remain required.

| Kind | Native layout | Rows | Main | Preprocessing | Public |
| --- | --- | ---: | ---: | ---: | ---: |
| 1 | Transfer | 16384 | 8506 | 80 | 264 |
| 2 | Unshield | 16384 | 8506 | 80 | 196 |
| 3 | Staged shield | 1024 | 7641 | 8 | 100 |
| 4 | Partial private recovery | 8192 | 8539 | 60 | 192 |
| 5 | Full private recovery | 8192 | 8539 | 60 | 160 |
| 6 | Partial public recovery | 8192 | 8539 | 60 | 164 |
| 7 | Full public recovery | 8192 | 8539 | 60 | 132 |
| 8 | Confidential transfer | 16384 | 8506 | 80 | 392 |
| 9 | Confidential unshield | 16384 | 8506 | 80 | 324 |
| 10 | Confidential shield | 1024 | 7641 | 8 | 228 |
| 11 | Confidential partial private recovery | 8192 | 8539 | 60 | 320 |
| 12 | Confidential full private recovery | 8192 | 8539 | 60 | 288 |
| 13 | Confidential partial public recovery | 8192 | 8539 | 60 | 292 |
| 14 | Confidential full public recovery | 8192 | 8539 | 60 | 260 |
| 17 | Ordinary recipient | 65536 | 24869 | 2075 | 1132 |
| 18 | Assigned disclosure | 65536 | 23973 | 2067 | 1132 |
| 19 | Retained remainder recipient | 65536 | 24229 | 2067 | 1100 |
| 20 | Recovery recipient | 65536 | 24869 | 2071 | 1100 |
| 21 | Serial Owner registration | 512 | 1953 | 11 | 64 |
| 22 | Viewing registration | 16384 | 9394 | 723 | 1216 |

Version7 supersedes unapproved versions1..6. Serial Owner keeps kind21 and retains
its compiled main/preprocessing widths to1953/11 and source fingerprint. Earlier
3540/4 entries/pins fail closed; there are no approved historical references. Kind alone
selects a public schedule, not an entire transaction-specific relation. The Rust
bridge must independently derive the full relation ID from the exact constructor
arguments, revision and selected supported reference; the artifact ID also binds the trusted
profile/key. Construct the correct typed AIR/vector. No private epoch belongs in that
public relation identity. Role, output ordinal, body/group context, current roots
and applicable authority remain native statement/authentication obligations.

The independent registry Digest is SHA3-512 of ASCII
`Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV7/` followed by the entire
1774-byte file. `Digest(encoded)` is a fixed-size offline integrity helper only:
it checks length and computes that digest, without checking header/layouts,
provenance or approval. A digest read from the same file is not an independent pin.

The reader never generates preprocessing, traces or proofs, never accesses the
filesystem/network, and never chooses deployment work limits. It supplies no
successful-verification token or transaction authority. Actual preprocessing
provenance, security/resource analysis, approved suite data, native transaction
acceptance and adversarial/runtime evidence remain required. `suitecatalog` selects
pins from current consensus. Current catalogs are empty, with no approved fallback.

[Serial Owner primitive runs](../../../docs/native-asset-privacy/OWNER_REGISTRATION.md) verified via direct pinned P3,
with degree5/16 quotient chunks and3,285,504 query-cell payload bytes. Worker times
were1.719s/168.828MiB and1.694s/187.578MiB observed RSS; the expanded run rejected
seven corrupted traces and three changed public/proof cases. They did not
exercise this registry/native codec or establish approved artifacts. The
[conditional4,226,753-byte complete-proof bound](../../../docs/native-asset-privacy/SERIAL_OWNER_LAYOUT.md) is not measured serialized size.
[Scoped artifact-bound regression](../../../docs/native-asset-privacy/SCOPED_SUITE_REGISTRIES.md)
now passes. [Native Owner registration](../../../docs/native-asset-privacy/NATIVE_OWNER_E2E_RESULTS.md) passes;
defaults remain disabled and full private-value delivery is pending.
