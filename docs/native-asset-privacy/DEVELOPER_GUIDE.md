# Native privacy implementation guide

Updated 2026-10-23. Start with [delivery status](END_TO_END_STATUS.md). This branch
contains disabled protocol components; it has no usable native transaction or
complete proof verifier. There is no supported wallet transfer command yet.

## Build the components

From the node repository root, use the project's configured Go toolchain:

```sh
go build -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/... ./ledger/eval ./ledger/apply ./config
go vet -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/... ./ledger/eval
go build -mod=readonly ./ledger/eval ./ledger/apply ./config
```

Then use the pinned Rust toolchain in `internal/assetprivacy/stark`:

```sh
cargo build --locked --features native-privacy --lib
cargo clippy --locked --features native-privacy --lib -- -D warnings
cargo fmt --check
cargo check --locked --no-default-features
```

The initial Rust dependency download needs network access. Add `--offline` after
caching the locked sources. These commands do not run tests or proofs. Set a
writable `GOCACHE` or Cargo target/cache directory when using a sandbox.

## Choose the correct component

| Need | API or implementation |
| --- | --- |
| Canonical private opening and local encryption | [note](../../internal/assetprivacy/note/README.md) |
| Fixed recipient/auditor/controller envelope | [envelope](../../internal/assetprivacy/envelope/README.md) |
| Immutable policy identity and shrinking powers | [policy records](POLICY_RECORDS.md) |
| Versioned public keys and owner commitments | [key epochs](../../internal/assetprivacy/keys/README.md) |
| Account freeze state and eligibility | [control](../../internal/assetprivacy/control/README.md) |
| Note membership and accepted-root state | [tree](../../internal/assetprivacy/tree/README.md) |
| Atomic encrypted outputs, pages and local recovery | [outputs](../../internal/assetprivacy/outputs/README.md) |
| Shared holder/controller spent marker | [nullifier](../../internal/assetprivacy/nullifier/README.md) |
| Final body/order binding | [statement](../../internal/assetprivacy/statement/README.md) |
| Bounded outer proof frame | [witness](../../internal/assetprivacy/witness/README.md) |
| Zero private inputs, two outputs | [ShieldAir](../../internal/assetprivacy/stark/src/shield_air/README.md) |
| Two private inputs, two outputs | [TransferAir](../../internal/assetprivacy/stark/src/transfer_air/README.md) |
| Two private inputs, no private outputs | [UnshieldAir](../../internal/assetprivacy/stark/src/unshield_air/README.md) |
| Remaining encryption composition | [Exact byte/binding contract](ENCRYPTION_RELATION.md) |
| Private candidate key derivation | [HKDF constraints](../../internal/assetprivacy/stark/src/hkdf_air/README.md) |
| Fixed envelope AEAD constraints | [GCM component](../../internal/assetprivacy/stark/src/gcm_air/README.md) |
| Practical private trace layout | [Scheduling direction](TRACE_ARCHITECTURE.md) |
| Private vertical field multiplication | [GHASH product](../../internal/assetprivacy/stark/src/ghash_vertical/README.md) |
| Complete vertical GHASH stage | [Fixed chain](../../internal/assetprivacy/stark/src/ghash_chain/README.md) |
| ML-KEM private noise polynomials | [PRF/CBD constraints](../../internal/assetprivacy/stark/src/mlkem_noise_air/README.md) |
| All 34 envelope cipher blocks | [Vertical AES](../../internal/assetprivacy/stark/src/aes_gcm_vertical/README.md) |
| Canonical ML-KEM field arithmetic | [Bounded coefficients](../../internal/assetprivacy/stark/src/mlkem_field/README.md) |
| Complete forward/inverse polynomial transforms | [Private NTT](../../internal/assetprivacy/stark/src/mlkem_ntt_air/README.md) |
| Polynomial product in the NTT domain | [Private product](../../internal/assetprivacy/stark/src/mlkem_product_air/README.md) |
| Canonical key/ciphertext packing and message decoding | [Private encoding](../../internal/assetprivacy/stark/src/mlkem_encoding_air/README.md) |
| Private seed-to-matrix polynomial with matched admission | [Composed matrix relation](MATRIX_SAMPLING.md) |

## Required transaction order

1. Read the authoritative network, asset, policy, current keys and eligibility.
2. Create fresh local note secrets, serials and blindings. Keep openings private.
3. Encrypt required disclosures using their exact role, epoch and output context.
4. Finalize every body field, ciphertext and ordinary atomic-group ID.
5. Recompute the group statement from those bodies and the pinned suite.
6. Construct the complete relation's public vector from that same statement.
7. Prove locally, then put its bounded witness outside the transaction bodies.

Steps 6–7 are not available as a complete native path. Existing AIRs omit
encryption and current-control checks. The outer witness frame is not a STARK
decoder. Do not replace either missing piece with a permissive verifier.

Future native execution must check proofs and all current authorization, accepted
roots, distinct fresh nullifiers, public conversion and storage funding. Apply
the resulting public/private balance and state changes in the same group COW;
discard it on failure. Internal persistence helpers do not grant permission.

## Contribution rules for this branch

New handwritten files stay within 100 physical lines; functions within 20,
including signatures and braces. Only generated lockfiles are exempt. Keep both
Go and Rust features opt-in and every consensus version disabled. Update the
status and component contract when behavior changes. Tests/proof runs are
deferred by the user; builds and static analysis establish only those properties.
The complete [north-star scope](IMPLEMENTATION.md) remains the delivery target.
