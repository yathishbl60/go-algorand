# Candidate proof components

Updated 2026-10-23. These components implement portions of the native spending
relation. None is an accepted transaction proof or a complete privacy protocol.
They use the [pinned candidate backend](CANDIDATE_PROFILE.md) and remain opt-in.

## SHA3-512 message constraints

`stark::sha3_air::Sha3Air` accepts a verifier-selected message length up to 1024
bytes. Its private trace includes the message bits and pinned Keccak permutation
columns. The 32 public values are little-endian uint16 limbs of the digest.

The AIR constrains:

- Boolean message bits, constant across trace rows.
- A zero initial capacity and the first padded message block.
- SHA3's 72-byte rate and `0x06`/final `0x80` padding, including a shared last byte.
- Every Keccak round through the upstream `KeccakAir`.
- XOR absorption into the rate and unchanged capacity between message blocks.
- A deterministic block schedule with all active-block flags cleared at the end.
- The first 64 output bytes of the final message permutation against the digest.

Requiring the schedule to finish prevents a shorter trace from avoiding the final
digest constraint. Subsequent padding permutations do not contribute to the hash.
The local generator currently creates 512 rows for all supported message lengths.
That is an implementation shape, not a measured network resource budget.

The 1024-byte limit is for this component, not a protocol payload limit. Message
length comes from the fixed composed relation, never from an untrusted proof.
The future node adapter must independently bound encoded proof dimensions/work.

## Canonical note commitment

`stark::note_air::NoteCommitmentAir` binds the same SHA3-512 domain and 512-byte
opening as the [Go note package](../../internal/assetprivacy/note/README.md).
Its constructor fixes the network, asset and policy from the public statement.
The public values are the output commitment's 32 uint16 limbs.

It constrains the domain/network/version/asset/policy prefix, all reserved padding
bytes, and nonzero owner/account/serial/blinding fields. Nonzero checks use inverse
witnesses for Boolean-bit sums; these sums cannot wrap in the base field. The
value is represented by its exact 64 bits, so the field modulus cannot truncate
the uint64 encoding. `value_bits` shares those bits with the conservation gadget.

A correctly opened commitment does not establish membership or spending authority.
Use this public-commitment interface for output notes. Input commitments must stay
private in shielded spends.

## Private note ownership

`stark::owned_note_air::OwnedNoteAir` composes the note constraints with a second
SHA3-512 relation for the network-bound 32-byte spending secret. It rejects the
zero secret and equates the resulting hash to the owner bytes inside the note.

Both the input note commitment and owner digest are private trace columns. The
component has no public values. A private-output builder connects hash outputs to
those columns; the note commitment is constant across rows and the owner digest
is bound to the note's constant message bits. It does not publish a routing key,
owner key or consumed-note commitment as a shortcut for composition.

`commitment_limbs` exposes the private commitment to the future membership
constraints; `value_bits` exposes the committed private amount to conservation.
These are in-process witness interfaces. Never serialize their traces to node RPC.

## Nullifiers and membership

`NullifiedNoteAir` binds the marker to the exact serial in the owned note.
`MemberNoteAir` composes ownership, nullifier derivation and all 32 membership
levels. Only nullifier and root are public; directions, siblings, input commitment
and intermediate hashes stay private. Node hashing matches the Go tree contract.
See [membership details](../../internal/assetprivacy/stark/src/membership_air/README.md).
The membership matrix remains~641 MB; assembly now retains one stage at a time.
`TransferAir` composes2 member inputs,2 outputs and exact conservation, with
conversion/group context in232 public scalars. [Layout](../../internal/assetprivacy/stark/src/transfer_air/README.md).

## Remaining composition

The complete relation still needs current eligibility/control authorization,
output encryption correctness,
mandatory disclosure, operation-specific arities and native group-verifier wiring.
Fixed2-in/2-out conservation and group-context public inputs are implemented. The owner component is not the controller recovery path;
both paths must eventually consume the same canonical nullifier.

There is no node verifier/decoder, proof transport, Go/Rust native adapter or
transaction dispatch yet. Resource measurements, cryptographic review, vectors
and adversarial validation remain missing. Tests are deferred by user instruction.

## Checks performed

Rust feature-enabled compilation, Clippy with warnings denied, formatting and
authored size checks pass. The full library and disabled-default configuration
are built separately. No proofs or constraint tests were executed. These checks
establish compilation/static properties; they do not establish proof correctness,
cross-language interoperability, soundness, witness privacy or production readiness.
