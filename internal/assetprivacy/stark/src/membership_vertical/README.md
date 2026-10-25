# Private staged depth-32 membership

`MembershipVerticalAir::new(network,asset,policy)` fixes the same nonzero domain
and SHA3-512 node contract as the Go tree and horizontal membership component:

```text
SHA3-512("Algorand/AssetPrivacy/Candidate1/MerkleNode/" ||
         network32 || assetBE8 || policy64 || level1 || left64 || right64)
```

The message is exactly 277 bytes: 148 domain bytes, one level byte and two
64-byte children. Thirty-two fixed levels run from leaf to root. Each level
uses a complete 97-row SHA3 frame: 96 rows for four Keccak permutations and one
further row after every block flag has cleared. The pinned Keccak round-flag
constraints have no final-row flag condition; row96 starts an unused padding
permutation. Frame transitions stop there, before the next level resets flags.

Only the final root is public, represented by 32 little-endian uint16 limbs.
The leaf, sibling hashes, directions and intermediate digests remain private.
`MembershipPath` is reused from `membership_air`: its u32 index selects one
Boolean direction per level, and its 32 siblings run bottom-up. Direction bits
are not published or routed through private lookup terminals.

## Fixed layout and transitions

The standalone trace has 4096 rows and 4950 main columns:

| Main range | Private content |
| --- | --- |
| 0..4853 | Reused 277-byte SHA3 component |
| 4853..4885 | Leaf limbs, constant through all rows |
| 4885..4917 | Current node state |
| 4917..4949 | Current frame digest |
| 4949 | Boolean direction |

The trusted preprocessing has 36 columns: 32 fixed level one-hots plus active,
first, last and transition flags for the SHA3 frames. The digest and direction
stay constant within each frame. The current node starts at the leaf, equals
the selected left/right hash child, and updates to the constrained digest only
on a frame's last row. Every other transition holds it unchanged. The hash
message fixes the domain and selected level; siblings are arbitrary private
hashes, while the selected current-child limbs reconstruct from Boolean bits.
The root on the outer last row equals all 32 supplied public limbs.

The 32 hash frames occupy rows0..3104, end excluded. Row3104 is required to carry
the final digest into current state. Crate-private `FRAME_ROWS=3105` includes it;
a parent must retain that row and use matching first/last/transition selectors.
Dropping this carry row would omit the final root transition. Standalone rows
3104..4095 retain the resulting root. Exact trusted preprocessing commitments
must come from the verifier's profile, never from a submitted proof.

## Interfaces and resource limits

`trace(leaf,&path)` constructs secret columns, `root(leaf,&path)` computes host
witness data, and `public_values(root)` encodes the verifier's supplied root.
`leaf_limbs(row)` exposes private leaf wires for the enclosing note or key
relation, only for an exact-width row. The crate-private bounded
`write_into(leaf,path,values,(stride,column,first_row))` writes 3105 rows and
returns the host root; `frame_preprocessed_trace()` supplies the same schedule.
Surrounding parent cells are preserved. No complete SHA3 child main matrix is
created; the shorter `Sha3Air.write_frame` copies from one Keccak temporary.

Standalone raw storage is 162,201,600 main bytes plus 1,179,648 preprocessing
bytes. The bounded parent frame represents 122,958,000 main and 894,240
preprocessing bytes. A default 512-row Keccak temporary adds 10,784,768 bytes.
These counts exclude allocator, hiding, LDE, quotient, commitment and other
prover overhead. No proving time or memory measurements have run.
The conservative source degree bound is seven standalone and eight with one
additional enclosing active frame. Actual quotient/opening budgets remain
subject to validation of the complete relation and pinned backend.

This component does not authorize a root, prove a leaf's meaning, enforce an
append index bound or reserve empty leaves. Native state must supply the correct
current key root or accepted note root in its distinct protocol role. The parent
must bind the private leaf to the same canonical note/key record and enforce
spending, current authority, control, conservation, disclosure and group rules.

Source inspection, compilation, formatting and physical size checks are the
available evidence. No tests, vectors, traces, constraint evaluations, proofs,
node runs or benchmarks were executed. This opt-in component does not establish
security review, privacy or native protocol readiness. Default builds expose no
API and no consensus version is enabled.
