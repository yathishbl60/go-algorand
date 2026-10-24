# Deterministic ML-KEM-768 encryption witness

`mlkem_host::encapsulate(public_key, m)` takes a canonical 1184-byte public key
and an explicit private `[u8;32]` message, returning `Option<Encapsulation>`.
The caller must generate a fresh uniform secret `m` with a secure RNG. This API
does not generate randomness, resample keys, encrypt a note, or verify a proof.
`None` rejects malformed keys or matrix-prefix exhaustion before returning a
ciphertext. The module is available only with the opt-in `native-privacy` feature.

The result contains `key_hash = SHA3-256(public_key)`, `shared_key`, `coins`, and
an `EncryptionWitness`. SHA3-512(m || key_hash) gives shared_key in its first
32 bytes and encryption coins in its last 32 bytes, matching FIPS 203
ML-KEM.Encaps_internal, Algorithm 17. The complete K-PKE.Encrypt computation
follows Algorithm 14:

1. Decode the first 1152 key bytes to three canonical 256-coefficient NTT
   polynomials `t`, rejecting every coefficient at or above 3329. The last
   32 bytes are `rho`. Length is checked before slicing or decoding.
2. Build all nine matrix polynomials with the exact first-accepted SampleNTT
   rule. Store `matrix[row][column] = SampleNTT(rho || column || row)`.
   Each uses the candidate fixed 16 SHAKE128 rate blocks, with no padding,
   reseeding or substitute rule if fewer than 256 values below 3329 appear.
3. Sample seven eta=2 polynomials from SHAKE256(coins || nonce), taking
   nonces 0,1,2 for `r`, 3,4,5 for `e1`, and 6 for `e2`. Forward-transform
   the first three into `r_hat` using the complete seven-stage NTT.
4. Compute each `u[i] = inverseNTT(sum_j matrix[j][i]*r_hat[j]) + e1[i]`
   and `v = inverseNTT(sum_j t[j]*r_hat[j]) + e2 + mu`, all modulo 3329.
   Products use the complete 128 quadratic NTT factors. Message bits are
   LSB-first and decompress to `mu[i]` in `{0,1665}`.
5. Compress each `u` to 10 bits and `v` to 4 bits using the exact existing
   witness rounding, then pack LSB-first. The ciphertext is exactly three
   320-byte `u` encodings followed by one 128-byte `v` encoding, 1088 bytes.

`EncryptionWitness` contains `public_polynomials`, `matrix_seed`, `matrix`,
seven sampled `noise` polynomials, `r_hat`, decompressed `message`, `u`, `v`,
and `ciphertext`. All polynomials use canonical `[u16;256]` values. The
`Encapsulation` fields include secret material; neither structure implements
`Debug` or serialization, and the module does not log their contents. Keep
these values in private prover memory. Only the ciphertext is publishable;
the encapsulation key itself can also be private in the enclosing relation.
Memory erasure and constant-time secret processing are not established.

The arithmetic reuses checked host helpers belonging to the corresponding
AIR components: NTT/product `values`, encoding `encode` and exact D=1/D=12
`decode`, noise `values`, matrix `values`, SHAKE128 `prefix`, and sampler
`values`. The trace generators and plain helpers share witness operations,
PRF/CBD arithmetic, first-accepted sampling and compression rounding. The
host API never allocates or extracts outputs from large field trace matrices.
SHAKE prefix generation retains only bounded byte/permutation-state buffers.

The admission bound matches the Go envelope parser: all nine matrices must
finish within 16 blocks. Exhaustion is rejected even for otherwise
standard-valid keys; coverage of all standard keys is not established.
A future proof profile must fix this bound and bind the same key seed,
matrix coordinates and first-accepted stream. A prover-supplied bound is
insufficient. There are 27 stored polynomial arrays, totaling 13824 raw
coefficient bytes; the complete result holds 15040 raw field/byte bytes
before compiler layout and temporary copies. These are static counts.

Source comparison used the installed Go reference
`crypto/internal/fips140/mlkem/mlkem768.go` (`kemEncaps`, `parseEK`,
`pkeEncrypt`) and `field.go` (`sampleNTT`, `samplePolyCBD`, `ntt`,
`inverseNTT`, `nttMul`, message decompression and encoding), corresponding
to FIPS 203 Algorithms 5,6,7,8,9,10,11,14 and 17. No new dependencies are used.

This host computation is witness generation, not algebraic enforcement.
Full private proof composition must still bind every derivation, key bit,
sample, matrix product, sum, transform, compressed output and ciphertext.
No native acceptance path is added. Runtime vectors, traces, constraint checks,
proofs, tests, interoperability and a cryptographic review remain unperformed.
