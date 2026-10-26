# Candidate privacy profile, revision 2

Updated 2026-10-25. Implementation choice, not a frozen network suite or security
approval. The user authorized implementation without supplying a reviewed profile.
The 128-bit post-quantum target remains; this document does not establish it.

## Pinned proof backend

- Plonky3 revision `1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f`, dated 2026-10-22.
- Upstream license: MIT OR Apache-2.0; our adapter: AGPL-3.0-or-later.
- Rust 1.99.1; all transitive dependencies pinned by the checked-in Cargo.lock.
- Goldilocks prime `2^64 - 2^32 + 1`; quintic challenge extension `X^5 - 3`.
- Hiding FRI with hiding Merkle commitments on both base and extension fields.
- SHA3-512, distinct domains for leaves, internal nodes and transcript hashes.
- Merkle digests: eight little-endian uint64 words; five random field-element
  salts per leaf. Transcript uses the full 64-byte hash output.
- FRI blowup 8, binary folds, 192 queries, final polynomial length 1; no grinding.
- Five random codewords; fresh OS-seeded generators for salts and trace masks.
- Minimum proposed trace height 512; the composed relation must satisfy the
  library's actual masking budget for every opening and quotient chunk.

The pinned configuration now has a [per-AIR Rust codec/verifier](VERIFICATION_BOUNDARY.md).
There is no node verifier adapter, approved registry artifact/pins or accepted native
proof encoding yet. Parameters are not transaction-selectable. Do not use example
testing parameters, deterministic prover seeds, non-hiding commitments or a
pairing-based compression stage.

[Pinned hiding-PCS source](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/fri/src/hiding_pcs.rs)
defines the enforced disclosure budget and fresh-opening restrictions. The
[pinned security module](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/uni-stark/src/security.rs)
computes relation-dependent bounds; query count alone is not a security estimate.
The full relation, quantum Fiat-Shamir analysis and witness-privacy composition
remain incomplete. No independent review covers this adapter.

## Integer relation component

`stark::amounts` constrains two private input and two private output uint64 values,
plus public debit/credit values supplied by the bound statement. Every private
bit is Boolean. Two 32-bit limb equations use a signed carry restricted to -2..2.
Their integer magnitudes stay below the base-field modulus, preventing modular
wraparound from satisfying a false conservation equation. Witness construction
uses uint128 sums. These bits must also feed the note/encryption constraints.

This component does not establish membership, ownership, nullifiers, encryption
correctness, policies or group binding. There is no native spend acceptance path.
Input/output counts and final resource limits await the complete relation.

## Note openings and ownership

The [note API](../../internal/assetprivacy/note/README.md) defines a canonical
512-byte opening with 241 meaningful bytes and zero padding. SHA3-512 domains
separate commitments, spending-key commitments and nullifiers. Private spending
secrets, serials and blindings each use 32 fresh random bytes. Nullifiers bind the
network, asset, policy, serial and canonical commitment under NullifierV2.
[Generation2 note creation](NULLIFIER_FRESHNESS.md) prevents commitment reuse;
holder/controller paths consume the same marker. Note-opening, ownership, nullifier and membership AIRs are implemented; complete
transfer composition and resource feasibility remain pending. See [proof components](PROOF_COMPONENTS.md).

## Note-encryption component

The [Go envelope package](../../internal/assetprivacy/envelope/README.md) uses
standard-library ML-KEM-768, HKDF-SHA3-512 and AES-256-GCM. Each fixed-size envelope
contains a version byte, 1088-byte KEM ciphertext, random 12-byte GCM nonce,
512-byte encrypted payload and 16-byte tag: 1629 bytes total.

Canonical associated data binds the genesis hash, asset, policy digest, key
epoch, recipient/auditor/controller role, transaction nonce, output index and
commitment. The encryption key and KEM ciphertext are bound into the KDF and AEAD
context. Recipient public keys are not carried as routing metadata.

This is a KEM/DEM candidate, not a recipient-anonymity proof. The complete proof
relation must constrain encryption of the exact committed note and all required
role disclosures. It must bind finalized group bodies after ciphertext creation.
[Staged GCM](../../internal/assetprivacy/stark/src/gcm_vertical/README.md) constrains
the complete AEAD; [single-envelope/note constraints](../../internal/assetprivacy/stark/src/envelope_air/README.md) now compose KEM/KDF/GCM; current control/spend integration remains.
Fresh randomness, fixed payload size and omission of public routing keys do not
alone establish shielded anonymity.

Candidate viewing-key admission fixes every matrix entry to at most 16 SHAKE128
rate blocks. All nine ordered coordinate entries must contain 256 accepted
coefficients. `CheckPublicKey` enforces this before standard key parsing;
otherwise standard-valid keys fail admission if a prefix exhausts. Coverage of
all standard keys is unproven. The same fixed bound is used by the composed
private [matrix relation](MATRIX_SAMPLING.md); no transaction-selected bounds.
The [bounded KEM relation](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/README.md) implements this rule; runtime validation and review remain.

[NIST FIPS 203](https://csrc.nist.gov/pubs/fips/203/final) defines ML-KEM;
[SP 800-227](https://csrc.nist.gov/pubs/sp/800/227/final) discusses KEM use.
Final profile work must track errata and justify this complete composition.

## Next integration work

Compose the implemented membership/ownership/nullifier constraints with current
spending/control authorization;
proof-constrained encryption and disclosures; exact statement/group binding;
bounded proof decoding and deterministic verification; then native dispatch,
storage funding and lifecycle transitions. The feature remains disabled in every
consensus version. Build success is not end-to-end or security validation.
