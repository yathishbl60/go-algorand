# Public note participants with hidden note values

The typed confidential constructors add account disclosure to the existing
ordinary transfer/unshield/shield relations. They preserve the same note ledger,
ownership, membership, current eligibility and exact conservation constraints.
The candidate remains unreviewed, disabled and without approved artifacts.

## Constructor and exact public values

`SpendGroupAir::confidential(network,asset,policy,operation,participants)` accepts
the same `Operation::Transfer(PublicValue)` or `Operation::Unshield{credit}` as
`new`. `ShieldGroupAir::confidential(network,asset,policy,debit,participants)`
uses the same positive debit and zero-private-input/two-output relation as `new`.
Every constructor returns None when account presence does not match the arity.

Participants have type `[[u8;32];4]`, ordered:

| Slot | Meaning |
| --- | --- |
| 0 | Private input0's committed account |
| 1 | Private input1's committed account |
| 2 | Private output0's committed account |
| 3 | Private output1's committed account |

Every real note's account must be nonzero. Absent shield inputs and unshield
outputs must be zero. Repeated real accounts are permitted, including change or
multiple notes belonging to one holder. Public conversion source/receiver and
outer fee-sponsor identity keep their existing native body fields and semantics.

The exact existing vector is retained as a prefix. All128 participant bytes are
then appended as individual public field cells, including canonical zero slots.
Each cell must equal its exact constructor byte, so non-byte alternatives fail.

| Relation | Base cells | Confidential cells | Rows/main/preprocessing |
| --- | ---: | ---: | --- |
| Transfer | 264 | 392 | 16384/8506/80 |
| Unshield | 196 | 324 | 16384/8506/80 |
| Shield | 100 | 228 | 1024/7641/8 |

Conversion, group digest and eligibility-root offsets inside each prefix remain
unchanged. The selected typed relation identity must include the full ordered
128 account bytes together with domain and conversion parameters. These are
public accounts, not private epochs or new encryption routing keys.

## Independent constraints and witness interfaces

For each input, the trusted active-frame flag selects SpendableNoteAir's account
bits. Those same bits already belong to the canonical committed opening and to
the current effective-unfrozen eligibility account. Confidential constraints fix
all256 of them to the declared account, using each byte's least-significant bit
first. Ownership, nullifier and accepted-root bindings remain unchanged.

For each output, the trusted note frame exposes the exact account bytes137..169
through NoteCommitmentAir::account_bits. Confidential constraints fix those bits
to the declared output account. The same output commitment is independently
joined to the complete recipient/encryption and mandated disclosure proofs.
Another opening/account cannot substitute without breaking commitment binding;
the candidate's cryptographic security still requires independent validation.

`trace` first checks supplied private openings against the constructor accounts,
then performs the existing canonical-note/conservation checks before allocating
the parent trace. Those host checks are witness conveniences; acceptance requires
the actual AIR constraints and all required proof slots. No trace was executed.

Public participant disclosure is intentional and cannot be undone by a later
shielded transfer. Note values remain private witnesses, while public debit or
credit amounts remain public conversion data. Wallets must explain that boundary.

No tests, vectors, traces, constraints, preprocessing, setup, proofs, benchmarks
or node operations ran. Static source properties do not prove privacy, soundness,
interoperability or practical resource feasibility. Native policy/authorization,
all proof checks, fees, storage, freshness and atomic application remain required.
