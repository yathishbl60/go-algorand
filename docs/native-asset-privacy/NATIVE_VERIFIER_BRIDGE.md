# Native Go/Rust proof-verification bridge

Updated 2026-10-27. Native spend/recovery snapshots now expose `Verify` methods
that forward their exact plans/context to an optional linked Rust backend. The
feature-enabled `algod` binary builds with the C ABI symbols present. This proves
build/link integration only: no node, bridge call, vector, public setup or proof
has run. Native transaction ingress/execution and all six scenarios remain missing.

## Trusted native caller

Construct `proofbridge.NewVerifier(registryBytes, independentPin, trustedLimits)`
from consensus-approved suite configuration. It owns a fixed registry snapshot,
checks exact registry metadata/digest, and requires ABI1 plus a matching compiled
source ID. Missing backend or mismatched source fails as `ErrUnavailable` before
proof work. No callback, helper process or fallback can report verification success.
There is still no installed/approved suite catalog or actual registry artifact.

Call `nativePlan.Verify(verifier, framedNAPW)`. The snapshot supplies authoritative
Domain/Suite/Group and exact public vectors, including zero-output operations. Go
checks suite equality, outer NAPW binding, bounded lengths and exact NAPB layout.
It builds a small trusted NAPV request. **NAPV is not transaction/witness/RPC data**;
accepting peer-selected requests would let peers choose their own context/keys.
The generic internal Go API still requires callers to establish native provenance.

Rust reconstructs concrete AIRs from that domain and exact public values: ordinary
conversion amounts, recovery amount/current authority, envelope context and assigned
authorities. It checks the main vector's group against the native snapshot and output
contexts against its domain/role/ordinal. Registry selection and typed bundle checks
bind every commitment/kind/role/profile/catalog before actual P3 verification.
Every component must succeed; one rejection rejects the entire bundle.

Success here checks proofs only. Ordinary/PQ signatures, current authority/state,
body-field matching, modes, public credits, funding/work pricing, replay prevention
and atomic ledger application remain separate mandatory native obligations. The
transaction pool/evaluator does not yet route a native transaction through this path.

## Build the linked backend

Core and FFI crates share `internal/assetprivacy/Cargo.lock` and the pinned parent
Rust toolchain. Default workspace member is the core crate; both features are empty
by default. The shared lock preserved existing dependency versions and adds the FFI
package. Core source identity now also covers workspace/FFI sources and the C header.

```sh
bash internal/assetprivacy/stark-ffi/build.sh debug
# Use release instead of debug for an optimized artifact; proving resources remain unmeasured.
go build -mod=readonly -tags algorand_native_privacy,algorand_native_privacy_stark ./cmd/algod
```

The script builds the real static library and installs an ignored host archive in
`stark-ffi/lib/` plus an ignored Go archive-digest marker to invalidate stale link caches. No binary/marker is committed. `NAP_STARK_TARGET_DIR` selects
Cargo output storage. Cross-builds require a Rust target matching Go's GOOS/GOARCH.
Current build/link evidence is Darwin only. The CGO adapter includes Linux/Darwin; other platforms or CGO-disabled builds
fail closed. `algorand_native_privacy` alone also returns unavailable. No consensus
version enables privacy, and linking the backend does not activate it.

## C ABI and memory ownership

The separate FFI crate contains the raw-pointer operations; the core crate retains
`forbid(unsafe_code)`. Three exports exist: `nap_privacy_abi`, source comparison
`nap_privacy_source_v1`, and `nap_privacy_verify_v1(request,len,bundle,len)`. Buffers
must be valid initialized immutable ranges for the entire call. Length/null checks
cannot establish arbitrary foreign-pointer validity; trusted Go owns/pins the ranges.
Rust retains/frees none of them. Go keeps both owned buffers alive until return.

Verification status0 means all actual Rust checks succeeded;1 is pointer/size,
2 artifact failure,3 other rejection,4 backend unwind. Source comparison status0
only establishes source compatibility, never proof validity. Go maps nonzero verify
statuses to rejection. Unwinds are caught before crossing C; aborts/OOM cannot be.

Go deliberately copies the bounded NAPW payload before FFI, isolating Rust's shared
borrow from caller mutation. The caller must keep input stable while Go copies it.
The copy is independently capped by MaxAllocationBytes; Rust decoder allocations
have a separate cap of the same value. Peak includes caller frame + Go payload copy
+ request/vector copies + Rust decoded data + backend/allocator overhead. These caps
are **not one aggregate budget**. No CPU meter, admission scheduler or fee model exists.

The [request reference](../../internal/assetprivacy/stark/src/verification/native/README.md)
specifies all bytes; request max15500, bundle max512 MiB, individual proof max128 MiB.
These guard ceilings are not measured safe node limits. Current proof widths remain
impractical. See [artifact selection](ARTIFACT_SELECTION.md) and the full
[delivery checklist](END_TO_END_STATUS.md). Approved artifacts/pins, native handlers,
other arities/platforms, runtime conformance, security/resource review and tests remain.
