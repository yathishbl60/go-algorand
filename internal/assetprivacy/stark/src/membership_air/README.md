# Candidate private membership component

`MembershipAir::new(genesis, asset, policy)` fixes a nonzero domain and a 32-level
binary tree. `MembershipPath` contains a private `u32` index and 32 bottom-up
siblings. Index bit 0 selects the leaf's position; bit 31 selects the root's child.
Each node uses the exact candidate Go tree encoding:

```text
SHA3-512("Algorand/AssetPrivacy/Candidate1/MerkleNode/" ||
         genesis32 || assetBE8 || policy64 || level1 || left64 || right64)
```

The leaf is the raw 64-byte note commitment. Domain construction, empty-leaf
reservation and append limits are enforced by the Go tree; the ledger must
authenticate an accepted root for this same domain.

Only the final root is public (32 little-endian uint16 limbs). All path bits,
siblings, intermediate nodes and the leaf remain private trace columns. Each
orientation bit is Boolean and constant across rows. Selected child limbs are
reconstructed from 16 Boolean message bits, so field aliases cannot bypass the
link between levels. Every hash uses the fixed domain and its exact level;
all 32 output digests are constrained by SHA3 and constant across rows.

`MemberNoteAir` additionally constrains the leaf to the actual private note in
`NullifiedNoteAir`, preserving the existing ownership and nullifier bindings.
Its 64 public limbs are the nullifier followed by the root. It exposes private
value bits for later transfer-conservation composition.

The final matrix composes 32 SHA3 hashes horizontally. From the pinned Keccak
column layout, it contains 156,384 columns by 512 rows, occupying 640,548,864
bytes of Goldilocks elements. Assembly allocates this destination once, then
generates, copies and drops one source hash at a time. No 32-element collection
of source matrices is retained. The 512-row hiding budget and constraints are
unchanged.

The previous join simultaneously retained the destination and 32 source matrices:
`640,548,864 + 32 * 19,877,888 = 1,276,641,280` bytes of matrix elements. The new
largest matrix overlap is during source generation, when the destination, one
source SHA3 matrix and its temporary Keccak matrix coexist:
`640,548,864 + 19,877,888 + 10,784,768 = 671,211,520` bytes. This removes
605,429,760 bytes from that static overlap. These counts describe matrix elements
only; small message buffers, allocator overhead, concurrency and process memory
are excluded. No runtime memory benchmark has been performed.

FFTs, hiding, commitments, ownership and transfer composition add further memory.
Production resource limits and a more efficient final layout remain required.

The components do not check root acceptance, nullifier freshness, transaction
group binding, transfer conservation, ciphertext correctness or issuer controls.
They have been compiled and statically checked; proofs and tests have not been
executed. Neither this composition nor its security parameters are audited.
