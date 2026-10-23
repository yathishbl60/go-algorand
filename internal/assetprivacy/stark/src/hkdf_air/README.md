# Candidate envelope HKDF constraints

`HkdfAir` implements the fixed candidate HKDF-SHA3-512 profile: 32-byte input
material, 64-byte salt, fixed context info and 32-byte result. It composes the
two private [HMAC components](../hmac_air/README.md).

Extract computes `PRK = HMAC(salt, shared_secret)`. All 512 PRK bits bind the
expand HMAC key. Expansion data is exactly ASCII
`Algorand/AssetPrivacy/Candidate1/AES256GCM` followed by byte `0x01`.
The result is the first 32 bytes of its digest. This is the one-block construction
in [RFC 5869 sections 2.2–2.3](https://www.rfc-editor.org/rfc/rfc5869), matching
the Go envelope's fixed use. Variable output lengths/info/salts are unsupported.

Use `HkdfAir::new()` and `trace(&shared32, &salt64)`. No values are public.
The generator reads extract output only to prepare the next witness; the AIR
independently binds every intermediate bit. Each source trace is released before
generating the next. Both HMAC components use 512 rows.

The parent must connect `shared_bit_column` to ML-KEM shared-secret bytes,
`salt_bits(row)` to the exact associated-data digest and `output_bits(row)`
to the actual AES-256 key. Byte bits are least-significant first. Row slices
require the exact width. Private traces must stay outside transactions/RPC/logs.

This is not an accepted encryption proof. ML-KEM, exact AAD binding, AES/GCM
composition, current key eligibility, mandatory disclosures and native verification
remain. No tests or proofs ran; builds/Clippy establish static properties only.
