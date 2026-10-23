# Candidate private note API

Build with `-tags algorand_native_privacy`. This package creates and encodes local
wallet/prover openings; it cannot authorize or submit native transfers.

1. Generate a private spending secret with `GenerateSpendingSecret`. Derive its
   network-bound `Owner` commitment. Keep the secret in encrypted local storage,
   separate from recipient/auditor encryption keys.
2. Call `New(asset, policyDigest, owner, account, value)`. The wallet generates
   independent serial and commitment-blinding randomness. The account is private
   note data; confidential-mode proofs will disclose and authenticate it.
3. Compute `Commitment(genesisHash)` and put it in the output encryption context.
4. Use `note.Encrypt(publicKey, context)` for recipient/auditor/controller copies.
   It checks asset, policy and commitment binding before encrypting.
5. A receiver calls `note.Decrypt(key, context, ciphertext)` locally. This checks
   AEAD authentication, canonical encoding and the advertised commitment.

Decryption success is not proof of an accepted payment. The wallet must still
verify block confirmation, the full spending proof and whether the nullifier is
spent. Those integrations are not implemented yet.

## Candidate plaintext encoding

All integer fields are big endian. The fixed 512-byte plaintext is:

| Byte interval (end exclusive) | Field |
| --- | --- |
| 0–1 | Version 1 |
| 1–9 | Asset ID, nonzero |
| 9–73 | Policy identity digest |
| 73–137 | Private spending-key commitment |
| 137–169 | Holder account |
| 169–177 | uint64 ASA value |
| 177–209 | Secret serial |
| 209–241 | Commitment blinding |
| 241–512 | Reserved, all zero |

Policy, owner, account, serial and blinding must be nonzero. Value zero is allowed
by this opening format; native operation rules and fees remain a separate check.
`Decode` rejects unknown versions, wrong lengths and nonzero padding.

Commitments use SHA3-512 over a distinct domain, genesis hash and the complete
canonical plaintext. Nullifiers use another domain, genesis hash, asset ID,
policy identity and secret serial. The holder and authorized controller derive
the same marker from the same note. Knowing a serial or marker does not establish
the spending-key preimage or current controller authorization.

The policy identity must remain stable across permitted authority changes. The
final state/proof specification must bind current powers and epochs separately.
The proof must constrain these exact hashes, uint64 values and ciphertexts; native
evaluation must reject repeated nullifiers, including duplicates within a group.
Do not accept these local helper outputs as proof verification.

The [candidate profile](../../../docs/native-asset-privacy/CANDIDATE_PROFILE.md)
records the unreviewed status. Matching note/ownership AIRs compile in the Rust
backend; interoperability is unverified. Membership, eligibility, control recovery,
encryption correctness inside the proof and recipient anonymity remain pending.
Tests and interoperability vectors are deferred by user instruction.
