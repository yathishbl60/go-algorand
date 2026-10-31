# Candidate confidential transfers, conversions and recovery

Updated 2026-10-31. Ordinary transfer, shield, unshield and all four recovery shapes have confidential
mode source integration, exposing authenticated note participants while keeping
private note amounts in the existing proof/encryption relation. All consensus
flags remain false, catalogs empty and prices zero. No proof or node scenario
has run; privacy, security and practical resources remain unverified.

## Canonical body and visibility

NAPT1 retains its225-byte header and existing input/output records. Mode2 appends
exactly128 bytes after all records: input0 account32, input1 account32, output0
account32 and output1 account32. Actual private-note slots must be nonzero;
absent slots must be zero. Transfer uses all four. Shield has zero input accounts;
unshield has zero output accounts. Recovery uses input0, leaves input1 zero and
uses output0/1 only for its actual outputs. Mode4 has no trailer and rejects nonzero
in-memory participant fields, preserving its previous bytes.

The overall body ceiling is10593 bytes. Missing/extra trailer bytes, combined or
unknown modes, zero actual participants and nonzero absent slots reject before
owned payload copies. Unknown recovery shapes reject through exact operation arity.
Public mode uses ordinary ASA transfers, subject to the asset's Public policy bit.

These participants are committed private-note owners, not the outer fee sponsor.
The positive public debit source and positive public credit receiver remain in
their existing public body fields. They need not equal a private-note participant.
Changing mode or any participant changes the finalized signed body/group.

## Complete account proof bindings

Typed confidential constructors fix the participant array from expected native
state. For each active input frame the AIR binds SpendableNoteAir account bits
to the declared input account. Existing note commitment, owner-secret, nullifier,
membership and current eligibility constraints still apply to that same note.
Recovery binds the same account bits from RecoveryNoteAir without requiring the
holder secret. Existing issuer authorization and remainder ownership remain mandatory.
For each active output frame it binds the NoteCommitmentAir account bytes137..169
to the declared output account. Byte bits are least-significant-first.

The complete existing output proof set still proves current viewing/spending
membership, applicable eligibility, encryption and mandatory disclosures for its
exact ordinary/recovered/retained class. Retained remainders preserve the old owner.
Its commitments must equal the main proof's output commitments. Under the
candidate commitment binding assumption, both relations concern the same opening,
including the publicly declared account. No host equality or arbitrary attached
address substitutes for these AIR constraints. Host preflight checks addresses
only to reject invalid witness inputs before allocating the trace.

Input/output amount bits, secret material and membership paths remain private
relation inputs. Public conversion amounts, aggregate supply, timing, counts and
sizes retain their existing visibility. Exposing an old shielded note's account
in confidential mode cannot make its disclosed identity private again. SDK
visibility warnings and complete scanning/proving flows remain outstanding.

## Exact selected layouts and statements

| Kind | Operation | Rows | Main | Preprocessing | Public cells | Party offset |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 8 | Confidential transfer | 16384 | 8506 | 80 | 392 | 264 |
| 9 | Confidential unshield | 16384 | 8506 | 80 | 324 | 196 |
| 10 | Confidential shield | 1024 | 7641 | 8 | 228 | 100 |
| 11 | Partial private recovery | 8192 | 8539 | 60 | 320 | 192 |
| 12 | Full private recovery | 8192 | 8539 | 60 | 288 | 160 |
| 13 | Partial public recovery | 8192 | 8539 | 60 | 292 | 164 |
| 14 | Full public recovery | 8192 | 8539 | 60 | 260 | 132 |

Each public vector preserves the complete old prefix, then appends128 individual
byte-valued cells. Group offsets stay200/132/68; conversion, current roots and
commitment offsets stay unchanged. Rows, workspaces and preprocessing schedules
are unchanged. Typed layout kinds and participant-bound relation IDs prevent
equal preprocessing roots from merging different modes or public statements.

Relation arguments are domain104, debitBE8+creditBE8 for transfer or the single
creditBE8/debitBE8 for unshield/shield, followed by the exact128 participant bytes.
Native Rust decoding enforces byte ranges and reconstructs these exact typed
constructors. Existing output kinds17/18 remain mandatory in their exact order;
confidential unshield has no output slots. Recovery retains its exact recovered/
retained/disclosure output classes. Recovery arguments append participants after
domain104+creditBE8+authority32; private shapes encode credit as zero. At most seven proofs
are required. Recovery group offsets128/96/100/68 stay fixed.

NAPK6 has20 entries/1774 bytes, ordered1..14,17..22. NAPV6 embeds it in a2152-byte
trusted header and has a16576-byte cap. Older unapproved formats reject; no
approved artifact, historical reference or production migration is introduced.

## Native state and delivery limits

The asset must allow Confidential. Every current PQ authorization, policy/suite,
freeze, accepted-root, nullifier, exact group, fee/work/storage and atomic COW
rule remains mandatory. Private-only creator inventory can use confidential
Shield when its declared mode permits it; public credits still require Public.
Retirement rejects these operations before proof work like other value operations.

[Final reconciliation](FINAL_RECONCILIATION.md) has supported-arity source routing; general note-count closure remains. Runtime validation, audits, approved artifacts,
resource calibration, APIs, SDK/wallet and all six demonstrations remain in
[the delivery checklist](END_TO_END_STATUS.md). No tests, vectors, traces,
constraint evaluations, setup, proofs, signatures or node execution have run.
