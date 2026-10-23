# Private ML-KEM polynomial encoding

`MlKemEncodingAir::new(D)` selects exactly D=12, 10, 4 or 1 and rejects every other
width before shifts or allocation. All variants have 512 rows, no public values
and verifier-chosen trusted preprocessing. This module is enabled only through
the parent crate's `native-privacy` feature. It is a component of encapsulation,
not a complete KEM proof or an accepted native operation.

Each row contains 256 private input coefficients followed by 256*D private packed
bits. Both arrays are constant across rows. Packed bit `i*D+j` is bit j of
coefficient i's encoded value, least significant bit first. Consecutive groups
of eight bits form bytes, again least significant bit first. Thus D=12 encodes
two coefficients per three bytes; D=10 encodes four per five bytes; D=4 encodes
two per byte; D=1 encodes eight per byte. Encoded lengths are 384, 320, 128 and 32
bytes respectively. `wires(row)`, `bit_column(i,j)` and `byte_bits(row,byte)`
expose private binding wires without adding public output values.

D=12 implements canonical ByteEncode12. Each input has a Boolean 12-bit
decomposition plus a Boolean 12-bit complement summing to 3328. Packed bits equal
that decomposition exactly; inputs 3329 or greater cannot be reduced silently.
D=10 and D=4 use the existing actual `mlkem_field::Compression` constraints:
`3329*(encoded + rollover*2^D) + remainder = input*2^D + 1664`. Input and remainder
are canonical, encoded bits and rollover are Boolean. This implements nearest
integer compression modulo 2^D, as used for ML-KEM-768 ciphertexts.

D=1 implements message ByteDecode1 followed by Decompress1: the coefficient is
exactly 1665 times the selected Boolean message bit. It does not compress an
arbitrary coefficient to one bit. All 11 spare local bits are constrained zero;
other variants likewise constrain every spare local bit zero.

Preprocessing has 256 one-hot selectors: row i selects coefficient i for
0<=i<256, and all later rows select nothing. A single 25-column canonical input,
12 local bits, one rollover and 25-column canonical remainder occupy 63 scratch
columns. Selection binds both the input scalar and its D packed bits on every
active row. Padding binds local input and bits to zero. For compressed variants
the padding remainder is 1664; for uncompressed variants rollover and all
remainder columns are zero. Each global coefficient is therefore checked by
the actual arithmetic relation, rather than only by witness-generation checks.

Main width is 256+256*D+63. Static base-element storage, excluding hiding,
commitments and prover/witness temporaries:

| D | Main columns | Main bytes | Preprocessing bytes |
|---|---:|---:|---:|
|12|3391|13,889,536|1,048,576|
|10|2879|11,792,384|1,048,576|
|4|1343|5,500,928|1,048,576|
|1|575|2,355,200|1,048,576|

Maximum source polynomial degree is 2, including trusted selector columns and
constant-row transition constraints. Trusted setup uses base log-height 9,
extended to 10 by the candidate hiding configuration. Verifiers must fix D,
height and the exact preprocessing commitment. Private input/output bindings
must stay inside the parent hiding proof; do not expose these wires or adopt
unreviewed cross-AIR lookup terminals. Ciphertext bytes require additional
binding to the corresponding public ciphertext relation in the full KEM.

`trace(coefficients)` rejects noncanonical inputs, and D=1 additionally rejects
values other than 0/1665, before constructing a bounded matrix. No traces,
vectors, constraint evaluations or proofs ran; build and source inspection
cannot establish runtime proof soundness, interoperability or side-channel
resistance. Full encapsulation composition remains unfinished.
