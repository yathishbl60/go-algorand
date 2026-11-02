# Private ownership and canonical note composition

This optional `native-privacy-compact` component combines canonical note bytes,
the existing SHA3-512 spending-key commitment, and private equality lookups in
one AIR. It supports one or two ordinary private inputs. It supplies neither
membership, nullifiers, conservation, current-key authorization nor recovery.
It is not a registered native proof kind or an approved consensus suite.

`OwnedNotesAir::<F>::new(network,asset,policy,group,count,rows)` fixes nonzero
network/asset/policy/group, count1..2 and a power-of-two height at least512*count,
at most65536. Its public program contains57 full-period columns. Every row has
1988 main cells: note11, Keccak1920, owner words32, selected word/inverse2 and
23 public-program relay cells. The row constraints have degree5.

`trace(notes,secrets)` validates all fixed512-byte openings, nonzero32-byte
spending secrets and the exact existing owner hash before private allocation.
For the owner hash, the complete first72 public prefix bytes are permuted once.
The remaining5 prefix bytes, secret32 and ordinary SHA3 padding enter the next
permutation. Capacity and all non-secret input bits remain constrained. No hash,
domain, note encoding or envelope format changes.

The parent maps each child's preprocessing window to constrained main relay
cells. Each relay equals its exact periodic program value. Lookup expressions
therefore resolve main cells, using the pinned lookup generator unchanged.
The four typed interactions share one35-field bus: owner digest send/receive
and indexed owner-word send/receive. Message purpose, note index and word index
are authenticated. Constant owner words bridge the final hash state to the
streamed note's odd-aligned owner bytes. Their holds end at each note boundary.

Pinned same-bus packing puts these interactions into one degree8 fraction plus
one accumulator:20 auxiliary base coordinates under D10. A single public lookup
terminal is constrained to zero. Per-component terminal values are not emitted.
This is a composition mechanism, not a complete anonymity/security argument.

`public_values()` returns64 base cells:32 little-endian uint16 program-digest
limbs and32 group limbs. The canonical digest covers every constructor argument,
field modulus, public input state and periodic cell. Native callers must derive
the expected constructor/group from authoritative state and finalized bodies.
Batch-STARK ignores `periodic_statement`; these verifier-owned public values
are absorbed before lookup challenges and must equal the expected values.

Use `compact::Value32`, `compact::Config` and fresh `compact::prover_config()`
for candidate backend work. The root `Config`/`prover_config()` retain Goldilocks.
The factory fixes192 queries, blowup8, max8-ary FRI, ten salts/codewords and fresh
OS entropy. It selects no native codec, trusted artifact or protocol activation.
Direct backend proofs require verification before publication; trace construction
does not authenticate a complete native transaction or impose a prover RAM bound.

Only512/1024-row composition was manually proved so far. Larger accepted trace
heights require the planned bounded storage/prover adapter before full usage.
Private trace/opening material must stay local. Automated project tests remain
deferred. [Runtime evidence](../../../../../docs/native-asset-privacy/OWNED_NOTES_RESULTS.md)
records actual checks and their limits.
