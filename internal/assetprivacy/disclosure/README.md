# Native disclosure viewing assignments

Opt-in Go package `disclosure` persists the current auditor/controller viewing
accounts for an immutable network, asset and policy identity. It does not verify
manager signatures or grant issuer powers. Disabled native transaction ingress exists.

`Open(KV,Domain,Flags)` validates the authoritative domain and required roles.
`Current(round)` rejects malformed state, mismatched domains/flags and future
updates. `Set(auditor,currentClawback,expectedEpoch,round)` requires exact epoch
compare-and-swap, increments without overflow and prevents time reversal. The
caller must supply authenticated current ASA parameters, not a proposed target.
KV access must be serialized; Apply must atomically commit or roll back its batch.

Required targets must be nonzero; absent roles must have zero targets. A current
nonzero ASA clawback becomes the controller viewing target. After clawback power
is relinquished, the previous explicit controller viewing target is retained:
immutable mandatory disclosure still applies, without restoring recovery power.
A first controller assignment after clawback has already been removed fails.

The record key is `aprv:d:` followed by assetBE8. Its exact 186-byte encoding is
version1, network32, assetBE8, policy64, flags1, auditor32, controller32, epochBE8,
UpdatedBE8. AuditorRequired=1 and ControllerRequired=2; other flag bits fail.
No implicit assignment, history migration or fallback to a proof-supplied account
is permitted. Assignment rotations apply prospectively to new envelopes.

Native `SetAssetPrivacyDisclosureTargets` checks the consensus flag before I/O,
then requires the supplied nonzero manager to equal the current ASA manager.
It derives policy/domain/flags and current clawback from group-local state. The
caller must still authenticate account/PQ signatures, suite, fees and funding.
Only the internal mutator opens a writable store; no public writable view exists.

`GetAssetPrivacyDisclosureTargets` fails if required assignments are missing.
If the ASA's current nonzero clawback differs from the assigned controller,
reads fail closed. The central native ASA parameter wrapper now synchronizes
required controller changes atomically, retaining the auditor and incrementing
the assignment epoch. Clawback relinquishment retains the prior viewing target;
manager relinquishment in the same configuration does not orphan synchronization.
Changed final authorities require current role keys before atomic-group commit.

AuditorView and ControllerView registration must match these assigned accounts.
A retained controller may rotate its encryption-only view key after relinquishing
clawback. ClawbackAuthority registration still requires the current nonzero ASA
clawback; FreezeAuthority and ManagerAuthority retain their own ASA checks.
No disclosure assignment or viewing-key membership grants spending authority.

[Authorized disclosure constraints](../stark/src/authorized_disclosure_air/README.md)
bind the exact role/account/current key root to a complete encrypted opening and
its public note commitment. Native source routing exists; full lifecycle/runtime
delivery remains unfinished. Builds and source checks provide no runtime/security evidence;
no tests, traces, vectors, constraint evaluations or proofs have run.
