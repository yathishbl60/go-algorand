# Compact private envelope AAD hash

`EnvelopeAadAir` constrains SHA3-512 of exactly
`Context.Bytes()[256] || P[1184] || Ckem[1088]`, a 2528-byte message.
The canonical ML-KEM public-key coefficients, rho and key epoch stay private. Its 512-bit
output is also private. The 1088 ciphertext bytes are public, represented by 544
little-endian uint16 limbs. This is an opt-in component, not native acceptance.

`new(context)` fixes 248 public context bytes, omitting the eight epoch bytes.
The private epoch is inserted as eight big-endian bytes at full-context offset 147.
`epoch_bits` uses byte order with LSB-first bits inside each byte. This
low-level hash component accepts arbitrary context bytes; the enclosing envelope
relation must validate the domain, version, fields and their authoritative
bindings. A transaction must never select a different context or schedule through
its proof. `public_values(Ckem)` encodes the expected public ciphertext.

The fixed trace has 1024 rows, 4846 main columns and 841 trusted preprocessing
columns. The first 864 rows contain 36 complete 24-row Keccak permutations at the
72-byte SHA3-512 rate. Later rows retain private globals and complete coefficient
canonicality checks. The initial capacity is zero; every later block XORs the
new rate bits into the previous output and retains the complete capacity.
The SHA3 suffix is `06` at byte 2528 and `80` at the last rate byte 2591.
The final permutation output at row 863 equals the private 64-byte AAD digest.

| Main range | Private content |
| --- | --- |
| 0..768 | Canonical key coefficients t below 3329 |
| 768..1024 | Rho bits, LSB first within bytes |
| 1024..1536 | AAD digest bits, LSB first within bytes |
| 1536..1600 | Private epoch bits, in big-endian byte order |
| 1600..4233 | Pinned Keccak round columns |
| 4233..4821 | Reusable 588-bit message window |
| 4821..4846 | Selected coefficient canonicality witness |

The t/rho/digest/epoch registers stay constant over every row. Every coefficient has
one trusted canonical-check row; each packed-key window reconstructs complete
12-bit coefficients from those same canonical globals. This fixes overlapping
windows uniquely, including the partial first key block. The trailing key bits
bind to rho, then zero beyond the key. No full key bit vector is repeated.

Key blocks 3..19 reuse 49-coefficient windows aligned to a complete coefficient.
Block 3 also absorbs the final 40 context bytes. Ciphertext starts exactly at
block 20. Blocks 20..34 reconstruct 36 public uint16 limbs each; block 35
reconstructs the final four limbs and has fixed padding. Boolean windows prevent
noncanonical public ciphertext limbs. Trusted block/end/frame selectors fix every
permutation, input binding and boundary. Their preprocessing commitment must come
from the profile, not the submitted proof.

`key_polynomials`, `rho_bits`, `epoch_bits` and `associated_bits` expose private slices only
for an exact-width row. The enclosing relation must equate t/rho to the same
encapsulation key, Ckem to the same KEM result, and the digest to the same HKDF
salt and GCM associated data. This component alone does not establish key
authority, valid encapsulation, note encryption, disclosure or group binding.

`trace(P,epoch,Ckem)` checks key length/canonicality before allocating private columns.
The crate-private `write_into(P,epoch,Ckem,values,(stride,column,first_row))` checks
bounds and key canonicality before mutation, writes only its 1024-row frame and
returns the host digest. It creates one small Keccak matrix at a time and
preserves surrounding parent cells. Host hashing is witness generation only.

Raw main storage is 39,698,432 bytes and preprocessing is 6,889,472 bytes at
eight bytes per field element. One 32-row Keccak temporary adds 674,048 bytes.
These static counts omit allocator, hiding, commitment and prover overhead.
The conservative source degree bound is five standalone and six with one
additional enclosing frame selector. Actual proving budgets remain unmeasured.

Formatting, compilation and source review are the permitted checks. The low-level
component permits epoch zero; the parent must enforce a nonzero authorized epoch.
No vectors,
tests, traces, constraint evaluations, proofs, interoperability runs or benchmarks
have been executed. Security review, side-channel behavior, zeroization and the
complete native protocol remain separate unfinished obligations.
