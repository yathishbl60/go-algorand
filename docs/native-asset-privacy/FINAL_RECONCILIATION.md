# Candidate private-only final reconciliation

Updated 2026-10-31. Source integration now allows the currently supported final
proof shapes to reconcile private-only supply and destroy the ASA in one atomic
group. All activation flags remain false, catalogs empty and prices zero. No
proof, destruction, cleanup or real-node scenario has run. General note-count
closure, security/resource review, client support and runtime evidence remain.

## Exact final pair

The final two transactions of the finalized group must be:

1. Native `aprv` Unshield2 (two actual holder inputs, zero outputs), or
   FullPublic7 recovery (one actual input, zero outputs), in an allowed private mode.
2. Ordinary `acfg` destruction of that same ASA, signed by its current manager.

The ASA policy must omit Public. Native credit must be positive, equal its entire
current aggregate private supply and go to the existing creator holding. Debit
is zero. Before credit, creator holding must equal Total minus private supply;
there is no other outstanding public value. Both legs share a nonzero group ID.
The destruction leg has zero AssetParams and no native declaration. It is last;
no transaction can spend the temporary creator credit between these two legs.

Full canonical body/group binding fixes every earlier group transaction, native
mode/accounts, amounts, fees, validity fields and exact destruction body. The
selected existing complete main proof must verify. Holder ownership/current
eligibility or current clawback authorization remains mandatory for its shape.
Confidential mode also binds the declared input accounts. No new proof kind,
public vector, witness format, suite approval or weaker proof path is introduced.

Current PQ authorization is independently checked for both transactions using
their actual group-local authorizers. Holder final unshield retains ordinary
freeze checks; recovery retains only its existing authorized freeze exception.
The manager's destruction signature cannot replace ownership or clawback proof.
A standalone private-only credit, partial amount, other receiver, wrong asset,
missing/changed destruction leg or unauthorized issuer fails.

## Proof-first sealing and actual destruction

Only canonical native transaction planning may mark the credit as final. Ordinary
mode validation still rejects other private-only public credits. Read-only plans
write no state and do not grant execution permission. After complete proof
verification, the execution child credits the creator, reduces private supply,
consumes each nullifier and records a closure plus a retirement marker.
The marker blocks activity until the immediately following destruction call.

That call checks exact destruction transaction ID, same round, initial epoch,
current manager/creator/total, policy/suite and complete zero-private reconciliation.
It removes policy, empty supply and retirement records, while creator attribution
still exists. It invokes the ordinary ASA applier, with its existing manager,
creator holding and resource checks, in that same disposable child.

The lower destruction guard permits retained indexed records only during this
validated call. Its private allowance is cleared on return, child allocation and
pool reset; it is never committed to a parent or encoded into ledger state.
Any failure discards that child, and a failed group leg discards the final credit,
nullifiers, closure, fees/work, inventory changes and ASA destruction together.

## Durable bounded cleanup

Deleting all historical records during destruction would require unbounded work.
Instead, `aprv:z:`+assetBE8 retains a charged313-byte
[closure record](../../internal/assetprivacy/closure/README.md), including creator,
manager, policy/suite, proof group digest, exact destruction ID, Total, round and
epoch1. Its inventory entry and all retained records remain creator-funded.
The actual ASA creator index must be absent before post-destruction cleanup.
Database errors never count as absence. Native value/key/control operations
cannot use a deleted ASA or create new records under its closed namespace.

Use existing NAPL1 Cleanup7, exact policy/suite and expected epoch, with its
1..32 record limit. The stored manager must give current PQ consent. Cleanup
requires policy/supply/retirement records already absent, walks at most limit+1
entries, deletes at most limit data keys and protects the closure. If data remains,
it increments epoch once. A stale epoch or corrupt index rejects atomically.

When only the closure remains, cleanup deletes it last, verifies empty inventory
and removes its metadata using the captured creator. The final empty creator
counter is released only if no other native asset obligations remain. Release
reduces minimum balance; it sends no Algo refund. Each batch pays the existing
coarse work/byte/base/PQ fees. APIs and SDK cleanup construction remain pending.

## Delivery limits

These are the existing two-input holder and one-input recovery shapes. They do
not close an arbitrary population of positive notes: fixed two-output transfers
cannot be assumed to consolidate it. General bounded staging/batching or additional
proof arities still need a complete design and native implementation. Existing
zero-private retirement/cleanup/destruction remains available separately.

Builds, vet, linking and source checks establish compilation properties only.
No tests, vectors, traces, setup, proofs or node execution have run. All six demos,
wallet/API delivery, recovery/upgrade evidence and external review remain in the
[end-to-end checklist](END_TO_END_STATUS.md). This is not a completed release.
