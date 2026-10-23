# Bounded candidate ML-KEM arithmetic

Typed components over q=3329, opt-in with `native-privacy`; no standalone
ML-KEM proof or native verifier. All wires are private unless a parent binds them.

`Canonical<T>` has a field value and two 12-bit decompositions: the value and
its gap to 3328. Boolean bits, exact reconstruction and value+gap=3328 force
the canonical range 0–3328. Each instance uses 25 columns.

`Add`, `Sub` and `Mul` constrain canonical operands/output. Addition has a Boolean
carry; subtraction a Boolean borrow. Multiplication constrains a canonical
quotient through a*b=3329*q+out. The largest products are below 11.1 million,
far below Goldilocks, so these integer equations cannot wrap modulo the field.
Add/Sub use 76 columns; Mul uses 100. Maximum degree is 2.

`Compression<T,D>` supports only D=4 and D=10, the ML-KEM-768 ciphertext widths.
Its D Boolean compressed bits, Boolean rollover and canonical remainder satisfy
3329*(compressed+rollover*2^D)+remainder=2^D*input+1664. This enforces exact
rounding and reduction modulo 2^D, including rounding 3328 to zero. It uses
51+D columns. Unsupported D returns no host witness and constrains false.

Call the matching `eval_*` function on the actual allocated wires. Each type's
`witness` rejects noncanonical inputs; these helpers construct columns without
evaluating or proving them. A parent must bind shared operands, NTT constants,
matrix/noise generation, byte packing and ciphertext/public-key encodings.
`ntt_twiddle(index)` and `ntt_gamma(index)` derive trusted canonical NTT constants
for indices 0–127 and reject larger indices. They are shared by transform/product
schedules; proof authors cannot replace the verifier's exact preprocessing.

Compilation, strict Clippy, formatting and size checks only. No arithmetic tests,
vectors, constraint evaluation or proofs ran; full KEM composition remains pending.
