# Candidate native lifecycle ingress

Updated 2026-10-30. NAPL1 adoption, key registration, disclosure assignment
and native freeze have codec, admission and evaluator routing. This is disabled
source integration. Other modes and full
native delivery remain outstanding in [the plan](README.md). No tests/proofs ran.

## Canonical operations

Transaction type is `aprv`, signed body `napb`, signed witness `napw`. [NAPL1](../../internal/assetprivacy/lifecyclebody/README.md)
leaves existing NAPT1 value bytes unchanged. The fixed150-byte header is magic/
version5, opcode1, assetBE8, policy64, suite64, expectedEpochBE8. Asset/suite are
nonzero; policy is zero only for adoption. Exact sizes, enum/flags, unused zero
fields and framing are checked before bounded owned copies. No secret is carried.

| Opcode | Exact body size and tail |
| --- | --- |
| 1 Adopt | 185: modes1, capabilities1, disclosures1, auditor32; policy/epoch zero |
| 2 Register | role1, account32, keyLengthBE2; owner64 (249), ML-KEM1184 (1369) or Falcon1793 (1978) |
| 3 Assign | 182: auditor32; controller derived from current ASA state |
| 4 Freeze | 183: targetAccount32, frozen1 restricted to0/1 |

Adopt validates policy syntax/Public mode and auditor presence iff required.
Assign/Freeze carry no independent policy override. Register accepts only
RecipientSpend2, viewing roles1/3/4 or admin roles5..7, with distinct possession proofs.
Registration account must equal outer Sender, and expected epoch must not overflow.

## Current authority and atomic application

Every operation requires configured execution, current catalog suite, final group
binding and actual top-level PQ verification against current sender/AuthAddr. The
authorizer must be off the Edwards curve. Classical/multisig/LogicSig alternatives
and native inner calls fail. A cache hit cannot replace current ledger checks.

Adopt/Assign require the current nonzero ASA manager as sender. Adoption requires
absent policy/private-supply records and exact current ASA capabilities; it preserves
holdings, supply and note trees. Created comes from inclusion round. Policy and
initial targets using expected epoch0 commit atomically. A required initial controller needs an
existing nonzero ASA clawback; a prior controller is not invented after relinquishment.

Assign compares/increments the current assignment epoch without overflow. Current
ASA clawback becomes controller; after relinquishment the prior viewing target
persists. Auditor presence follows immutable flags; viewing grants no recovery.

Freeze needs current nonzero ASA freeze and retained CanFreeze. Its expected epoch
is the target's native control epoch. Only the marker/eligibility change. With a
public holding, effective frozen is publicFrozen OR nativeFrozen; without one,
marker overrides DefaultFrozen and absence uses default. Public freeze still wins.

One disposable child commits policy/assignment/key/control/root changes and
[creator obligations](NATIVE_STORAGE.md); later group failure discards them. A
funding payment may precede the operation. [JSON MBR](ACCOUNT_REPORTING.md) quotes
existing obligations; future growth still needs an SDK estimate.

## Separate registration possession

Register requires current policy/suite and sender equal its registered account.
Admin roles also require the current ASA principal and freeze/clawback capability where
applicable. Expected epoch must equal current role history (zero if absent).
Created and next epoch derive from ordered ledger state, never caller claims.
Registration changes role records/history/roots only: it neither rekeys AuthAddr
nor grants issuer powers beyond the account’s current ASA role.

The exact new Falcon1024 key verifies a typed [NAPR1 possession signature](../../internal/assetprivacy/registrationproof/README.md).
Its message is the fixed Registration/Falcon1024 domain plus the complete group
context digest. That context binds genesis, target, suite, operation, asset/policy,
role/account, exact key, expected epoch and every finalized body. Actual
Falcon1024 VerifyBytes runs before native writes. Current account PQ consent is a
separate check; the new key's signature cannot authorize somebody else's account.
Falcon1024 must be enabled even if account consent uses another enabled scheme.

NAPR has magic4/version1/kind1/signatureLengthBE2 plus1..Falcon1024MaxSignatureSize
signature bytes. No transfer witness or success callback substitutes for it. One
coarse work unit and proof-fee factor are charged, plus full signed bytes and base/
PQ fees. History, current pointer and key root update atomically after verification;
admin records grant no recipient eligibility. No Rust/P3 artifact is read here.
Adopt/Assign/Freeze require empty witness and zero proof units, but still pay full
signed-byte/base/PQ fees and storage. Framing alone proves no possession.

## Bootstrap and remaining possession work

Policy.Identity includes Created. Confirm adoption before deriving its actual
identity for key registration; an uncertain future inclusion round cannot be
assumed. Finalize fees/group before creating possession or value proofs; changed
fees change the signed group. Conservative encoding maxima are needed up front.

[RecipientSpend registration](OWNER_REGISTRATION.md) now verifies a separate kind21
NAPW proof of the nonzero owner secret, bound to the exact current body/group.
It reserves one work unit and atomically updates key/eligibility state only after
real verification. [Viewing registration](VIEWING_REGISTRATION.md) uses kind22 proof of a private seed for the exact ML-KEM key.
Syntax, encapsulation or membership cannot replace that possession relation.

[Ordinary ASA configuration](AUTHORITY_SYNCHRONIZATION.md) now synchronizes policy/controller state and checks changed final authorities' keys before group commit.
Require all mandatory keys before private value ingress. Complete new-ASA/private-only lifecycle, cleanup/refunds, APIs,
SDK/wallet, all modes/recovery, restoration/upgrades and six real-node scenarios.
Admin registration alone does not establish end-to-end delivery or crypto security.
