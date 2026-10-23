# Vertical candidate GCM AES stage

`AesGcmAir` constrains all 34 AES-256 blocks under one private expanded key:
zero input for H, nonce||BE32(1) for the tag mask, and counters 2–33 for
the 512-byte payload. It includes every key-schedule and cipher round.
This is the AES stage; GHASH, KDF, KEM, note/context bindings and native
verification must still be composed before an envelope can be accepted.

`trace(key32, nonce12, plaintext512)` constructs private columns only.
`public_values(nonce12, ciphertext512)` encodes 12 nonce-byte scalars followed
by 256 little-endian u16 ciphertext limbs. The caller supplies the statement's
ciphertext. The constraints equate it to plaintext XOR the AES counter stream.

The fixed main layout uses 11,216 Boolean columns and 512 rows. Thirteen key
stages occupy rows 0–12; 34 groups of 14 cipher rounds occupy rows 13–488;
the remaining rows pad the trace. Sixteen S-box slots and four MixColumns slots
are reused by row. All S-boxes remain constrained, including unused slots.
Round transitions apply only inside a block; each block resets from its input.
The full expanded key, plaintext, H and tag mask are constant across rows.

`wires(row)` exposes private key32/plaintext512/H16/tag-mask16 bit slices for
direct parent constraints. Bytes are concatenated in ordinary order, each with
LSB-first bits. GHASH's MSB-first wires require reversing bits within each byte.
Do not publish these wires or connect separate AIRs with unreviewed private buses.

The 61 preprocessed columns contain 13 key-stage, 14 round and 34 block flags.
Use the verifier's trusted fixed preprocessing at base log-height 9, with the
candidate hiding configuration's extended height 10. A prover-selected schedule
is invalid. Maximum algebraic degree is 4 before quotient accounting.

Static base-element storage: 45,940,736 main bytes plus 249,856 preprocessed
bytes; this excludes witness-generation temporaries and all proving overhead.
Builds/Clippy/format/size checks only; no trace, constraint, vector or proof runs.
This unreviewed component is compiled only with `native-privacy`.
