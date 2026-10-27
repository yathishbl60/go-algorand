# Independently pinned preprocessing registry reader

`artifactregistry` is opt-in Go code under `algorand_native_privacy`. It reads an
exact candidate registry of ten public preprocessing commitments. It does not
approve a cryptographic suite, authenticate a proof or enable native acceptance.
There is no default approved registry or fallback to proof-advertised metadata.

`Load(encoded,Pin{Profile,Implementation,Digest})` requires three nonzero64-byte
pins supplied independently by trusted suite configuration. The file, transaction,
proof header, RPC caller or evidence manifest must not choose those pins. The
native boundary must additionally ensure the profile is allowed by its consensus
configuration and matches the authoritative policy suite. A local profile evidence
Manifest verifies supplied file integrity; it is not a preprocessing-key authority.

Load first checks pins and exact size, then copies a fixed954-byte snapshot. It
checks header/profile/implementation/digest and every compiled layout before
allocating/publishing the returned Registry. No input buffer is retained. The
registry contains fixed value arrays; Profile(), Implementation(), Digest(),
Entries() and Entry(kind) return copies. Unknown kinds are absent. A zero-value
Registry is not a loaded registry. Callers must not concurrently mutate the input
while Load copies it; subsequent buffer mutation does not affect the result.

## Exact candidate binary layout

The file is exactly954 bytes, with no trailing bytes, padding, native integer
sizes, aliases or optional records. The header occupies134 bytes:

| Byte interval, end excluded | Encoding |
| --- | --- |
| 0..4 | ASCII `NAPK` |
| 4 | Version1 |
| 5..69 | Profile64, identical to independent Pin.Profile |
| 69..133 | Selected reference64, identical to legacy Pin.Implementation |
| 133 | Count10 |

Exactly ten82-byte records follow in order kinds1,2,4,5,6,7,17,18,19,20. Each
record contains kind1, rowsu32LE, mainWidthu32LE, preprocessingWidthu32LE,
publicCountu32LE, flags1 and eight rawu64LE root words (64 bytes). Flags must
be3: bit0 main-next and bit1 preprocessing-next. Unknown bits fail closed.
Root words are SHA3 commitment digest words, not Goldilocks field cells. They
are never reduced or range-checked as field representatives. All64-byte root
outputs, including zero, are permitted; independent pins and provenance determine
trust. Missing keys or placeholders never create an approved registry implicitly.

| Kind | Native layout | Rows | Main | Preprocessing | Public |
| --- | --- | ---: | ---: | ---: | ---: |
| 1 | Transfer | 16384 | 8506 | 80 | 264 |
| 2 | Unshield | 16384 | 8506 | 80 | 196 |
| 4 | Partial private recovery | 8192 | 8539 | 60 | 192 |
| 5 | Full private recovery | 8192 | 8539 | 60 | 160 |
| 6 | Partial public recovery | 8192 | 8539 | 60 | 164 |
| 7 | Full public recovery | 8192 | 8539 | 60 | 132 |
| 17 | Ordinary recipient | 65536 | 24869 | 2075 | 1132 |
| 18 | Assigned disclosure | 65536 | 23973 | 2067 | 1132 |
| 19 | Retained remainder recipient | 65536 | 24229 | 2067 | 1100 |
| 20 | Recovery recipient | 65536 | 24869 | 2071 | 1100 |

Dimensions match the current NullifierV2 Rust native AIR layouts. Kind alone
selects a public schedule, not an entire transaction-specific relation. The Rust
bridge must independently derive the full relation ID from the exact constructor
arguments, revision and selected supported reference; the artifact ID also binds the trusted
profile/key. Construct the correct typed AIR/vector. No private epoch belongs in that
public relation identity. Role, output ordinal, body/group context, current roots
and applicable authority remain native statement/authentication obligations.

The independent registry Digest is SHA3-512 of ASCII
`Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV1/` followed by the entire
954-byte file. `Digest(encoded)` is a fixed-size offline integrity helper only:
it checks length and computes that digest, without checking header/layouts,
provenance or approval. A digest read from the same file is not an independent pin.

The reader never generates preprocessing, traces or proofs, never accesses the
filesystem/network, and never chooses deployment work limits. It supplies no
successful-verification token or transaction authority. Actual preprocessing
provenance, security/resource analysis, approved suite data, native transaction
acceptance and adversarial/runtime evidence remain required. `suitecatalog` selects
pins from current consensus. Current catalogs are empty, with no approved fallback.

No tests, digest vectors, traces, preprocessing, constraint evaluations or proofs
have run. Builds, vet, source review and physical limits establish only the
reported source/compilation properties. Defaults remain disabled.
