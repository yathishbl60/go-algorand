# Candidate account eligibility state

This disabled native persistence component represents account-specific privacy
freeze/revocation state. It does not implement anonymous eligibility proofs or
accept a freeze transaction. The north star requires current authorization state
at evaluation; recent note commitment roots never authorize an older freeze state.

## Domains and epochs

`Domain` contains network32, assetBE8, immutable policy64 and account32. All
components must be nonzero. Keys start with `aprv:f:` plus this 136-byte domain.
Suffix `c` stores the current pointer; `e` plus epochBE8 stores immutable history.
Values repeat the full domain to reject swaps between accounts/assets/policies.

A pointer is exactly 145 bytes: version 1, domain 136, epochBE8. A record is exactly
154 bytes: version 1, domain 136, epochBE8, UpdatedBE8, Frozen1. Frozen accepts only
0 or 1. Epochs begin at 1 and advance by exactly 1 without overflow. Writes require
the expected current epoch, or 0 when absent, and Updated equal to the evaluator
round. Future/unknown/malformed records, stale expected epochs, missing history
and attempts to overwrite immutable history fail closed. Control records cannot
be deleted or reset to bypass a freeze. Every action advances its epoch, even
if the boolean state stays unchanged.

## Effective frozen state

The raw marker and effective eligibility are separate APIs. Effective state is:

| Account state | Effective frozen |
| --- | --- |
| Existing public holding | Public holding.Frozen OR native marker.Frozen |
| No public holding, native marker exists | Native marker.Frozen |
| Neither public holding nor native marker | ASA DefaultFrozen |

Thus a native marker unfreeze cannot bypass an existing public freeze. For a
private-only holder of a default-frozen ASA, the absence of a marker denies
eligibility; an explicitly authorized native unfreeze can permit private use.
The default does not override an existing public holding's explicit state.
Opting out of a public holding does not discard its independent native marker.
Reads remain possible after freeze rights are relinquished. Explicit native
freeze actions require the current nonzero ASA freeze address and CanFreeze.
Public holding synchronization can preserve existing state after relinquishment.

## Native helpers and caller obligations

`OpenAssetPrivacyControl` binds the domain to the current policy identity and
requires the disabled consensus flag. `GetAssetPrivacyControl` returns the raw
marker. `AssetPrivacyFrozen` combines it with current public holding/default
state and returns the native marker epoch (zero if absent).

`SetAssetPrivacyFrozen` takes an explicit authority address and expected epoch.
Matching the current ASA freeze address is only an identity comparison. The
caller must verify transaction/PQ authorization, required key possession, current
state binding, fees and storage funding before calling it. This helper adds no
permissive verifier or accepted operation. Auditor/viewing keys never authorize
this action. Native writes atomically update immutable history/current pointer
in a child COW; failure discards it. Group failure must discard its parent COW.

Public holding writes/deletions now synchronize control state in a child COW.
Actual Frozen transitions advance the epoch and synchronize the native marker;
an ordinary unfreeze transition clears the corresponding native freeze. Balance
writes retain independent markers, and opt-ins never clear an existing freeze.
Deletion preserves public.Frozen OR marker.Frozen, including explicit false when
needed to preserve eligibility for a default-frozen asset. This closes the frozen
close-out bypass. Disabled paths perform no extra I/O. Idempotent public freeze
actions do not clear independent native markers; native actions use Set above.
Ordinary freeze/configuration ingress still requires PQ authorization before
activation. Private note eligibility also requires an authenticated
eligibility/revocation tree and a proof binding the hidden holder to its current
account/epoch. Account markers alone do not preserve anonymous eligibility, and
AssetPrivacyFrozen is a state accessor rather than proof verification. A caller
must invalidate/rebuild a proof after relevant control changes.

No tests or proofs were added or executed, as requested. Build/vet only establish
static properties. The consensus flag remains false for all versions.
