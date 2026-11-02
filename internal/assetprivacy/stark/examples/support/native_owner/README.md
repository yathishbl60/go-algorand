# Test-only native Owner fixture producer

This example exists for isolated integration tests. Its spending secret is the
known public constant `[51;32]`; it MUST NEVER hold real assets. This is not a
production key generator, wallet API or supported prover CLI. It does not enable
any consensus version or approve the resulting suite.

```sh
cargo build --manifest-path internal/assetprivacy/Cargo.toml \
  -p algorand-asset-privacy-stark --features native-privacy \
  --example native_owner_fixture --release --locked --offline
```

Run the built `examples/native_owner_fixture` binary:

```sh
native_owner_fixture setup OUTPUTDIR
native_owner_fixture prove OUTPUTDIR PUBLIC_CONTEXT_FILE
```

`setup` performs actual public preprocessing and emits a scoped NAPK7 with one
real Owner registration key. It writes `registry.napk` (1774bytes), `profile.bin`,
`implementation.bin` and `digest.bin` (64bytes each). The profile is `[31;64]`.
The fixed setup domain affects no preprocessing schedule. Fresh compiled source
fingerprints require regenerating these files and the matching linked verifier.

The context file must contain exactly168 bytes: network32, asset uint64 little
endian8, policy64 and finalized group64. Zero fields and trailing bytes reject.
The test caller derives these public values from its authoritative transaction.

`prove` loads the setup pins, checks the compiled reference, regenerates the actual
public setup and checks its root against the registry. It constructs the exact
dynamic owner statement and runs `Prepared::prove`, including canonical encoding
and mandatory self-verification with fresh OS-seeded private masks. It writes
`proof.bin`, `bad-proof.bin` and `owner.bin`. The bad proof changes one canonical
base coordinate in the first opened trace value and must produce exactly
`Error::InvalidProof` through the typed verifier before publication. Its framing,
field encoding and artifact identity remain valid, so node tests can exercise
cryptographic rejection. This helper is test-only; it exposes no production decoder.
Outputs use exclusive creation; existing files are never replaced. Use a fresh
output directory for another fixture.

The proof cap is4,900,000 bytes, decoder cap64MiB and raw trace cap16MiB. These
are fixture bounds, not release resource calibration. This command does not
sign/broadcast a transaction, authenticate a PQ account or apply ledger state.
Go/node integration tests must exercise those remaining boundaries independently.
