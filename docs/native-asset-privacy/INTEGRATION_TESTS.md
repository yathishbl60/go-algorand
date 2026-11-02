# Native privacy integration validation

Updated 2026-11-02. End-to-end regression tests are now explicitly authorized and
required for delivery. Unit tests may remain deferred during refactoring. Tests
cannot guarantee100% safety; complete security/protocol review remains separate.

## Implemented proof-pipeline regression

Run from the repository root with the pinned Rust toolchain:

```sh
cargo test --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy-compact \
  --test compact_owned_notes --release --locked --offline -- --test-threads=1
```

The release run passed: one serial integration test, six actual proof generations,
21.69s runtime. It uses the published OS-seeded compact configuration, actual
setup/proving and the stock verifier. It covers valid1/2-note ownership, padded
height, amount endpoints, malformed witnesses, changed network/asset/policy/group,
altered proof openings, inverse/program-relay corruption and indexed private-link
swaps. Each swapped link passes ordinary local constraints, produces a nonzero
lookup terminal and must be rejected by the verifier. Debug backend builds check
lookup balance earlier; release execution is required for verifier-path evidence.

Source: `internal/assetprivacy/stark/tests/compact_owned_notes.rs` and its small
support modules. The entire target is guarded by `native-privacy-compact`.
No new dependency, default feature or native proof-kind selection was introduced.

## Backend differential regression

Replace the target above with `--test compact_storage`. The release run passed:
one serial test,0.12s. It compares stock masked coefficients and all random
codeword columns, shifted-coset tiles, budget/range rejection, original-row
recovery and actual full-versus-strip quotient values. These are backend
integration comparisons, not native transaction tests.

## Existing ASA behavior

The existing two-node `TestAssetSend` passed in24.79s with freshly built
`algod`, `goal` and `kmd`, compiled with native privacy and Rust FFI build tags.
The network uses `ConsensusFuture`, where privacy remains disabled. This checks
ordinary ASA transactions, funding and authorization against the changed tree.
It does not test privacy transactions or every existing feature.

```sh
NODEBINDIR=/path/to/fresh/bin TESTDATADIR="$PWD/test/testdata" TESTDIR=/tmp/nap-e2e \
go test -tags algorand_native_privacy,algorand_native_privacy_stark \
  ./test/e2e-go/features/transactions -run '^TestAssetSend$' -count=1 -v -timeout=5m
```

The initial sandbox run was denied localhost socket binding. The authorized
rerun outside the sandbox passed. No consensus flag or production network changed.

## Remaining delivery coverage

These are ownership-component proof-pipeline tests. They do not execute native
transactions, ledger changes, wallets or nodes. The six required real-node
scenarios remain0/6. Native proof acceptance, invalid-proof atomic rollback,
supply/nullifier invariants, replay/catchup, recovery, upgrade compatibility,
resource bounds and existing-feature regressions need actual complete coverage.

The compact storage/quotient path has separate bounded differential measurements;
it is not yet selected by proof generation. Its measurements cannot substitute
for the complete8GiB prover and5MB signed-transaction gates. See
[delivery status](DELIVERY_PROGRESS.md) and [storage evidence](COMPACT_STORAGE_RESULTS.md).
