# Candidate Rust proof verification boundary

Updated 2026-10-27. The opt-in Rust library has a bounded per-AIR codec and calls
pinned `verify_with_preprocessed` for actual cryptographic checking. This is
**not native transaction acceptance**. No proofs, codec round trips, public setup
or adversarial execution have run; tests/runtime evidence remain deferred.

## Trusted selection contract

Use `verification::selection::Registry::load(bytes, pin)` with independently trusted
profile, supported implementation reference and registry-file digest. The fixed
registry is checked before publishing keys; reference compatibility is compiled
and its historical array is currently empty. [Consensus selection](NATIVE_SUITE_CATALOG.md)
approves no default file/suite/key. A profile::Manifest is never suite authority.

Typed `Prepared` factories construct one of the six native AIR types and derive a
canonical full relation ID from its exact public constructor arguments and selected
registry reference: prepared.relation_id(&registry), not a no-argument method. A proof
cannot choose an arbitrary AIR, relation label, row count or preprocessing key.
`Registry::bind(prepared)` selects the exact layout key and returns `Verifier<A>`.
Generic `Artifact::from_trusted` and `Verifier::from_trusted` are now internal;
old experimental direct-construction examples are superseded by this API.

The artifact ID binds the backend/configuration, selected reference, profile/registry,
full relation ID, dimensions/row-access shape and preprocessing root. Its domain is
`Algorand/AssetPrivacy/Candidate1/VerifierArtifactV2/`; older experimental IDs are
incompatible. The wire carries only an ID for equality checking, never key selection.
Metadata hashing and independent file pins establish integrity/provenance contracts,
not security approval or proof that the selected relation is correct.

`Verifier::verify(bytes, expected_values, limits)` requires exact native public
values, in order, as `u16` cells. The proof cannot supply them. The backend replays
Fiat–Shamir and checks openings, FRI and constraints against those values and the
selected key. Success does not authenticate signatures, fees, modes, roots, authority,
body matching or state application. The native caller still checks every obligation.

## Developer use

With Rust feature `native-privacy`, a transfer preparation has this shape:

```rust,ignore
use algorand_asset_privacy_stark::verification::{DecodeLimits, selection::{Prepared, Registry}};

let registry = Registry::load(&registry_bytes, independently_approved_pin)?;
let prepared = Prepared::transfer(native_domain, public_conversion)?;
let _relation_id = prepared.relation_id(&registry);
let verifier = registry.bind(prepared)?;
let limits = DecodeLimits::new(wire_budget, allocation_budget)?;
let bytes = verifier.encode(&local_proof, limits)?;
verifier.verify(&bytes, &ledger_expected_values, limits)?;
```

The inputs above must come from the application; no approved registry/proof fixture
exists. `encode` serializes/checks shape; it does not verify. Backend proof/key types
are re-exported for proving integration. [Artifact selection](ARTIFACT_SELECTION.md)
defines all factories, relation preimages and public setup. Private proving still
uses fresh OS-seeded `prover_config()`; deterministic public setup never exposes a
seeded Config and is never called by the verifier or build checks.

## Admission and failure behavior

- No default budget exists. Positive `DecodeLimits` are capped at128 MiB wire and
  512 MiB decoder allocation requests. These are guards, not measured node limits.
- Each vector length is checked against remaining bytes and an aggregate allocation
  quota before fallible reservation. Charges use fixed64-bit units independent of
  host size_of; these are admission units, not actual heap/CPU caps. Fixed opening
  widths are rejected before allocation; [ABI2 outcomes](NATIVE_VERIFIER_BRIDGE.md) distinguish failures.
- Fixed-depth parsing rejects noncanonical field words, nonzero grind witnesses,
  trailing bytes, wrong versions/IDs/degrees. The backend independently derives
  and checks quotient/FRI/MMCS geometry.
- Accounting covers requested vector storage, excluding allocator metadata,
  public-vector conversion, backend copies, symbolic constraints, CPU, preprocessing
  generation and caller buffers. Native admission needs measured work/memory limits.
- MemoryLimit is deterministic quota rejection; AllocationFailure is a local
  resource fault. ABI2 maps proof rejection to ErrRejected, actual allocation
  failure to ErrResource and trusted-request/artifact/backend faults to
  ErrUnavailable. Never convert local faults into invalid-peer evidence. Aborts
  or panic-abort builds cannot be recovered by catching unwinds.

The [per-AIR codec](../../internal/assetprivacy/stark/src/verification/README.md)
and [exact main/output bundles](PROOF_BUNDLES.md) provide current spend/recovery
planning and typed Rust dispatch. Native plans retain value-copy Domain/Suite/Group
context, including zero-output operations; their Main/Outputs accessors also copy
plan objects. The [native bridge](NATIVE_VERIFIER_BRIDGE.md) links; current-consensus
VerifyAssetPrivacySpend/Recovery select approved metadata and dispatch exact plans.
Catalogs are empty; native transaction ingress and execution remain missing.

## Remaining delivery gates

Produce and independently approve actual preprocessing artifacts/profile pins;
populate the empty consensus catalog and connect native transaction ingress; bind
proof checking to authenticated bodies and atomic state execution; measure
masking, quotient, proof size, resource limits and composed security. Then complete
pool/API/SDK/wallet/recovery support and the six real-node scenarios. See the full
[delivery checklist](END_TO_END_STATUS.md). Runtime validation and independent review
remain outstanding; all features stay disabled by default.
