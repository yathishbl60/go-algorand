# Commitment-bound markers and unique native note creation

Updated 2026-10-25. The serial-reuse correction is implemented in Go/Rust and native
storage, behind disabled flags. Source/build evidence exists; no runtime proofs,
adversarial tests or completed security review establish the full guarantee yet.

## Corrected construction

The old candidate hashed network/asset/policy/serial only. Reusing a consumed
serial in a different canonical output could give it an already-spent marker.
Honest randomness and input freshness alone did not constrain malicious outputs.

The current exact245-byte preimage is:

1. ASCII Algorand/AssetPrivacy/Candidate1/NullifierV2/ (45 bytes).
2. Genesis32, assetBE8 and immutable policy64 (prefix total149).
3. Private serial32 (bytes149..181).
4. Canonical private note commitment64 (bytes181..245).

SHA3-512 yields the64-byte marker; every hash output, including zero, is valid.
The complete canonical commitment already binds owner/account/value/serial/blinding
and network/asset/policy. Go Note.Nullifier(network) keeps its API and validates/
computes that commitment. Rust's shared nullifier_v2 helper fixes the same encoding.
Older NullifiedNoteAir, ordinary SpendableNoteAir and RecoveryNoteAir all bind the
same PRIVATE digest into that hash. No input commitment or future output marker
is published. The older Rust nullifier helper now returns Option after canonical
validation rather than hashing an arbitrary malformed opening.

## Native uniqueness supplies the other half

[notetree](../../internal/assetprivacy/notetree/README.md) atomically indexes every
created commitment and rejects reuse for that entire native domain. Spent notes
and expired note roots do not remove the creation record. New native output plans
check this invariant and append rechecks it at application. Both native append
helpers use the guarded wrapper; OpenAssetPrivacyTree exposes read methods only.
The private opener also validates the supplied policy against current declaration.

Conditioned on the hash/binding assumptions, two distinct canonical commitments
produce different marker preimages even if serials repeat. A globally new accepted
commitment cannot deliberately reproduce a previously accepted note's marker
without a hash collision. Recreating the exact old commitment is instead rejected
by native creation uniqueness. Same-note holder and recovery actions still derive
the identical marker and conflict in the same shared native spent namespace.

This argument also requires complete proof verification, valid native marker
consumption, trusted suite/artifact selection and intact deterministic consensus
state. A map lookup is no private nonmembership proof or universal corruption
detector. There is no claim about arbitrary out-of-band KV deletion or Byzantine
storage. Every future pruning/replay/migration path must preserve the creation map.

## State/profile generation and resources

Generation2 note-state guard records exactly177 bytes: byte2, domain104,
countBE8, root64. The permanent creation record is173 bytes: byte2, domain104,
commitment64, indexBE4. Both are under aprv:nt:assetBE8; generic aprv:t: tree
metadata/nodes/history remain unchanged. Nonempty unguarded trees are rejected,
not silently migrated. Root/count changes without guard updates fail on reopen.
Key/eligibility mutable registries retain their separate namespaces and behavior.

The marker's distinct V2 domain, generation2 guard and revised trusted schedules
must be pinned together in a new profile/artifact. No old serial-only artifact
may remain accepted under an unchanged old suite identifier. Native allowed-suite
registration and the [Rust/native bridge](VERIFICATION_BOUNDARY.md) remain unfinished. Existing
experimental state/proofs have no automatic migration or compatibility promise.

The four-block marker hash needs97 rows, including a final flag-clear pad. Ordinary
input active rows rise7356->7380; transfer outputs now end at15784 within16384.
Recovery input rises3690->3714 and partial-private outputs end at4738 within8192.
Main widths/heights/public schemas and conservative degree bounds are unchanged;
actual quotient/hiding/composed security and practical resource budgets remain open.

Source audits found no native note-store open using generic tree.Open/OpenMutable:
private native opens/loaders use notetree. Ordinary and both recovery recipient
planners apply the same creation check. Features remain opt-in; no consensus
version enables privacy. Tests, vectors, traces, constraint evaluations, proofs,
node/replay scenarios and benchmarks remain deferred by user instruction.
Full verifier/authentication/transaction/SDK delivery and the north star remain.
