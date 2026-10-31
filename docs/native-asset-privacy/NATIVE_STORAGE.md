# Creator-funded native storage accounting

Updated 2026-10-29. Native KV mutations now update creator obligations in the same
COW as their data changes. This is inactive source integration: no consensus
version enables privacy or supplies nonzero storage prices. No runtime funding,
withdrawal, closeout, replay or node scenario has been executed.

## Attribution and canonical record

The [storage package](../../internal/assetprivacy/storage/README.md) recognizes
policy, supply, marker, disclosure, output, tree, permanent note-creation,
key/control history and wrapped authorization/eligibility namespaces. Canonical
key shapes and nonzero asset IDs are checked. Unknown/malformed `aprv:` keys fail;
unrelated KV/AVM box accounting retains its existing path. The private meter's
counter namespace cannot be mutated through ordinary native KV writes.

Every recognized asset entry is charged to its authoritative ASA creator, not
the fee sponsor, recipient, manager or caller-supplied address. Zero, FeeSink,
RewardsPool and StateProofSender cannot become storage payers. One account counter
aggregates that creator's obligations across all its assets.

Its key is ASCII `aprv:storage:` plus creator32:45 bytes. Its value is version1,
Itemsu64BE and Bytesu64BE:17 bytes. A first counter therefore starts at one item
and 62 bytes for itself. Full key/value lengths are charged, including internal
tree nodes, history, stream metadata and markers. These are logical KV units,
not measured database/index/filesystem heap or disk overhead.

The counter remains charged after the last other entry is removed. It cannot be
removed implicitly through account closeout or ASA destruction. Destruction
rejects any existing native supply or policy record, even with zero private
supply/full creator holding, preserving asset parameters for storage attribution.
Authenticated cleanup, refunds and migration remain separate protocol work.

## Central mutation and checked prices

roundCowState.kvPut/kvDel call chargeAssetPrivacyStorage before their data delta.
The meter reads the authoritative old value/current creator, replaces the old
footprint with the new one, checks current funding, writes the counter directly
into the same delta, and touches the payer account for later balance checks.
Direct counter writes avoid recursive billing; ordinary callers cannot supply
their own old lengths or obligation totals.

An insertion adds one item and key+value bytes. Overwrite removes the old complete
footprint and adds the new one. Deletion removes only that entry's footprint;
its own account-counter item/62 bytes remain. Absent-to-absent deletion is a no-op.
All count/length additions, subtractions and multiplication are checked. Corrupt
records, impossible counters, unknown namespaces and arithmetic failures reject.

The extra minimum balance is:

`Items*NativeAssetPrivacyFlatMinBalance + Bytes*NativeAssetPrivacyByteMinBalance`

Both prices must be positive; there is no free native-storage fallback. Current
prices are all zero. The required amount is added, without overflow, to the
creator's ordinary minimum balance under current consensus parameters. Funding
checks use reward-adjusted balances and enforce MaximumMinimumBalance when set.
This locks balance; it does not transfer a storage fee or choose approved prices.

Incidental writes from ordinary holding/freeze/key/eligibility hooks use the same
meter, so they cannot create unbilled native state. Full mutation batches must
stay in one child/group COW: later failure discards both records and obligations.
Funding is checked as writes occur. A third party can place an ordinary payment
to the creator in an earlier atomic-group leg; funding placed only after the
native mutation does not satisfy that earlier check. Group failure rolls it back.

## Withdrawals, closeout and AVM

Generic modified-account validation calls checkAssetPrivacyStorageBalance before
zero-account and reserved-address shortcuts. Consequently an account cannot evade
its persistent obligation by becoming empty or closing its Algo balance. The
creator is explicitly touched even when a mutation changes only native KV data.
The AVM-facing minimum-balance getter also includes the current native obligation,
so application-visible withdrawal checks do not quote a smaller native minimum.

Full and basic JSON account responses now add the native obligation to their
existing `min-balance`, using KV and consensus prices at the returned account round.
Unavailable/pruned rounds and corrupt counters fail instead of quoting less.
[Account reporting](ACCOUNT_REPORTING.md) describes format boundaries. SDK funding
estimators, cleanup/refunds and private-recipient reimbursement remain pending.
Replay/catchpoint behavior of the existing native KV path remains unverified.

## Remaining evidence and scope

[Native evaluator execution](NATIVE_EXECUTION.md) couples these charges to proof
checking and atomic public/private application. Policy/key ingress and [creation](NATIVE_CREATION.md) have source routing; private-only final reconciliation, other modes, operator
pricing, normative sibling specifications, complete API/SDK/wallet and real-node delivery remain unfinished.

Only compilation, vet and source checks have been used. No storage vectors,
funding/withdrawal/closeout tests, proofs, setup, runtime provider reads, replay,
benchmarks or node execution have run. Numeric prices, resource/upgrade evidence
and the complete [north-star gates](END_TO_END_STATUS.md) remain outstanding.
