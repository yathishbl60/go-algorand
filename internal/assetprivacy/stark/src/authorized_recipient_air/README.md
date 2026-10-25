# Recipient envelope with current keys and eligibility

`AuthorizedRecipientAir` joins complete candidate envelope encryption with current
viewing/spending-key membership and effective-unfrozen account membership. It is
an ordinary new-recipient output component, not complete spending, recovery or
native transaction verification. The `native-privacy` feature remains opt-in.

`new(EnvelopeContext)` requires envelope role1. It fixes the same public network,
asset and policy for the envelope, RecipientView record(key role1), RecipientSpend
record(key role2), and current eligibility record. Viewing/spending memberships
use one current key root; eligibility uses a separate typed current root.
All three private accounts equal the account in the encrypted canonical opening.
The envelope key/rho and AAD epoch equal the viewing record's fields. The note's
owner commitment equals the spending record's owner. The eligibility record must
be effectively unfrozen, derived from current native holding/marker/default state.

Viewing/spending epochs and eligibility revision are independent streams. The
relation never equates them. Each record's private canonical leaf commits its own
nonzero epoch/revision and native creation/update round. Current-root provenance
supplies freshness; the verifier must derive both roots from current group-local
state after preceding members. Accepted note-root history never authorizes stale
key or eligibility records. Native registration authentication, possession and
freeze-state projection remain necessary before records enter their roots.

These checks select the current owner for an ordinary new output. Prospective
rotation must not invalidate old notes: input spending still proves the secret
for the owner already committed in that note, plus current eligibility/control.
The sender does not need that recipient's spending secret. Authorized partial
clawback must preserve remainder to its original owner even when frozen; it needs
a separate recovery relation and must not blindly use this ordinary recipient path.

## Interfaces and public statement

`RecipientInput` contains EnvelopeInputs, viewing Created/path, spending epoch/
Created/path, and eligibility_revision/eligibility_updated/eligibility_path. Account
and owner are derived from the same opening; viewing epoch comes from the private
envelope input. Callers supply fresh secret KEM randomness, GCM nonce and current
local paths. Secret inputs and traces must remain local to the prover.

`public_values(&envelope1629,current_key_root64,current_eligibility_root64)` returns
1132 field scalars. Scalars 0..1068 are the exact [EnvelopeAir vector](../envelope_air/README.md),
including the 248-byte public context without key epoch. Scalars 1068..1100 are the
current key root; 1100..1132 are the current eligibility root, each 32 little-endian
uint16 limbs. No account, viewing key, epoch, revision, update round, leaf digest,
path, direction/index or ownership witness is added publicly.

`trace(input,current_key_root,current_eligibility_root)` checks canonical note/key
admission and all three exact record/path/root relationships before allocating
the large main matrix. It returns RecipientWitness{trace,public_values,envelope}.
Host checks construct a witness; AIR constraints independently enforce the field
relationships. Neither proof-advertised roots nor an independent root-only
eligibility proof can substitute: the same private account is wired directly
between the eligibility child and recipient root registers inside this AIR.

`wires(row)` returns exact-width private account, owner, viewing key/rho,
independent epochs and amount bits. `value_bits(row)` returns the committed
uint64 as 64 least-significant-first bits for later conservation composition.
The amount belongs to the same encrypted/committed opening. Group context,
debits/credits and complete conservation/spending remain separate obligations.

## Fixed layout and limits

| Stage | Row interval, end excluded |
| --- | --- |
| Complete KEM/AAD/HKDF/GCM/note envelope | 0..47882 |
| Viewing record and depth-32 current key membership | 47882..52011 |
| Spending record and depth-32 current key membership | 52011..55628 |
| Effective-unfrozen record and current eligibility membership | 55628..59245 |
| Padding with fixed private roots | 59245..65536 |

Main width stays 24869: 1984 private root registers and 22885 reusable scratch.
Every root is constant through all stages/padding; epochs/account/owner/rho/amount
are Boolean bit fields and canonical t scalars bind through both key relations.
Eligibility revision/update stay inside their child frame; no parent copy is
needed. Trusted preprocessing has 2075 columns. Each child receives exact fixed
stage flags/public slices. No serialized private cross-AIR lookup terminal exists.

The conservative source degree bound is 9 on the deepest membership path:
SHA3 absorption 6, membership hash frame+1, registered-key/eligibility frame+1,
outer recipient frame+1. Actual quotient/hiding/security analysis remains pending.
No proof execution or symbolic constraint evaluation has run.

Raw main storage is 13,038,518,272 bytes; preprocessing adds 1,087,897,600 bytes.
These omit child schedule scratch, allocator, hiding/LDE, quotient, commitments
and all other prover overhead. Bounded writers avoid full child main matrices;
practical resource feasibility remains unmeasured and unresolved.

Mandatory auditor/controller disclosures, confidential-mode participant bindings,
complete spend/recovery/group relations, native decoding/work limits and
verification/execution remain unfinished. Current key/eligibility membership
cannot replace those independent obligations or grant issuer recovery power.

Builds, Clippy, formatting and physical limits establish source properties only.
No tests, vectors, traces, constraints, proofs, interoperability runs, node demos
or benchmarks have run. No cryptographic review, recipient anonymity,
constant-time behavior or zeroization guarantee is established.
