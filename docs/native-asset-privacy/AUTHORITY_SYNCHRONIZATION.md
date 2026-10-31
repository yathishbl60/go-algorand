# Candidate ASA authority synchronization

Updated 2026-10-30. Ordinary ASA configuration now has disabled source integration
for native policy and controller assignments. All consensus flags remain false,
catalogs empty and execution/storage prices zero. No configuration, possession
proof, accepted transaction or node scenario has run.

## Ordered parameter and policy changes

The tagged central `PutAssetParams` wrapper uses actual normalized ASA parameters,
not raw proposed configuration fields. For an existing native policy it creates
one disposable child for the parameters, current policy capabilities and required
controller assignment. A failure discards that child; later atomic-group failure
discards the enclosing group. Missing policy or disabled/default builds retain
the ordinary parameter path. [Retirement and cleanup](RETIREMENT_CLEANUP.md) now guard final asset destruction.

The previous current-capability bits must match the previous nonzero Freeze and
Clawback addresses. New bits derive from the resulting addresses and may only be
relinquished. Modes, disclosures, initial capabilities, suite, creation round and
policy identity stay fixed. ASA non-authority fields remain immutable, and cleared
Manager, Reserve, Freeze or Clawback addresses cannot be restored by this wrapper.
Clearing Manager together with other rights leaves the final policy synchronized.

Normal ASA manager authorization still applies. Privacy administration separately
requires actual current top-level PQ consent; native administrative inner calls
remain rejected. The synchronization helper accepts no signature substitute and
does not authorize a configuration transaction on its own.

## Controller viewing and historical material

When Clawback changes and the immutable policy requires controller disclosure,
the existing required assignment must be present and valid. A new nonzero
Clawback becomes the controller viewing account. A zero Clawback retains the
previous explicit viewing target, preserving mandatory disclosure without
restoring seizure authority. The auditor stays unchanged. The assignment epoch
increments with the ordinary overflow/time checks; unrelated configuration does
not rotate it. Missing or inconsistent required assignments fail closed.

Old key history, current-record leaves and encrypted outputs are retained. They
remain available for historical decryption and do not grant an old principal the
current ASA powers. Recovery checks current nonzero Clawback and retained policy
capability; disclosure checks the current assigned account. Existing public and
native frozen markers are not cleared or rewritten across all holders.

## Atomic-group key readiness

Before committing an atomic group, the validator compares group-start and final
ASA parameters for configured assets with a policy already present at group
start. A changed final nonzero Manager, Freeze or Clawback account needs its
current role-appropriate Falcon registration and current authorization-root
membership. A changed nonzero Clawback also needs ControllerView when disclosure
is mandatory. Old-principal records cannot supply these requirements.

The group may therefore place `acfg` first, followed by the new principals' NAPL
registrations. Each registration separately verifies current PQ account consent,
the correct possession proof, policy/suite and expected epoch before publishing
its key. Final checking occurs before group publication, including accepted-block
evaluation paths. Registration/proof/byte fees and storage funding still apply.

This is a final-state check. An intermediate A-to-B-to-A configuration does not
require B's registration when the committed authority is unchanged. Every
intermediate administrative action still follows ordered current authority and
PQ authorization. Unchanged roles and new-policy adoption/bootstrap do not gain
new registration requirements from this check. Reserve has no separate native
administrative registration role.

## Verification scope and remaining delivery

Builds, static analysis and source review provide the current evidence; this
document describes candidate source semantics. No tests, runtime proofs, key or
signature generation, replay, catchup or restoration scenarios have run. The
candidate remains unreviewed and disabled. [Native creation](NATIVE_CREATION.md) has source routing; private-only final reconciliation,
cleanup/refunds, other modes, APIs, SDK/wallet, resource calibration and all six
real-node scenarios remain in [the delivery checklist](END_TO_END_STATUS.md).
