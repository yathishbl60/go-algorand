# Viewing-key possession: exact public-key relation

Updated 2026-10-30. The full candidate public-key generation relation now joins
private seed/noise/transforms, public matrix products and exact packed key bytes.
Viewing roles1/3/4 now route to real selected kind22 verification as disabled
source integration. Every consensus flag remains false and every suite catalog
empty. NAPK4 adds the layout slot, without approved artifacts or historical pins.

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
Private rho remains available to bind the exact public key's matrix seed.

[MlKemKeygenNttAir](../../internal/assetprivacy/stark/src/mlkem_keygen_ntt_air/README.md)
binds all six forward transforms to that same seed/noise root. Its inputs
alias the existing vectors; transformed outputs remain constant through padding.

[MlKemKeygenAir](../../internal/assetprivacy/stark/src/mlkem_keygen_air/README.md)
adds the exact public1184-byte registration key. Its trusted constructor decodes
canonical coefficients and derives all nine matrix polynomials from public rho.
Private seed expansion must yield that same rho. There is no private matrix seed
to hide in registration, so publicly derived constants are sufficient here;
encryption retains its separate private-recipient matrix constraints.

The parent binds proven transformed secret vectors to `A*s_hat`, using explicit
key-generation row ordering. Each canonical coefficient satisfies
`product+e_hat=t_hat+BooleanCarry*3329`. All packed public-key bytes are fixed public
values; the group digest occupies32 additional little-endian16-bit limbs. Proven
transformed vectors and group registers remain constant through the full schedule.

## Composition APIs and source dimensions

| Component | Rows | Main columns | Preprocessing | Public values |
| --- | ---: | ---: | ---: | ---: |
| Seed expansion | 512 | 3442 | 0 | 0 |
| Seed plus six noise vectors | 4096 | 7826 | 28 | 0 |
| Seed/noise plus six forward NTTs | 16384 | 9362 | 570 | 0 |
| Exact public key and finalized group | 16384 | 9394 | 723 | 1216 |

Standalone seed expansion has no selected verifier layout. The parent supplies
actual trusted frame flags, rather than manufacturing a dummy preprocessing key.
Wire accessors expose private seed/rho/sigma bits and ordered polynomial slices
for later direct bindings; these are witness interfaces, not ledger responses.

The enclosing schedule uses a compact10240-row seed/noise/NTT frame, a2048-row
product frame and4096 inactive rows. Compact writers retain every active row and
the final NTT boundary while omitting only standalone inactive padding. Child
writers check their target rectangles and reuse scratch. The enclosing trace
constructs matching private inputs before allocating its matrix and does not hold
a second complete child main matrix. No writer or trace has been executed.

## Native integration and remaining delivery

The source relation binds knowledge of `d` to the exact key's K-PKE secret
polynomials. It does not encode a full ML-KEM decapsulation key or validate entropy.
The [native viewing registration contract](VIEWING_REGISTRATION.md) now includes:

1. Recovery of the exact public key from the canonical lifecycle body and inclusion
   in the trusted artifact relation identity with an explicit domain.
2. Binding the relation to final group, suite, role/account, current PQ authorization,
   policy/expected epoch, real artifact selection and atomic native writes.
3. One exact kind22 MainOutput=0xffffffff/role0 bundle with1216 public cells,
   current-state work/fee/storage obligations and disposable-child rollback.

This adds no verifier-success fallback. Encryption or membership cannot substitute
for possession. Matching SDK/wallet proving and recovery flows, approved artifacts,
resource evidence, later validation and the six node scenarios remain required
by [the delivery status](END_TO_END_STATUS.md).

## Evidence limits

Build/static checks establish compilation and source structure only. No tests,
vectors, traces, constraint evaluations, setup/preprocessing, proofs or node runs
occurred. No complete key pair or registration proof was generated. Entropy,
cryptographic security, interoperable key generation and runtime bounds remain
unverified. Tests remain deferred by user instruction.
