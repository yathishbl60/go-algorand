# Compact private ML-KEM derivation

`MlKemStreamDerivationAir` constrains the canonical ML-KEM-768 encapsulation key
`P`, `H = SHA3-256(P)`, and `(K, coins) = SHA3-512(m || H)`. Its key,
message, hash, shared key and coins remain private. It exposes no public values.
The enclosing relation must authorize `P` and bind the matrix, noise and K-PKE
relations to the returned private wires.

The constructor fixes 1,024 rows, 5,810 main columns and 792 trusted preprocessing
columns. The preprocessing schedule is verifier-owned. The key hash occupies
nine 24-row Keccak frames; the expansion occupies one further frame. Other rows
carry the same private global registers and complete canonical coefficient checks.

The first 768 globals are canonical coefficients below 3,329. Remaining globals
are Boolean rho, message, key hash, shared key and coin bits. Every global stays
constant across all rows. A selected canonical witness checks each coefficient
once, including coefficients first consumed before their canonical-check row.

Each key block uses a 1,104-bit private window aligned to a complete 12-bit
coefficient. Window coefficients equal those same canonical globals. Overlapping
windows therefore agree without publishing key bytes or hash intermediates.
The trailing key bits equal rho; bits beyond the key are zero. Actual absorption
adds SHA3's `06` suffix and final `80` byte at their fixed positions.

The initial capacity is zero. Every subsequent key block absorbs into the full
previous Keccak state, including the unchanged capacity. The final key digest
equals the private hash bits used in the expansion. Expansion input and output
bindings fix all 64 message/hash bytes and all 64 shared-key/coin bytes.

`key_polynomials`, `rho_bits`, `message_bits`, `shared_secret_bits` and
`coins_bits` return slices only for an exact-width row. `trace(P, m)` constructs
private witness columns. The crate-private `write_into(P, m, values, target)`
writes one bounded frame into a parent stride. Length, bounds and canonical key
validation precede mutation and trace allocation. It creates only one small
Keccak permutation matrix at a time.

The raw main matrix occupies 47,595,520 bytes at eight bytes per field element;
trusted preprocessing occupies 6,488,064 bytes. These counts exclude proving,
commitment, allocator and caller overhead. The absorption constraint has source
degree five before an enclosing frame adds its selector.

Only source inspection, formatting and compilation are authorized at this stage.
No traces, constraints, proofs, interoperability vectors, tests or benchmarks
have run. This relation does not establish protocol readiness, security review,
constant-time witness generation or witness zeroization. The parent feature
remains disabled by default.
