# Expected native recovery proof statements

Opt-in `recoveryplan` derives main proof statements for one actual private input
and four recovery shapes. It grants no account/issuer authority and performs no
cryptographic proof verification, amount checking or state application. There is
no callback, permissive verifier or permission token in this package.

| Kind | Ordered actual private outputs | Public credit | Vector scalars |
| --- | --- | --- | --- |
| PartialPrivate=1 | recovered0, original-owner remainder1 | zero | 192 |
| FullPrivate=2 | recovered0 | zero | 160 |
| PartialPublic=3 | original-owner remainder0 | positive | 164 |
| FullPublic=4 | none | positive | 132 |

`Statement` supplies Kind, Input, Outputs, Credit and finalized Group digest.
Input is `spendplan.Input`: nullifier64, accepted note root64 and Count native
metadata. All nullifier hashes, including zero, are valid encodings. Note root,
group and actual output commitments must be nonzero. Count must be1..2^32;
unknown kinds, wrong arities and duplicate output commitments fail. Count never
enters the AIR vector and these checks cannot authenticate an accepted root.

`Config` supplies the exact native Domain and nonzero current ASA clawback
account Authority. Generic Config is trusted input, not proof of rights or a
signature. Authority is distinct from the controller viewing assignment: retained
viewing access after relinquishment cannot restore recovery power. Native callers
must derive the current policy CanClawback and nonzero current ASA clawback,
match the actual transaction sender and verify the required ordinary/PQ signatures
over every finalized body before accepting recovery. Auditor/view data can permit
mathematical witness construction; it never grants native recovery authorization.

## Exact public layouts

Every digest is32 little-endian uint16 limbs. Credit is four least-significant-
first uint16 limbs; Authority is32 individual u8 scalars. Domain is fixed by the
trusted relation constructor. Intervals below exclude their end.

| Field | PartialPrivate | FullPrivate | PartialPublic | FullPublic |
| --- | --- | --- | --- | --- |
| Input nullifier | 0..32 | 0..32 | 0..32 | 0..32 |
| Input note root | 32..64 | 32..64 | 32..64 | 32..64 |
| First output commitment | 64..96 | 64..96 | 64..96 | absent |
| Second output commitment | 96..128 | absent | absent | absent |
| Credit | absent | absent | 96..100 | 64..68 |
| Group | 128..160 | 96..128 | 100..132 | 68..100 |
| Current clawback account | 160..192 | 128..160 | 132..164 | 100..132 |

`Build` snapshots these exact vectors. Plan exposes Domain, Authority, Kind,
Input, Outputs and Values through copied getters. No mutable caller slices or
proof-advertised public vectors are retained. `RemainderIndex()` returns1 for
PartialPrivate or0 for PartialPublic, with true; full variants return false.
`CheckOutputs` matches exact domain, commitment order, zero-based Record.Output
ordinals and canonical record framing. Record.Index is a tree allocation index.
This structural match does not verify any proof or policy disclosure assignment.

The expected main AIR must prove canonical input membership and its shared
holder/recovery nullifier, positive recovered value, positive remainder for
partial variants, exact conservation and remainder equality to the input's
private account and already-committed owner. No private input commitment,
selected account/path, note opening or owner is added to this public vector.
Build alone proves none of these properties.

## Output roles and current native state

Recovered private output0 uses RecoveryRecipientProof Kind4/1100: current
view/spend registration, with the native clawback exception for receiver freezes.
Only the original-owner remainder at RemainderIndex uses RetainedRecipientProof
Kind3/1100: current viewing membership, with no current spending-owner or Frozen=0
requirement. The main proof preserves that remainder's original account/owner.
Neither class may replace ordinary Kind1/1132. Native evaluation must select the
exact role kind, constructor and trusted artifact; equal vector lengths or
proof-advertised labels cannot choose a weaker relation. The exception requires
current issuer authentication, not merely a matching account or a planned vector.

Every private output, including remainder, still requires all policy auditor and
controller disclosure proofs for the same committed opening and current assigned
viewing accounts. Complete main and output-role proofs join only on those already-
public new output commitments under the canonical hash binding assumption.

Native root/count acceptance, fresh nullifier checks, finalized body/group and
nonce/ciphertext/conversion correspondence, mode permissions, public-credit
recipient/holding/funding checks, work limits and current issuer authorization
remain mandatory. Full public recovery has no output-role proof, but still needs
the authenticated current clawback action and complete main proof. Native private-
only exit/recovery rules must be enforced by the future transaction body handler;
these generic variants imply no permission to create public value in that mode.

Key rotation is prospective. Keep old owner secrets and the decryption material
needed for earlier recipient/controller epochs. Authority handover must retain
required old viewing material; neither roots nor a new account/key magically
decrypt prior notes. Retained viewing data alone cannot revive relinquished power.

Feature/default activation remains off. Builds, vet and source/size checks are
the available evidence. No tests, cryptographic vectors, traces, constraint
evaluations, proofs, benchmarks or node demonstrations have run. Complete native
verification/execution, resource/security evidence and wallet/SDK delivery remain.
