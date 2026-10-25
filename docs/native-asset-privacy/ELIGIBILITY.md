# Current account eligibility and freeze snapshots

Updated 2026-10-25. Native snapshot publication and private ordinary-recipient
eligibility constraints exist behind disabled feature flags. Issuer
control/recovery relations and native transaction verification remain
unfinished. An eligible account snapshot is not account consent or issuer power.

## Native source and atomic publication

Effective Frozen is public holding.Frozen OR marker.Frozen when a holding exists;
without a holding it is the explicit native marker, or immutable ASA DefaultFrozen
when no marker exists. A marker unfreeze cannot bypass an existing public freeze.
Removing freeze authority preserves every existing status. Ordinary ASA
reconfiguration cannot change DefaultFrozen, so it requires no global refresh.

[eligibility.Store](../../internal/assetprivacy/eligibility/README.md) maintains
one stable tree index per explicitly initialized account. Its canonical154-byte
record contains version1, network32, assetBE8, immutable policy64, account32,
revisionBE8, UpdatedBE8 and Frozen1. The leaf hashes the exact45-byte candidate
Eligibility domain followed by that record. Revision advances only on a changed
effective status; it is independent of marker/key epochs. An unchanged reconcile
still verifies current record/index/tree consistency before returning without writes.

Native publication occurs inside the same outer child COW as each source mutation:

- Recipient viewing/spending registration initializes or reconciles the snapshot
  with key history, current pointers and the current-key authorization root.
- Native SetAssetPrivacyFrozen updates marker history/current pointer and then
  derives effective status from the still-current public holding/default state.
- Public holding writes synchronize markers, put/delete the holding, and then
  reconcile effective status, including unchanged-Frozen balance writes.

Any error discards the complete source and snapshot change. The public
OpenAssetPrivacyControl view now permits Current/Epoch reads only. Writable
control/eligibility stores remain private to native mutation helpers; there is
no externally exposed arbitrary Frozen snapshot or registration endpoint.
Actual signature/PQ/possession/suite/consent checks and storage charges remain
native transaction ingress obligations, not consequences of matching an address.

Existing public holders need no implicit scan at adoption: required recipient key
registration initializes the account before private output membership is possible.
Missing eligibility membership never means eligible. Old experimental records
are not implicitly migrated. Snapshot/index state has no supported deletion API.
A wholly missing account mapping is indistinguishable from an uninitialized one;
this is not an authenticated nonmembership proof or a tree-wide corruption scan.
Normal native execution must preserve every initialized mapping and stable index.

Closeout preserves public.Frozen OR marker.Frozen, including explicit false for
default-frozen assets. ASA destruction deallocates global parameters before its
last holding deletion; that hook uses the supplied policy declaration and already
preserved explicit marker instead of querying unavailable ASA parameters. It
never interprets missing parameters as an unfrozen default. Privacy-state cleanup
and restart/replay/catchpoint behavior still require complete implementation/evidence.

## Current roots and private account binding

AssetPrivacyEligibilityRoot returns a typed current root/count from group-local
state. GetAssetPrivacyEligibility checks current membership and native source
consistency. AssetPrivacyEligibilityWitness requires the exact current record.
These are internal local proving APIs; public account-specific witness requests
would themselves reveal routing information and are not approved RPC contracts.

[EligibleRecordAir](../../internal/assetprivacy/stark/src/eligible_record_air/README.md)
constrains the exact canonical private record with Frozen=0, nonzero account and
nonzero revision. [CurrentEligibilityAir](../../internal/assetprivacy/stark/src/current_eligibility_air/README.md)
adds current depth32 membership. Its only public inputs are root64 as32 LEu16
limbs; account, revision, Updated, leaf/index/path remain private. Native root
provenance supplies Updated<=round and actual source status, not a proof's claims.

[AuthorizedRecipientAir](../../internal/assetprivacy/stark/src/authorized_recipient_air/README.md)
wires that same private account directly to the encrypted canonical opening and
current recipient viewing/spending records. The added3617-row stage fits within
65536 rows without increasing main width. Its1132-scalar vector adds current
eligibility root at1100..1132 after the current key root. Native output planning
derives both roots and uses the exact role-specific schema.

An independent root-only proof could select a different account; it cannot be
joined to a note by comparing public roots. Input spending must instead bind
current eligibility directly to its private committed account without publishing
that input commitment. Key rotation must not invalidate the old committed owner
secret. [Ordinary staged input/group composition](SPENDING_COMPOSITION.md)
now supplies that binding. [Recovery constraints](RECOVERY.md) preserve old-owner
remainders without the ordinary freeze predicate; native integration remains.

Frozen=0 applies to ordinary holder transfers/new receipts. Authorized clawback
now has a separate relation preserving the original frozen owner/remainder,
with current controller authentication still mandatory outside the AIR. Do not route that remainder through the ordinary recipient predicate.
Disclosure viewing membership likewise grants no spending or recovery power.

Rebuild/recheck proofs and caches against current roots after relevant preceding
group changes. Historical note roots never authorize stale eligibility or keys.
No tests, vectors, traces, constraint evaluations, proofs, benchmarks or node demos
have run. Builds/vet/Clippy/source checks are the available evidence; practical
resources and cryptographic/composed security remain unresolved.
