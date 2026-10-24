# Private K-PKE polynomials to public ciphertext

`MlKemCiphertextAir` constrains all ML-KEM-768 ciphertext compression and
packing from private canonical polynomials u0/u1/u2/v. The public statement
is exactly 544 little-endian u16 limbs encoding 1088 bytes: three 320-byte
10-bit u encodings followed by one 128-byte 4-bit v encoding. This is a
component of K-PKE, not a full KEM proof or a native acceptance path. It is
compiled only when the parent crate enables `native-privacy`.

The 1024 coefficient scalars stay constant across rows. Trusted preprocessing
selects coefficient i in row i for i=0..1023; all later selectors are zero.
One reusable compression gadget checks the selected input and remainder in
0..3328, Boolean compressed bits and a Boolean rollover. The exact equation
is 3329*(encoded+rollover*2^D)+remainder=input*2^D+1664. D=10 applies to the
first 768 coefficients; D=4 applies to the final 256 and all padding rows.
Each coefficient is therefore checked by the constraints, not just host code.

Ciphertext bit position is 10*i for i<768, or 7680+4*(i-768) afterward.
Each row reconstructs two local 16-bit Boolean words and equates them to the
corresponding public limbs selected by the trusted coefficient flags. The
second limb is zero if it would lie beyond the final ciphertext limb. The
compressed bits equal the relevant contiguous bits from those local words,
using the fixed position modulo 16. This handles coefficients crossing limb
boundaries and preserves ordinary byte order with LSB-first packed bits.
All unused local compressed bits are zero. No 8704-column repeated ciphertext
bit vector, private lookup bus or public polynomial values are needed.

Padding selects input zero, local public words zero and compressed bits zero.
The D=4 equation fixes the remainder to 1664. Boolean gap decompositions
prevent field wraparound in canonical checks; every scalar compression
equation is far below Goldilocks. Maximum source degree is 3, including
compression-kind gating. Enclosing frame gates may increase that degree.
Full quotient/hiding and protocol resource budgets remain pending.

The fixed layout has 2048 rows and 1119 main columns:

| Columns | Meaning |
| --- | --- |
| 0..1024 | Private u0/u1/u2/v scalar coefficients |
| 1024..1049 | Selected canonical input |
| 1049..1061 | Twelve local compressed bits, with unused bits zero |
| 1061 | Boolean compression rollover |
| 1062..1087 | Canonical division remainder |
| 1087..1119 | Two local Boolean public-word decompositions |

Preprocessing has 1024 one-hot coefficient columns. Raw main and preprocessed
matrices occupy 18,333,696 and 16,777,216 element bytes respectively, excluding
witness temporaries, hiding expansion, commitments and all proving overhead.
These are static counts, not memory or timing measurements. Trusted setup uses
base log-height 11, extended to 12 by the candidate hiding configuration.
The verifier must fix this exact schedule, dimensions and public limb schema.

`coefficients(row)` returns the private 1024-scalar input slice and rejects a
row of the wrong width. `public_values(ciphertext)` encodes the public limbs.
`trace(u,v)` returns `CiphertextWitness` containing private trace columns and
the public ciphertext/vector; it rejects noncanonical input before arithmetic.
The parent must bind the polynomials to actual constrained K-PKE operations.
Neither the trace nor its private input polynomials should be logged or sent
through node RPC.

The crate-private `write_into(&u,&v,values,(row_width,start_column,first_row))`
checks checked arithmetic, exact row stride and space for the complete frame,
then validates/prepares all inputs before mutation. It overwrites exactly its
1119 columns for 2048 rows; surrounding columns and rows remain untouched.
It returns the ciphertext without retaining a second field trace matrix.
The standalone trace constructor delegates to that same writer.

Reference source: Go 1.26.3 `crypto/internal/fips140/mlkem/field.go`,
`ringCompressAndEncode10` and `ringCompressAndEncode4`, and the existing
`mlkem_field::Compression` relation. Only source/format/size/build checks are
performed here. No tests, trace generation, runtime vectors, constraint
execution, proofs or interoperability checks have validated this component.
