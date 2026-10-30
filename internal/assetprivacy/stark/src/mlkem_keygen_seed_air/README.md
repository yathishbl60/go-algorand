# Private ML-KEM key-generation seed expansion

MlKemKeygenSeedAir fixes the33-byte SHA3-512 message shape, constrains its final
byte to3 and exposes private rho/sigma output bits. The32-byte seed, digest limbs
and output bits remain private. No public values or native acceptance are added.

The hash uses the existing SHA3 constraints. Its32 digest limbs are bound to512
Boolean output bits, split256/256. Constant registers prevent a parent from using
different output seeds at different stages. The hash-message seed bits already
have Boolean/transition constraints. The rank byte is checked explicitly.

`new` constructs the fixed circuit. `wires` exposes exact private seed/rho/sigma
slices only for a correctly sized row. `trace` constructs a512-row matrix;
crate-private `write_into` validates the full destination before writing its
512-by3442 rectangle and preserves surrounding rows/columns. No trace ran.

The [seed/noise parent](../mlkem_keygen_noise_air/README.md) provides actual stage
preprocessing and binds all six noise polynomials. This standalone component has
no selected verifier/artifact and is not a public-key possession proof. The
[full contract](../../../../../docs/native-asset-privacy/KEYGEN_POSSESSION.md)
identifies public-key matching and registration work still required.

No tests, vectors, traces, proofs or key-generation operations ran. Compilation
and source checks do not establish soundness, privacy or deployment readiness.
