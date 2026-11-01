# Candidate public authorization queries

Updated 2026-11-01. New opt-in ledger queries provide public current-key and
effective-freeze records with their membership paths. They expose no private key,
owner secret, note opening or output-selected private epoch. All consensus flags
remain false and suite catalogs empty. No query, proof or node scenario has run.

## Ledger methods

| Method | Public result |
| --- | --- |
| LookupNativeAssetPrivacyAuthorization(round,asset) | Current ASA authorities, default freeze, assigned viewing targets and distinct key/eligibility roots/counts |
| LookupNativeAssetPrivacyKey(round,asset,account,role) | Current registration, optional exact membership path and separate policy/assignment flags |
| LookupNativeAssetPrivacyEligibility(round,asset,account) | Effective frozen state, optional marker and indexed eligibility record/path |

Each method holds trackerMu.RLock for its entire operation. The ASA resource,
policy, assignment, registration/current pointer, control marker, public holding,
root and path are read at the same selected round. Headers choose the feature
gate and approved catalog; existing ledger snapshot rules still apply. Database
errors and unavailable historical state discard the entire response; there is
no latest-round fallback. Returned material is owned, including public-key bytes.
The typed ReadOnly adapter rejects every attempted store Apply.

These are ledger methods, not additional REST routes in this slice. Existing
[REST queries](REST_QUERIES.md) still expose asset state, encrypted outputs and
note witnesses. HTTP/SDK serialization for these new methods remains pending.

## Authorities and assigned views

AuthorizationSnapshot identifies current Manager, Freeze and Clawback ASA account
addresses, DefaultFrozen, the exact current disclosure assignment, and typed
keyauth.Root/eligibility.Root values with their counts. Live policy capability
bits must agree with current nonzero freeze/clawback addresses. Total/manager
must agree with the pinned asset snapshot. Missing required viewing assignments,
future-dated records or a controller assignment conflicting with nonzero current
clawback reject. Removing clawback may retain the old controller viewing target;
that viewing assignment does not restore recovery power.

Root types remain distinct from the note root. A root returned for an older
selected round is historical data, not permission to use it for current native
execution. Current-key and eligibility roots never use the note-root history
acceptance rule. Native evaluation must recompute them after preceding group legs.
These queries do not simulate future group changes or authorize signatures.

## Key registration and membership

Role values remain RecipientView1, RecipientSpend2, AuditorView3, ControllerView4,
FreezeAuthority5, ClawbackAuthority6 and ManagerAuthority7. Unknown roles and zero
accounts reject before tracker reads. RecipientSpend public material is the owner
commitment; viewing roles carry registered encapsulation keys. Current epoch and
creation are public registration metadata, separate from private output choices.

KeySnapshot reports PolicyAllowed and Assigned independently of Record/Witness:

- Recipient view/spend roles are self-managed by the requested account.
- Manager/freeze/clawback roles match the corresponding current ASA principal.
- Auditor/controller views match the authoritative current viewing assignment.
- Assigned is false whenever the immutable/current policy disallows that role.

A former issuer's current registration may remain in the global key tree. Its
membership path still authenticates that stored registration; Assigned=false
prevents interpreting it as the live issuer. No key-record membership substitutes
for current account AuthAddr/PQ consent. Conversely, an assigned principal with
no registration has Record=nil and Witness=nil, even if its account address is
correct. That absence is not a cryptographic nonmembership proof.

For a present registration, its current pointer selects the record and epoch.
The registry path must authenticate that exact record under the selected current
key root/count. Historical key records are not silently substituted. Old viewing
secrets and historical decryption workflows still belong in wallet backup/recovery.

## Effective eligibility

The query rechecks the same native effective-freeze rule used in execution:
with a public holding, Frozen is holding.Frozen OR marker.Frozen; without a
holding, an existing marker takes precedence, otherwise DefaultFrozen applies.
An explicit false marker can therefore preserve an authorized unfreeze after a
public holding disappears. Marker epochs and eligibility revisions are distinct.

A present eligibility record must agree with this recomputed value and have an
exact authenticated path under the returned eligibility root/count. Missing
Record/Witness grants no recipient/spend eligibility, even when the reported
effective Frozen value is false. The query writes no missing record and performs
no registration. Authorized recovery retains only its existing freeze exceptions.

Retired and closed assets reject these queries because cleanup may remove their
key/control/authorization state. Ordinary ASAs without native policy also reject.
Use an available historical round only for archival inspection, without treating
its old roots, authority or eligibility as current execution permission.

## Remaining delivery

HTTP/OpenAPI mapping for these records, SDK/wallet proving, key rotation/backup,
general note-count closure, simulation, validation and all six demos remain in
[the delivery checklist](END_TO_END_STATUS.md). Builds, vet, linking and source
checks establish compilation properties only. Tests, vectors, race/retention
cases, runtime paths, signatures and proofs remain deferred.
