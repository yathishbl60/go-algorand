# Candidate vertical 37-block GHASH chain

`GhashChainAir` implements the exact chain used by the candidate envelope:
four 16-byte AAD blocks, 32 public ciphertext blocks, and the fixed length block
`BE64(512) || BE64(4096)`. The hash subkey H, all 64 AAD bytes and the result remain
private. This is not a complete AES-GCM, KEM/KDF or accepted native spending proof.

The main trace has 1537 Boolean columns. Bit arrays use transmitted MSB-first
order; the AAD's bits run through its bytes in that same order.

| Columns | Contents |
| --- | --- |
| 0–127 | Constant H |
| 128–639 | Constant 64-byte AAD |
| 640–767 | Constant declared result |
| 768–895 | Chain accumulator Y before the current block |
| 896–1023 | Multiplicand X = Y XOR data |
| 1024–1151 | Current shifted multiplier V |
| 1152–1279 | Selected V bits |
| 1280–1407 | Current product accumulator Z |
| 1408–1535 | Current 16-byte data block |
| 1536 | Selected X bit |

The fixed 8192-row preprocessing matrix has 165 columns: 128 one-hot bit-step
selectors and 37 block selectors. Rows 0 through 4735 perform all 37 products;
remaining rows contain zero selectors. Each product starts with V=H, Z=0 and
X=Y XOR data. First Y is zero. At each product end, Y becomes Z XOR selected V.
At the final end, this product is also bound to the declared result. X/data hold
except at product boundaries. V/Z hold throughout inactive padding. All H/AAD/
result bits are bound across every transition, and all main values are Boolean.

The only public values are 256 consecutive little-endian uint16 limbs of the
512 ciphertext bytes. `public_values` builds this vector. CT data blocks are
reconstructed from their Boolean MSB-first bits and constrained to those limbs,
which also enforces their uint16 ranges. No key, AAD or intermediate product
is a public value. `wires(row)` provides private H/AAD/result references for an
enclosing relation, which must bind them directly to the actual AES/KDF context.

Use pinned `setup_preprocessed(config, air, 13)`, retain the trusted verifier key,
and use the corresponding `prove_with_preprocessed`/`verify_with_preprocessed`
APIs. Under the current hiding PCS, extended log-height is 14. Never accept a
submitter's replacement preprocessing key or shape. There is no private bus or
lookup-terminal disclosure. The final relation still needs a complete schedule
and hiding analysis for all commitments and opening points; this component
alone does not establish complete zero knowledge or the privacy security target.

Maximum degree is four counting transition and preprocessing gates. The running
condition is `active - end`, avoiding an unnecessary product of selectors.
Wire-format decoding, native dispatch and cryptographic acceptance are absent.

At eight bytes per field value, raw main storage is 100,728,832 bytes and raw
preprocessing storage is 10,813,440 bytes. These are static matrix sizes, not
measured proving costs or production resource limits; masks, FFTs, commitments
and enclosing computations require additional memory.

Witness generation uses the same start/advance/end conventions as the AIR.
No tests, runtime constraint checks, proofs or cryptographic verification were
executed. All code remains behind the disabled native-privacy feature.
