# Recipient envelope with current viewing and spending records

`AuthorizedRecipientAir` joins a complete candidate envelope with current-key
membership for both the recipient's viewing key and registered spending owner.
It is a recipient-output component, not complete spending, issuer authorization
or native transaction verification. The `native-privacy` feature remains opt-in.

`new(EnvelopeContext)` requires envelope role1. It fixes the same public network,
asset and policy for the envelope, RecipientView record(key role1), and
RecipientSpend record(key role2). Both records must be members of the identical
current authorization root. The private account is the account in the encrypted
canonical opening. The encrypted viewing key/rho and AAD epoch equal the viewing
record's fields. The note's owner commitment equals the spending record's owner.

Viewing and spending epochs are independent streams. The relation does not
equate them. It binds each record's own nonzero epoch and creation round into its
private canonical leaf. Current-root provenance supplies registration freshness;
the native verifier must obtain that root from current group-local state, never
from a proof or accepted note-root history. Native registration authentication
and key-possession checks remain necessary before records enter that state.

These checks select the current owner for a new recipient output. Prospective
rotation must not invalidate old notes: input spending still proves the secret
for the owner already committed in that note, plus current eligibility/control.
This output relation does not require the sender to know the recipient's secret.

## Interfaces and public statement

`RecipientInput` contains an `EnvelopeInputs`, viewing record Created/path, and
spending epoch/Created/path. Account and owner are derived from the opening;
the viewing epoch comes from the same private envelope input. Callers supply
fresh secret KEM randomness, fresh GCM nonce and current local registry paths.
The trace and all private inputs must remain local to the prover.

`public_values(&envelope1629,current_root64)` returns 1100 field scalars.
Scalars 0..1068 are the exact [EnvelopeAir vector](../envelope_air/README.md),
including the 248-byte public context with no key epoch. Scalars 1068..1100 are
the current key root as 32 little-endian uint16 limbs. No account, routing key,
epoch, leaf digest, path, direction/index or ownership witness is added publicly.

`trace(input,current_root)` performs canonical key/note, candidate admission and
both path/root checks before allocating the large main matrix. It returns
`RecipientWitness { trace, public_values, envelope }`. Host checks construct a
witness; the AIR independently enforces the same field relationships.

`wires(row)` returns exact-width private account, owner, viewing key/rho,
independent epochs and amount bits. `value_bits(row)` returns the committed
uint64 as 64 least-significant-first bits for later conservation composition.
Amounts are copied from the same canonical opening encrypted and committed by
the envelope. Group context, debits/credits and conservation are still separate.

## Fixed layout and limits

| Stage | Row interval, end excluded |
| --- | --- |
| Complete KEM/AAD/HKDF/GCM/note envelope | 0..47882 |
| Viewing record and depth-32 current membership | 47882..52011 |
| Spending record and depth-32 current membership | 52011..55628 |
| Padding with fixed private roots | 55628..65536 |

There are 24,869 main columns:1984 private root registers and22,885 reusable
scratch columns. Every root stays constant across all stages and padding;
epochs/account/owner/rho/amount are Boolean bit fields, with canonical t scalars
bound by both key relations. Trusted preprocessing has 2071 columns. Each child
receives the exact fixed stage flags and public slice; private cross-stage values
are direct in-process wires, with no serialized private lookup terminal.

The conservative source degree bound is nine on the deepest membership path:
SHA3 absorption6, membership hash frame+1, registered-key frame+1, outer frame+1.
The actual quotient/hiding/security budget must be evaluated before proof/profile
approval. No proof execution or symbolic constraint evaluation has run.

The raw main matrix is13,038,518,272 bytes; preprocessing adds1,085,800,448 bytes.
These counts exclude child schedule scratch, allocator, hiding/LDE, quotient,
commitment and all other prover overhead. Bounded writers avoid full child main
matrices, but resource feasibility remains unmeasured and unresolved.

Current freeze/opt-in/eligibility, policy-required auditor/controller disclosures,
confidential-mode participant bindings, complete spend/group relations, native
proof decoding/work limits and verification/execution are unfinished. Viewing
or spending record membership cannot replace these independent obligations.
This component cannot grant a policy auditor assignment or restore issuer powers.

Source reviews, builds, strict Clippy, formatting and physical limits are the
available evidence. No tests, vectors, traces, constraints, proofs, interoperability
runs, node demos or benchmarks have run. No cryptographic review, recipient
anonymity, constant-time behavior or zeroization guarantee is established.
