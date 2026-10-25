# Current recipient keys for a recovered private receipt

RecoveryRecipientAir binds complete envelope encryption to current RecipientView
and RecipientSpend membership for the private account in that note. It uses the
current registered owner for a NEW recovered receipt. It omits the ordinary
Frozen=0 predicate because native ASA clawback bypasses receiver freezes.
The native-privacy feature remains disabled by default.

This is not the original-owner remainder relation. RetainedRecipientAir instead
preserves a possibly older owner through the main recovery proof and requires
only current viewing membership. Ordinary outputs use AuthorizedRecipientAir,
which additionally requires current effective-unfrozen eligibility. These are
three distinct expected artifacts; a proof cannot choose the weaker relation.

new(EnvelopeContext) requires recipient role1 and fixes network/asset/policy for
complete EnvelopeAir and current viewing/spending records. Private t/rho and
viewing epoch match encryption and the viewing record. Both registered accounts
equal the note's private account. Its owner matches the current spending record;
viewing and spending epochs remain independent. No holder secret is required to
create an output, and no account/key/epoch/path is added publicly.

RecoveryRecipientInput contains EnvelopeInputs, view_created/view_path and
spend_epoch/spend_created/spend_path. trace(input,current_key_root) checks
canonical note/key admission and both exact current paths before allocation;
it returns RecoveryRecipientWitness{trace,public_values,envelope}. Inputs and
traces remain local. Caller entropy, independent hiding and current native
registration/possession checks remain mandatory.

public_values(wire1629,current_key_root64) returns1100 scalars: complete envelope
vector1068 and current key root32 LEu16 limbs. This is the same length as a
retained remainder vector, but it proves additional current spending membership.
Do not infer the expected artifact from vector length or advertised proof kind.
Generic proofplan.BuildRecoveryRecipient selects Kind4 only for a preauthorized
recovered receipt; BuildRemainder selects Kind3; ordinary Build keeps Kind1/1132.
Native planning derives all roles/current roots, with no proof-supplied overrides.

Stages are envelope0..47882, view membership47882..52011, spending membership
52011..55628, then padding to65536. Main width24869 is1984 constant private roots
plus22885 shared scratch. Roots contain canonical t/rho, private account/owner,
independent epochs and amount bits. Preprocessing width2071 fixes the exact
schedule. Bounded writers avoid full child main matrices.

Raw main storage is13,038,518,272 bytes; preprocessing adds1,085,800,448 bytes,
excluding hiding/LDE, quotient, commitments and other overhead. Source degree
upper bound is9. Actual security/resource budgets and usable proving costs
remain unresolved. This component is not a ready wallet or protocol activation.

Native verification must require current retained CanClawback/current nonzero ASA
clawback, the actual current issuer's ordinary/PQ full-body authorization and the
complete recovery main proof. Every immutable-policy auditor/controller envelope
must prove the same opening/commitment against current assignments. The receiver
freeze exception is limited to authorized recovery and grants no authority itself.
Public/private mode, funding, accepted roots and shared-nullifier checks remain
native obligations. A planner or header comparison verifies no proof.

Native decoder/verifier/transaction ingress and [output nullifier freshness](../../../../../docs/native-asset-privacy/NULLIFIER_FRESHNESS.md)
remain unfinished. No tests, vectors, traces, constraint evaluations, proofs,
benchmarks or node demos have run. Build/Clippy/source limits establish only
compilation/source properties, with no completed cryptographic review.
