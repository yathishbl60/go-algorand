# Candidate AES-256-GCM constraints

`Gcm<T>` describes the exact envelope AEAD component: 12-byte nonce, 64-byte
associated-data digest, 512-byte plaintext and 16-byte tag. It follows
[NIST SP800-38D sections 6–7](https://doi.org/10.6028/NIST.SP.800-38D).
Other lengths, non-96-bit nonces and truncated tags are unsupported.

`eval_gcm` checks one shared AES-256 key expansion and 34 cipher blocks:

| Block | AES plaintext | Purpose |
| --- | --- | --- |
| 0 | 16 zero bytes | GHASH subkey H |
| 1 | Nonce followed by BE32(1) | Tag mask |
| 2–33 | Nonce followed by BE32(index) | Payload stream blocks 0–31 |

Each ciphertext bit equals its plaintext bit XOR the corresponding stream bit.
All external byte bits are Boolean. AES bytes use least-significant bit first.
Each block binds all its rounds to that same constrained expanded key.

GHASH binds H to AES block 0, then four AAD blocks, 32 ciphertext blocks and
BE64(512) followed by BE64(4096). Byte-bit order is explicitly converted to
the GHASH wire convention. The tag equals GHASH XOR AES block 1; a host-only
tag equality never substitutes for these constraints.

`Gcm<Value>::witness(key32, nonce12, associated64, plaintext512)` constructs
private values. AES blocks and GHASH products are heap-backed and generated
one at a time. Individual large temporaries still need stack/allocation
measurement. This generator is not proof verification.

This is a typed component with no standalone AIR, trace codec, public-value
schema or native verifier. The parent must allocate/bind fields, call `eval_gcm`,
connect original key words to HKDF, plaintext to the committed note and AAD to
the context/key/KEM hash. It must bind nonce/ciphertext/tag to the actual envelope.
Inputs are not automatically public. Key eligibility, mandatory disclosures and
finalized-group binding remain parent obligations.

Separate columns for every typed field would total 4,025,556: 2,188,276 AES,
1,828,352 GHASH and 8,928 external bits. At 512 rows that is 16,488,677,376 bytes
of base field elements before hiding or prover overhead. This is static
arithmetic, not an implemented trace allocation, measured memory or acceptable
production budget. Practical vertical scheduling/private stage linking remain.

Ungated degree is at most two. New scheduling/gates must re-evaluate degree,
quotient/opening counts and hiding budgets. No tests, vectors or proofs ran.
Build/Clippy and source review establish static evidence only. Full KEM/KDF/note
composition, privacy validation and native acceptance remain unfinished.
