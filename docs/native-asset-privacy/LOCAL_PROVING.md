# Candidate local typed proving API

Updated 2026-11-01. Rust feature `native-privacy` exposes local proving through
[typed Prepared factories](../../internal/assetprivacy/stark/src/verification/selection/prepared.rs).
This API has build/static evidence only. [Bounded Owner primitive runs](FEASIBILITY_RESULTS.md)
used direct P3 setup/prove/verify; no complete artifact-bound native proof exists.
Every consensus version remains disabled; there are no approved suite artifacts.

## Trusted inputs and preparation

Load `selection::Registry` with independently pinned profile, implementation and
registry digest. NAPK6 still contains all20 current layout kinds; producing a
single proof does not permit replacing it with a one-entry/unpinned registry.
Use the exact typed factory for the finalized operation/domain. Public statement
values must come from the trusted finalized native group, not a proof or witness.

`Prepared::setup_public()` creates reusable public preprocessing data offline.
`PublicSetup` keeps that data private and exposes its public `entry()`; it is not
an arbitrary setup-data injection API. Its layout/root/metadata must match the
pinned registry and the Prepared layout before proving. All20 registry layouts
retain their kind IDs. Changed source fingerprints do not make older references
compatible: unsupported historical references remain rejected. See
[artifact selection](ARTIFACT_SELECTION.md) for the separate approval obligations.

## Resource checks before private construction

`verification::ProveLimits::new(trace_bytes, decode_limits)` supplies a local
main-trace ceiling plus existing canonical proof wire/allocation ceilings.
`prepared.prover_requirements(limits)` returns ProveRequirements: rows, width,
preprocessed_width, public_values and nominal trace_bytes. These fields contain
no witness material. Check them before allocating a private matrix.

`prepared.prove(registry, setup, owned_trace, public_u16, limits)` consumes an
already-built RowMajorMatrix. It checks exact width/element count and vector
capacity bytes, but cannot undo the trace's earlier allocation.
`prepared.prove_with(registry, setup, public_u16, limits, build)` checks pinned
setup, public length and nominal trace budget before calling the builder.
The builder receives the exact typed AIR and returns only a private matrix;
it cannot choose verifier keys, entropy, prover configuration or acceptance.
Post-construction shape/capacity checks still apply.

These are local ceilings, not total-memory or CPU budgets. Owner registration's
512×3540 main cells occupy14,499,840 raw bytes; shield's1024×7641 occupy62,595,072.
Preprocessing, Keccak scratch, hiding/LDE/quotient matrices and potentially large
Plonky3 temporaries are additional. These are source counts, not measured peaks,
practical feasibility evidence or recommended deployment limits.

## Owner-registration caller fragment

This unexecuted fragment assumes an already trusted registry, matching setup,
limits, domain, statement and private nonzero32-byte spending secret. Statement
is `owner_registration_air::PublicStatement { owner, group }`; both digests are
nonzero64-byte values, with owner bound to the network/secret and group finalized.
No example secret, setup root or proof bytes are supplied.

```rust
use algorand_asset_privacy_stark::verification::{Error, public_values_u16};
use algorand_asset_privacy_stark::verification::selection::Prepared;

let prepared = Prepared::owner_registration(domain)?;
let cells = prepared.air().public_values(&statement).ok_or(Error::PublicValues)?;
let public = public_values_u16(&cells)?;
let encoded = prepared.prove_with(&registry, &setup, &public, limits, |air| {
    air.trace(secret, &statement)
        .map(|witness| witness.trace)
        .ok_or(Error::TraceShape)
})?;
```

The typed public helper yields64 field cells: owner32 then group32, as uint16
limbs. `public_values_u16` rejects noncanonical/out-of-range cells; it never
converts private trace columns. Owner trace construction checks the nonzero
secret and exact network-bound owner commitment before allocating its matrix.
Other relations require their own typed statement/witness builders and context.

## Entropy, output and failure boundaries

Each backend prove call creates a fresh OS-seeded `prover_config()` for hiding
masks/commitments. Public preprocessing configuration is not exposed as a prover
configuration. Proofs are canonically encoded under the pinned typed verifier,
then mandatorily self-verified with those exact public values before bytes return.
No caller callback can replace that verification or mark a proof accepted.

The backend/encode/self-verify pipeline catches unwinding panics as BackendPanic.
This does not silence Rust panic hooks, debug constraint diagnostics, or an OOM
abort. A builder panic is outside that catch boundary. Raw traces/openings are
secret and must never be logged, uploaded or persisted unencrypted. Consuming a
matrix does not guarantee zeroization of Rust allocations or earlier copies.

The returned bytes are one canonical typed proof, not an assembled NAPW witness,
signed transaction or accepted block. Exact bundle assembly, native fees/group
binding, local proving CLI, full wallet/SDK workflow and all real-node scenarios
remain pending in [delivery status](END_TO_END_STATUS.md). Self-verification does
not establish soundness, privacy, authorization or resource feasibility.
