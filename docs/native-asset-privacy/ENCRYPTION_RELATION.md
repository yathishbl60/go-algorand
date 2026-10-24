# Candidate encryption relation integration contract

Derived from `internal/assetprivacy/envelope/{context,aead,seal,open,format}.go`
and the local Go standard-library HKDF, ML-KEM and random-nonce GCM sources.
[EnvelopeAir](../../internal/assetprivacy/stark/src/envelope_air/README.md) implements these bindings for one envelope. Current key authority and complete native composition remain.
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
[Forward/inverse NTT](../../internal/assetprivacy/stark/src/mlkem_ntt_air/README.md) and bounded coefficient/compression constraints now exist.
[NTT-domain products](../../internal/assetprivacy/stark/src/mlkem_product_air/README.md), [canonical packing](../../internal/assetprivacy/stark/src/mlkem_encoding_air/README.md) and [bounded matrix sampling](MATRIX_SAMPLING.md) exist; [bounded full KEM](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/README.md) now composes them.
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

The native verifier must supply/bind the genesis, asset, policy and private active key epoch,
role, transaction nonce, output index, commitment and complete envelope bytes.
The selected `P` must be authorized for the note/account and disclosure role by
current registry/policy state; keep shielded recipient identity/key witnesses
private and prove their required registry bindings. Enforce every mandated role
envelope. A valid encryption under an arbitrary private key is insufficient.
Finish encryption before computing the finalized group statement. Bind all final
bodies, ciphertexts and policy/profile context through the group digest supplied
to the proof transcript; keep proof witnesses outside those bodies to avoid a cycle.

## Resource implications and remaining status

The [staged GCM AIR](../../internal/assetprivacy/stark/src/gcm_vertical/README.md)
now joins all 34 AES and 37 GHASH blocks and the tag in one AIR, with private key,
AAD and plaintext. Its 276 public scalars encode nonce/CT/tag. The 8192-row layout
uses 11,728 columns, or 768,606,208 raw main bytes before hiding/prover overhead.
This is complete AEAD algebraic composition, not a benchmarked full-envelope proof.
[Bounded full encapsulation](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/README.md)
now binds canonical P/m to K/r, all nine matrices, seven noise samples and public
1088-byte Ckem. It uses the candidate's fixed 16-block matrix admission rule.
[EnvelopeAir](../../internal/assetprivacy/stark/src/envelope_air/README.md) now feeds the same private P/K and public Ckem
through AAD/HKDF/GCM to the canonical committed note. Key registry authority,
mandatory role disclosures, spending integration and native acceptance remain.
The public statement omits the key epoch; private nonzero epoch bits feed AAD
and must be authenticated against the same private key by the registry proof.
See [resource constraints](CRYPTO_RESOURCES.md). No accepted native proof exists.
No tests, traces, proof generation or verification were performed.
