# Fixed-output SHAKE256 constraints

`Shake256Air::new(message_len)` constrains exactly 128 output bytes of SHAKE256
for a verifier-fixed message length through 4096 bytes. It reuses the constrained
Keccak absorption core with rate 136 bytes, capacity 64 bytes and suffix `0x1f`.
The definition follows [FIPS 202 section 6.2](https://doi.org/10.6028/NIST.FIPS.202).
The 128-byte result fits the first 136-byte squeeze block; no extra output
permutation is needed. This API does not implement longer XOF output or SHAKE128.

Message bytes are Boolean bits, constant across rows. The AIR constrains zero
initial capacity, domain padding, every absorb/permutation and the first 128
output bytes. Block selectors must finish before the trace ends. Public values
are 64 little-endian uint16 limbs; `output_values` constructs that exact array.
The parent can route those limbs into private columns, as ML-KEM noise does.

`trace(message)` generates private columns and rejects the wrong message length.
`trace_rows()` reports its generator height. A 33-byte seed/nonce message uses
512 rows. `message_bit_column(byte, bit)` exposes the same message bit columns,
rejecting invalid indices before arithmetic. Bytes use least-significant bits first.

The new internal rate/suffix fields preserve original SHA3-256/512 parameters:
rates 136/72, suffix `0x06`, digest sizes 32/64. Public constructors fix each mode;
proof bytes cannot choose arbitrary sponge parameters. No new dependencies were
needed. Compilation/source review is the current evidence; no tests or proofs ran.
This is not the full ML-KEM matrix sampler or an accepted native privacy proof.
