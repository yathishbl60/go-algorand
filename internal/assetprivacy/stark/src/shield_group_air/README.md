# Staged native shield relation

`ShieldGroupAir` proves zero private inputs and exactly two canonical private
output commitments with a positive public debit and zero public credit. It uses
no dummy membership/input witness. Network, asset, policy and debit are fixed by
the trusted native caller. Output ciphertexts and current recipient/disclosure
membership remain required separate proofs in the complete native bundle.

## Native layout and public values

This is artifact layout kind3, or confidential kind10, in NAPK version5. The schedule has1024 rows,
7641 main columns and8 preprocessing columns, with both next-row openings. Two
512-row NoteCommitmentAir frames reuse a7350-column workspace after291 globals.
The constructor checks that the actual child width equals that fixed width.

| Public uint16 cell range | Content |
| --- | --- |
| 0..32 | Output0 commitment, consecutive little-endian uint16 limbs |
| 32..64 | Output1 commitment, same representation |
| 64..68 | Debit uint64, least-significant limb first |
| 68..100 | Finalized group-context digest, little-endian uint16 limbs |

The root globals contain four64-bit Boolean amount arrays, three carry bits and
32 group limbs. Both input arrays are constrained to zero. Exact integer
conservation uses the same bounded carry/range constraints as other native plans.
Each output amount array equals the value bits inside the corresponding complete
canonical note commitment frame. Globals persist through every row transition.
Debit public limbs equal the constructor debit; group globals equal the same
public vector consumed by the transcript. No witness-selected domain is accepted.

`ShieldGroupAir::confidential(network,asset,policy,debit,participants)` additionally
binds public note participants ordered input0,input1,output0,output1. The two input
addresses must be zero; the two real output addresses must be nonzero. It appends
128 individual byte cells after the unchanged100-cell prefix, for228 public cells.
Rows/main/preprocessing remain1024/7641/8; no extra witness columns are needed.
Each active output frame binds its committed account bytes137..169 to the declared
address, with bytewise LSB-first bits, and public cells equal constructor bytes.
Existing complete recipient/encryption proofs join through the same commitments;
host checks or a public address alone cannot replace those independent proofs.
The public debit source remains the existing native body field, not a dummy input.

Eight trusted preprocessing flags select active/first/last/transition for the
two child frames. Output commitments enter the respective NoteCommitmentAir
public range. Reordered output roles or different preprocessing dimensions cannot
select this relation through a proof. Registry pins and relation IDs are still
independent trusted consensus inputs, not values approved by an artifact itself.

## Host witness and verification boundary

`trace(openings,statement)` validates nonzero/distinct expected commitments,
nonzero group, complete canonical note openings, and uint128 conservation before
allocating its private matrix. It writes each child directly into the parent
workspace and fills globals throughout. It does not allocate two full child main
matrices. `public_values(statement)` returns100 base cells or228 confidential cells.
Confidential trace preflight also matches output opening accounts before allocation;
the AIR enforces the same binding independently. No trace was executed.

Raw main cells total62,595,072 bytes and preprocessing65,536 bytes. Keccak scratch,
allocator, commitments, hiding/LDE/quotient matrices and proving overhead remain
additional. These are source layout counts, not measured resource guarantees.

Typed preparation `Prepared::shield(domain,debit)` constructs this exact AIR;
the native verifier derives debit from cell64 and checks group from cell68. Bundle
verification joins commitments at main offset0 to ordinary recipient proofs and
all required disclosure proofs. Public holder funding/current PQ authorization,
allowed mode, private supply, output freshness/storage and atomic application
remain native ledger obligations. This component grants no transfer permission.

The older [ShieldAir](../shield_air/README.md) remains a standalone component;
it lacks this selected staged/native schedule and cannot replace kind3's key.
No compatibility reference was added for the changed format/relation. Existing
unapproved version1 files are not silently reinterpreted as version2.

## Evidence

Source/format/physical-limit checks are available. No traces, preprocessing,
proofs, constraint evaluations, vectors, tests or native runtime requests have
been executed. Security, interoperability and practical resource feasibility
remain unproven. Every current consensus version leaves privacy disabled.
