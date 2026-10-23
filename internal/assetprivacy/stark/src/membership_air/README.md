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

This composition allocates 32 SHA3 traces horizontally. From the pinned Keccak
column layout, the membership matrix alone contains 156,384 columns by 512 rows
(640,548,864 bytes of Goldilocks elements). Trace assembly additionally retains
the 32 source matrices. These are static allocation counts, not measured prover
memory. FFTs, hiding, commitments and ownership composition add further memory.
Production resource limits and a more efficient layout remain required.

The components do not check root acceptance, nullifier freshness, transaction
group binding, transfer conservation, ciphertext correctness or issuer controls.
They have been compiled and statically checked; proofs and tests have not been
executed. Neither this composition nor its security parameters are audited.
