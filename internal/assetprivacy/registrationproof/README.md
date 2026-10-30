# Candidate Falcon admin registration possession

This opt-in Go package frames and verifies possession of a new Falcon1024
administrative key. It requires `algorand_native_privacy`; every consensus version
still disables native privacy. It does not authorize the current account, assign
issuer powers, register a key by itself or prove viewing/spending-key possession.

## Typed message and framing

`Message(group)` returns exactly the ASCII bytes:

`Algorand/AssetPrivacy/Candidate1/Registration/Falcon1024/`

followed immediately by the nonzero 64-byte group-context digest. The caller must
reconstruct that digest with `statement.Bind` over the complete finalized ordinary
group, target index, authoritative genesis and suite. That digest commits the
registration operation, exact new key, role/account, asset/policy, expected epoch
and all other transaction fields. A proof-supplied digest grants no authority.

The candidate proof frame is:

| Byte range | Meaning |
| --- | --- |
| 0..4 | ASCII NAPR |
| 4..5 | Version1 |
| 5..6 | Kind1, Falcon1024 admin possession |
| 6..8 | Signature length uint16BE |
| 8..end | Exact compressed Falcon1024 signature |

The signature must have 1..`crypto.Falcon1024MaxSignatureSize` bytes; the frame
must end exactly after them. Unknown versions/kinds, empty or oversized signatures
and trailing bytes fail. These are framing bounds, not evidence of possession.
There is no interpretation as an existing NAPW transfer proof or callback result.

`Check` validates framing without copying. `Encode` returns an owned exact frame;
it does not generate or verify a signature. `Decode` validates the complete frame
before making an owned bounded signature copy. Keep inputs stable until return.

## Verification and authority boundary

`Verify(publicKey,group,frame)` checks canonical Falcon1024 public-key syntax,
decodes the frame, constructs the typed message and invokes the repository's
existing `crypto.Falcon1024Verifier.VerifyBytes`. It copies the fixed public key
and verifies the compressed signature under that exact new key. Invalid signature
errors wrap `crypto.ErrPQFalcon1024SigInvalid`; there is no permissive fallback.

The administrative roles share the same material encoding. The helper uses
ManagerAuthority only for that encoding check; it does not establish the role or
manager identity. The ledger must restrict registration to the accepted admin
roles, match the account to the top-level sender/current issuer principal, require
actual current PQ account authorization, compare current policy/suite and epoch,
and derive the record domain/Created from authoritative state. The new key's
signature cannot replace those checks or restore relinquished issuer capability.

The caller must require Falcon1024 enabled under the current consensus version,
charge work and complete signed-byte fees, enforce creator storage funding, and
commit history/pointer/authorization/eligibility changes atomically. Never expose
this verifier as a callback whose success accepts arbitrary registration state.
RecipientSpend and ML-KEM viewing registrations need their separate possession
relations; this package accepts neither key kind.

## Evidence limits

Tagged compilation and vet passed; authored physical line checks passed.
No signatures, vectors, tests, cryptographic proofs, node operations or runtime
verification have been generated or executed. This source integration does not
establish candidate security, protocol readiness or end-to-end delivery.
