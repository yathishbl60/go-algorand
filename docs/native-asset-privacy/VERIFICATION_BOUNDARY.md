# Candidate Rust proof verification boundary

Updated 2026-10-25. The opt-in Rust library now has a bounded per-AIR binary codec
and calls the pinned backend's `verify_with_preprocessed` for cryptographic checking.
This is **not native transaction acceptance**. No proof, codec round trip or
adversarial execution has run; tests and runtime evidence remain deferred.

## Caller contract

`verification::Artifact::from_trusted(profile, relation, rows, key)` takes a
locally selected preprocessing key. It never reads a key from proof bytes.
The caller must establish the commitment's provenance from the exact preprocessing
schedule, pinned backend and reviewed configuration. Its width, single-root cap
and hiding-extended degree are checked against the expected AIR and row count.
An arbitrary key with a matching width is not a trusted artifact.

`relation` must identify the AIR kind, relation revision and every constructor
argument, including network, asset, policy, operation and applicable envelope or
authority context. Construct the AIR from the same trusted configuration and
ledger-derived context. The generic library cannot prove those selections came
from an approved registry. No native allowed-suite/artifact registry exists yet.
V2 suites must reject serial-only artifacts; a caller-supplied label is not evidence
that a preprocessing commitment or AIR implements that revision.

`Verifier::from_trusted(air, artifact)` owns this selection. Its artifact ID hashes
the backend pin/configuration, supplied profile/relation IDs, dimensions, row-access
shape and preprocessing commitment. A proof carries that ID only for equality
checking; it cannot use it to select a different key or relation. Metadata hashing
provides integrity binding, not security approval or a soundness argument.

`Verifier::verify(bytes, expected_values, limits)` requires the exact public vector
from native planning, in order, as `u16` cells. The proof cannot supply this vector.
The backend replays Fiat–Shamir and checks openings, FRI and AIR constraints against
that vector and the selected preprocessing key. Returning success here does not
authenticate signatures, fees, modes, roots, authority, body matching or state writes.

## Developer use

With the Rust feature `native-privacy` enabled, use the same verifier object for
local serialization and verification:

```rust,ignore
use algorand_asset_privacy_stark::verification::{Artifact, DecodeLimits, Verifier};

let artifact = Artifact::from_trusted(profile_id, relation_id, rows, trusted_key)?;
let verifier = Verifier::from_trusted(expected_air, artifact)?;
let limits = DecodeLimits::new(wire_budget, allocation_budget)?;
let bytes = verifier.encode(&local_proof, limits)?;
verifier.verify(&bytes, &ledger_expected_values, limits)?;
```

The inputs above are supplied by the application; there is no production key or
proof fixture. `encode` checks framing/shape and serializes; it does not verify.
`BackendProof` and `PreprocessedVerifierKey` are re-exported for integration.
Provers must still use OS-seeded `prover_config()` and the matching trusted schedule.
The deterministic configuration used by verification is private to the crate;
verification consumes no prover entropy and does not expose a seeded prover API.

## Admission and failure behavior

- No default admission budget exists. `DecodeLimits::new` requires positive local
  budgets, capped at 128 MiB wire and 512 MiB decoder allocation requests. These
  ceilings are implementation guards, not measured safe node limits or fee rules.
- Each vector length is checked against remaining bytes and an aggregate allocation
  quota before fallible reservation. Container storage, field cells and cap roots
  are charged. Fixed opening-row widths are rejected before row allocation.
- The parser has fixed nesting depth, rejects noncanonical field words, requires
  zero grinding witnesses, and rejects trailing bytes, wrong versions/IDs/degrees.
  The backend independently derives and checks quotient/FRI/MMCS geometry.
- Decoder accounting covers requested vector storage. It does not cover allocator
  metadata, public-vector conversion, backend copies, symbolic constraints, CPU,
  preprocessing generation or the caller's already allocated input/proof buffers.
  Native admission needs separate measured work/memory limits and scheduling.
- Rejections return stable `verification::Error` categories. A backend unwind is
  converted to `BackendPanic`; allocator aborts or a panic-abort build cannot be
  recovered this way. Panic conversion is not a substitute for adversarial checks.

The exact candidate wire format is documented in the
[codec reference](../../internal/assetprivacy/stark/src/verification/README.md).
The outer [NAPW frame](../../internal/assetprivacy/witness/README.md) remains opaque.
[Exact main/output bundles](PROOF_BUNDLES.md) now provide native expected-slot
planning and typed Rust dispatch for the current spend/recovery arities. Native
artifact selection and the Go/Rust bridge remain; no parser authorizes a transaction.

## Remaining delivery gates

Establish trusted artifact production/provenance and native suite selection; bridge
the exact expected bundle to Rust; connect verification to
authenticated native transaction bodies and atomic execution; measure masking,
quotient, proof-size, work/memory and composed security parameters. Then complete
pool/API/SDK/wallet/recovery support and all six real-node scenarios. Follow
[the canonical delivery checklist](END_TO_END_STATUS.md); all runtime validation
and independent review evidence remain outstanding.
