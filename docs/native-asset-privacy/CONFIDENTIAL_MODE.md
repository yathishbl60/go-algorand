# Candidate confidential transfers and conversions

Updated 2026-10-31. Ordinary transfer, shield and unshield now have confidential
mode source integration, exposing authenticated note participants while keeping
private note amounts in the existing proof/encryption relation. All consensus
flags remain false, catalogs empty and prices zero. No proof or node scenario
has run; privacy, security and practical resources remain unverified.

## Canonical body and visibility

NAPT1 retains its225-byte header and existing input/output records. Mode2 appends
exactly128 bytes after all records: input0 account32, input1 account32, output0
account32 and output1 account32. Actual private-note slots must be nonzero;
absent slots must be zero. Transfer uses all four. Shield has zero input accounts;
unshield has zero output accounts. Mode4 has no trailer and rejects nonzero
in-memory participant fields, preserving its previous bytes.

The overall body ceiling is10593 bytes. Missing/extra trailer bytes, combined or
unknown modes, zero actual participants and nonzero absent slots reject before
owned payload copies. Mode2 recovery operations remain unsupported and reject.
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
For each active output frame it binds the NoteCommitmentAir account bytes137..169
to the declared output account. Byte bits are least-significant-first.

The complete existing output proof set still proves current viewing/spending
membership, eligibility, encryption and mandatory auditor/controller disclosures.
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

Each public vector preserves the complete old prefix, then appends128 individual
byte-valued cells. Group offsets stay200/132/68; conversion, current roots and
commitment offsets stay unchanged. Rows, workspaces and preprocessing schedules
are unchanged. Typed layout kinds and participant-bound relation IDs prevent
equal preprocessing roots from merging different modes or public statements.

Relation arguments are domain104, debitBE8+creditBE8 for transfer or the single
creditBE8/debitBE8 for unshield/shield, followed by the exact128 participant bytes.
Native Rust decoding enforces byte ranges and reconstructs these exact typed
constructors. Existing output kinds17/18 remain mandatory in their exact order;
confidential unshield has no output slots. At most seven proofs are required.

NAPK5 has16 entries/1446 bytes, ordered1..10,17..22. NAPV5 embeds it in a1824-byte
trusted header and has a16248-byte cap. Older unapproved formats reject; no
approved artifact, historical reference or production migration is introduced.

## Native state and delivery limits

The asset must allow Confidential. Every current PQ authorization, policy/suite,
freeze, accepted-root, nullifier, exact group, fee/work/storage and atomic COW
rule remains mandatory. Private-only creator inventory can use confidential
Shield when its declared mode permits it; public credits still require Public.
Retirement rejects these operations before proof work like other value operations.

Confidential recovery and private-only final reconciliation still need their
complete native proof paths. Runtime validation, audits, approved artifacts,
resource calibration, APIs, SDK/wallet and all six demonstrations remain in
[the delivery checklist](END_TO_END_STATUS.md). No tests, vectors, traces,
constraint evaluations, setup, proofs, signatures or node execution have run.
