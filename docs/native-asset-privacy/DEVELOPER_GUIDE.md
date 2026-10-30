# Native privacy implementation guide

Updated 2026-10-30. Start with [delivery status](END_TO_END_STATUS.md). This branch
contains disabled value execution and NAPL1 lifecycle/admin-possession registration.
No protocol is activated and no wallet transfer command is supported.

## Build the components

From the node repository root, use the project's configured Go toolchain:

```sh
go build -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/... ./data/transactions/... ./ledger/eval ./ledger/apply ./config
go vet -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/... ./data/transactions/... ./ledger/eval
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
| Policy identity, shrinking powers and NAPL1 lifecycle ingress | [policy records](POLICY_RECORDS.md), [ingress contract](LIFECYCLE_INGRESS.md) |
| Recipient envelope with current viewing/spending membership | [AuthorizedRecipientAir](../../internal/assetprivacy/stark/src/authorized_recipient_air/README.md) |
| Required output roles and expected verifier vectors | [Proof planning](PROOF_PLANNING.md) |
| Current key records and stable-index authorization root | [Registry and native interfaces](KEY_AUTHORIZATION.md) |
| Key epochs and new Falcon admin possession | [keys](../../internal/assetprivacy/keys/README.md), [NAPR](../../internal/assetprivacy/registrationproof/README.md) |
| Atomic freeze snapshots and private recipient eligibility | [Current eligibility](ELIGIBILITY.md) |
| Unique native note creation, read-only paths and root history | [Generation2 note tree](../../internal/assetprivacy/notetree/README.md) |
| Atomic encrypted outputs, pages and local recovery | [outputs](../../internal/assetprivacy/outputs/README.md) |
| Shared holder/controller spent marker | [nullifier](../../internal/assetprivacy/nullifier/README.md) |
| Final body/order binding | [statement](../../internal/assetprivacy/statement/README.md) |
| Bounded outer proof frame | [witness](../../internal/assetprivacy/witness/README.md) |
| Native zero-input/two-output shield | [ShieldGroupAir and native plans](NATIVE_SHIELD.md) |
| Two ordinary eligible inputs with Transfer2/Unshield0 outputs | [Staged spending](SPENDING_COMPOSITION.md) |
| Issuer recovery and frozen-owner remainder | [Recovery contracts](RECOVERY.md) |
| Native body/admission, current PQ authority, proof routing and atomic application | [Integration](NATIVE_TRANSACTION_INTEGRATION.md), [execution](NATIVE_EXECUTION.md), [storage](NATIVE_STORAGE.md) |
| Consensus suite metadata, references and admission | [Native suite catalog](NATIVE_SUITE_CATALOG.md) |
| Single committed-note envelope | [EnvelopeAir typed context and witness API](../../internal/assetprivacy/stark/src/envelope_air/README.md) |
| Private candidate key derivation | [HKDF constraints](../../internal/assetprivacy/stark/src/hkdf_air/README.md) |
| Exact native proof checking and ABI2 failures | [Authoritative bridge entrypoints](NATIVE_VERIFIER_BRIDGE.md) |
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
| Canonical key, shared secret and encryption coins | [Private KEM derivation](../../internal/assetprivacy/stark/src/mlkem_stream_derivation_air/README.md) |
| Full deterministic KEM witness construction | [Host encapsulation](../../internal/assetprivacy/stark/src/mlkem_host/README.md) |
| All AES/GHASH blocks and final tag in one AIR | [Staged GCM](../../internal/assetprivacy/stark/src/gcm_vertical/README.md) |
| Matrix/key NTT dot products | [Four-dot relation](../../internal/assetprivacy/stark/src/mlkem_dot_air/README.md) |
| Private u/v to public ciphertext | [Ciphertext packing](../../internal/assetprivacy/stark/src/mlkem_ciphertext_air/README.md) |
| Complete conditional K-PKE arithmetic | [Staged encryption relation](../../internal/assetprivacy/stark/src/mlkem_pke_air/README.md) |
| Canonical key/m to shared secret/coins and public KEM ciphertext | [Full bounded encapsulation](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/README.md) |

## Required transaction order

1. Read the authoritative network, asset, policy, current keys and eligibility.
2. Create fresh local note secrets, serials and blindings. Keep openings private.
3. Encrypt required disclosures using their exact role, epoch and output context.
4. Reserve a conservative proof/PQ-byte fee upper bound, then finalize bodies/group IDs.
5. Recompute the group statement from those bodies and the pinned suite.
6. Construct the complete relation's public vector from that same statement.
7. Prove locally, then put its bounded witness outside the transaction bodies.

Confirm NAPL adoption before deriving its inclusion-round policy identity. Steps6–7
have no supported end-to-end builder; viewing possession, approved artifacts,
shield and runtime evidence remain absent. Spend/Recovery select current catalog pins.

The [native evaluator](NATIVE_EXECUTION.md) checks current PQ authorization, roots,
fresh markers, public conversion, proofs, fees and creator storage funding. Its
child-COW applies verified public/private changes atomically. Internal persistence
helpers alone grant no permission. NAPR admin possession also binds the final fee/group.

## Contribution rules for this branch

New handwritten files stay within 100 physical lines; functions within 20,
including signatures and braces. Only generated lockfiles are exempt. Keep both
Go and Rust features opt-in and every consensus version disabled. Update the
status and component contract when behavior changes. Tests/proof runs are
deferred by the user; builds and static analysis establish only those properties.
The complete [north-star scope](IMPLEMENTATION.md) remains the delivery target.
