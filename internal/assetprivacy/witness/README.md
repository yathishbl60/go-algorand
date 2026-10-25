# Candidate opaque proof envelope

This package is compiled only with `algorand_native_privacy`. It frames opaque
proof bytes and verifies their outer binding and size. It does not parse, prove,
or verify the embedded STARK, and must not authorize native execution.

The canonical byte format is:

| Offset | Length | Contents |
| --- | --- | --- |
| 0 | 4 | ASCII `NAPW` |
| 4 | 1 | Version `1` |
| 5 | 64 | Pinned candidate profile digest |
| 69 | 64 | Recomputed finalized group-statement SHA3-512 digest |
| 133 | 4 | Nonzero proof byte length, unsigned big-endian uint32 |
| 137 | Declared length | Opaque proof bytes |

Every field is mandatory. Unknown magic/version, mismatched bindings, oversized
or zero lengths, truncation, and trailing bytes are rejected. Profile selection
must provide a nonzero expected profile digest. Every statement digest byte
pattern is permitted, including the all-zero hash output.

`Limits.MaxProofBytes` must come from trusted profile or consensus configuration;
never take it from the witness or a submitter's request. Zero bounds and bounds
whose complete envelope cannot fit the platform's `int` are rejected. Header
validation checks the declared length before allocating or copying proof bytes.

`Encode` returns a new envelope and copies the supplied proof. `Decode` validates
the complete input before making an owned proof copy. It never lends input
storage to the caller. `Read` reads a fixed 137-byte header first, validates it,
then allocates the bounded payload and requires EOF immediately afterward. Use
it with a finite transaction frame, not a persistent connection containing
multiple frames. Network read deadlines remain the caller's responsibility.
Do not mutate input buffers concurrently with encoding or decoding.

Errors have the exported `witness.Error` type and support `errors.Is` and
`errors.As`. Stream truncation and I/O errors additionally wrap the underlying
reader error. A malformed frame returns no proof slice.

The group digest binds canonical finalized transaction bodies and their order,
profile selection, and target index through the separate statement package.
Keep this envelope and its embedded proof outside those transaction bodies,
alongside signature/proof witnesses, so the proof does not hash itself. Finish
note encryption first using its pre-encryption context; finalize ciphertexts and
all body fields next; compute the group statement; then prove and frame it.
The verifier recomputes the group digest from the accepted transaction bodies
and compares it with the header; it must not trust the header's advertised value.

An outer byte limit does not bound nested STARK allocation, dimensions, recursion,
verification work, or proving resources. A [per-AIR Rust codec/verifier](../stark/src/verification/README.md)
now exists; native artifact selection, bundling, the Go/Rust bridge and measured
work limits are still required before native transaction acceptance. This
format contains no note plaintext, private spending key, or encryption key.
