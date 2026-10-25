# Private canonical effective-unfrozen record

`EligibleRecordAir::new(network,asset,policy)` constrains the candidate current
eligibility leaf for one private account with Frozen=0. This is a record hash,
not membership, holder consent, ownership, spending or issuer authorization.
The library remains behind opt-in `native-privacy`.

The exact 199-byte SHA3-512 preimage is ASCII
`Algorand/AssetPrivacy/Candidate1/Eligibility/` (45 bytes), then a canonical
154-byte record: version1, network32, assetBE8, policy64, account32, revisionBE8,
UpdatedBE8 and Frozen1. Constructor network/asset/policy fields must be nonzero.
The account occupies message bytes150..182, revision182..190, Updated190..198
and Frozen byte198. Network/asset/policy/version/domain/Frozen are fixed by AIR
constraints. Account and revision are nonzero, using bit-sum inverse witnesses.

Frozen=0 is a constant constraint on all eight bits, rather than an unconstrained
host assertion. Every message bit is Boolean through Sha3Air. Account, revision,
Updated and the private hash output remain constant throughout the record frame.
Updated is an opaque committed uint64; native current-root provenance supplies
its round validity. Revision is independent of control-marker and key epochs.
No equality to any viewing/spending epoch is introduced.

`trace(account,revision,updated)` constructs a private trace after canonical field
checks. `commitment(...)` computes the matching candidate digest without trace
allocation. Neither verifies a proof. The crate-wide bounded writer validates
its shape/inputs before mutation and fills512 rows directly in parent storage.
`wires(row)` exposes private account/revision/Updated bits and commitment limbs
for in-process composition. Digest limbs are never added to public inputs.

The component has512 rows,4262 main columns, no preprocessing and zero public
values. Its raw main matrix requires17,457,152 bytes, before allocator, hiding,
LDE, quotient and commitment overhead. Constant private digest registers connect
Sha3Air's public-output interface to private parent columns; no private lookup
terminal is serialized. The conservative source degree bound is6 before an
outer stage mask. Actual quotient and security budgets remain unverified.

[CurrentEligibilityAir](../current_eligibility_air/README.md) joins this exact
private leaf to the current eligibility root. Native state must authenticate an
effective-frozen snapshot derived from public holding/control/default state,
not a caller-proposed Frozen flag or the raw marker alone. A record hash cannot
turn an absent or outdated snapshot into permission.

No tests, vectors, trace generation, constraint evaluations, proofs, benchmarks
or node demos have run. Builds, Clippy, formatting and physical source checks
provide compilation/source evidence only. No cryptographic review or privacy
claim is established by this component.
