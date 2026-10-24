# Complete fixed-envelope GCM AIR

`GcmVerticalAir` constrains AES-256-GCM for a 32-byte private key, 12-byte public
nonce, 64-byte private AAD, 512-byte private plaintext, 512-byte public ciphertext
and 16-byte public tag. It compiles only with `native-privacy`. It has no KEM,
KDF, note authority, native transaction acceptance or reviewed security claim.

All 34 AES blocks and all 37 GHASH blocks execute in one AIR. AES block zero
encrypts the all-zero block to obtain H. Block one encrypts nonce || BE32(1) to
obtain the tag mask. Blocks two through 33 encrypt nonce || BE32(counter), and
their streams XOR the plaintext. GHASH processes four AAD blocks, 32 ciphertext
blocks and BE64(512) || BE64(4096), in that order. The tag is the complete GHASH
result XOR the AES tag mask; every tag bit is constrained, not trusted from the
witness constructor.

The public schema has exactly 276 field values: indices 0..12 are nonce bytes,
12..268 are 256 little-endian u16 ciphertext limbs, and 268..276 are eight
little-endian u16 tag limbs. Both child relations consume the same ciphertext
slice. Boolean bit reconstructions enforce byte/limb ranges. No key, plaintext,
H, mask or AAD field is added to public values.

The fixed root trace has 8192 rows and 11,728 main columns. Rows 0..512 execute
the complete existing AES schedule; rows 512..8192 execute GHASH, including
4736 active product steps followed by padding. The GHASH frame's first Y is
zero. Its local H and AAD are bound to the constant root wires, reversing bits
within each byte to match GHASH's transmitted MSB-first representation.

| Root columns | Meaning |
|---|---|
|0..1920|Constant expanded AES key, including the private input key|
|1920..6768|AES scratch during AES; GHASH uses 1920..3457 afterward|
|6768..10864|Constant private plaintext bits|
|10864..10960|Constant nonce bits, bound to public nonce during AES|
|10960..11088|Constant H bits, constrained by AES block zero|
|11088..11216|Constant tag-mask bits, constrained by AES block one|
|11216..11728|Constant private AAD bits|

Root key, plaintext, nonce, H, mask and AAD wires are Boolean and remain equal
across every root transition, including the phase boundary. This prevents the
GHASH phase from substituting an unrelated H or mask after AES finishes.
`wires(row)` exposes only private input key, plaintext and AAD slices, in byte
order with LSB-first bits. A parent must bind them directly inside the hiding
proof to its key derivation and note relations.

Trusted preprocessing has 234 columns: 61 AES schedule columns, 165 GHASH
schedule columns, and four flags for each frame (active, first, last, transition).
Child windows receive exact main/public/preprocessing slices. Their row-boundary
selectors come from the fixed frame flags, and every child constraint is
multiplied by its frame's active flag. Scratch can therefore reset at row 512
while root constants stay bound. The proof author cannot select phases,
addresses, counters, bounds or preprocessing commitments. No private lookup bus
or separate proof terminal connects the phases.

Maximum source polynomial degree is 5 after frame gating. This is a source
count, not final quotient/proof accounting. Trusted setup uses base log-height
13, extended to 14 by the candidate hiding configuration. The exact fixed
preprocessing artifact must be trusted by the verifier.

`trace(key, nonce, associated, plaintext)` returns private columns plus the
public nonce/ciphertext/tag vector in `GcmWitness`. `public_values(...)` exposes
the same encoding for supplied ciphertext/tag. Neither method evaluates
constraints or generates a proof. Scoped AES and GHASH writers populate the
root matrix directly; no second full GHASH trace is retained. Shape checks
precede writer slicing and use checked row/column arithmetic.

Raw base-element storage is 768,606,208 main bytes and 15,335,424 preprocessing
bytes. Dedicated constant AAD accounts for 33,554,432 main bytes; it allows simple
private parent bindings on every row. Host preparation still uses expanded-key,
individual AES-block and small schedule temporaries. Counts exclude hiding
expansion, commitments and prover allocations. Practical memory, throughput and
final profile budgets remain unresolved; no measurements ran.

Only builds, strict Clippy, formatting, source review and static size checks
are allowed at this stage. No trace generation, interoperability vectors,
constraint evaluations, proofs or tests have validated this composition.

For parent composition, `FRAME_ROWS=5249` retains all 512 AES rows, all 4736
GHASH product rows and one final carry row. `frame_preprocessed_trace()` clips
the trusted schedule and sets the final GHASH row's last/transition flags to
1/0. This prevents a nested child transition from reading the next parent phase.
Standalone `ROWS=8192` and its preprocessing/API remain unchanged.

The bounded `write_into(key,nonce,AAD,plaintext,values,(stride,column,first))`
writes only this 5249-row frame and returns ciphertext/tag. Checked bounds
precede mutation; direct AES/GHASH writers avoid a full child main matrix.
The shortened frame needs its matching trusted schedule and enclosing frame
flags; it is not interchangeable with the standalone schedule. Every active
GHASH step, final product/tag binding and one carry transition are retained.
This shortening has source-only evidence; no runtime trace or proof has run.
