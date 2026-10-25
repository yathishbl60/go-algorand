# Native privacy STARK backend

This Rust library supplies the pinned hiding-proof configuration and integer
conservation, SHA3, note-opening and private-ownership components for native privacy. It is not a complete
transfer prover/verifier. See the [candidate profile](../../../docs/native-asset-privacy/CANDIDATE_PROFILE.md)
for choices, unresolved security arguments and integration obligations.

Default features are empty. Enable `native-privacy` explicitly; this does not
enable the Go build tag or any consensus version. Cargo dependencies are optional
and the default library exposes no privacy API. The Rust crate is not yet linked
into `algod` and cannot authorize a transaction.

Use the pinned Rust 1.99.1 toolchain and checked-in Cargo.lock:

```sh
cd internal/assetprivacy/stark
cargo check --locked --features native-privacy
cargo clippy --locked --features native-privacy --lib -- -D warnings
cargo fmt --check
cargo check --locked --no-default-features
```

These commands compile/analyze code; they do not run tests. An initial dependency
download needs network access. Subsequent checks can add `--offline` after the
locked sources are cached. Generated Cargo.lock is exempt from the user’s
100-line limit; handwritten files and new functions keep the 100/20-line limits.

`prover_config()` returns a configured hiding FRI/STARK backend or an OS entropy
error. Each call obtains fresh entropy for the salt and masking generators. Do not
introduce a deterministic seed in the prover path or reuse private opening data.
The library enforces its masking budget; 512 rows is only the candidate minimum,
not permission to skip checks for the actual composed relation.

`Amounts::witness` builds Boolean amount/carry columns after uint128 conservation.
`amounts::eval` constrains those columns in the composed AIR. Public debit/credit
amounts must come from the bound statement; the same private bits must be used
by note commitments and encryption. Ownership, membership, nullifiers, controls
and disclosure correctness remain separate required constraints.

`Sha3Air` constrains complete fixed-length SHA3-512 hashing. `NoteCommitmentAir`
adds canonical note encoding and public domain/asset/policy binding. `OwnedNoteAir`
links the spending-secret hash to the note owner while retaining input commitments
and owner keys in private columns. See [proof components](../../../docs/native-asset-privacy/PROOF_COMPONENTS.md)
for exact boundaries and the composition still required.

[Private KEM derivation](src/mlkem_derivation_air/README.md) binds canonical key
bytes to K/coins. [Deterministic host encapsulation](src/mlkem_host/README.md)
constructs full K-PKE witness data; host computation is not proof enforcement.
[Staged GCM](src/gcm_vertical/README.md) now constrains all cipher/hash/tag blocks
in one AIR. [Complete single-envelope constraints](src/envelope_air/README.md)
now join KEM/AAD/HKDF/GCM to the same canonical opening/commitment; current
authority, mandatory disclosures and spend/native composition remain unfinished.
[Staged K-PKE arithmetic](src/mlkem_pke_air/README.md) now binds all transforms,
dot products, error/message additions and public ciphertext packing; its matrix,
noise, key and message inputs are now bound by [full bounded encapsulation](src/mlkem_encapsulation_air/README.md).
That relation has only Ckem public; registry authority remains. Public module paths remain stable through gated registries.

The [per-AIR codec and Rust verifier](src/verification/README.md) now provide bounded
serialization and actual backend checking against trusted artifacts/public vectors.
The [outer proof frame](../witness/README.md) still carries opaque bytes. [Typed
bundles](../../../docs/native-asset-privacy/PROOF_BUNDLES.md) join current main/output
plans; the Go/Rust bridge, artifact selection, measured budgets and acceptance
remain unfinished. No proofs or codec round trips have run. The source pin and lockfile make
the implementation reproducible; they do not provide a security audit.
