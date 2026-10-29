# Candidate native lifecycle ingress

Updated 2026-10-29. NAPL1 adoption, disclosure assignment and native freeze control
now have codec, admission and evaluator routing. This is disabled source integration,
not accepted runtime evidence. Registration, shield, other modes and complete
native delivery remain outstanding in [the plan](README.md). No tests/proofs ran.

## Implemented canonical operations

Transaction type remains `aprv`, with signed body `napb` and signed witness `napw`.
The [lifecycle codec](../../internal/assetprivacy/lifecyclebody/README.md) uses NAPL
version1; existing NAPT1 spend/recovery bytes are unchanged. The fixed150-byte
header is magic/version5, opcode1, assetBE8, policy64, suite64, expectedEpochBE8.
Asset/suite are nonzero. Current-policy operations require nonzero policy identity.
Exact lengths, version, enum/flags, unused zero fields and complete framing are
checked before tail reads. No lifecycle body contains secret keys or proof roots.

| Opcode | Exact body size and tail |
| --- | --- |
| 1 Adopt | 185: modes1, capabilities1, disclosures1, auditor32; policy/epoch zero |
| 2 Register | Reserved; rejected even if otherwise framed |
| 3 Assign | 182: auditor32; controller derived from current ASA state |
| 4 Freeze | 183: targetAccount32, frozen1 restricted to0/1 |

Every accepted operation requires empty `napw`; an unrelated value proof cannot
turn an unsupported operation into an accepted one. Adoption validates policy
syntax and retained Public mode; nonzero auditor is present exactly when required.
Assign/Freeze carry no independent mode/capability/disclosure override.

## Current authority and atomic application

All three operations require configured execution, a suite in the fully validated
current catalog, nonzero sender, final ordinary group binding and actual top-level
PQ verification against current sender/AuthAddr. The authorizer must be off the
Edwards curve. Classical/multisig/LogicSig alternatives and native inner calls fail.
A signature cache hit cannot replace the current ledger authorization check.

Adopt/Assign require sender equal the current nonzero ASA manager. Adoption also
requires absent policy and private-supply records, exact current ASA capabilities,
and no change to existing public holdings, supply or note trees. It derives Created
from inclusion round, writes the policy and creates initial disclosure assignments
at expected epoch0 atomically. A required initial controller must have an existing
nonzero ASA clawback address; no prior controller is invented after relinquishment.

Assign compares the supplied expected epoch with current assignment state and
increments by one without overflow. The current ASA clawback becomes controller;
after relinquishment the prior controller viewing target persists. Auditor presence
must match immutable policy flags. Assignment grants viewing access, not recovery.

Freeze requires the current nonzero ASA freeze account and retained CanFreeze.
Its expected epoch applies to the target's native control record. It writes only
the native marker and reconciles eligibility; it does not rewrite public holdings.
For an existing public holding, effective frozen is publicFrozen OR nativeFrozen.
Without a public holding, a native marker overrides DefaultFrozen; absence uses
the default. Native unfreeze therefore cannot clear a public holding freeze.

One disposable child applies all policy/assignment/control/root changes and
[creator storage obligations](NATIVE_STORAGE.md). It commits only on success;
later group failure discards them. A payment funding the creator can precede the
operation in the same group. [JSON minimum balances](ACCOUNT_REPORTING.md) include
existing obligations; future growth still requires an SDK funding estimate.

These operations perform zero P3 checks, reserve no block proof-check units and
read no registry artifact. They still pay ordinary transaction/PQ fees plus the
full canonical signed-byte surcharge and satisfy configured execution/storage
requirements. Byte and group limits apply before exact fee marshaling.

## Bootstrap and missing key possession

Policy.Identity includes Created. Confirm adoption before deriving its actual
identity for later key registration; predicting an uncertain inclusion round is
invalid. Keys/proofs cannot use an assumed future policy identity. Single-round
bundling would require explicit semantics and measured proving latency.

[Key writes](../../ledger/eval/assetprivacy_keys_write.go) update history, current
pointers, authorization roots and eligibility, but do not authenticate registration.
ViewKeyAir, SpendKeyAir and RegisteredKeyAir prove record/hash membership, not
possession of a newly registered ML-KEM private key or note spending secret.
The [artifact registry](../../internal/assetprivacy/artifactregistry/types.go) and
[bridge](../../internal/assetprivacy/proofbridge/verify.go) have no admission relation.

Registration must bind a separately typed possession proof to genesis, final group,
target index, operation, asset/policy/suite, role/account, exact new key and expected
epoch. Owner registration proves the network-bound secret preimage; ML-KEM needs
valid matching private-key possession; Falcon needs new-key possession plus current
account consent. Syntax, encapsulation, membership or transfer proofs cannot replace
these checks. No success callback or key-acceptance fallback exists here.

## Remaining native delivery

Add possession relations, pins/artifacts, decoder/work/fee bounds and actual verifier
routing before enabling registration/rotation. Require policy-mandated current keys
before private value ingress. Complete native shield, new-ASA/private-only lifecycle,
authority rotation/relinquishment synchronization, cleanup/refunds, APIs, SDK/wallet,
all modes/recovery, restoration/upgrades and six real-node scenarios. Adoption alone
does not establish end-to-end delivery or validate the cryptographic candidate.
