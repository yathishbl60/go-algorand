# Candidate native policy declaration at ASA creation

Updated 2026-10-31. Native declaration and private-only issuer conversion now have
disabled source integration. Every consensus flag remains false, suite catalogs
empty and execution/storage prices zero. No accepted creation, proof or node
scenario has run; the candidate remains unreviewed.

## Creation body and authorization

An ordinary `acfg` with ConfigAsset0 may carry `napb` containing NAPL1 Create5.
The existing ASA parameters remain in their ordinary signed fields; no second
asset-metadata encoding or alternative asset ID is introduced. No `napw` is
permitted. Create is rejected on `aprv`, existing-asset reconfiguration and every
other transaction type. Creation without a declaration retains ordinary behavior.

The exact185-byte body uses the150-byte lifecycle header and35-byte declaration
tail: modes1, capabilities1, disclosures1 and auditor32. Asset, policy and expected
epoch are zero; suite is nonzero and selected from current consensus. Unused
fields must be zero. Declared freeze/clawback capabilities must match the actual
ASA addresses. Auditor presence is exact; required controller disclosure needs a
nonzero initial Clawback. Allowed modes may omit Public at native creation.

The creator needs actual current top-level PQ consent against Sender/AuthAddr,
including the off-Edwards-curve authorizer requirement. Existing ASA field rules,
group binding, signed-byte/base/PQ fees, limits and creator storage funding still
apply. Creation has zero proof-check units and creates no private notes. Native
creation through an inner transaction is rejected. A cache result cannot replace
the current ledger authorization check.

## Atomic state and bootstrap

The ordinary creation allocates the asset ID and creator's initial holding. The
same disposable execution state writes the exact77-byte native policy record and
initial disclosure assignment. The policy uses the actual allocated ID and actual
inclusion round; the transaction cannot choose them. Failure discards both ASA and
native state, including storage obligations. Initial private supply remains zero.

The initial manager may be zero: an issuer can declare an immutable native asset
at birth. This does not relax the rule excluding immutable legacy assets from
later adoption. Existing-asset adoption still requires the current manager and
preserves Public mode and all public holdings.

Confirm creation before deriving policy identity and registering keys. The
identity includes the actual creation round, so assuming a future inclusion round
is insufficient. Register current recipient/spending and mandatory disclosure
keys with their separate possession proofs before private value ingress. Creation
does not manufacture keys or approve artifacts.

## Private-only creator inventory

The creator's ordinary initial allocation remains issuer inventory. When Public
is excluded, only that asset creator may debit this holding through native
Shield3, using the same complete main/output proof set, finalized group and
current required keys. No other public debit is authorized by this exception.
The amount is bounded by the actual holding and remaining public aggregate;
the identical amount increases private supply and becomes two private outputs.

All native public debits and ordinary credits check effective public/native freeze state.
A native freeze cannot be escaped by converting inventory into another account's
notes. Source registration is not required merely to debit a real holding;
recipient outputs still require their current keys and effective eligibility.
Planning rejects frozen, missing or insufficient debit holdings before proof work;
application repeats those checks in the execution child.

Ordinary public transfers and all ordinary public credits/withdrawals remain
forbidden by a private-only policy. This conversion grants no general Public mode
and does not change asset ID or total supply. Final private-only reconciliation
atomically with destruction and authenticated state cleanup remain unfinished.

## Evidence and remaining delivery

Build/static checks and source review establish only compilation and stated source
properties. No tests, vectors, signatures, key generation, traces, setup/proofs,
runtime verification, replay or node scenarios have run. Practical resources,
security, all modes, cleanup, APIs, SDK/wallet and all six real-node demos remain
required by [the delivery checklist](END_TO_END_STATUS.md).
