# Private ML-KEM NTT-domain multiplication

`MlKemProductAir` constrains one complete product of two 256-coefficient NTT
polynomials. It is a private KEM component, not encapsulation or a native verifier.
It compiles only under opt-in `native-privacy`.

The relation follows [FIPS 203](https://doi.org/10.6028/NIST.FIPS.203), Algorithms
11–12. For each of 128 adjacent coefficient pairs it constrains
c0=a0*b0+gamma*a1*b1 and c1=a0*b1+a1*b0 modulo 3329, where
gamma=17^(2*BitRev7(pair)+1) modulo 3329. This is multiplication in the NTT
domain's quadratic factors, rather than coefficient-wise scalar multiplication.

`trace(left256, right256)` rejects coefficients outside 0–3328 and constructs
private columns only. `wires(row)` exposes the private left/right/output slices.
All three polynomials remain constant across rows. The first 128 rows select
every pair exactly once with trusted preprocessing; subsequent rows pad the trace.
The pair's gamma is constrained against that schedule on every row.

Four canonical operands and a canonical gamma feed five bounded modular products.
Two bounded modular sums produce canonical c0/c1. Multiplication reuses `Mul`
with shared operand wires; each product allocates only its output and quotient.
Addition reuses `Add` with shared intermediate wires and Boolean carries.
Every operand/output is range constrained. No equation approaches Goldilocks;
the largest scalar products are below 11.1 million. Maximum degree is 2 before
quotient/proof accounting. All results bind to the corresponding output slice.

The layout has 1195 main columns: 768 polynomial scalar wires, 125 operand/gamma
columns, 250 product columns and 52 sum/carry columns. There are 512 rows and
129 preprocessed columns: 128 pair selectors plus gamma. Static element storage
is 4,894,720 main bytes plus 528,384 preprocessed bytes, excluding witness and
prover overhead. No memory or timing measurements have run.

Use the trusted exact schedule at base log-height 9, extended to 10 by the
candidate hiding configuration. Never accept proof-supplied gamma/address flags.
Both inputs, output and all intermediates are private; do not serialize them or
join separate AIRs with unreviewed private lookup terminals.

A parent still must bind matrix sampling, noise/NTT, vector sums, inverse NTT,
message encoding, ciphertext compression/packing, key hashes and KDF/AEAD.
Builds, strict Clippy, source/format/size checks only. No tests, trace generation,
constraint evaluation, interoperability vectors or proofs ran.
