# Candidate AES byte constraints

This module supplies composable, degree-two constraints for AES byte arithmetic.
Reference: [NIST FIPS 197-upd1](https://doi.org/10.6028/NIST.FIPS.197-upd1),
updated May 9, 2023, sections 4.2, 4.4 and 5.1.1. The inspected standard defines
the field polynomial and the inversion/affine construction of the AES S-box.
No external S-box table or host-computed output equality is trusted by the AIR.

Every byte contains eight Boolean bits in least-significant-bit-first order.
`eval_byte` checks this representation. `eval_xtime` checks multiplication by x
modulo `x^8+x^4+x^3+x+1` (`0x11b`): shift one position and reduce with `0x1b`
when the high bit is set. Its input and output are both range constrained.

`GfMul` contains 200 Boolean columns: two input bytes, seven successive xtime
powers of the left operand, eight selected bytes, and eight XOR accumulators.
Each selected bit equals one power bit times the corresponding right-input bit.
XOR is constrained as `a+b-2ab`, with selected bits stored separately to keep
constraint degree at two. `sums[7]` is the product; there is no unbound duplicate
output. Every intermediate byte is range constrained.

`SBox` contains 241 columns including its multiplication component. The left
operand is the input and the right operand is its inverse. A Boolean zero flag
forces both operands to zero in the zero branch. The constrained product must
be zero in that branch and one otherwise, which also forces the correct branch
for input zero. Four XOR stages constrain the affine map, then the output is
XORed with `0x63`; the zero input therefore produces `0x63`.

`GfMul<Value>::witness` and `SBox<Value>::witness` generate private intermediate
columns. Host arithmetic exists only for witness construction. Verifier-side
constraints independently enforce multiplication, inverse selection and affine
steps. Calling a witness constructor does not check a proof.

These helpers consume supplied columns and add constraints. An enclosing AIR
must allocate those columns, invoke every relevant evaluator, bind input and
output bits to its actual cipher/key schedule, and provide necessary row
transitions. Helpers do not expose public values or impose row transitions.

Every ungated helper constraint has degree at most two over the existing
Goldilocks field. Current backend parameters are unchanged. Composition with
additional gating raises degree and requires a fresh quotient/hiding-budget
assessment; no general security or resource claim follows from this local bound.

Remaining encryption obligations include AES-256 key expansion, all 14 cipher
rounds, round-key/state binding, counter construction, GHASH over GF(2^128),
authenticated associated data, length encoding and tag/ciphertext equality.
ML-KEM encapsulation, KDF binding, recipient/auditor/controller disclosure and
the complete transfer statement are separate relations. This module alone does
not establish AES-GCM encryption correctness or authorize a transfer.

No tests or proofs were executed. Compilation and static analysis are the current
validation boundary; cryptographic review and parameter analysis remain open.
