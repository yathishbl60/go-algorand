# Candidate retirement, bounded cleanup and destruction

Updated 2026-10-31. Disabled source integration now retires reconciled assets,
deletes bounded inventory batches and finalizes ordinary ASA destruction with
storage release. All consensus flags remain false, catalogs empty and prices
zero. No retirement, cleanup, proof, refund or node scenario has been executed.

## Begin retirement

NAPL1 Retire6 is exactly150 bytes: the existing header with nonzero asset,
current policy/suite and expected epoch0, with no tail or witness. The current
nonzero ASA manager must give actual top-level PQ consent. The asset must have
zero private supply and its entire fixed total in an existing creator holding.
Retirement is irreversible; an immutable ASA without a manager cannot use it.

The indexed `aprv:x:`+assetBE8 record stores version1, manager32, policy64,
suite64, creationRoundBE8 and cleanupEpochBE8:177 bytes. Epoch starts at1.
The actual policy identity binds network and asset. Future record creation
rounds, malformed records and mismatched current principals fail closed.
The record is charged to the asset creator, including its inventory entry.

Retired assets reject value operations, registrations, assignment/freeze changes,
public transfers and ordinary parameter changes. Central holding/parameter and
native-write guards reinforce this rule. The policy and aggregate supply remain
available while cleanup proceeds. Cleanup and final destruction still require
the stored/current manager; changing an account's AuthAddr does not change that
principal, and each action verifies the account's actual current PQ authorizer
and current network hash, including accepted-block evaluation paths.

## Bounded cleanup

NAPL1 Cleanup7 is152 bytes: the current asset/policy/suite header, the exact
current cleanup epoch, and a record limitBE2 in1..32. Its witness must be empty.
The ledger rechecks authority and complete supply reconciliation. It reads at
most limit+3 inventory entries and chooses at most limit data keys, preserving
policy, supply and retirement records. It accepts no caller-supplied deletion
list. The bounded walk checks key digests, links, presence/length, and full-chain
count/byte/tail totals when it reaches the end.

Every deletion updates neighbors, inventory totals and the creator's obligation.
After a successful nonempty batch, the cleanup epoch increments exactly once.
A stale retry rejects. If only protected records remain, Cleanup rejects with
the ready-for-destruction error; submit ordinary destruction instead.

Each accepted cleanup batch reserves one unit from the existing coarse native
work bucket and pays one work/proof fee factor plus signed-byte/base/PQ fees.
No cryptographic proof is supplied or implied by that unit. The32-record ceiling
is a candidate deterministic bound, not a measured safe deployment budget.
Failure discards records, counter changes, epoch advancement and work reservation
with the execution/group COW. A later failed group leg discards the whole group.

## Final destruction and storage release

Submit ordinary `acfg` destruction after all nonprotected records are removed.
The evaluator rechecks current/stored manager, policy/suite, zero private supply
and full creator holding. A complete bounded inventory walk must contain only
policy, optional empty supply, and retirement records. It removes those records
and their entries, then requires empty inventory before removing its metadata.

All deletion accounting occurs while ASA parameters still identify the creator.
The ordinary destruction then runs in the same disposable child, including its
existing manager/holding/resource checks. A failure rolls everything back.
The underlying destruction guard independently rejects remaining native state;
removing ASA parameters cannot orphan a partially cleaned inventory.

Each deleted footprint releases its minimum-balance requirement, not an Algo
transfer. If the creator counter equals precisely its own one-item/62-byte
footprint after finalization, it is removed too. Otherwise it remains charged
for the creator's other native assets. No refund is sent to private recipients,
managers or third-party funders. Native histories, roots, ciphertext streams and
nullifiers are removed only after retirement blocks all further asset activity.
Historical decryption keys and receipts must remain in wallets or archives.

Ordinary assets without native state preserve their destruction path. The
absence-first authorization lookup also preserves normal zero-holding closeout
after a deleted ASA. Group-end authority readiness skips a finally deleted ASA;
destruction's current authorization and cleanup guards still apply.

## Remaining evidence and private-only exit

These transitions do not create a public withdrawal path for private-only assets.
Their proof-bound final reconciliation atomically with authorized destruction
still needs implementation. Private-only assets already fully held by the
creator with zero private supply can use the retirement path above.

Builds, vet, formatting and source/link checks provide current evidence only.
No tests, transition vectors, runtime cleanup, replay, crash recovery, catchpoints
or node demonstrations have run. Security/resource review, all modes, APIs,
SDK/wallet and the six scenarios remain in [the delivery checklist](END_TO_END_STATUS.md).
