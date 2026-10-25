# Generation2 native note commitments

Opt-in notetree wraps the generic depth32 tree with global note-creation
uniqueness. Every canonical note commitment may be appended only once per native
network/asset/policy domain, including commitments of already-spent notes. This
combines with commitment-bound NullifierV2 to address serial-reuse freshness;
[the construction and limits](../../../docs/native-asset-privacy/NULLIFIER_FRESHNESS.md)
remain subject to complete verification/runtime/security evidence.

The generic tree and mutable key/eligibility trees are unchanged. Native note
code must use this wrapper, never generic Append/OpenMutable. Store exposes only
Root, Count, Witness, AcceptRoot, CheckNew and AppendAtRound; its underlying tree
is not exported. KV access must be serialized and Apply atomically commit or
roll back the whole batch. This is not a thread-safe or Byzantine-KV interface.

## Records and explicit generation

Generic nodes/metadata/history retain their normal aprv:t:assetBE8 keys and hash
contract. Additional keys use aprv:nt:assetBE8:

- m: generation byte2, domain104(network32/assetBE8/policy64), countBE8, root64;
  exact177 bytes. Root/count must match the current generic tree metadata.
- c followed by commitment64: generation byte2, domain104, commitment64,
  indexBE4; exact173 bytes. It is a permanent creation map, not spent-state data.

Open rejects a nonempty generic tree without the generation2 marker. Wrong
version/domain/root/count or malformed data fail closed. An absent marker is
allowed only for the empty tree. No old experimental state is implicitly migrated
and the guard validates state only. Trusted suite/artifact selection must
separately pin NullifierV2/new schedules and reject old serial-only artifacts.

A generic append/replacement that changes root/count without updating the guard
is detected on reopen. Every existing creation mapping is checked against its
actual current tree leaf/index. Witness also requires the reverse mapping to
name that same index. Missing indexed paths or mismatched commitments fail closed.
The wrapper has no replacement/deletion/pruning API; creation entries survive
spending and note-root-history expiry.

CheckNew rejects zero reserved commitments and any valid existing creation entry.
A missing entry is absence in trusted consensus state, not an authenticated
nonmembership proof or a scan of the entire tree/database. Out-of-band deletion
of a map entry/orphan state is not fully detectable. Correct native execution,
state replay and all future pruning/migration rules must preserve the invariant.

AppendAtRound checks freshness/staleness, stages generic nodes, current metadata,
round history, creation map and generation marker, then calls parent Apply once.
Failure retains no advanced wrapper snapshot and no partial parent writes. Native
COW output append additionally couples the encrypted stream in the same outer
child; later group failure must discard that parent COW as usual. Reopen after
other writes. Current roots/counts remain snapshot getters.

## Native boundaries and pending work

OpenAssetPrivacyTree now returns a read-only native view. Its private mutator
and output-stream loader derive/validate the current immutable policy identity
and use notetree. AppendAssetPrivacyCommitment and AppendAssetPrivacyOutput
therefore both enforce uniqueness. Ordinary/recovered/remainder output planners
also CheckNew before constructing role vectors; append rechecks against then-current
state. A planned vector is no permission token or proof-verification result.

The low-level commitment-only helper does not append ciphertext stream records;
actual native transactions must use the complete atomic output path, enforce
stream/tree count equality, verify every main/role proof, and apply funding/balances.
Generation2 adds two persistent records per append; storage pricing, destruction
cleanup, replay/restart/catchpoint/catchup behavior and artifact/version rollout
remain unfinished. No consensus version enables privacy or its root-history ring.

No tests, vectors, traces, constraint evaluations, proofs, benchmarks or real-node
scenarios have run. Builds, vet, source review and physical limits establish only
those reported properties, not full freshness/security or native acceptance.
