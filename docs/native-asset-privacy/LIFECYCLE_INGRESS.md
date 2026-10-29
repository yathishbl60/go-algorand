# Candidate lifecycle ingress contract

Updated 2026-10-29. This is the next implementation contract, not accepted wire
syntax or a deployed feature. Complete native delivery still includes every mode,
shield, administration, SDK/wallet and real-node scenarios in [the plan](README.md).
All consensus versions remain disabled; no tests or possession proofs have run.

## Existing state and missing authorization

[Policy writes](../../ledger/eval/assetprivacy_policy_write.go) check manager
identity, immutable declarations and retained capabilities. [Key writes](../../ledger/eval/assetprivacy_keys_write.go)
atomically update history, current pointers, authorization roots and eligibility.
[Disclosure assignment](../../ledger/eval/assetprivacy_disclosure_write.go) and
[freeze control](../../ledger/eval/assetprivacy_control_write.go) also exist.
These are persistence primitives; their address arguments are not signatures.

ViewKeyAir, SpendKeyAir and RegisteredKeyAir hash public key records and prove
current membership. They do not prove possession of a newly registered ML-KEM
private key or note spending secret. The [registry](../../internal/assetprivacy/artifactregistry/types.go)
has ten transfer/output relations; [proofbridge](../../internal/assetprivacy/proofbridge/verify.go)
has no registration relation. Never accept registration using syntax checks,
ordinary transfer proofs, or an injectable success callback.

## Proposed exact operation separation

Keep transaction type `aprv`, signed body field `napb`, witness field `napw`.
Preserve existing `NAPT` version1 transfer bytes. A separate bounded lifecycle
codec would use `NAPL` version1: magic/version5, opcode1, assetBE8, policy64,
suite64, expectedEpochBE8; fixed header150 bytes. Reject other versions, opcodes,
extra bytes, invalid enum/length values and zero domain components. Keep the
existing10465-byte body ceiling; preflight the exact operation before ownership
copies. These discriminators are proposed reservations, not implemented values:

| Opcode | Exact body tail and purpose |
| --- | --- |
| 1 Adopt | modes1, capabilities1, disclosures1, auditor32; policy/expectedEpoch must be zero |
| 2 Register | role1, account32, keyLengthBE2, exact role-specific public key |
| 3 Assign | auditor32; controller derived from current ASA clawback address |
| 4 Freeze | targetAccount32, frozen1 restricted to0/1 |

For existing policies, policy/suite must equal current authoritative values.
Adoption derives Created from inclusion round and capabilities from ASA params;
assert requested capabilities match exactly and retain public mode. No lifecycle
body carries private secrets, local artifacts, caller-selected roots or prices.
Epoch0 means absent; mutation increments by one with checked arithmetic.

## Principals, proofs and group execution

Every operation requires actual top-level PQ verification against current
sender/AuthAddr and an off-Edwards-curve authorizer. No native inner calls or
classical/multisig/LogicSig alternative. The sender must itself be the applicable
principal: current manager for Adopt/Assign, current freeze account for Freeze,
registered account for Register. Fee sponsors use separate atomic-group legs.
Role checks additionally require current manager/freeze/clawback identities or
current auditor/controller assignments; recipient roles authorize only themselves.

Adopt/Assign/Freeze need no value proof; require empty `napw`, exact body syntax,
current authority/expected epoch, normal PQ fees and creator storage funding.
Register requires a separately typed possession proof binding genesis, complete
final group, target index, operation, asset/policy/suite, role/account, exact new
public key and expected epoch. Owner registration proves the network-bound
nonzero secret preimage. ML-KEM registration must prove possession of a valid
private key corresponding to the exact admitted public key; encapsulation alone
proves no such thing. Administrative Falcon records require new-key possession
bound to the same operation, in addition to current account authorization.

Derive all domains, next epochs and Created from ordered COW state. Check full
proofs before writing. One child must commit policy/assignment/key/control/root
changes and [creator obligations](NATIVE_STORAGE.md) together. Later group failure
discards them. Funding can precede the operation in the group. JSON account
min-balance reporting includes existing obligations; future growth still needs
an SDK estimate. Preserve ordinary holdings, supply and existing note trees.

## Bootstrap and full implementation order

Policy.Identity hashes Created; adoption requires Created equal current round.
Consequently keys/proofs referring to a new policy cannot assume an uncertain
future inclusion round. Confirm zero-value adoption first, then derive the actual
identity and register keys. A bundled alternative needs explicit single-round
semantics and measured proving latency; silently predicting Created is invalid.

1. Implement lifecycle codec/dispatch, PQ principal checks, explicit no-proof
   operations, exact fees and atomic policy/disclosure/freeze execution.
2. Add registration possession relations, trusted artifact entries, statement
   binding, decoder/work/fee bounds and real Go/Rust verifier routing; then expose
   registration/rotation. Do not enable it before those checks exist.
3. Require all policy-mandated issuer/viewing/spending records before private
   value ingress. Registration must reconcile current eligibility; old roots or
   epochs cannot authorize a fresh operation. Add new-ASA creation/private-only
   lifecycle and native shield ingress without weakening existing-asset adoption.
4. Synchronize ordinary ASA authority rotation/relinquishment with policy,
   assignments and registry eligibility atomically. Keep historical decryption
   records and prevent restored powers. Complete destruction/refunds, APIs,
   builders, all modes, recovery, node/wallet restoration and upgrade evidence.

Adoption alone is the first step, not the north-star deliverable. All new code
must retain20-line functions/100-line authored files, default-disabled gates and
explicit incomplete evidence until the complete protocol has been validated.
