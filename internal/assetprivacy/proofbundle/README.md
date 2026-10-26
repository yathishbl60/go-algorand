# Native privacy proof bundle planning and framing

Build tag: `algorand_native_privacy`. This package composes exact expected proof
sets and frames opaque per-AIR bytes. It does not check STARKs, authenticate a
transaction or authorize state writes. No runtime codec/proof checks have run.

Use a native spend/recovery snapshot's `Bundle()` method. It fixes the main
kind, ordered output commitments and required recipient/disclosure proof kinds.
Alternatively, `Spend(main, outputs)` or `Recovery(main, outputs)` accepts trusted
local plans. Those generic entry points do not establish current-state provenance.
Missing output plans and inconsistent domain/commitment/ordinal/kinds fail.

`Plan.Slots()` returns ordered immutable descriptors; `Slot.Values()` copies each
expected public vector. Main is first with role zero/output `0xffffffff`. Each
actual output then has its required recipient followed by optional auditor and
controller. Ordinary output plans cannot substitute recovery or retained kinds.
Recovery selects retained recipients only at the main plan's remainder index.

```go
bundlePlan, err := nativeSnapshot.Bundle()
// Handle err before using bundlePlan. Limits come from trusted local configuration.
encoded, err := proofbundle.Encode(bundlePlan, orderedPerAIRProofs, limits)
// Handle err; serialization is not proof verification.
err = proofbundle.Check(encoded, bundlePlan, limits)
```

`Encode` uses only plan-selected descriptors. `Check` validates the complete exact
set, nested proof magic/minimum header, bounded lengths and absence of trailing
bytes. `Decode` performs that complete preflight before allocating owned proof
copies; failure returns no partial result. Its conservative charge includes the
encoded byte length and 24 bytes per Go slice descriptor. Caller input/plans and
allocator metadata are outside that accounting. Do not mutate buffers during calls.

`Limits` has `MaxBundleBytes`, `MaxProofBytes`, `MaxAllocationBytes`; no defaults
are supplied. Guard ceilings are 512 MiB bundle, 128 MiB per proof, and 512 MiB
owned codec allocation. These are not measured safe consensus work/fee budgets.
All lengths are checked before the codec allocates output buffers.

The [format/composition contract](../../../docs/native-asset-privacy/PROOF_BUNDLES.md)
documents tags, endian conventions, supported arities and resource limits. The
[Rust typed dispatcher](../stark/src/verification/bundle/mod.rs) checks the same
framing, shared commitments and artifact headers, then invokes every actual
per-AIR verifier. Cross-language conformance/proof execution remain unverified.

Embed the bytes in the separately bound [NAPW witness](../witness/README.md) only
after selecting the trusted profile and recomputing finalized-body group context.
Approved suite/artifacts, Go/Rust transport, authenticated native transaction
bodies, fees/funding/work bounds and atomic state application remain unfinished.
