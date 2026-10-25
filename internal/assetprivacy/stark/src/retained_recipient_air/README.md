# Viewing encryption for an original-owner recovery remainder

`RetainedRecipientAir::new(EnvelopeContext)` proves a complete candidate envelope
for a remainder returned to its original owner during authenticated recovery,
including when that owner is frozen. Constructor role must be recipient1. The
Rust `native-privacy` feature remains opt-in and disabled by default.

The envelope and current RecipientView registered record(key role1) use the same
public network, asset and policy. Their private canonical t/rho and nonzero
viewing epoch agree. The registered private account equals the account in the
canonical opening at bytes137..169. The complete EnvelopeAir binds encryption,
canonical note opening and its exact public output commitment. No recipient
account, routing key, epoch, owner, leaf or private path is published.

This relation intentionally has no current RecipientSpend-owner equality or
effective-unfrozen predicate. A partial recovery must preserve the input note's
original owner/account even after spending-key rotation or a freeze. The main
recovery relation must enforce those exact equalities across its input and
remainder and bind the same already-public NEW output commitment. A commitment
join relies on the candidate binding assumption and complete native verification
of every required proof; no private input commitment is exposed as a join.

This weaker component alone proves neither original-owner retention nor recovery
authority, ownership-secret control, conservation, spend permission or a valid
transfer. Ordinary outputs must use AuthorizedRecipientAir, with current spending
registration and effective eligibility. A separate recovered receipt
uses RecoveryRecipientAir, preserving current owner selection while allowing
the native issuer-clawback exception for receiver freezes. Only the retained remainder of an
authenticated recovery may use this relation; the proof cannot choose it.

## Interfaces and exact public vector

RetainedInput contains EnvelopeInputs, view_created and view_path. Account comes
from the same private canonical opening. `trace(input,current_key_root64)` checks
canonical note/key admission and exact current viewing-record/path/root before
allocating the large matrix. It returns RetainedWitness{trace,public_values,envelope}.
Fresh secret KEM randomness, GCM nonce and independent hiding entropy remain
caller obligations; secret inputs and traces must stay local to the prover.

`public_values(&wire1629,current_key_root64)` returns1100 field scalars. Entries
0..1068 are the complete [EnvelopeAir vector](../envelope_air/README.md), including
the248-byte public context with private key epoch omitted. Entries1068..1100 are
the current key root64 as32 little-endian uint16 limbs. There is no eligibility
root or public account trailer. `wires(row)` exposes exact-width private t/rho,
epoch and account registers only for in-process composition.

The native verifier must obtain the expected current key root from group-local
state after preceding changes. Accepted note-root history cannot authenticate a
stale viewing record. Current-root provenance also supplies native Created-round
and registration checks; key possession, authenticated ingress, suite and funding
must be checked before admitting registrations to that state.

[proofplan.BuildRemainder](../../../proofplan/README.md) selects Kind3 with these
1100 values and every immutable-policy auditor/controller disclosure role.
Ordinary Build remains Kind1/1132 with its stronger recipient relation. Native
verification must use the exact expected relation and trusted artifact, never
proof-advertised kind, role labels, length or common public-vector prefix alone.
Before selecting the weaker remainder kind, native evaluation must establish
current policy CanClawback and current nonzero ASA clawback identity; required
account/PQ signatures and the full recovery proof remain mandatory.

## Fixed layout and resource limits

The envelope occupies rows0..47882, then current viewing-record membership uses
47882..52011, followed by padding to65536. Main width24229 contains1344 private
root registers: canonical t768, rho256 bits, epoch64 bits and account256 bits.
The remaining22885 columns are shared child scratch. Every root stays constant
across all stages/padding; rho/epoch/account are Boolean and t is canonical
through the child relations. No redundant parent opening/owner copy is needed.
Preprocessing width2067 supplies exact fixed child schedules and eight outer
frame flags. Bounded writers avoid separate full child main matrices.

Raw main storage is12,702,973,952 bytes; preprocessing adds1,083,703,296 bytes. These omit
hiding/LDE, quotient, commitments, allocator and all other prover overhead.
The conservative source degree upper bound is9 through nested current-key
membership hash masks. Actual quotient/hiding/security and practical resource
budgets remain unmeasured and unvalidated. No runtime proof evidence exists.

Every required auditor/controller envelope still must commit the same opening
and match current assigned viewing targets. Their viewing keys gain no spending
or issuer recovery rights. Native proof decoding, verification, transaction
execution, recovery signatures, full wallet/SDK support and real-node demos
remain unfinished. Viewing-key rotation cannot revoke already disclosed data.

No tests, vectors, traces, constraint evaluations, proofs, interoperability runs
or benchmarks have run. Builds, Clippy, formatting and size checks establish
source properties only; no cryptographic security review, recipient anonymity,
constant-time behavior or zeroization guarantee is established.
