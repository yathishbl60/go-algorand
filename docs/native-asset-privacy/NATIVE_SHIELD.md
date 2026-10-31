# Candidate native public-to-private shielding

Updated 2026-10-30. Shield tag3 now has canonical body, native plans, exact bundles
and Go/Rust verifier source routing. This is inactive source integration, not a
usable transfer or runtime demonstration. Every consensus flag remains false,
suite catalogs nil, prices/work capacity zero and approved artifacts absent.
[Viewing-key registration](VIEWING_REGISTRATION.md) now has separate possession source routing.

## Body, authority and current state

NAPT1 Shield uses mode Shielded4, zero private inputs and exactly two outputs.
Debit must be positive, DebitSource equal outer Sender, and credit/receiver zero.
There are no dummy notes, membership paths or spent markers for shielding. The
existing225-byte header and bounded envelope/output codec remain unchanged.
Maximum shield body size is10193 bytes, below the existing10465-byte ceiling.

The evaluator checks actual top-level PQ consent against the current account's
AuthAddr, including the off-Edwards-curve authorizer requirement. Current policy
must allow Shielded and normally Public; [private-only creator inventory](NATIVE_CREATION.md) is the sole debit exception. Policy/suite identity and output disclosure
flags must match authoritative state; a body cannot choose weaker roles or keys.

AssetPrivacyShieldProofPlan derives the exact target body, finalized group and
native domain. It checks output records against current tree/stream allocation,
transaction ID, evaluation round, operation ordinals0/1 and ordered commitments.
Current output plans require ordinary recipient viewing/spending membership,
effective eligibility and every policy-required auditor/controller disclosure.
Neither a main conservation proof nor a valid body grants account authority.

## Exact public vector and complete proof set

The main shield statement has100 uint16 public cells:

| Cell range | Meaning |
| --- | --- |
| 0..32 | Output0 commitment as consecutive little-endian uint16 limbs |
| 32..64 | Output1 commitment, same encoding |
| 64..68 | Public debit, least-significant uint16 limb first |
| 68..100 | Final group-context digest as little-endian uint16 limbs |

The selected relation is staged ShieldGroupAir, not the older isolated ShieldAir.
It uses1024 rows,7641 main columns,8 preprocessing columns and100 public values.
Two512-row note frames reuse one workspace, with shared constant amount/carry/
group registers. It constrains zero private-input amounts, exact uint64 debit
conservation, both output note commitments and their amount bits. Public group
cells bind to constant trace registers. Trusted preprocessing selects both frames.

proofbundle.Shield requires main kind3 then both ordinary-recipient output proofs
and mandatory disclosure proofs. Retained/recovery recipient relations cannot
substitute. Count is3..7, matching Body.ProofChecks and the coarse block budget.
The ordinary NAPW group/suite binding encloses the exact NAPB component set.
No callback or parser-only result can authorize a debit or append.

## Atomic public/private application

The same execution child checks group fees and reserves the complete proof count,
then obtains the current consensus-pinned registry and verifies every actual
main/output proof through the optional Rust backend. Only after success does it
debit Sender's existing opted-in, effectively public/native-unfrozen holding, increase private supply
by that debit, and append both fresh encrypted outputs/commitments. The source must be effectively unfrozen under public and native markers. It consumes
no input nullifiers. Output creation freshness still uses permanent native state.

The [creator storage meter](NATIVE_STORAGE.md) charges the complete mutation set.
A later error discards public/private state and work reservations with the child;
later group failure discards the enclosing group. Existing public accounting,
minimum balance and ordinary transaction/PQ fees remain mandatory. Reserve a
conservative full signed-byte fee before finalizing the group and proving.

## Versioned artifact/request boundary

NAPK version4 has thirteen canonical82-byte entries and a134-byte header:1200 bytes.
Order is1..7,17..22; kind3 pins ShieldGroupAir dimensions above.
The digest domain is Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV4/.
NAPV version4 embeds that registry in its1578-byte trusted header; the request
remains at most15746 bytes. It is internal verifier data, not a peer-selected API.

These formats supersede unapproved versions1..3. Old bytes/pins fail closed, with no
compatibility fallback. There are no approved historical suite references. Future
live suites/artifacts require explicit retained compatible dispatch; this change
supplies no production migration or historical acceptance evidence.

## Evidence and remaining delivery

Go package build/vet and source/physical-limit checks passed for body/planning
components. Rust workspace feature/default checks, Clippy, release FFI and
linked algod builds passed. No runtime verifier checks occurred. No tests,
codec round trips, vectors, traces, setup, proofs, signatures or node runs occurred.

Approved preprocessing/pins, practical
resource feasibility, other modes/arities, complete lifecycle, APIs/SDK/wallet,
restoration/upgrades and six real-node scenarios remain [delivery gates](END_TO_END_STATUS.md).
