# Candidate native retirement record

This opt-in codec uses `algorand_native_privacy`. It stores public retirement
facts and authenticates no account, proof, supply or deletion request. Current
consensus versions remain disabled, unpriced and without approved suite entries.

`Key(asset)` is ASCII `aprv:x:` followed by assetBE8; asset must be nonzero.
The exact177-byte value is version1, manager32, policy64, suite64, createdBE8 and
cleanupEpochBE8. Manager/policy/suite and epoch must be nonzero. Encoding returns
owned bytes; decoding copies arrays and rejects unknown versions/trailing bytes.
The ledger rejects future Created rounds and derives all initial fields from
current authenticated state. Epoch begins at1 and advances on nonempty cleanup.

The record is indexed and creator-funded like other native data. While it exists,
the ledger blocks new asset activity and native data writes, allowing only its
authenticated cleanup advancement and final destruction. The full
[retirement/cleanup contract](../../../docs/native-asset-privacy/RETIREMENT_CLEANUP.md)
defines current PQ authority, zero-supply/full-holding checks, protected records,
bounded batches, work fees, epoch compare-and-swap and final counter release.

No tests, vectors, runtime transitions, proofs or node scenarios have run.
Compilation and static checks do not establish lifecycle or security correctness.
