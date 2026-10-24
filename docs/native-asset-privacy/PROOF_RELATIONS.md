# NAP-01 statement and witness obligations

Draft obligations, 2026-10-22. This refines the north-star behavior; the selected
implementation candidate is recorded in [CANDIDATE_PROFILE.md](CANDIDATE_PROFILE.md).
It is not a completed NAP-01 profile or NAP-02 normative consensus specification.

## Shared statement

Every value proof binds the network domain, asset, operation, privacy policy,
suite version, key epochs, validity interval and exact ordered group context.
It binds the target transaction index, accepted commitment root, current
authorization root, nullifiers, output commitments, ciphertexts and disclosures.
Public conversions additionally bind the public accounts and integer amounts.

Private witnesses include input note openings/membership paths, the required
spending or recovery authority material, output note openings, encryption
randomness, recipient key/epoch and private eligibility paths. Neither RPC nor node simulation
receives those witnesses. The node receives the proof and public statement.

The proof establishes the following predicates as one composed relation:

- Every input note exists under the specified commitment root, belongs to this
  asset/policy domain, and is spendable through the selected authorization path.
- Every input/output amount lies within the allowed uint64 range.
- Over mathematical integers, input value plus public debits equals output
  value plus public credits. Field arithmetic must not permit wraparound.
- Each consumed note produces its unique domain-bound nullifier. Holder spend
  and authorized recovery of the same note produce the same nullifier.
- Each output commitment opens to the exact note encrypted for its recipient
  and, when required, auditor and controller; all required outputs are present.
- Disclosed participant identities in confidential mode agree with registered
  keys used in the relation. Shielded mode exposes no recipient routing key.
- Proof of viewing/audit access cannot be substituted for spending authority.

## Per-operation requirements

| Operation | Additional relation or ledger condition |
| --- | --- |
| Register/rotate private keys | Prove control under the reviewed schemes; bind account, roles and epochs; existing outer authorization remains required |
| Shield | Bind actual public debit and holder consent; create equal private output value; check existing frozen/opt-in state |
| Private transfer | Consume valid inputs and create equal private outputs; enforce permitted mode and current eligibility |
| Unshield | Bind actual public credit and recipient; conserve any private change; enforce public-exit policy |
| Partial clawback | Use currently authorized controller path, consume the same markers as holder spends, and preserve remainder for original owner |
| Freeze/control change | Authenticate declared authority; update current eligibility/epoch state without expanding relinquished rights |
| Destruction reconciliation | Require final supply reconciliation and authorized destruction in the same atomic group; disallow a standalone private-only exit |

These are composition requirements, not independent permissive verifier methods.
The final profile must make each operation's exact public/witness fields explicit.

## Separate encryption context from proof context

An encryption associated-data value cannot include the ciphertext it produces.
Define a domain-separated, canonical pre-encryption output context containing
the required network, asset, policy, suite, key-role/epoch and output identifiers.
Freeze its exact fields and any fresh transaction nonce in NAP-01/NAP-02.
Recipient/auditor/controller contexts must bind their distinct roles. Key epochs
are constrained private inputs, not serialized recipient routing metadata.

Create commitments and ciphertexts, finalize transaction bodies and their normal
group ID, then prove against the full finalized public statement. The proof
binds the ciphertexts to that final group even though encryption happened first.
Do not put the final group ID into encryption associated data when that ID
depends on transaction bodies containing the ciphertexts.

Keep proof witnesses outside transaction IDs. Include every group member and
its order in the proof context. Evaluate binding the canonical bodies directly,
rather than relying only on existing transaction/group digest values: the current
implementation uses SHA-512/256 for those identifiers. The selected profile must
justify its own binding strength. This is a review obligation, not a claim that
this draft upgrades the rest of Algorand's security.

## Ordered ledger checks remain necessary

After stateless proof verification, ledger evaluation must recheck nullifier
freshness, accepted root history, current authorization root/epochs, actual public
balances, frozen/opt-in status, retained powers and storage funding. Earlier
members of the same group affect these checks. A cached proof result does not
authorize stale state. Distinguish rebuildable stale-state failures from invalid
proofs without disclosing witnesses.

Commit note, nullifier, supply, public-balance and storage changes together, or
roll them all back. Sum debits/credits with checked integer arithmetic and reject
negative or over-total private supply. Fees are ALGO and cannot silently change
ASA conservation. Crash/replay/catchup must reproduce these effects.

## Repository integration points

Existing transaction/group hashing is in
[transaction.go](../../data/transactions/transaction.go) and
[crypto/util.go](../../crypto/util.go). Native witness integration belongs beside
[SignedTxn](../../data/transactions/signedtxn.go); group verification is in
[verify/txn.go](../../data/transactions/verify/txn.go).
Existing asset authorization and destruction rules are in
[ledger/apply/asset.go](../../ledger/apply/asset.go).
The new [policy package](../../internal/assetprivacy/policy/README.md) covers only
mode/disclosure immutability, eligibility and capability monotonicity.
