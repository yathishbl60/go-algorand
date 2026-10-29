# Consensus suite catalog and native verification admission

Updated 2026-10-28. Catalog selection and native verification helpers compile;
no suite entries, approved artifacts, runtime proofs or native transaction
acceptance exist. Every current version, including ConsensusFuture, has
EnableNativeAssetPrivacy=false and NativeAssetPrivacySuites=nil.

## Metadata and complete validation

ConsensusParams.NativeAssetPrivacySuites is an omitted-when-empty JSON slice of
NativeAssetPrivacySuite values. Each record contains:

| Field | Meaning |
| --- | --- |
| ID64 | Policy suite/profile ID |
| Reference64 | Supported frozen implementation reference |
| RegistryDigest64 | Independently pinned digest of the complete954-byte NAPK file |
| MaxBundleBytes | Explicit bundle byte cap |
| MaxProofBytes | Explicit individual proof byte cap |
| MaxAllocationBytes | Go payload-copy cap and separate Rust decoder charge quota |

[suitecatalog.Lookup(proto,id)](../../internal/assetprivacy/suitecatalog/README.md)
and Validate first return policy.ErrDisabled when the feature is off. Enabled
catalogs have at most 16 records and are copied into fixed storage. Every record
is checked before selection: nonzero IDs/reference/digest, unique IDs even for
identical duplicates, and shared proofbridge.Limits.Validate bounds. A malformed
later entry rejects the whole catalog. ErrCatalog denotes invalid metadata;
ErrUnknown denotes an absent ID after full validation. Empty enabled metadata
is valid but approves nothing. There is no default, fallback or wire-selected pin.

Caps are bundle bytes 7..512MiB, proof bytes 73..128MiB and no larger than bundle, allocation
1..512MiB. These are guard ceilings, not measured safe node budgets or fee pricing.
ConsensusProtocols.DeepCopy and Merge's incoming add/update path clone suite
slices and ApprovedUpgrades maps; nil is preserved. Returned Suite pins/limits are value copies.
Lookup does no hashing, I/O, backend call, setup or proof checking.

## Authoritative native path

Use roundCowState.VerifyAssetPrivacySpend or VerifyAssetPrivacyRecovery. Each
rebuilds the current plan before verification: GetAssetPrivacyPolicy gates before
I/O, validates its suite against current consensus, derives domain/current state,
and Bind recomputes finalized body/order context using that authoritative suite.
Recovery also checks retained/current clawback identity; identity is not a signature.
The plan retains immutable Domain/Suite/Group and copied main/output statements.

The private suite verifier calls Lookup(cs.proto,plan.Context().Suite), then
Suite.Verifier(registryBytes). Callers provide no Pin, Limits or Verifier override.
Registry bytes may be untrusted but must match the consensus-selected pins. Local
registry installation/compatibility failures are wrapped as ErrUnavailable.
The generic plan.Verify(verifier,frame) remains an internal cryptographic helper;
it cannot replace this current-consensus admission path in future transaction routing.

Successful checking performs no state writes and grants no transaction authority.
Current account consent, conversion state, mode rules, pricing, replay protection
and atomic application remain required. The aprv type and stateless signed framing
exist; evaluator dispatch/public RPC do not route through these helpers yet. See [integration](NATIVE_TRANSACTION_INTEGRATION.md).

## Reference identity and upgrade retention

Reference maps to Pin.Implementation, the existing NAPK/NAPV wire field name.
It is distinct from verification::implementation_id(), the current compiled build
fingerprint. Registry::load requires reference_supported(reference); the explicit
COMPATIBLE_REFERENCES array is currently EMPTY. Thus only the current fingerprint
is supported today. Peers, artifact files and configurable lists cannot extend
that compiled compatibility set. Source compatibility alone is not suite approval.

prepared.relation_id(&registry) uses the selected registry reference, exact layout
and canonical constructor arguments. Artifact IDs also bind that reference,
profile, registry, relation, dimensions/configuration and trusted root. A future
reviewed implementation may recognize a frozen reference while its own fingerprint
changes, preserving IDs only with identical constraints, schedules, encodings,
parameters, decoder charges and artifact-ID construction. Incompatible changes
need versioned dispatch; changes to hashed inputs have no historical compatibility today.

Every live asset's immutable Suite must remain recognized across protocol/client
upgrades, with its pinned artifacts and compatible verification implementation.
Dropping it can block policy reads, public holding synchronization and private
verification. Define normative migration/cleanup before removal; there is no silent
fallback, implicit policy rewrite or automatic historical-proof migration. The
bounded 16-entry catalog does not remove this live-asset/history retention obligation.

## ABI and resource outcomes

ABI2 keeps the _v1 function names. Status3 maps to ErrRejected for proof failure,
including deterministic wire/shape/decoder-quota violations. Status1 request failure,
2 artifact failure,4 caught unwind and unknown statuses map to ErrUnavailable;
5 actual fallible allocator failure maps to ErrResource. Local failures must never
be treated as invalid-peer evidence. See [bridge details](NATIVE_VERIFIER_BRIDGE.md).

Rust decoder charges use a fixed 64-bit model, not host size_of: Value8, Challenge40,
digest64, Vec/Cap24 and input/FRI-phase opening72. Child elements are charged
separately. Charges are deterministic admission units, not actual heap/CPU limits.
Caller frame, owned Go copy/request, per-proof decoded data and backend/allocator
costs are additive; no aggregate memory meter or measured validation budget exists.

No tests, setup, traces, proofs or benchmarks have run. Sibling ../specs and
../go-algorand-sdk checkouts are absent; normative consensus/specification and SDK
integration remain external deliverables. Catalog plumbing does not complete the
[north-star delivery scope](END_TO_END_STATUS.md).
