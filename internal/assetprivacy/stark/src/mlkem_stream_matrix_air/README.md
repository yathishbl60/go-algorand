# Private streamed matrix SampleNTT

`MlKemStreamMatrixAir::new(first, second)` accepts each coordinate in `0..3`.
It constrains SHAKE128 of `rho32 || first || second`, followed by the first
256 accepted 12-bit candidates below 3329. The seed and coefficients are
private columns. An enclosing hiding AIR must bind both to its other relations;
this component alone does not prove complete KEM encryption or note ownership.
It is exported only with the disabled-by-default `native-privacy` feature.

## Fixed shape and wires

The trace has 4096 rows, 4872 main columns, 118 preprocessing columns, and no
public values or lookup buses. Half-open main-column ranges are:

| Columns | Meaning |
| --- | --- |
| 0..2633 | Pinned Plonky3 Keccak scratch |
| 2633..2889 | 256 private little-endian seed bits |
| 2889..2989 | 100 carried 16-bit Keccak state limbs |
| 2989..4333 | 1344 Boolean rate bits, little-endian |
| 4333..4589 | 256 constant output coefficients |
| 4589..4846 | 257 Boolean one-hot cursor positions |
| 4846..4858 | Selected 12-bit candidate |
| 4858..4870 | 12-bit comparison remainder |
| 4870 | Rejection flag |
| 4871 | Acceptance flag |

`seed_bits(row)` and `coefficients(row)` require exactly 4872 columns and return
fixed-size borrowed arrays. Public `trace(seed)` returns `None` on exhaustion.
The crate-private `write_into(seed, values, (stride, start_column, first_row))`
returns the 256 canonical coefficients. Invalid target bounds, non-integral
parent rows, and exhaustion leave the parent buffer unchanged. Success overwrites
all component cells in its 4096-row frame, including padding and inactive scratch.

## Schedule and constraints

Each of 16 blocks occupies 136 rows: a complete 24-round Keccak permutation,
then 112 consecutive 12-bit candidates from that block's 168 rate bytes. Thus
2176 active rows are followed by 1920 padding rows. All 16 permutations are
constrained, even if the output is already full. This admission bound matches
the Go 16-prefix policy; it does not assert every standard ML-KEM seed fits.

Preprocessing columns 0..112 select candidate positions. Columns 112..116 are
Keccak active/first/last/transition flags. Column 116 selects later block starts;
117 carries rate bits except between active blocks. Preprocessing comes solely
from `BaseAir::preprocessed_trace()` and must be included in trusted setup with
these exact coordinates and shape. A prover must never supply another schedule.

The initial preimage is the seed, the two coordinate bytes, suffix `0x1f`,
zero fill, final rate-byte `0x80`, and zero capacity. The state buffer starts
zero and changes only on the transition after each last Keccak row, when all
100 limbs equal the actual final-round output. Later first preimages equal
that buffer. The rate bits are Boolean, constant throughout each block, and
bound in 84 little-endian limbs to that same final-round result. Padding holds
the final state and rate.

Trusted selectors force every candidate's bits, with zero outside sample rows.
The Boolean equation `candidate + 767 = 4096 * reject + remainder12` forces
rejection exactly at 3329. Acceptance equals sample-active times non-rejection
times non-full cursor. The one-hot cursor starts at zero, advances exactly on
acceptance, absorbs at 256, and must end at 256. Each accepted candidate equals
the cursor-selected constant output coefficient. This traversal also constrains
every output coefficient to its canonical residue, without a separate range bus.

## Source-derived degree and resources

The pinned Keccak AIR uses degree-at-most-three expressions (including XOR3,
chi, parity, and transition gates). The local frame's active gate raises its
bound to four. Root sampling, cursor, state, and rate equations have degree at
most three, counting trusted preprocessing as degree one and standard root
boundary selectors as degree zero. Hence this component has conservative maximum
degree four; one enclosing active frame raises the bound to five. Prover setup
must budget for that composed degree. These are source bounds, not execution
measurements or a security review.

Goldilocks main cells require 159645696 bytes (152.25 MiB), preprocessing
3866624 bytes (3.6875 MiB). The direct writer retains one padded 32-row Keccak
matrix (674048 bytes), 17 small host states, and 2688 prefix bytes, rather than
allocating full SHAKE or sampling component matrices. These sizes exclude prover
commitments, quotient expansion, allocator overhead, and enclosing trace storage.

No tests, vector runs, constraint evaluation, proof generation, performance
measurement, or independent cryptographic review were performed for this module.
Runtime validation remains deferred under the user's current instruction.
