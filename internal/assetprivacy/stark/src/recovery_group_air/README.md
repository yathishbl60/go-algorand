# Recovery conservation and original-owner remainder

RecoveryGroupAir proves one canonical input's value is split into recovered
value and any retained remainder. It does not authenticate an issuer signature.
Current native policy/clawback authority and ordinary/PQ full-body authorization
are mandatory outside the AIR. The native-privacy feature stays disabled.

new(network,asset,policy,operation,authority32) fixes a nonzero expected current
ASA clawback account. Merely asserting those public bytes proves no possession.
RecoveryNoteAir supplies private canonical input, shared public nullifier and
accepted note membership without requiring the holder secret or Frozen=0.
No private input commitment/account is published for composition.

| Operation | Actual outputs, in order | Public scalars |
| --- | --- | --- |
| PartialPrivate | Recovered receipt, retained remainder | 192 |
| FullPrivate | Recovered receipt | 160 |
| PartialPublic{credit} | Retained remainder | 164 |
| FullPublic{credit} | None | 132 |

Public credit must be positive. Private recovered value and every partial
remainder must be positive, constrained by Boolean bit sums and private inverses.
Absent amount slots are explicitly zero; no placeholder output notes exist.
Exact uint64/carry conservation enforces input=recovered+remainder+public credit.
For a partial operation, the remainder's canonical account AND owner commitment
must equal the input's private fields, even after key rotation or a freeze.

PublicStatement holds InputPublic{nullifier,root}, actual ordered output
commitments and finalized group digest. GroupInput contains RecoveryInput and
actual output openings. trace preflights shape, canonicality, expected nullifier,
accepted root, output commitments, account/owner retention, positive amounts and
checked integer conservation before parent allocation. Traces remain local.

The public vector is nullifier32/root32 LEu16 limbs, each output commitment32
LEu16 limbs, public credit4 LEu16 limbs only for public variants, group32 LEu16,
and expected authority32 u8 scalars. public_values constructs this exact vector;
its whole contents enter the pinned transcript. Authority/public credit equal
the trusted constructor constants. Native code must never trust proof-advertised
operation, authority, schedule, artifact or expected public values.

The input stage occupies0..3714, then each actual output uses512 rows. Active
rows end at4738 for PartialPrivate,4226 for FullPrivate/PartialPublic,3714 for
FullPublic; padding extends to8192. Main width8539 uses549 constant parent roots
and7990 shared input scratch. Roots contain four amount fields256bits, source
account256bits, source owner32limbs, carry3 and positivity inverses2. Only the
first input amount is real; its second amount slot stays zero. The source account
and owner equal the input and each retained remainder through trusted stage masks.

Preprocessing width60 includes48 input schedule columns and12 outer flags.
Bounded child writers avoid separate full child main matrices. Raw main storage
is559,611,904 bytes; preprocessing adds3,932,160 bytes before hiding/LDE, quotient,
commitments, scratch and allocator overhead. Source degree upper bound is9.
Actual quotient/hiding/composed security and practical resources remain unvalidated.

A recovered private receipt uses RecoveryRecipientAir: current viewing/spending
keys, with the ASA clawback exception for receiver freezes. A retained remainder
uses RetainedRecipientAir: current viewing key without replacing the old owner.
Every required auditor/controller disclosure still encrypts that same canonical
opening and matches current native assignment. These independent role proofs join
only on already-public NEW output commitments, under binding/security assumptions.
They must all be verified with the exact expected artifacts and fresh hiding entropy.

Native recovery must check retained current CanClawback, nonzero current ASA
clawback, actual sender/PQ signatures, modes/private-only exit rules, accepted root,
fresh shared marker, output policy, real credits/funding and atomic state changes.
The weaker receipt/remainder relations can never be selected by an ordinary
transaction or a proof's role label. [Native planning](../../../../../docs/native-asset-privacy/RECOVERY.md)
currently supplies statements only; body parsing and deterministic verification
remain unfinished. [Nullifier freshness](../../../../../docs/native-asset-privacy/NULLIFIER_FRESHNESS.md)
implements the V2 marker/global-uniqueness correction; runtime evidence remains.

Old note recovery requires the old decrypted opening or retained viewing keys;
current roots/new keys cannot reconstruct unavailable secrets. Audit access alone
never grants recovery power. No tests, vectors, traces, constraint evaluations,
proofs, benchmarks or node demos have run, and no security approval is established.
