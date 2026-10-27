# Opt-in native verifier C boundary

This crate belongs to the parent Rust workspace and shares its Cargo.lock and
Rust1.99.1 toolchain with the core verifier. Default features are empty. Feature
`native-privacy` links the real core verifier; without it no C ABI exports exist.
The core crate remains unsafe-free. Only this small wrapper borrows raw C ranges.

```sh
bash internal/assetprivacy/stark-ffi/build.sh debug
# release is the default mode when no argument is supplied
```

The script installs a host static archive under ignored `lib/`; artifacts are not
committed. An ignored Go archive-digest marker makes changed Rust archives invalidate Go link caches. Set NAP_STARK_TARGET_DIR for Cargo output storage. A cross-build must
set a matching Rust CARGO_BUILD_TARGET and Go GOOS/GOARCH; mismatched archives do
not work. The Go adapter currently supports Linux/Darwin with CGO and both privacy
tags; unsupported/unlinked builds reject verification as unavailable.

`include/nap_privacy.h` defines ABI1. `nap_privacy_abi()` returns the ABI number.
`nap_privacy_source_v1(expected,64)` compares the independently expected source ID.
`nap_privacy_verify_v1(request,len,bundle,len)` borrows trusted native request data
and the opaque proof bundle for actual checking. Buffers must be initialized, valid,
immutable, and alive for the entire call; Go supplies owned pinned byte arrays.
Null/length guards cannot validate arbitrary foreign memory. Rust never retains
or frees those ranges. No Rust-owned allocation or pointer crosses back to Go.

Only verification status0 means all proofs passed. Status1 rejects bad sizes/ranges,
2 artifacts,3 other errors,4 unwinds. Source status0 means compatibility only.
Unwinds are caught at the C boundary; panic-abort/OOM and invalid caller pointers
cannot be recovered this way. No callback or configurable success implementation
exists. The wrapper performs no ledger writes or signature authorization.

The [native bridge contract](../../../docs/native-asset-privacy/NATIVE_VERIFIER_BRIDGE.md)
and [trusted request schema](../stark/src/verification/native/README.md) define the
caller trust boundary. Do not accept peer-supplied native requests. The source
fingerprint includes this crate's manifest/sources/header and the shared workspace
lockfile. It does not attest a compiler or approve cryptography.

The core/FFI crates and tagged node build/link; symbols were inspected. No C ABI
execution, proofs, setup, vectors or tests have run. Approved keys/pins, native
transaction handlers, other platforms, runtime validation and practical resource
limits remain unfinished. No consensus version enables this code.
