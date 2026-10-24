# Private vertical ML-KEM NTT

`MlKemNttAir::forward()` and `::inverse()` constrain a complete 256-coefficient
transform with fixed trusted preprocessing and no public values. These are KEM
components, not standalone encapsulation proofs or accepted native operations.
They compile only with `native-privacy`.

The schedules follow [FIPS 203](https://doi.org/10.6028/NIST.FIPS.203), Algorithms
9–10, and were read against Go 1.26.3's `crypto/internal/fips140/mlkem/field.go`.
Both use seven stages of 128 butterflies. Forward lengths descend 128 to 2,
with increasing twiddle indices; inverse lengths ascend 2 to 128, with decreasing
indices. Twiddles are 17^BitRev7(k) modulo 3329. The inverse also constrains all
256 multiplications by 3303, the inverse of 128 modulo 3329.

`trace(input256)` rejects noncanonical u16 inputs and generates private columns.
`wires(row)` exposes input/output scalar coefficients for direct parent bindings.
Each slice has 256 values in the canonical 0–3328 range. Input/output are constant
across rows, equal to the initial/final state respectively. Intermediate state
uses 256 scalar registers. The two selected operands have canonical decompositions
on every row; all other coefficients persist unchanged.

Forward butterflies constrain t=zeta*b, low=a+t, high=a-t modulo q. Inverse
butterflies constrain low=a+b, d=b-a, high=zeta*d modulo q. Products have bounded
canonical quotients/remainders; sums/differences use Boolean carry/borrow. Thus
the largest integer equations are below 11.1 million and cannot wrap Goldilocks.
The final inverse scaling uses the same bounded quotient/remainder construction.
Padding has no selected addresses and preserves state. Maximum source polynomial
degree is 3, including row-transition selectors; quotient/proof budgets are pending.

The range invariant depends on the trusted stage-zero schedule. Forward pairs
are (j,j+128), j=0..127. Inverse groups of four pair positions 0/2 then 1/3. Thus
all 256 initial registers are selected exactly once before their first overwrite.
An untouched register equals its initial input by transition constraints; its
first read is range checked. Every write comes from a canonical low/high output.
This establishes bounded input/state/output coefficients for every satisfying
trace without repeating 256 range decompositions on every row. Changing the
schedule requires preserving that invariant or adding explicit input checks.

The layout uses 920 main columns: 256 state, 256 input, 256 output, six 25-column
canonical operands/arithmetic values and two Boolean carry/borrow columns.
Preprocessing has 514 columns: 256 left addresses, 256 right addresses, one twiddle
and one scaling flag. Forward has 896 operations plus padding to 1024 rows;
inverse has 896 butterflies and 256 scaling operations plus padding to 2048 rows.
The verifier chooses direction and exact preprocessing, never the proof author.

Use trusted preprocessing at base log-height 10 (forward) or 11 (inverse), extended
to 11/12 by the candidate hiding configuration. Trusted artifact preparation and
the complete KEM composition remain unfinished. Do not expose private wires or
use unreviewed cross-AIR lookup terminals to connect them.

Static base-element bytes: forward main 7,536,640/preprocessing 4,210,688;
inverse main 15,073,280/preprocessing 8,421,376. These exclude witness/prover
temporaries, hiding expansion and commitments; no memory measurements ran.

Source review, builds, strict Clippy, formatting and size checks only. No traces,
vectors, round trips, constraint evaluation or proofs ran. The implemented matrix,
NTT-domain product and packing components still require full KEM composition.
