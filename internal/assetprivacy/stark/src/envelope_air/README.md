# Candidate committed-note envelope relation

`EnvelopeAir` constrains one complete candidate envelope and its canonical note
opening in one AIR. It is feature gated by `native-privacy`; no native verifier
or transaction acceptance path consumes it yet. Current key authority, role
requirements, eligibility and final group bindings remain enclosing obligations.

The relation connects canonical private P, secret m, KEM shared secret/coins,
all nine bounded matrix samples, seven noise samples and public Ckem. The same
P/Ckem feed SHA3-512(Context||P||Ckem). That digest is both the HKDF salt and
GCM AAD. The same K feeds HKDF; its 32-byte output feeds AES-256-GCM. GCM's
512-byte plaintext equals the canonical opening hashed by NoteCommitmentAir.
That hash must equal the commitment in the public context. No intermediate K,
key, opening, recipient routing identifier or digest is published for composition.

## Developer interfaces

Construct `EnvelopeContext::new(EnvelopeFields { ... })` from authoritative
statement fields. Integers use the same big-endian encoding as Go's Context.
`EnvelopeContext::parse([u8;248])` accepts its public byte form: the Go context
with epoch bytes147..155 omitted. Both reject wrong domain/version/role and
required public zero fields. Supply the nonzero epoch as a private input. The
32-byte context nonce differs from GCM's 12-byte nonce. A context alone grants no key
authority. `EnvelopeAir::new(context)` fixes this exact validated statement.

`public_values(&wire_envelope)` takes exactly 1629 bytes and rejects a version
other than 1. It yields the full 1068-scalar verifier vector:

| Public interval, end excluded | Encoding |
| --- | --- |
| 0..248 | Public context bytes with no key epoch, one u8 per scalar |
| 248..792 | All 1088 Ckem bytes, 544 little-endian u16 limbs |
| 792..804 | GCM nonce, twelve u8 scalars |
| 804..1060 | GCM ciphertext, 256 little-endian u16 limbs |
| 1060..1068 | GCM tag, eight little-endian u16 limbs |

The AIR equates all public context bytes to its constructor context. Its children
bind every ciphertext/nonce/tag scalar. Build this vector from the final wire
envelope and trusted statement, never from proof-advertised values or metadata.

`trace(EnvelopeInputs { public_key, message, epoch, nonce, opening })` constructs secret
columns and returns `EnvelopeWitness { trace, public_values, envelope }`.
Key canonicality, bounded matrix admission and canonical opening/commitment
checks occur before the large allocation. Callers provide fresh secret uniform m
and a fresh GCM nonce; the AIR cannot prove entropy quality. Traces contain
secrets and must never be serialized to RPC or included in a public receipt.

`wires(row)` returns exact-width private key/opening/K/epoch and commitment slices.
`value_bits(row)` returns the committed uint64 amount as 64 least-significant
first bits for later conservation composition. These are in-process interfaces.

## Trusted schedule and resources

| Stage | Row interval, end excluded |
| --- | --- |
| Complete bounded KEM | 0..57856 |
| Streamed 2528-byte associated-data hash | 57856..58880 |
| Exact candidate HKDF | 58880..59392 |
| All AES/GHASH operations plus final carry | 59392..64641 |
| Canonical opening and commitment | 64641..65153 |
| Padding with constant root registers | 65153..65536 |

The relation uses 22,885 main and 2059 trusted preprocessed columns. Its 6241
root columns hold t/rho/K/AAD/AES key/opening/commitment/private epoch/inverse.
The remaining 16,644 columns are shared stage scratch. Every root remains constant across all stages
and padding. Each stage binds to those same roots, preventing substitution.
Only unused trailing KEM/GCM padding is removed. The compact GCM frame retains
all 4736 GHASH rows and a final carry row with nested transition disabled.

The schedule must have a trusted profile commitment. Exact height, columns,
frame boundaries, hash variants, role/context and public schema cannot be chosen
by the proof. No private cross-AIR lookup terminal is used. Trusted absorption
END flags occur strictly before each hash frame's final row, so an additional
transition filter is unnecessary. The conservative source degree bound is seven
including nested active/transition selectors; runtime degree/security budgets
and trusted artifact preparation remain unverified.

Raw main storage is 11,998,330,880 bytes; preprocessing adds 1,079,508,992 bytes.
The total is about 12.18 GiB before child schedule scratch, allocator, hiding,
LDE, quotient, commitment and other prover overhead. Bounded writers avoid full
child main matrices. This remains an unmeasured, impractical resource budget;
further layout work is necessary before protocol limits or activation.

The fixed matrix prefix is an additional candidate admission rule; see
[bounded sampling](../../../../../docs/native-asset-privacy/MATRIX_SAMPLING.md).
Mandatory recipient/auditor/controller envelopes must share the required note
opening and satisfy current registry/policy authorization in the final relation.
The epoch appears only in private witness columns; the same nonzero epoch bits
feed AAD and must feed the future authenticated registry relation.
This component alone cannot establish recipient anonymity or a valid transfer.

Builds, strict Clippy, formatting, physical size checks and source reviews are
the current evidence. No vectors, traces, tests, constraint evaluations, proofs,
interoperability runs, node demos or benchmarks have run. No cryptographic
security review, constant-time behavior or zeroization guarantee is established.
