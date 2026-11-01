# Serial recipient owner-secret registration relation

OwnerRegistrationAir proves the network-bound SHA3-512 SpendingKey commitment for
a private nonzero32-byte secret. Public values remain64 uint16 cells: owner32
then finalized group32, both consecutive little-endian limbs.

The109-byte message is ASCII `Algorand/AssetPrivacy/Candidate1/SpendingKey/`,
network32 and secret32. SHA3-512 absorbs two72-byte-rate blocks with standard
padding and all24 Keccak rounds per block. Each round is scheduled in three
stages: Theta, Rho/Pi, then Chi/Iota. Fixed initialization/prefix, capacity,
absorption, padding, final digest and group binding remain constrained.

The512-row frame uses1953 main columns:1600 Boolean state bits,320 C parities,
a private inverse and32 group limbs. Its11 preprocessing columns are four
schedule selectors plus seven Iota round-constant bits. C parity is an ungated
five-way XOR; the actual symbolic degree is5. Group/inverse cells stay constant.

The second-block absorption XOR of before/after Boolean state derives the256
secret bits. Their bounded sum times the private inverse equals1, rejecting a
zero secret. Final digest limbs equal public owner; group limbs equal public group.
`public_values` rejects zero owner/group. `trace` checks nonzero secret and exact
network-bound owner before allocation; independent AIR constraints enforce the
relation. Native layout kind21 now pins512/1953/11/64, replacing3540/4 dimensions.
Old source fingerprints/layouts do not become compatible or approved implicitly.

The first serial direct-P3 proof run completed in1.719s/168.828MiB observed peak
RSS. An expanded run completed in1.694s/187.578MiB, with valid trace constraints,
seven corrupted-trace rejections and changed owner/group/proof-opening rejection.
Symbolic degree5,16 quotient chunks and3,285,504 query-cell payload bytes were
observed. These direct-P3 runs did not exercise the canonical native codec or node.

[Serial layout analysis](../../../../../docs/native-asset-privacy/SERIAL_OWNER_LAYOUT.md)
gives a conditional complete-proof upper bound of4,226,753 bytes; actual serialized
native proof size is not established here. See [the native contract](../../../../../docs/native-asset-privacy/OWNER_REGISTRATION.md)
and [bounded primitive evidence](../../../../../docs/native-asset-privacy/FEASIBILITY_RESULTS.md).
No suite was activated, native transaction accepted or repository test files added.
A primitive proof grants no account/spending authority and establishes no general
soundness, privacy, resource feasibility or full-protocol validation.

Native callers still check current PQ Sender/AuthAddr, registeredAccount==Sender,
policy/suite/epoch, final group binding and exact one-slot NAPW before state writes.
Viewing-key possession uses the separate typed ViewingRegistration relation(kind22).
