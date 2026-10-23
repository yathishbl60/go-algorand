# Private ML-KEM SampleNTT prefix AIR

`MlKemSampleAir::new(rate_blocks)` accepts fixed bounds `1..=16` before
allocation. It receives exactly `rate_blocks * 168` private bytes from a
separately constrained SHAKE128 stream. `trace(prefix)` returns `None` when
the length differs or the prefix contains fewer than 256 candidates below
3329. One or two rate blocks can never complete the relation. Larger prefixes
can also exhaust: this cap is a candidate profile limit, not a guarantee for
every FIPS 203 seed. No zero padding, reseeding or alternate sampling rule
substitutes for exhaustion.

The relation implements the FIPS 203 SampleNTT ordering used by the local Go
reference `/usr/local/go/src/crypto/internal/fips140/mlkem/field.go:sampleNTT`.
Each three bytes give `d1 = b0 + 256*(b1 & 15)` followed by
`d2 = (b1 >> 4) + 16*b2`. The first 256 values strictly below 3329 become
the output polynomial, in original order. Values equal to 3329 are rejected.
After the 256th accepted value, the cursor absorbs and remaining bytes cannot
change outputs. The prefix still requires full SHAKE binding in composition.

Every candidate occupies a trusted preprocessed row with a one-hot position.
Private prefix bits stay Boolean and constant across rows. The schedule binds
the current 12 candidate bits to the next contiguous 12 prefix bits. For the
12-bit candidate `x`, Boolean `reject` and 12 Boolean remainder bits enforce
`x + 767 = 4096*reject + remainder`. All terms are bounded far below the
Goldilocks modulus, fixing `reject` exactly when `x >= 3329`.

A private 257-way Boolean one-hot cursor starts at zero and must finish at
256. Boolean `accept = active*(1-reject)*(1-cursor[256])` forces every early
valid value to advance it, and no invalid value can advance it. The next row
holds the cursor or advances exactly once. An accepted candidate must equal
the constant output selected by the current cursor. Completion therefore
links every output index exactly once to a canonical accepted value; the
prover cannot skip, reorder, repeat or replace accepted values. No private
lookup bus or public coefficient, candidate, seed, or cursor is used.

`prefix_bit_column(byte, bit)` exposes private LSB-first byte bits for direct
enclosing-AIR binding. `coefficients(row)` returns the private canonical
256-value output. Neither accessor establishes the SHAKE relation itself.
The enclosing relation must bind every prefix bit to the complete selected
SHAKE128 prefix and bind the SHAKE seed and matrix coordinates to ML-KEM.
The crate-private `write_into` writes directly into a bounded enclosing matrix,
sharing prefix columns without allocating a second complete sampler trace.
It checks lengths, checked row arithmetic and component bounds, and derives
all 256 accepted values before writes. Every component cell is overwritten;
surrounding columns remain untouched. Standalone `trace` delegates to it.

With `b = rate_blocks`, `C = 112*b`, `P = 1344*b`, and
`R = max(512, next_power_of_two(C+1))`, the main trace has `P+539` columns:
prefix bits `[0,P)`, output values `[P,P+256)`, cursor flags
`[P+256,P+513)`, candidate bits `[P+513,P+525)`, comparison remainder bits
`[P+525,P+537)`, reject at `P+537`, and accept at `P+538`.
The trusted preprocessed trace has `C` columns, with row `i` selecting
candidate `i` for `i<C` and all-zero padding thereafter. At least one padding
row makes the post-final-candidate cursor visible. Maximum algebraic degree
is 3, including transition gates. There are no public values.

At 16 blocks, `R=2048`, main width is 22043 and preprocessing width is 1792.
Raw Goldilocks matrices require 361152512 main bytes and 29360128
preprocessed bytes, excluding prover commitments, temporaries and hiding
extension. These are static layout counts, not measured memory or timing.
Setup must commit to the verifier-selected fixed schedule and base height;
the transaction cannot choose its own preprocessing commitment or bound.

This component has no native acceptance path. No runtime witness, constraint,
proof, or test execution has validated it, and it is not a reviewed security
profile or a complete ML-KEM relation.
