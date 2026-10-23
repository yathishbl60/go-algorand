# Final atomic-group proof context

This candidate package is compiled only with `algorand_native_privacy`. It binds
proof context to finalized unsigned transaction bodies. Native execution and
proof verification do not call it yet.

Construct ciphertexts first, finalize every transaction body and ordinary group
ID, then call `Bind(genesis, suiteDigest, targetIndex, bodies, limits)`. The
arguments contain full `transactions.Transaction` values, including final group
IDs. Signature/proof witnesses are outside those bodies and outside this digest.
The suite digest must be fixed by the verifier's pinned profile.

`Limits` are trusted profile/consensus configuration. `MaxBodyEstimate` and
`MaxGroupEstimate` bound `Transaction.Msgsize()` estimates before marshaling;
they are not ordinary wire-size ceilings. Estimates include omitted fields, so
directly copying a maximum wire size can reject otherwise valid transactions.
Choose and freeze these budgets with the completed resource specification.
`MaxGroupSize` also comes from trusted consensus rules.

The input transaction values must already satisfy bounded-decoding/field limits.
This package additionally rejects empty/oversized groups, invalid target indices,
mixed genesis hashes, reversed validity windows, inconsistent or incorrect group
IDs, duplicate bodies and exhausted encoding budgets. It does not replace ordinary
WellFormed, signature, fee, state or complete privacy-relation checks.

The digest is SHA3-512 over this unambiguous sequence:

1. ASCII `Algorand/AssetPrivacy/Candidate1/Group/`.
2. Genesis hash (32 bytes), pinned suite digest (64 bytes).
3. Target index and group count (each 4-byte big endian).
4. Each canonical `protocol.Encode(Transaction)` body in order, preceded by its
   8-byte big-endian encoded length.

Full bodies are hashed, independently of the shorter ordinary group-ID digest.
Moving a transaction, changing its validity window/terms/ciphertext, changing the
target index, network or suite changes this candidate proof context.

`Values(digest)` supplies 32 little-endian uint16 scalars. Add these to the full
relation's public-value count and supply the same recomputed scalars to proving
and native verification. Pinned Plonky3 absorbs every public value in its
constraint-phase transcript before challenge sampling. Computing this digest,
passing it only to an AIR constructor, or trusting a sender-supplied digest does
not establish proof binding.

This public-context digest contains no private openings or spending keys. Never
include the proof witness itself: doing so would create a proof/group cycle.
There is no completed native proof adapter or accepted wire suite yet. Builds and
static analysis are performed; tests remain deferred by user instruction.
