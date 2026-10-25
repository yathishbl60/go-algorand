# Expected statements for output role proofs

Opt-in `proofplan` derives public field scalars for the future native verifier.
It does not parse or verify STARK proofs, authorize transactions or append notes.
There is no permissive verifier, proof callback or successful-verification token.

`Build(outputs.Record,Config)` takes a finalized public output and trusted current
state. Config contains the exact tree domain, immutable disclosure flags, typed
current key/eligibility roots and assigned auditor/controller accounts. It rejects zero domain
fields/root, unknown flags, missing required or unexpected targets, domain/policy
mismatches, malformed envelope framing and absent/extra role ciphertexts.

The returned Plan always has a recipient Slot, then auditor if required, then
controller if required. `Slots()` returns a new slice; each Slot exposes `Role()`,
`Kind()` and `Values()`. Values returns a separate copy. No input wire buffer or
caller-owned vector is retained. `CheckRoles(roles)` requires exact count and
order, rejecting empty plans, extra/missing/duplicate/wrong-role labels. This
check is structural: every proof still needs actual cryptographic verification
against its exact expected relation, trusted artifact and native-derived vector.

| Vector interval, end excluded | Encoding |
| --- | --- |
| 0..248 | Canonical public envelope context as u8 scalars |
| 248..792 | 1088-byte KEM ciphertext as 544 little-endian u16 limbs |
| 792..804 | Twelve GCM nonce bytes as u8 scalars |
| 804..1060 | 512-byte GCM ciphertext as 256 little-endian u16 limbs |
| 1060..1068 | Sixteen GCM tag bytes as eight little-endian u16 limbs |
| 1068..1100 | Current key root64 as 32 little-endian u16 limbs |
| 1100..1132, recipient | Current eligibility root64 as32 little-endian u16 limbs |
| 1100..1132, disclosure | Expected assigned authority32 as u8 scalars |

RecipientPublicValues=1132 matches AuthorizedRecipientAir. DisclosurePublicValues
=1132 matches AuthorizedDisclosureAir. Their same-length trailers have distinct
semantics: the expected role/relation must select the root-limb or authority-byte
schema; length or role-label comparisons alone do not verify either proof.
`envelope.Context.PublicBytes()` produces
the exact248-byte public context by omitting private epoch bytes147..155 from
the256-byte encryption context. Epoch is ignored by PublicBytes; callers need
no viewing-key epoch to derive verifier inputs. Nonzero public fields, role,
format/version and big-endian asset/output encoding retain their normal checks.
The actual private nonzero epoch still feeds AAD and registered-key constraints.

Native `AssetPrivacyOutputProofPlan(asset,record)` checks the consensus feature
before I/O, derives the immutable policy/domain/flags and assigned targets from
current group-local state, and reads the current typed key and eligibility roots.
Required missing or stale controller assignments fail closed. It requires record.Round to equal
the current round. There are no caller overrides for native domains/targets/roots.
Generic Build is also usable by a local prover with the same trusted snapshot.

This is one output's statement plan. The native transaction handler must match
record metadata, context nonce/output ordinal and every ciphertext to actual
finalized transaction bodies. It must validate operation arity/output ordering,
commitment rules, complete main conservation/spend/group proof, accepted input
roots, current input eligibility, issuer authorization, nullifiers and funding.
The ordinary recipient relation now directly binds its hidden account to current
effective-unfrozen membership. A root-only separate proof cannot make that join.
The handler, proof decoder/verifier and full input/control composition remain
unfinished. Authorized recovery remainders to frozen original owners need a
separate relation; this ordinary recipient path must not block that obligation.
A plan or a role label cannot replace those obligations.

All new output role proofs and the main proof can join on the same already-public
canonical note commitment, under the binding assumption and exact domain match.
[Composition obligations](../../../docs/native-asset-privacy/PROOF_PLANNING.md)
explain why no private input commitment or lookup terminal should be published.
The main proof must bind those same ordered output commitments and finalized
body/group digest. Every proof requires independent fresh hiding entropy.

Plans are snapshots, not cache authorization: rebuild/recheck them after every
preceding group mutation, including key/assignment/ASA changes. Suite/artifact,
role, full vector, both current roots, group/body context and state identities must be
part of any future verification cache key. Never accept a proof's advertised
root, authority, schedule key or public vector as the expected statement.

Feature/default builds, vet, source/layout review and size checks establish no
cryptographic correctness or runtime privacy evidence. No tests, vectors,
traces, constraint evaluations, proofs, benchmarks or node demos have run.
