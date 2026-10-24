# Candidate private HMAC-SHA3-512 constraints

`HmacAir` accepts exactly a 64-byte private key and a verifier-fixed data length
from 0 through 1024 bytes. This covers the candidate HKDF; arbitrary key lengths
are unsupported. SHA3-512's HMAC block is 72 bytes, per
[FIPS 202 Table 3](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf).

The inner prefix is `(key || eight zero bytes) XOR 0x36`, followed by data.
The outer prefix uses `0x5c`, followed by all 64 inner digest bytes. Both full
hashes use the SHA3 AIR. Boolean key bits bind both prefixes. Inner digest
limbs bind the exact outer message bits, which are constant across rows.

The outer digest binds 512 Boolean output bits, constant across rows. There are
zero public values: key, data, intermediate digest and output remain private.
The component publishes neither a PRK nor a derived encryption key.

Use `HmacAir::new(data_len)` and `trace(&key64, data)` for witness construction.
Parents connect operands/results through `data_bit_column`, `key_bits(row)`
and `output_bits(row)`. Bytes use least-significant bit first. Invalid byte/bit
indices return None before index arithmetic; row slices require the exact width.

The generator allocates its destination once, then copies/drops one hash at a
time. Both permitted hashes fit 512 rows. The larger SHA3 cap does not increase
this HMAC data limit. These constraints do not establish that operands came from
the required KEM/AAD or that output drives the actual AES key; the parent must
bind those relations. No tests or proofs ran. Build/Clippy checks are static only.

The crate-private bounded `write_into(key, data, values, (stride,column,first))`
writes exactly 512 rows and returns the 64-byte output. It validates length and
checked matrix bounds before mutation. Both SHA3 messages write directly into
the parent; only Keccak scratch and small host messages/digests are retained.
Standalone trace construction delegates to this writer. No runtime equivalence
check, trace, constraint evaluation or proof has run.
