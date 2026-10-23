# Private SHAKE128 matrix prefix

`Shake128MatrixAir::new(first, second, blocks)` constrains a SHAKE128 prefix
of rho32||first||second. Both coordinate bytes must be 0–2; blocks must be 1–16.
The verifier fixes these parameters and exact preprocessing. No seed, prefix
or digest is public. The module is opt-in under `native-privacy`.

The initial state contains the 32 seed bytes, the two coordinate bytes and
SHAKE's suffix byte 0x1f at offset34. Rate byte167 contains 0x80; other rate
bytes and the entire capacity are zero. All 100 input limbs are bound on the
first row. The pinned `KeccakAir` constrains each permutation and round flag.
Its complete 200-byte result becomes the next permutation input when another
squeeze block is required, including the capacity. No additional absorption
or padding occurs between squeeze blocks. Each block contributes 168 bytes.

Private seed and output bits remain Boolean and constant across rows. The
84 little-endian u16 rate limbs from every selected final round equal the same
output bit vector. Byte order is ordinary concatenation, with LSB-first bits.
`wires(row)`, `seed_bit_column` and `output_bit_column` expose private bindings.
Go 1.26.3's SHA3/ML-KEM sources supplied the rate, padding and byte-order reference.

Preprocessing has one column per block, set at row 24*(block+1)-1 only. Use
trusted setup; a proof-selected schedule could skip output constraints.
Standalone height is 512, with width 2889+1344*blocks. At 16 blocks the main
matrix is 99,913,728 raw bytes and preprocessing is 65,536 bytes. No timing
or memory measurements ran. Complete quotient/hiding budgets remain pending.

The crate-private composition path promotes the height to 512/1024/2048 while
keeping the same selected squeeze rows and padding remaining permutations.
The direct writer checks matrix arithmetic and column bounds before writing;
it generates only a Keccak temporary and returns the exact squeezed bytes.
`trace(seed)` uses that same writer. It constructs witnesses without evaluating
constraints or proving anything.

This module alone does not sample a polynomial. Use the
[composed matrix AIR](../mlkem_matrix_air/README.md) to connect every output bit
to the exact rejection sampler. Full KEM, authoritative key/rho bindings and
current native authorization are still absent. No tests, trace generation,
vectors, constraint evaluation or proofs have run.
