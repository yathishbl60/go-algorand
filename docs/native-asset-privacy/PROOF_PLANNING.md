# Output proof composition and native expected statements

Updated 2026-10-25. [proofplan](../../internal/assetprivacy/proofplan/README.md)
now constructs exact expected public vectors for one output's required roles.
[Native assignments](../../internal/assetprivacy/disclosure/README.md) and
[disclosure constraints](../../internal/assetprivacy/stark/src/authorized_disclosure_air/README.md)
exist. These components do not constitute a complete native verifier or transfer.

## Joining only already-public commitments

The main shield/transfer relation commits each new canonical opening to a public
output note commitment under the network, asset and immutable policy. Each
recipient/disclosure envelope relation constrains its encrypted canonical opening
to that same commitment and domain. Under SHA3-512 binding, accepting every
required relation with identical expected commitments links the openings without
serializing an extra witness-dependent digest or compressed private tuple.

This uses the existing public output commitment only. Input note commitments,
selected accounts, paths, viewing keys and epochs remain private. Splitting input
membership/ownership/control proofs by publishing their private commitments would
change the privacy contract and is not permitted by this composition design.
The pinned batch lookup terminal is also unsuitable as a private join; see
[trace architecture](TRACE_ARCHITECTURE.md). No such lookup dependency was added.

A binding hash argument alone does not establish composed soundness or zero
knowledge. Independent fresh hiding/prover entropy, exact Fiat-Shamir domain
separation, multi-proof soundness/quantum budget, trusted artifact preparation and
full security analysis remain required. No proof/runtime evidence exists yet.

## Required native verification sequence

1. Parse bounded canonical transaction bodies, operation arity and outer witness.
   Obtain the expected suite/version and trusted schedule/verifier artifacts.
2. Derive the current group-local immutable policy, assignment accounts and key
   root after preceding group mutations. Derive accepted input roots separately.
3. Match every output record to its finalized body/context/ordinal/commitment and
   exact policy ciphertext roles. Reject extra, missing or malformed envelopes.
4. Build each output Plan. Require exactly recipient, optional auditor, optional
   controller proof slots in that order. Structural CheckRoles does not verify.
5. Decode and cryptographically verify every required proof using the exact
   native-derived vector and expected relation/artifact. Never trust a proof's
   advertised public values, role labels, root, authority or verification key.
6. Verify the complete main spend/conservation/control/group relation against
   the same ordered output commitments and full finalized body/order digest.
7. Recheck all state-dependent authorization and work bounds before atomically
   applying balances, nullifiers, outputs, tree roots and storage charges.

The existing internal AppendAssetPrivacyOutput helper still requires its caller
to perform these steps. No role plan grants permission to invoke that helper.
Current eligibility, freeze/recovery authority, remaining operation arities,
embedded proof decoding, deterministic verifier and transaction ingress remain
pending. Merely comparing witness headers or statement digests cannot replace
cryptographic proof verification or those missing relations.

## Practical implementation contract

Use `AssetPrivacyOutputProofPlan` at the native boundary; it supplies no caller
root/authority/domain overrides and checks the feature before I/O. Generic
`proofplan.Build` is for a trusted local snapshot; its Config is not authenticated
by its Go type. `Slot.Values()` copies canonical uint16 scalar values suitable
for checked conversion to the pinned field. It exposes no private epoch.

The expected recipient relation is AuthorizedRecipientAir: current viewing and
spending registrations must match the opening's private account and owner, with
independent epochs. The expected disclosure relation is AuthorizedDisclosureAir:
current viewing membership must match the explicitly assigned public account.
The auditor/controller account must not equal the private recipient by accident
of wiring, and viewing membership must never grant spending or recovery power.

Auditor assignment changes and key rotations are prospective. Controller viewing
assignment follows current nonzero ASA clawback and is retained after relinquished
power, so immutable mandatory disclosure remains possible without restoring that
power. A changed nonzero clawback with an unsynchronized assignment fails closed.
Full configuration integration must keep this transition atomic before activation.

Plans and proof-cache entries must expire on relevant preceding group changes.
Future cache keys must cover trusted suite/artifact, complete vectors, role,
finalized group/body context and current state. Historic note roots may be valid
for input membership; they can never authorize stale viewing/spending records.

The current matrices remain too large for a practical protocol budget. No tests,
proofs or benchmarks have run. Builds, vet, Clippy and source/size checks establish
only the reported source and compilation properties. The full north-star plan,
security/resource evidence and all six real-node demonstrations remain required.
