# Candidate encryption relation integration contract

Derived from `internal/assetprivacy/envelope/{context,aead,seal,open,format}.go`
and the local Go standard-library HKDF, ML-KEM and random-nonce GCM sources.
This specifies required bindings; the complete constrained relation is absent.
No recipient-anonymity or cryptographic-security claim follows from this contract.

## Exact context and envelope bytes

All integer context fields are unsigned big-endian; offsets are zero-based.
`Context.Bytes()` is exactly 256 bytes, concatenated without length prefixes:

| Offset | Bytes | Field |
| --- | --- | --- |
| 0 | 42 | ASCII `Algorand/AssetPrivacy/Candidate1/Envelope/` |
| 42 | 1 | Version `1` |
| 43 | 32 | Genesis hash |
| 75 | 8 | Asset ID |
| 83 | 64 | Policy digest |
| 147 | 8 | Key epoch |
| 155 | 1 | Role: recipient `1`, auditor `2`, controller `3` |
| 156 | 32 | Transaction encryption-context nonce |
| 188 | 4 | Output index |
| 192 | 64 | Note commitment |

Go validation requires nonzero asset, epoch, genesis, context nonce, policy and
commitment, and a known role. The 32-byte context nonce differs from GCM's nonce.
Envelope bytes are version `1` at 0, ML-KEM ciphertext at 1..1088, GCM nonce at
1089..1100, encrypted payload at 1101..1612, and tag at 1613..1628: 1629 total.
No public-key routing field is serialized. The plaintext is the exact canonical
512-byte note opening, including its zero reserved tail, used by its commitment.

## KEM, associated data and HKDF

Let `P` be the canonical 1184-byte ML-KEM-768 encapsulation key, `Ckem` the
1088-byte KEM ciphertext, and `m` fresh secret 32-byte encapsulation randomness.
The relation must enforce modulus/length checks for `P`, then ML-KEM encapsulation:
`h = SHA3-256(P)`; `G = SHA3-512(m || h)`; shared secret `K = G[0:32]`;
coins `r = G[32:64]`; `Ckem = K-PKE.Encrypt(P, m, r)`, with exact ML-KEM-768
sampling, NTT/modular arithmetic, compression and canonical packing semantics.
It must not accept an unconstrained KEM shared secret or encryption randomness.

The actual GCM associated data is the 64-byte digest
`A = SHA3-512(Context.Bytes() || P || Ckem)` of exactly 2528 bytes. The same `A`
is the HKDF salt. HKDF uses SHA3-512, 72-byte HMAC blocks, and this exact 42-byte
ASCII info string: `Algorand/AssetPrivacy/Candidate1/AES256GCM`.
`PRK = HMAC-SHA3-512(key=A, data=K)`; `T1 = HMAC-SHA3-512(key=PRK, data=info || 0x01)`;
AES key `k = T1[0:32]`. Both HMAC keys are 64 bytes, zero-padded to 72 bytes before
XOR with ipad `0x36` or opad `0x5c`; no long-key hashing branch is needed here.
The four SHA3-512 HMAC messages have lengths 104, 136, 115, 136 bytes respectively.

SHA3-256/512, private HMAC and the exact candidate HKDF are now constrained.
`Sha3Air` supports fixed messages through 4096 bytes, covering the AAD and
public-key hashes. The AAD hash needs 1024 rows; current note relations use 512,
so a consistent enclosing schedule remains required. Fixed-output SHAKE256 and
[eta=2 noise](../../internal/assetprivacy/stark/src/mlkem_noise_air/README.md) now exist.
SHAKE128 matrix sampling, polynomial/NTT, compression/packing and full KEM remain.
Their intermediate bits and digest limbs must connect directly to KEM, HKDF,
AES and the authoritative key bindings, rather than host-only witness checks.

## GCM and native public bindings

GCM convention: [NIST SP800-38D](https://doi.org/10.6028/NIST.SP.800-38D), sections 6–7.
For the public 12-byte GCM nonce `N`, `J0 = N || 0x00000001`.
Constrain `H = AES256(k, 0^128)` and tag mask `AES256(k, J0)`.
Payload blocks 0..31 use `AES256(k, N || BE32(2 + block))`; XOR those bytes with
the same committed opening to obtain the serialized 512 ciphertext bytes.
GHASH uses wire-order bits, reduction polynomial `x^128+x^7+x^2+x+1`, and 37
blocks: four blocks of `A`, 32 ciphertext blocks, then `BE64(512)||BE64(4096)`.
The 16-byte tag is the resulting GHASH value XOR the tag mask.

The native verifier must supply/bind the genesis, asset, policy, active key epoch,
role, transaction nonce, output index, commitment and complete envelope bytes.
The selected `P` must be authorized for the note/account and disclosure role by
current registry/policy state; keep shielded recipient identity/key witnesses
private and prove their required registry bindings. Enforce every mandated role
envelope. A valid encryption under an arbitrary private key is insufficient.
Finish encryption before computing the finalized group statement. Bind all final
bodies, ciphertexts and policy/profile context through the group digest supplied
to the proof transcript; keep proof witnesses outside those bodies to avoid a cycle.

## Resource implications and remaining status

One 512-byte envelope needs 34 AES block evaluations: hash subkey, tag mask and
32 payload counters. Sharing the current 14,452-column key schedule still gives
2,188,276 columns with the present block layout, or 8,963,178,496 bytes at 512
rows and eight bytes per field element, before GHASH, hashes, KEM, masking or
prover overhead. This is a static layout calculation, not measured memory or a
production limit. A vertical/shared schedule is needed for practical composition.
The [GCM component](../../internal/assetprivacy/stark/src/gcm_air/README.md) now
constrains the AEAD bindings under one shared key, with witness helpers. It has
no practical trace wrapper or full KEM/KDF/note composition. Its naive layout
totals 4,025,556 logical columns including GHASH/external bits; see
[resource constraints](CRYPTO_RESOURCES.md). It is not an accepted native proof.
No tests, proof generation or proof verification were performed for this document.
