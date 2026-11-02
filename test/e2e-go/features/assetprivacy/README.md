# Native Owner registration regression

This target requires actual Rust proving and native-capable node binaries. It
never substitutes a verifier or enables a shipped consensus version. It creates
an isolated two-node network with a public-only policy and a scoped Owner suite.
Its spending secret is deliberately public test data; never use it for real assets.

Build from the repository root using the pinned Rust/Go toolchains:

```sh
CARGO_TARGET_DIR=/tmp/nap-rust-target cargo build \
  --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy \
  --example native_owner_fixture --release --locked
NAP_STARK_TARGET_DIR=/tmp/nap-rust-target \
  bash internal/assetprivacy/stark-ffi/build.sh release
mkdir -p /tmp/nap-node-bin
go build -tags algorand_native_privacy,algorand_native_privacy_stark \
  -o /tmp/nap-node-bin/ ./cmd/algod ./cmd/goal ./cmd/kmd
```

Run with localhost process/socket access:

```sh
NAP_OWNER_FIXTURE_BIN=/tmp/nap-rust-target/release/examples/native_owner_fixture \
NODEBINDIR=/tmp/nap-node-bin TESTDATADIR="$PWD/test/testdata" TESTDIR=/tmp/nap-owner-e2e \
go test -tags algorand_native_privacy,algorand_native_privacy_stark \
  ./test/e2e-go/features/assetprivacy -run '^TestNativeOwnerRegistration$' \
  -count=1 -v -timeout=5m
```

The named target fails if the fixture binary is missing. Public artifact setup
must match the linked verifier's current source fingerprint. Rebuild both after
Rust source changes. All current production consensus defaults remain disabled.

Coverage includes real artifact generation/pinning, Falcon authorization, rejection
of private policy adoption under a partial suite, finalized group/fee binding,
actual proof generation/canonical encoding, native node admission and block
acceptance. A canonical changed opening must produce cryptographic rejection;
the preceding payment and key state must remain unchanged. The original valid
group then registers epoch1. The test checks exact wire sizes, peer state agreement
and owner state/public supply after a peer restart using the same database.

This is key registration, not a private asset transfer, wallet restoration or a
complete privacy scenario. The full north-star scope and six real-node privacy
scenarios remain tracked in
[delivery status](../../../../docs/native-asset-privacy/DELIVERY_PROGRESS.md).
