# Body-derived native ledger proof planning

Updated 2026-10-29. These internal helpers derive statements from canonical native
transaction fields and current COW state. They do not execute a transaction or
establish signed consent. The [evaluator](NATIVE_EXECUTION.md) now wraps them in
current PQ consent and atomic application; no proof/node execution has occurred.

Use `roundCowState.AssetPrivacyTransactionProofPlan(group,target)` to prepare an
exact plan, or `VerifyAssetPrivacyTransaction(group,target,registry,framed)` to
also check its complete proof bundle through the real optional Rust verifier.
Neither accepts caller drafts, output records, pins, budgets or verifier callbacks.
Keep the finalized group and supplied buffers stable throughout the call.

## Selection and projection

The consensus flag is checked before state I/O. Group count/target are bounded;
limits derive from current MaxTxGroupSize and MaxTxnBytesPerBlock. The target type
must be `aprv`; nativebody.Decode validates its exact version1 body and CheckSender
binds a positive public debit to the outer Sender. Zero debit allows fee sponsorship.
This account equality does not establish a valid signature or current AuthAddr.

The current policy must exist, and its canonical identity and immutable suite must
match the body. Policy reads validate the whole current consensus suite catalog.
The selected shielded mode must be permitted; positive public conversions also
require public mode. Every output's disclosure flags must equal current immutable
policy flags. Unknown/unsupported modes and relation arities do not fall back.

Before hashing a target or deriving its TXID, assetPrivacyBindBodies checks trusted
body/group encoding estimates and recomputes final group/order/network/suite context.
Output metadata then derives from the current encrypted-output stream and matching
commitment tree count. Capacity subtraction is checked before uint32 index conversion.
Round, TXID, output ordinal and Domain derive from that native snapshot. Nonces,
commitments, role ciphertexts and disclosure flags come exclusively from the body.

Transfer/unshield project the exact two input nullifiers/roots/counts and actual
output commitments plus debit/credit. Recovery projects one input, operation kind,
actual outputs and public credit. Existing current-state planners check accepted
roots, fresh markers, required key/eligibility/disclosure roots and current recovery
authority. Recovery additionally requires the target Sender to equal current
retained ASA clawback authority; signatures remain a separate mandatory check.

The returned AssetPrivacyTransactionPlan exposes copied spend/recovery snapshots,
a value-copy PublicConversion (asset, debit/source, credit/receiver) and immutable
Domain/Suite/Group context. Public account fields are committed by TXID and final
group binding; they cannot be supplied independently of the privacy body.

VerifyAssetPrivacyTransaction selects registry pins and limits from current
consensus again, then checks the exact plan through the real proof bridge. Missing
or incompatible local registry/backend availability is not invalid-proof evidence.
A generic signature-cache hit must never skip this proof/current-state work.

## No encryption or transaction-ID cycle

Envelope AAD includes network, asset, policy, private key epoch, role, nonce,
output ordinal and commitment. It excludes ciphertexts, group digest, TXID, round
and storage index. Therefore encryption precedes the canonical transaction body;
then finalize the group, derive proof statements and prove locally. Proofs remain
in SignedTxn.NativeAssetPrivacyProof outside TXID. Stream metadata does not enter
AAD or constructor identity, avoiding both ciphertext/TXID and proof/group cycles.

## Evaluator application and remaining obligations

The native dispatcher now verifies current PQ consent, public holdings, fees and
proofs before child-COW application of holdings, supply, markers and outputs.
[Storage obligations](NATIVE_STORAGE.md) and proof-count reservations share rollback.
The public planning/verifying helpers alone still grant no transaction authority.
Native inner transactions are rejected. Other modes/arities, lifecycle ingress,
REST/SDK/wallet and real-node scenarios remain pending. Flags remain false,
catalogs empty and execution/storage parameters zero.

Builds, vet and source checks cover compilation and documented source invariants;
they do not establish cryptographic correctness, historical byte equivalence,
privacy, replay/restart behavior, resource feasibility or end-to-end delivery.
Tests, vectors, proofs, public setup and node runs remain deferred.
