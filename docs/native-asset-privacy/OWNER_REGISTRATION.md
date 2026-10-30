# Candidate recipient spending-key possession

Updated 2026-10-30. NAPL Register now admits RecipientSpend role2 using a separate
zero-knowledge owner-secret relation. This is disabled source integration; all
consensus flags remain false and catalogs empty. No proof or node operation ran.
ML-KEM viewing registration still rejects, so private transfers remain unusable.

## Exact body and current account consent

The NAPL1 header stays150 bytes. Register2 adds role1, account32, keyLengthBE2 and
exact role material. RecipientSpend2 uses owner64, totaling249 bytes. Admin roles
retain their1978-byte Falcon encoding and NAPR possession. Viewing roles1/3/4
remain unsupported. Unknown roles, bad lengths, zero owners and trailing bytes
reject before owned key copying. The body never carries the private secret.

The registered account must equal outer Sender. Actual top-level PQ authorization
uses that account's current AuthAddr and an off-Edwards-curve authorizer. Policy,
suite, final group and expected epoch are checked against ordered native state.
Recipient registration authorizes only the caller's own role; it grants no issuer
power, changes no AuthAddr and moves no public or private value.

## Owner relation and expected proof

OwnerRegistrationAir proves:

`owner = SHA3-512(ASCII SpendingKey domain || genesis32 || secret32)`

The exact domain is `Algorand/AssetPrivacy/Candidate1/SpendingKey/`, matching
`note.Owner`. The109-byte message includes a nonzero private spending secret.
SHA3 constraints enforce Boolean constant message bits and the correct digest.
A bounded secret-bit sum times a private inverse equals1, rejecting zero without
field wraparound. The secret never enters a native body or public proof vector.

The64 public uint16 cells are owner32 then finalized group32, each consecutive
little-endian limbs. Fixed preprocessing provides active/first/last/transition
flags for the512-row hash frame. Group root cells remain constant and equal the
public group. The relation has3540 main columns,4 preprocessing columns and64
public cells; these are static dimensions, not measured memory or proof size.

Native execution derives the expected owner from the exact signed registration
body and recomputes the complete group with statement.Bind. The group commits
network, target position, suite, asset/policy, role/account, owner, expected epoch
and every finalized transaction field. A peer-selected statement cannot replace it.

The exact bundle is one kind21, MainOutput=0xffffffff, role0 slot with no outputs.
Its NAPW frame must match the current suite/group. Go and Rust select the concrete
OwnerRegistrationAir through independently pinned registry metadata. Neither a
transfer proof, Falcon NAPR frame, key membership nor public syntax can substitute.

## Ordered application, fees and formats

Before proof work, the ledger compares current key epoch and validates the account,
policy and role. It reserves one coarse proof-check unit, loads the pinned local
artifact and invokes actual Rust verification. Only then does one child publish
key history/current pointer, authorization root and derived eligibility snapshot,
including creator storage obligations. Later failure discards all tentative state
and the reservation. Missing viewing keys do not grant recipient membership.

Registration pays one proof-fee factor, complete signed-byte fees and ordinary PQ/
transaction fees. Conservative pricing for a nonempty owner witness alone grants
no admission: mandatory group checking validates exact NAPW framing and binding.
Choose a sufficient fee bound before group finalization and proving.

NAPK version3 has12 entries/1118 bytes, appending kind21 after20. NAPV version3
has a1496-byte header and15664-byte cap, preserving the largest seven-slot value
bundle. Unapproved prior formats/pins fail closed; no approved historical reference
has been added. This is no production migration or artifact approval.

## Evidence and remaining work

Go/Rust compilation and static checks establish source properties only. No tests,
vectors, traces, setup/preprocessing, proofs, signatures, runtime verifier calls or
node scenarios have run. Soundness, witness privacy, practical resources, restart
and interoperability remain unverified. See [delivery status](END_TO_END_STATUS.md).

Viewing-key private possession, approved artifacts/pins, other modes/arities,
complete lifecycle, APIs, SDK/wallet and all six node scenarios remain required.
