# Candidate SHA3-256 and SHA3-512 constraints

`Sha3Air::new(bytes)` selects SHA3-512; `new256(bytes)` selects SHA3-256.
The verifier fixes the variant and message length, from 0 through 4096 bytes.
A proof cannot select its own rate, digest length or message size. Capacities in
[FIPS 202 section 6.1](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf)
give byte rates 72 and 136 respectively. Padding uses `0x06` plus the final
block byte's `0x80` bit, including when both share a byte.

Message bits are Boolean and constant across rows. Initial capacity, every
Keccak permutation, absorption and capacity carry are constrained. Block
selectors must finish before the trace ends, preventing a short-trace bypass.
SHA3-512 has 32 public little-endian uint16 digest limbs; SHA3-256 has 16.
`digest_values` and `digest256_values` produce these respective layouts.

A parent can replace public digest limbs with bound private columns, as
HmacAir does. It must enforce their row consistency and consuming bindings.
A host hash or constructor parameter cannot substitute for those connections.

`trace_rows()` gives the greater of 512 and the next power of two covering all
required 24-round permutations. Existing note/owner/membership hashes stay at 512 rows.
The 2528-byte envelope AAD hash needs 1024 rows; maximum SHA3-512 needs 2048 rows.
Horizontal composition must align these schedules; current 512-row transfer
components do not automatically compose with longer hashes. The future proof
decoder must enforce trusted dimensions and work budgets.

4096 bytes is a component cap, not a consensus payload limit. It covers the
candidate AAD and ML-KEM public-key hashes. SHAKE/XOF squeezing and KEM
constraints remain separate. No tests, vectors, proofs or measurements ran.
Builds and Clippy establish static properties, not security approval.
