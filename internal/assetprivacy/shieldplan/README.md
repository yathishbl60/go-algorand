# Candidate native shield statement plan

This package derives the public main-proof vector for the candidate zero-private-
input shield relation. It requires `algorand_native_privacy`. It checks no proof,
account authorization, public balance, recipient key or output encryption.
Every current consensus version keeps native privacy disabled.

## Construction and ownership

`New(domain,group,debit,outputs)` takes a nonzero network/asset/policy domain, the
nonzero finalized group-context digest, a positive uint64 public debit, and exactly
two distinct nonzero output commitments. There are no dummy private inputs,
nullifiers, membership roots, credit fields or optional output slots.

The plan stores those arguments by value. `Domain`, `Group` and `Debit` return
value snapshots. `Outputs` and `Values` return fresh owned slices, so changing
returned data cannot mutate the plan. A caller must handle a failed constructor
before using the plan; its zero value grants no native permission.

The100 uint16 public cells match the existing Rust ShieldAir statement:

| Cell range | Meaning |
| --- | --- |
| 0..32 | Output0 commitment, consecutive little-endian uint16 limbs |
| 32..64 | Output1 commitment, same digest encoding |
| 64..68 | Public debit, least-significant uint16 limb first |
| 68..100 | Final group-context digest, consecutive little-endian uint16 limbs |

`CheckOutputs(records)` additionally requires two complete validated output
records with matching authoritative domain, operation ordinals0/1 and ordered
commitments. Tree allocation indices are separate from operation ordinals.
It does not establish freshness, final transaction/body identity, policy roles or
current recipient eligibility; those remain ledger checks.

`NewConfidential(...,participants)` adds confidential kind10 with228 public cells:
the same100-cell prefix plus128 byte-valued participant cells. Input slots0/1
must be zero and output slots2/3 nonzero. `Confidential()` and `Participants()`
return value snapshots. Main note constraints bind the disclosed accounts to the
same commitments used by complete output encryption proofs. [Mode details](../../../docs/native-asset-privacy/CONFIDENTIAL_MODE.md).

## Complete proof bundle

`proofbundle.Shield(main,outputPlans)` joins main kind3/public100 to exactly two
ordinary recipient output plans and each output's policy-required disclosure
slots. It compares domain, ordinal and commitments and rejects retained/recovery
recipient substitutions. The normal bundle maximum remains seven slots: one main
plus up to three roles for each of the two outputs; confidential main uses kind10/public228.

The main conservation proof alone cannot authorize a shield. The native caller
must derive every statement from ordered current state, prove complete encryption,
current recipient viewing/spending membership and eligibility, check all mandatory
auditor/controller roles, authenticate the public debit source, enforce policy,
fees/work/storage funding, debit the public holding, increase private supply and
append fresh outputs atomically. Group failure must discard all tentative state.
Neither this plan nor bundle framing supplies a verifier success callback.

## Evidence limits

Tagged Go compilation and vet passed; authored file/function limits passed.
No tests, vectors, codec round trips, traces, constraint evaluations, signatures,
proofs or node execution occurred. This source-level vector construction does not
establish cryptographic soundness, privacy, resource feasibility or native
end-to-end acceptance. See the full [delivery status](../../../docs/native-asset-privacy/END_TO_END_STATUS.md).
