# ML-KEM-768 eta=2 noise component

`MlKemNoiseAir::new(nonce)` fixes one nonce byte in the verifier's relation.
It constrains `PRF_2(seed, nonce)` and `SamplePolyCBD_2` for a private 32-byte seed.
Reference: [NIST FIPS 203](https://doi.org/10.6028/NIST.FIPS.203), August 13, 2024,
section 4.1 PRF definition and section 4.2 Algorithm 8. ML-KEM-768 uses eta=2 for
both of its noise parameters.

The exact PRF message is the 32 seed bytes followed by the single nonce byte.
Its 128 output bytes come from SHAKE256, using the existing `Shake256Air` with
message length 33. The SHAKE output fits in its first 136-byte rate block, so no
additional squeeze permutation is required. The SHAKE component verifies the
domain suffix, padding, absorption, Keccak permutation and output extraction.

All 64 SHAKE output limbs are routed into private parent columns. They are bound
to 1,024 Boolean bits in the FIPS BytesToBits order: least-significant bit first
within each byte, bytes in original order. Bits are constant across trace rows;
each little-endian 16-bit limb is reconstructed from those bits. The fixed nonce
is constrained directly in the SHAKE message columns.

For coefficient i, the signed CBD value is
`b[4*i] + b[4*i+1] - b[4*i+2] - b[4*i+3]`. Five Boolean one-hot flags select
exactly one value from -2 through 2. The selected signed value must equal that bit
expression. The canonical residue is separately constrained to 3327, 3328, 0, 1 or 2
using the same flags. This prevents field aliases or treating negative noise as
an unconstrained large Goldilocks value. Nonce, coefficients and PRF output are
not public values of this component.

`seed_bit_column(byte, bit)` exposes private seed columns for later randomness
binding. It rejects byte indices outside 0..32 and bit indices outside 0..8.
`coefficients(row)` returns exactly 256 canonical residue columns and rejects
a row with the wrong width. The enclosing K-PKE relation must bind those columns
to its polynomial/NTT and ciphertext construction and bind the seed to its
derived encryption randomness. Separately constructed noise polynomials must
share the intended seed and use the specified nonce sequence.

`trace(seed)` constructs the SHAKE trace and CBD witness columns. A crate-private
`write_into(seed, values, (stride, column, first_row))` writes exactly the 512-row
component frame into a larger destination after checking bounds and row shape.
It overwrites every component cell and preserves all other rows and columns.
The SHA3 direct writer uses only its bounded Keccak temporary, so it avoids a
second complete SHAKE or noise matrix during enclosing trace construction. The pinned
sha3 crate does not export SHAKE; host witness generation therefore uses the
existing pinned `KeccakF` permutation with the fixed 33-byte SHAKE message,
suffix 0x1f and final rate-byte 0x80. The AIR independently constrains the whole
SHAKE/CBD relation. Host witness generation is not an independent implementation
or a proof-verification result.

The component has zero public values and 5,522 private columns: 2,898 from SHAKE,
64 digest limbs, 1,024 bits, 1,280 flags and 256 coefficients. Its 512-row matrix
occupies 22,618,112 bytes of Goldilocks elements, before source matrices, hiding,
commitments or prover temporaries. CBD auxiliary constraints have degree at most
two; SHAKE retains the existing Keccak constraint degrees and backend settings.

Enabled Clippy and default-disabled compilation pass. Functions have at most 20
lines; files have at most 100. No tests or proofs were executed. This component
does not establish matrix sampling, the complete K-PKE/ML-KEM relation, KDF/GCM
binding, or native privacy execution. Review and runtime validation remain open.
