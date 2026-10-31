# Issuer recovery and frozen-owner remainders

Updated 2026-10-31. Recovery constraints, authoritative planning, native issuer
authorization and atomic application have inactive source integration. No accepted
recovery transaction has been demonstrated. [V2 marker/creation correction](NULLIFIER_FRESHNESS.md) now
has source implementations; runtime proofs and native execution remain gates.

## Four exact shapes and value preservation

| Variant | Actual private outputs | Public scalars |
| --- | --- | --- |
| PartialPrivate | Recovered receipt0, original-owner remainder1 | 192 |
| FullPrivate | Recovered receipt0 | 160 |
| PartialPublic | Original-owner remainder0 | 164 |
| FullPublic | None | 132 |

Every variant consumes one actual canonical input. [RecoveryNoteAir](../../internal/assetprivacy/stark/src/recovery_note_air/README.md)
proves its accepted note membership and same commitment-bound V2 marker as ordinary
holder spending, without requiring the holder secret or Frozen=0. The input
commitment/owner/path stay private; Shielded4 also hides the account. Confidential2
binds the public input0 account to that same input opening. Merely knowing an opening, including
through audit access, does not confer any recovery authority.

[RecoveryGroupAir](../../internal/assetprivacy/stark/src/recovery_group_air/README.md)
constrains exact integer conservation and positive recovered value; every partial
remainder is also positive. Full variants instantiate no remainder. Any remainder
keeps the input's same private account AND owner commitment, even after spending
key rotation or freeze. New recovered private notes use current recipient owners.
Public credits are positive constructor constants; private values remain hidden.

The main vector contains input nullifier32/root32 LEu16 limbs, each actual output
commitment32 LEu16, public credit4 LEu16 only for public variants, full group
context32 LEu16, then expected current clawback account32 u8 scalars. This account
is native-derived statement data, not a proof of key possession or a signature.
All expected operation/amount/artifact choices come from native body parsing.
[Confidential2](CONFIDENTIAL_MODE.md) appends128 account bytes to each vector,
selecting kinds11..14 with320/288/292/260 cells. Actual output identities bind to
their commitment frames; inactive slots are zero. The remainder account must
still equal the input account. Authority and group offsets remain unchanged.

## Three recipient proof classes

Ordinary recipient Kind1/1132 verifies current viewing/spending registrations and
effective-unfrozen membership. It cannot be replaced by either recovery class.
Recovery follows native ASA clawback's exception for source AND receiver freezes:

- New recovered receipt uses [RecoveryRecipientAir](../../internal/assetprivacy/stark/src/recovery_recipient_air/README.md),
  Kind4/1100: current viewing/spending registrations for the opening account/owner,
  with no Frozen=0 predicate.
- Retained remainder uses [RetainedRecipientAir](../../internal/assetprivacy/stark/src/retained_recipient_air/README.md),
  Kind3/1100: current viewing registration, with the old owner/account preserved
  by the complete main proof. It need not equal the new registered spending owner.

Every applicable auditor/controller disclosure remains required under immutable
policy and current assigned viewing targets. All role proofs bind the same
canonical opening to the same already-public NEW output commitment as the main
proof. Input commitments never become public joins. Native artifact selection
must distinguish Kind3 and Kind4 despite their equal public-vector lengths.
A proof's label or common prefix cannot choose the weaker relation.

## Native planning and required authority

[recoveryplan.Build](../../internal/assetprivacy/recoveryplan/README.md) constructs
exact immutable vectors and ordered output/remainder indices. Native
AssetPrivacyRecoveryProofPlan derives immutable domain/suite and requires retained
current CanClawback plus current nonzero ASA clawback. It recomputes full finalized
group bodies/context and requires the target Header.Sender to equal that current
account. There is no caller-chosen native authority override.

Header equality authenticates no signature. Actual ordinary/rekey/PQ authorization
under current credentials, full main/role proof verification and native body-field
matching remain mandatory. An auditor or former controller may construct arithmetic
witnesses, but cannot authorize execution through this read-only planner. Removing
clawback power rejects planning even if old viewing material is still available.

The planner checks accepted root/count, current shared marker freshness, exact
ordered output commitments/ordinals, target transaction ID and current round. It
selects recovery-specific recipients only through the fixed main variant and
RemainderIndex. Ordinary native output planning remains Kind1. Public recovery
requires Public mode plus a private-input mode; private recovery requires a private
mode. A public-credit shape never creates an exit from a private-only policy.
Atomic final-destruction reconciliation is a separate unfinished operation.

Plans perform no state writes or proof checks; current consensus fixes group limits.
Native parsing, exact visibility/participant constraints, credit destination,
funding, fees, coarse work bounds and atomic application have source integration.
Runtime evidence and practical resource calibration remain unfinished.
Recheck after prior group changes; history never restores authority.
FullPublic has no output role plans or recipient-key queries.

RecoveryNote main/pre matrices are261,816,320/1,572,864 bytes; RecoveryGroup uses
559,611,904/3,932,160 bytes. Recipient envelope classes still require many GiB before
prover overhead. Source degree bounds are8 for recovery input and9 for its parent
and recovery receipt classes. Actual security/resource budgets remain unvalidated.
Prospective rotation requires retaining historical decryption/opening material.
Changing an ASA controller address requires a real historical-key/data handover;
new roots/current keys cannot recover unavailable old ciphertexts. Native enforced
handover and complete wallet backup workflows remain unfinished, not inferred.
No tests, vectors, traces, setup, proofs, benchmarks or node scenarios have run. Builds, vet, Clippy, formatting and source/size review establish
only compilation properties. Activation remains disabled.
