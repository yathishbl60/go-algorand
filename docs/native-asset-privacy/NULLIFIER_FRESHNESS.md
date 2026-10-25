# Unresolved candidate output-nullifier freshness

Updated 2026-10-25. Source review identified a candidate protocol gap. No native
private transaction is accepted and all activation flags remain disabled. This
must be fixed before activation or a claim that every accepted output is spendable.

## Current source behavior

The current holder and recovery marker is SHA3-512 of candidate Nullifier domain,
network, asset, policy and private serial. Canonical notes require a nonzero serial
but do not prove it fresh. Spending/conservation relations match each INPUT marker
to that input's committed serial, and native planning checks that marker unspent.
New output commitments are locally distinct, but their private serials need not be.

A sender can reuse an input's serial in a canonical output with a different amount
or blinding. That output has a different note commitment but the same marker as
the consumed input. It would be unspendable once the input marker is consumed.
Reusing another known previously spent serial has the same issue. Two distinct
output commitments can also share a serial/marker. This is a source inference;
no trace, proof, runtime attack, test or native acceptance has been executed.

Honest local note constructors use fresh randomness, but that is not a consensus
proof against a malicious constructor. Checking input freshness alone, attaching
valid ciphertexts, or requiring registered recipients does not close this gap.
Rejecting only same-operation serial reuse would not establish global freshness.

## Next protocol work

The marker construction and native note-creation rules must jointly guarantee
fresh spendable outputs without publishing a future output marker or linking
private input commitments. Two approaches need a precise implementation decision:

- Bind the marker to its canonical note commitment as well as serial, and enforce
  global per-domain uniqueness of note commitments across every native append path.
  A private hash binding then gives distinct note commitments distinct markers,
  under the hash assumptions. Holder/recovery constructions must remain identical.
- Prove a new output's privately computed marker absent from authenticated current
  spent state, with a complete private nonmembership relation and current-root gate.
  Merely reading the public spent KV namespace does not supply that proof.

The first approach appears smaller with current components, but is not implemented
or frozen by this document. It changes the marker preimage and requires auditing
Go local APIs, all Rust old/new holder/recovery relations, private digest bindings,
native output commitment indexing, atomic rollback, root/version/migration rules,
wallet scanning and every protocol specification/profile artifact. All append
helpers and writable store surfaces must preserve any chosen uniqueness invariant.

No implicit migration of existing experimental state is justified. A missing map
entry alone cannot prove global nonmembership or detect arbitrary out-of-band
corruption. Protocol versioning and trusted state provenance must make the native
invariant explicit. Local duplicate-output checks are not a global uniqueness gate.

Implement the selected rule, preserve same-note holder-versus-recovery conflict,
then add adversarial/runtime evidence when tests are authorized. Update the pinned
profile/statement/wallet contracts consistently; do not enable an old marker suite
with a new invariant or mix proof versions. Complete native verifier/transaction
integration, other cryptographic obligations and the full north-star scope remain
required. Current build/static checks establish neither freshness nor security.
