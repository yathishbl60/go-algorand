# Candidate proof components

Updated 2026-10-24. These components implement portions of the native spending
relation. None is an accepted transaction proof or a complete privacy protocol.
They use the [pinned candidate backend](CANDIDATE_PROFILE.md) and remain opt-in.

## SHA3 message constraints

`Sha3Air` accepts a verifier-selected variant and length through 4096 bytes.
Its private trace includes message bits and Keccak permutation columns.
SHA3-512 has 32 public uint16 digest limbs; SHA3-256 has 16, both little endian.

The AIR constrains:

- Boolean message bits, constant across trace rows.
- A zero initial capacity and the first padded message block.
- SHA3's 72/136-byte rates and `0x06`/final `0x80` padding, including shared bytes.
- Every Keccak round through the upstream `KeccakAir`.
- XOR absorption into the rate and unchanged capacity between message blocks.
- A deterministic block schedule with all active-block flags cleared at the end.
- The selected 64/32 output bytes of the final permutation against the digest.

Requiring the schedule to finish prevents a shorter trace from avoiding the final
digest constraint. Subsequent padding permutations do not contribute to the hash.
`trace_rows()` gives 512–2048 rows by variant/length; existing note hashes stay at 512 rows.
See [hash composition requirements](../../internal/assetprivacy/stark/src/sha3_air/README.md).

The 4096-byte limit is for this component, not a protocol payload limit. Message
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
mandatory disclosure/spend integration and native verifier wiring.
[ShieldAir](../../internal/assetprivacy/stark/src/shield_air/README.md) has zero private inputs/two outputs;
[UnshieldAir](../../internal/assetprivacy/stark/src/unshield_air/README.md) has two inputs/zero outputs.
Neither uses dummy notes. [AES byte constraints](../../internal/assetprivacy/stark/src/aes_air/README.md)
[GCM](../../internal/assetprivacy/stark/src/gcm_air/README.md) and private [HMAC](../../internal/assetprivacy/stark/src/hmac_air/README.md)/[HKDF](../../internal/assetprivacy/stark/src/hkdf_air/README.md) exist; [bounded KEM](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/README.md) exists; [single-envelope/note constraints](../../internal/assetprivacy/stark/src/envelope_air/README.md) now join them.
[Private key-record constraints](../../internal/assetprivacy/stark/src/view_key_air/README.md) now match the native current-key registry. [Registered recipient](../../internal/assetprivacy/stark/src/authorized_recipient_air/README.md) joins current viewing/spending and effective-eligibility membership to the same private envelope account. [Assigned disclosures](PROOF_PLANNING.md) exist; [Ordinary input/group composition](SPENDING_COMPOSITION.md) now exists; [Recovery/remainders](RECOVERY.md) now have separate constraints; [V2 markers/creation uniqueness](NULLIFIER_FRESHNESS.md) are implemented; native verification/runtime evidence remain.
The owner component is not the controller recovery path;
both paths must eventually consume the same canonical nullifier.

[Opaque witness framing](../../internal/assetprivacy/witness/README.md) bounds outer bytes only.
[Rust per-AIR checking](VERIFICATION_BOUNDARY.md) and [typed bundles](PROOF_BUNDLES.md)
exist; [typed selection](ARTIFACT_SELECTION.md) exists; approved pins, bridge and native dispatch remain unfinished.
Resource measurements, cryptographic review, vectors
and adversarial validation remain missing. Tests are deferred by user instruction.

## Checks performed

Rust feature-enabled compilation, Clippy with warnings denied, formatting and
authored size checks pass. The full library and disabled-default configuration
are built separately. No proofs or constraint tests were executed. These checks
establish compilation/static properties; they do not establish proof correctness,
cross-language interoperability, soundness, witness privacy or production readiness.
