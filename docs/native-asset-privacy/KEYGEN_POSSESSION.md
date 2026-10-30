# Viewing-key possession: seed and noise stages

Updated 2026-10-30. Two private ML-KEM key-generation components now exist.
They are intermediate constraints for the complete registration relation, not an
accepted possession proof. Viewing roles1/3/4 still reject. Every consensus flag
remains false and every suite catalog empty; no new artifact entry was added.
The Rust source fingerprint changes; no historical approval or migration is added.

## Relation implemented

[NIST FIPS 203](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf), Algorithm13,
starts ML-KEM-768 key generation by expanding `d32 || byte3` with SHA3-512 into
rho32 and sigma32. Its first six PRF/CBD polynomials use sigma and nonces0..5:
three secret polynomials followed by three error polynomials.

[MlKemKeygenSeedAir](../../internal/assetprivacy/stark/src/mlkem_keygen_seed_air/README.md)
constrains the complete33-byte hash input, fixes the parameter byte and binds the
private output limbs to Boolean rho/sigma bits. The private input seed and outputs
remain constant across rows. It exposes no public values or secret serialization.

[MlKemKeygenNoiseAir](../../internal/assetprivacy/stark/src/mlkem_keygen_noise_air/README.md)
joins that seed expansion to six existing eta2 noise components. Each nonce is
fixed by the circuit constructor; every noise seed equals the same proven sigma.
The six canonical coefficient vectors are bound into constant parent registers.
Private rho remains available for the subsequent matrix relation.

The trusted schedule has one seed frame and six noise frames,512 rows each, then
512 padding rows. Its28 preprocessing columns are seven sets of active/first/
last/transition flags. Public/proof metadata cannot choose or skip those stages.
Global Boolean seed bits and coefficient registers remain constant through padding.

## Composition APIs and source dimensions

| Component | Rows | Main columns | Preprocessing | Public values |
| --- | ---: | ---: | ---: | ---: |
| Seed expansion | 512 | 3442 | 0 | 0 |
| Seed plus six noise vectors | 4096 | 7826 | 28 | 0 |

Standalone seed expansion has no selected verifier layout. The parent supplies
actual trusted frame flags, rather than manufacturing a dummy preprocessing key.
Wire accessors expose private seed/rho/sigma bits and ordered polynomial slices
for later direct bindings; these are witness interfaces, not ledger responses.

Both components provide trace constructors and bounded crate-private direct
writers. Writers check the full target rectangle before writes, preserve cells
outside it, and reuse hash/noise scratch. The parent clears its rectangle and
writes all frames and root registers without allocating a second child trace.
These are source properties; no writer or trace has been executed.

## Remaining complete relation

Knowledge of a derivation seed or six vectors alone does not establish possession
of the exact registered public key. The enclosing relation still must:

1. Derive every admitted matrix polynomial from the same rho, with the candidate's
   exact bounded sampling rules and correct key-generation row/column ordering.
2. Bind forward NTTs of all six vectors and the correct modular matrix-vector
   products plus error. Encryption's transposed multiplication is insufficient.
3. Bind canonical packed public coefficients and rho to every byte of the actual
   registration key, without a host-only equality or independent witness copy.
4. Join that complete relation to final group, suite, role/account, current PQ
   authorization, policy/expected epoch, real artifact selection and native writes.

No native viewing-key admission, verifier-success hook or syntax-only fallback was
added. The existing encapsulation relation cannot substitute for private-key
possession. Full delivery, resource feasibility and the six node scenarios remain
required by [the delivery status](END_TO_END_STATUS.md).

## Evidence limits

Build/static checks establish compilation and source structure only. No tests,
vectors, traces, constraint evaluations, setup/preprocessing, proofs or node runs
occurred. No complete key pair or registration proof was generated. Entropy,
cryptographic security, interoperable key generation and runtime bounds remain
unverified. Tests remain deferred by user instruction.
