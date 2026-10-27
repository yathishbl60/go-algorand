# Native proof-verification bridge

Go tag `algorand_native_privacy` exposes the API. Actual CGO verification additionally
requires `algorand_native_privacy_stark`, CGO, Linux/Darwin and the built Rust archive.
Other builds return `ErrUnavailable`; no stub/callback reports successful checking.
The feature remains disabled in every consensus version.

`NewVerifier(registryBytes, Pin, Limits)` validates/owns an independently pinned
954-byte artifact registry and checks the linked ABI/reference compatibility. Pins and
limits must come from consensus-approved local suite configuration, never peer
proofs/requests, the artifact itself or an evidence Manifest. `suitecatalog` selects
these values from current consensus; all current catalogs are empty. No approved
registry file or pins have been produced. The zero verifier cannot verify anything.

Use the current native snapshot's `Verify` method, which supplies `Bundle()` and
value-copy Domain/Suite/Group. The generic `Verifier.Verify(plan, context, frame)`
requires the same authoritative provenance. It compares Suite with the configured
profile, validates the outer NAPW binding, copies the bounded payload, checks the
complete expected NAPB framing, and sends exact public vectors/context to Rust.
The trusted internal NAPV request is at most15500 bytes; it is not a network format.

Rust rebuilds the concrete AIR constructors and relation/artifact IDs, checks main
group/output-domain correspondence and every expected proof via the pinned P3
backend. A nil result establishes cryptographic checking only: no signatures,
fees, state writes, atomic execution or transaction acceptance happen here.
Native transaction/pool ingress and authorized suite installation remain unfinished.

`Limits` requires positive MaxBundleBytes (<=512 MiB), MaxProofBytes (<=128 MiB),
MaxAllocationBytes (<=512 MiB). Go's owned payload copy and Rust's decoded-data
quota are separate equal caps. Caller frame, small request/vector copies, backend
quota uses fixed64-bit canonical element charges on every architecture, rather than
host `size_of`. Work/copies and allocator overhead are additional; there is no aggregate memory or
CPU/fee guarantee. These are implementation guards, not safe deployment budgets.

ABI2 maps invalid proofs to `ErrRejected`, local allocator failure to `ErrResource`,
and other local configuration/backend failures to `ErrUnavailable`. Deterministic
decode quota violations reject proofs; physical allocation failure does not.
Native admission also classifies missing/incorrect local registry installation
as unavailable. Callers must not cache local failures as invalid transactions.

Caller input must stay stable during decoding/copy. Go-owned payload/request buffers
are immutable for FFI and held with runtime.KeepAlive until return. Rust never
retains/frees either Go range. The separate thin FFI crate has the unsafe pointer
operations; the core verifier forbids unsafe code.

[Build/link and ABI details](../../../docs/native-asset-privacy/NATIVE_VERIFIER_BRIDGE.md)
and [request bytes](../stark/src/verification/native/README.md) describe the current
contract. The feature-enabled node links; no bridge execution, conformance vector,
preprocessing, constraint or proof run has occurred. Tests remain deferred.
