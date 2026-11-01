# Candidate recipient spending-key possession

Updated 2026-11-01. NAPL Register admits RecipientSpend role2 through a separate
owner-secret relation. Serial Owner primitive proofs now have bounded direct-P3
evidence; no artifact-bound native transaction was accepted. Consensus flags
remain false and catalogs empty. [Viewing registration](VIEWING_REGISTRATION.md)
uses the separate ML-KEM relation (kind22).

## Exact body and current account consent

The NAPL1 header remains150 bytes. Register2 adds role1, account32, keyLengthBE2
and exact role material. RecipientSpend2 uses owner64, totaling249 bytes. Admin
roles retain their1978-byte Falcon encoding and NAPR possession. Viewing roles
1/3/4 use1369-byte bodies and kind22. Bad lengths, zero owners and trailing bytes
reject before owned key copying. The body never carries the private secret.

Registered account must equal outer Sender. Actual top-level PQ consent uses its
current AuthAddr and an off-Edwards-curve authorizer. Policy, suite, final group
and expected epoch are checked against ordered native state. Registration grants
no issuer power, changes no AuthAddr and moves no public or private value.

## Serial Owner relation and expected proof

OwnerRegistrationAir proves:

`owner = SHA3-512(ASCII SpendingKey domain || genesis32 || secret32)`

The exact domain is `Algorand/AssetPrivacy/Candidate1/SpendingKey/`, matching
`note.Owner`. The109-byte message requires two72-byte-rate sponge blocks with
standard SHA3 padding and24 Keccak rounds per block. Each round uses three
serial stages: Theta, Rho/Pi, Chi/Iota. Initialization, fixed prefix, absorption,
capacity preservation, padding, final digest and finalized group stay constrained.

Every state bit is Boolean. At the second-block absorption transition, XOR of
before/after Boolean state bits derives the256 secret bits. Their bounded sum
times a private inverse equals1, rejecting zero without field wraparound. The
secret never enters a native body or public vector. Ungated five-way C parity
keeps the measured symbolic constraint degree at5, rather than multiplying it
by a stage selector.

The512-row frame has1953 main columns: state1600, C parity320, inverse1 and
group32. Preprocessing has11 columns: four schedule flags and seven Iota constant
bits. The64 public uint16 cells remain owner32 then group32, in consecutive
little-endian limbs; group cells are constant and equal the public group.

Native execution derives owner from the signed registration body and recomputes
the complete group through statement.Bind. The group commits network, target
position, suite, asset/policy, role/account, owner, expected epoch and finalized
transaction fields. A peer-selected statement cannot replace it.

The exact bundle is one kind21, MainOutput=0xffffffff, role0 slot with no outputs.
NAPW must match current suite/group. Go and Rust select OwnerRegistrationAir
through independently pinned metadata. Transfer proofs, Falcon NAPR, membership
or public syntax cannot substitute for this relation and current account consent.

## Ordered application, fees and formats

Before proof work, the ledger checks current key epoch/account/policy/role,
reserves one coarse work unit, loads the pinned local artifact and invokes Rust
verification. Only then does a child publish key history/current pointer,
authorization root and derived eligibility, including creator storage obligations.
Later failure rolls back all tentative state and the consensus reservation.
Missing viewing keys do not grant recipient membership.

Registration pays proof, complete signed-byte and ordinary PQ/transaction fees.
A fee bound grants no admission; finalize fees/group before constructing a proof.
NAPK6 remains20 entries/1774 bytes, including kinds21/22. NAPV6 retains its2152-byte
header/16576-byte cap. Serial Owner changes compiled kind21 dimensions and source
fingerprint; stale layouts/pins fail closed. No approved compatibility reference,
suite artifact or production migration has been added.

## Runtime evidence and remaining work

The first serial Owner direct-P3 run verified in1.719s with168.828MiB observed peak
RSS. The expanded run completed in1.694s with187.578MiB observed peak RSS: valid
constraints passed, seven deliberately corrupted trace cases rejected, and changed
owner/group/proof opening rejected. Both used fresh proofs; symbolic degree was5,
with16 quotient chunks and3,285,504 bytes of FRI input query-cell payload alone.
These are bounded worker observations, not deployment budgets or a security audit.

[Serial layout analysis](SERIAL_OWNER_LAYOUT.md) gives a conditional complete
canonical proof upper bound of4,226,753 bytes. Actual serialized native proof size
has not been established here. Direct pinned-P3 setup/prove/verify does not exercise
Prepared, the canonical native codec, independent registry pins or node admission.
No automated test suite ran and no repository test files were added. See [feasibility](FEASIBILITY_RESULTS.md).

Recipient/encryption feasibility, approved artifacts, complete lifecycle/arities,
SDK/wallet flows, recovery and all six node scenarios remain in
[delivery status](END_TO_END_STATUS.md). Primitive rejection cases do not establish
soundness, privacy, interoperability or native end-to-end acceptance.
