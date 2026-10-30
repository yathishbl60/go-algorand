# Candidate public-key epoch persistence

This tagged internal package stores preauthorized public-key records. It does
not expose or authorize a registration transaction, verify key possession,
prove account control, or grant issuer powers. Native helpers require the disabled
`EnableNativeAssetPrivacy` consensus flag and an existing native privacy policy.

## Roles and key material

| Role | Material | Native eligibility check |
| --- | --- | --- |
| RecipientView | ML-KEM768 public key | Nonzero account; caller must authorize |
| RecipientSpend | Nonzero 64-byte note owner commitment | Nonzero account; caller must authorize |
| AuditorView | ML-KEM768 public key | Auditor disclosure declared |
| ControllerView | ML-KEM768 public key | Controller disclosure; current ASA clawback account |
| FreezeAuthority | Falcon1024 public key | Retained freeze capability; current ASA freeze account |
| ClawbackAuthority | Falcon1024 public key | Retained clawback capability; current ASA clawback account |
| ManagerAuthority | Falcon1024 public key | Current nonzero ASA manager account |

Viewing keys never authorize spending or issuer actions. RecipientSpend is the
existing network-bound SHA3 owner commitment; knowledge of its public value is
not authorization. Spending requires the private 32-byte secret preimage relation
inside the note ownership AIR. There is no Falcon-in-circuit fallback.

ML-KEM parsing uses the existing envelope parser and Go ML-KEM modulus/size
validation. Falcon parsing uses the repository Falcon1024 public-key size,
header 10, and its 1024 packed 14-bit coefficients, each below 12289. This follows
the pinned `github.com/algorand/falcon` `modq_decode` shape. A syntactically valid
key is not evidence it was generated honestly or that anyone knows its secret.
`ValidatePublicKey(role,bytes)` exposes this shared syntax check and rejects unknown
roles; Record.Validate additionally checks domain and nonzero epoch.

## Domains and epochs

`Domain` includes network 32, asset BE8, policy 64, account 32, and role 1 (137 bytes).
Every component must be nonzero where applicable; roles must be known. Policy
identity comes from the native immutable policy declaration. Different roles,
accounts, networks, assets and policies have separate epoch streams.

Epochs start at 1 and advance by exactly 1, without overflow. `Created` must equal
the evaluator round for writes and cannot move backwards across rotations.
Multiple rotations in one round are allowed. Historical records cannot be
overwritten and remain readable after issuer accounts/capabilities change.
Read helpers return data only; an old record cannot restore a removed authority.

All keys start with `aprv:k:` followed by the full 137-byte domain. Suffix `c`
addresses the current pointer; `e` plus epochBE8 addresses immutable history.
The pointer repeats version 1, domain 137 and epochBE8 (146 bytes). Each historical
record repeats version 1, domain 137, epochBE8, CreatedBE8, and exact public-key
bytes (154 bytes plus role-specific material). Unknown versions, malformed
shapes, swapped domains/epochs, future creation rounds, missing pointed-to
records and attempted history overwrites fail closed.

## Internal APIs and required callers

`Open(KV, Domain)` creates a store. `Current(round)` and `Epoch(epoch, round)`
read validated records. `Put(record, round)` enforces syntax and epoch progression.
`KV.Apply` must serialize and atomically apply the history/pointer batch or roll
it back on error. The COW adapter copies writes into a child, publishes it only
on success, then recycles it. Group failure must discard the surrounding group
COW. No local setting or transaction can enable this feature.

Native `OpenAssetPrivacyKeys` derives the authoritative policy identity.
`PutAssetPrivacyKey` additionally checks the table's native eligibility rules.
Internal callers must verify complete account authorization and role-appropriate
possession, suite, fees and storage funding. [NAPL admin registration](../../../docs/native-asset-privacy/LIFECYCLE_INGRESS.md)
accepts admin roles5..7, with Sender equal registered account/current ASA role,
current off-curve PQ consent and separately verified new-key Falcon1024 possession.
Expected epoch and current policy/suite bind the mutation; Created is ledger-derived.
[NAPR](../registrationproof/README.md) signs the complete final group and key bytes.
It updates history/pointer/current root atomically and consumes one coarse work
unit plus proof/byte fees and creator storage. It never rekeys AuthAddr or creates
new ASA powers. The Falcon1024 scheme must be enabled; no Rust artifact is read.
RecipientSpend now verifies its owner-secret NAPW proof and reconciles eligibility.
Viewing roles use separate kind22 seed possession. Auditor/controller targets derive from authenticated
[assignments](../disclosure/README.md), including retained viewing after relinquishment.

Admin possession verification source now exists; no signatures, key generation,
tests, possession proofs or node runs occurred. Tagged builds/vet and authored
limits passed. Complete lifecycle and runtime evidence remain.

Native key reads expose a view without Put; writes use PutAssetPrivacyKey.
Native key writes now atomically update the separate [current-key registry](../keyauth/README.md)
with history/pointer state. Prior unindexed experimental state fails closed;
current roots do not use note-root history. Proof membership and all current
role/spending/eligibility checks remain caller integration obligations.
