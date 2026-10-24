# Private ML-KEM encapsulation derivation

`MlKemDerivationAir` constrains canonical private key bytes and the two hashes
that derive the ML-KEM shared secret and encryption coins. It has no public
values and compiles only with `native-privacy`. It is not a full KEM proof.

`new()` fixes 1024 rows, a 1184-byte SHA3-256 key hash and a 64-byte SHA3-512
expansion message. For private key P and explicit secret randomness m32, the
relation constrains h=SHA3-256(P), G=SHA3-512(m||h), K=G[0..32], r=G[32..64].
The caller must generate fresh uniform m with a secure RNG. The relation does
not reject zero m, which is a possible standard randomness value.

The first 1152 P bytes encode exactly 768 little-endian 12-bit coefficients.
Trusted preprocessing selects every coefficient once in rows 0–767. Twelve
selected Boolean bits and twelve Boolean gap bits satisfy value+gap=3328,
so values at or above 3329 cannot be reduced silently. Selection uses the exact
same private P message bits as the key hash. Padding selects zero. The final
32 P bytes are rho, without a nonzero restriction.

The private key digest has 16 little-endian u16 limbs. They equal the second
32 bytes of the same private expansion message; its first 32 bytes hold m.
The expansion's 32 private digest limbs equal 512 Boolean output bits. Both
digests and output bits stay constant across rows. SHA3 constrains all message
bits, padding, permutations and final digest extraction. No host hash or public
digest replaces a private binding.

`key_bits`, `rho_bits`, `message_bits`, `shared_secret_bits` and `coins_bits`
return private byte-order slices with LSB-first bits. They reject rows with the
wrong width. A parent must bind these same bits to canonical key-polynomial
decoding, all matrix/noise operations, K-PKE encryption and the downstream KDF.
Current registry/key-epoch authorization and bounded matrix admission are not
established by this component. All nine matrix relations remain required.

The main layout is 15,844 columns: key hash 12,114, expansion hash 3,146,
48 private digest limbs, 512 output bits and 24 canonical scratch bits. It uses
129,794,048 raw base-element bytes plus 6,291,456 preprocessed bytes. Source
auxiliary constraints have degree at most 2; the SHA3 component retains its
pinned Keccak/absorption degrees. Full quotient/hiding budgets remain pending.

SHA3's crate-private height promotion fixes both components at 1024 rows while
preserving their default standalone heights. Its direct writer allocates only
a 21,569,536-byte Keccak temporary per hash and writes the enclosing trace
directly. These are static counts, excluding allocations, hiding and proving
overhead. No memory or timing measurements ran.

Use the verifier's exact 768-column selector schedule at base log-height 10,
extended to 11 by the candidate hiding configuration. Neither height nor the
schedule commitment can come from a proof. `trace(P,m)` rejects noncanonical
key coefficients before construction; it does not run constraints or prove.

Source comparison used Go 1.26.3's `kemEncaps` and `parseEK`. Compilation,
strict Clippy, formatting, size checks and source review only. No tests,
vectors, traces, constraint evaluation or proofs validated this component.
