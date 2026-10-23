# Candidate native policy records

Added 2026-10-23. Policy state is now connected to native COW storage behind the
disabled feature flag. No privacy transaction or public endpoint calls it yet.

## Identity and canonical encoding

The record is exactly 77 bytes: version1, modes1, initial-capabilities1,
current-capabilities1, disclosures1, suite64, creation-roundBE8. Unknown versions,
wrong lengths, invalid flags, missing recovery disclosures and a zero suite digest
are rejected. Current capabilities must be a subset of initial capabilities.
The native getter also rejects a creation round in the future.

The key is ASCII `aprv:p:` followed by the asset's 8-byte big-endian ID.
It is separate from AVM boxes, notes, spent markers and tree records.

`Record.Identity` hashes ASCII `Algorand/AssetPrivacy/Candidate1/Policy/`, genesis32,
assetBE8, version1, modes1, initial-capabilities1, disclosures1, suite64 and
creation-roundBE8 with SHA3-512. Mutable current capabilities are excluded.
This identity therefore remains stable when rights are relinquished. Key epochs
and their current authorization state must be bound separately by the full proof.

## Native accessors

`GetAssetPrivacyPolicy` checks the consensus flag before I/O, confirms that the
ASA exists, and returns canonical current group-local state and explicit presence.
Missing records do not authorize privacy. Errors must not be treated as absence.

`AdoptAssetPrivacyPolicy` requires the current nonzero manager address, no prior
record, public mode, a current creation round, and an exact match to existing ASA
freeze/clawback powers. Initial and current capabilities must agree at adoption.
This method cannot grant powers to an immutable legacy asset.

`UpdateAssetPrivacyPolicy` requires the current manager address, preserves the
immutable identity fields and permits only capability relinquishment. Resulting
policy capabilities must match the current ASA freeze/clawback addresses.
Within the future atomic administrative action, remove freeze/clawback keys
before updating the policy, and clear the manager afterward if relinquishing it.
All intermediate COW changes must commit or roll back together.

These helpers receive a manager address after caller authentication. They do not
verify signatures, PQ key control, the accepted suite, policy registration proofs
or storage funding. A future native handler must perform those checks. Merely
supplying the manager address is never authorization at an RPC boundary.

Private-only creation needs a distinct authenticated creation integration; the
existing-asset adoption helper intentionally retains public mode. Authority key
epochs, issuer recovery and disclosure-key records remain unimplemented.

Builds/static analysis are permitted and performed. Tests, signature/replay
vectors and protocol execution remain deferred/unverified.
