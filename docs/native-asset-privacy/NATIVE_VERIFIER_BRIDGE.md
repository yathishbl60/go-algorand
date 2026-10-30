# Native Go/Rust proof-verification bridge

Updated 2026-10-30. Current-consensus shield/spend/recovery helpers select an
optional linked Rust backend. A feature-enabled algod binary builds with the C
symbols present. This is build/link evidence only: no bridge call, public setup,
node, vector or proof has run. New shield Rust/FFI and linked algod builds passed.

## Trusted native caller

Use VerifyAssetPrivacyTransaction and its shield/spend/recovery plans. They rebuild
current plans and select pins/budgets from authoritative policy and current
[consensus suite catalog](NATIVE_SUITE_CATALOG.md). No caller-selected verifier,
Pin or Limits argument exists on this path. All current catalogs are empty and
privacy remains disabled. There is no approved/installed registry artifact.

Suite.Verifier calls proofbridge.NewVerifier with the selected values. It owns
an exact1036-byte registry, checks metadata/digest and requires ABI2 plus a supported
implementation reference. The legacy Implementation/source wire field holds this
reference; it is not necessarily the current compiled fingerprint. The compiled
historical compatibility array is currently empty, so only today's fingerprint
is supported. Missing/incompatible backend fails closed before proof work.

Generic nativePlan.Verify(verifier,framedNAPW) is a lower-level cryptographic
helper. Its immutable snapshot supplies Domain/Suite/Group and exact vectors,
including zero-output operations. Go checks suite equality, NAPW profile/group,
bounds and every NAPB descriptor, then constructs a small trusted NAPV request.
**NAPV is not transaction/witness/RPC data** and peers cannot select its context,
registry or budgets. NAPV2 has a1414-byte header with NAPK2/1036-byte registry;
old unapproved v1 bytes/pins reject. No historical approved references exist.

Rust derives concrete AIRs from native domain/values: conversion amounts, recovery
current authority, envelope contexts and assigned disclosure targets. It checks
main group binding and output domain/role/ordinal. Typed registry/bundle selection
binds all required kinds, commitments and public values; actual P3 verification
must succeed for every component, including staged ShieldGroupAir kind3.

Proof checking writes no state and grants no transaction authority. Stateless
PQ-envelope/group/witness checks exist; current ledger consent, modes, balances,
funding, replay protection and atomic application remain mandatory. [Native
integration](NATIVE_TRANSACTION_INTEGRATION.md) remains incomplete; helpers do not accept transactions.

## Build the linked backend

Core/FFI crates share internal/assetprivacy/Cargo.lock and the pinned toolchain.
Their features are empty by default; the core crate is the default workspace member.

```sh
bash internal/assetprivacy/stark-ffi/build.sh debug
go build -mod=readonly -tags algorand_native_privacy,algorand_native_privacy_stark ./cmd/algod
```

Use release for an optimized archive; proving costs remain unmeasured. The script
installs an ignored host archive and Go archive-digest marker to prevent stale
link caches; neither is committed. NAP_STARK_TARGET_DIR selects Cargo storage.
Cross-builds need matching Rust and Go targets. Current link evidence is Darwin;
Adapters target Linux/Darwin; other platforms, CGO-disabled builds and the
privacy tag alone fail closed. Linking never activates a consensus version.

## ABI2 and failure classification

The separate FFI crate owns raw pointers; the core keeps forbid(unsafe_code).
Exports are nap_privacy_abi, nap_privacy_source_v1 and nap_privacy_verify_v1.
The source function tests supported reference compatibility, not proof validity.
_v1 function names remain under ABI2; callers check ABI2 before using them.

| Verify status | Meaning | Go outcome |
| --- | --- | --- |
| 0 | All selected actual proof checks succeeded | nil, without transaction authority |
| 1 | Invalid foreign range/trusted native request | ErrUnavailable |
| 2 | Trusted artifact installation/selection failure | ErrUnavailable |
| 3 | Invalid/malformed proof or deterministic proof quota failure | ErrRejected |
| 4 | Caught backend unwind | ErrUnavailable |
| 5 | Actual fallible allocation failure | ErrResource |
| Unknown | Unsupported backend outcome | ErrUnavailable |

Registry installation failures in the authoritative native wrapper are local
ErrUnavailable errors. Never classify local resource/backend faults as invalid-peer
evidence. Catching unwinds cannot recover allocator aborts or panic-abort builds.

Go owns/pins valid initialized immutable ranges until return; Rust retains/frees
none of them. Length/null checks cannot establish arbitrary foreign-pointer validity.
The input frame must stay stable during Go's copy; KeepAlive preserves both owned
Go buffers until FFI returns. No C allocation or caller-owned payload borrow escapes.

## Resource limits

Go payload-copy size is separately capped by MaxAllocationBytes. Rust proof
parsing uses the same numeric cap as a deterministic charge quota under a fixed
64-bit model: Value8, Challenge40, digest64, Vec/Cap24, input/FRI-phase opening72.
Child elements are charged separately. Quota exhaustion is deterministic proof
rejection; actual reservation failure is local ErrResource. Actual heap use,
backend copies, symbolic constraints, allocator metadata and CPU remain outside it.

Peak includes caller NAPW + owned Go payload + small request/vector copies + one
Rust-decoded proof + backend overhead. These are additive, not one aggregate cap.
Rust verifies components sequentially. Coarse proof-count/fee formulas exist; measured node
budget exists. [Request bytes](../../internal/assetprivacy/stark/src/verification/native/README.md)
remain max15582, bundle512MiB, proof128MiB. These are guards, not practical limits.
Approved artifacts, historical-suite retention, other modes/arities,
runtime conformance and security/resource evidence remain [delivery gates](END_TO_END_STATUS.md).
