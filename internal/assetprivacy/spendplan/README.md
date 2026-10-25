# Expected main spending proof statements

Opt-in Go package `spendplan` derives exact expected public scalars for the complete
candidate main spending/group relation. It does not parse/verify proofs, accept
transactions, grant consent or mint a permission token. No proof-verifier callback
or native operation handler is supplied by this package.

`Build(Statement,Config)` snapshots one of two fixed operation shapes:

- Transfer: two actual private inputs and two actual private output commitments.
- Unshield: two actual private inputs, zero private outputs and Debit=0.

There are no optional/dummy inputs. A zero amount does not remove an input's
membership, ownership, nullifier or current-eligibility obligations. Statement
contains Kind, Inputs[2], Outputs, Debit, Credit and finalized Group digest.
Input is public Nullifier64, accepted note Root64 and native metadata Count.
Config contains the trusted tree Domain and typed current EligibilityRoot.

Domain fields, note roots, group/current eligibility root and actual output
commitments must be nonzero. Input counts must be1..2^32. Nullifiers must be
distinct; all64-byte nullifier values, including zero, retain their valid SHA3
encoding contract. Input roots may be identical. Duplicate output commitments,
wrong output count, unknown operation or a nonzero Unshield debit fail.

## Exact public vectors

Every digest uses32 little-endian uint16 limbs. A public uint64 amount uses four
least-significant-first uint16 limbs. Domain is fixed separately by the trusted
relation constructor; Count is native accepted-root metadata, never an AIR scalar.

| Public field | Transfer264 interval | Unshield196 interval |
| --- | --- | --- |
| Input0 nullifier | 0..32 | 0..32 |
| Input0 note root | 32..64 | 32..64 |
| Input1 nullifier | 64..96 | 64..96 |
| Input1 note root | 96..128 | 96..128 |
| Output0 commitment | 128..160 | absent |
| Output1 commitment | 160..192 | absent |
| Debit | 192..196 | absent |
| Credit | 196..200 | 128..132 |
| Group digest | 200..232 | 132..164 |
| Current eligibility root | 232..264 | 164..196 |

Intervals exclude their end. Unshield has no padded output or debit slots.

`Plan.Domain()` returns the trusted domain by value; `Kind()` selects the relation. `Values()`, `Inputs()` and
`Outputs()` return independent copies; no caller-owned mutable slice or vector is
retained. Build computes vectors directly from the expected native statement,
never from proof-advertised public values or metadata. The native verifier must
use these exact scalars and the expected constructor/suite/trusted artifact.

`CheckOutputs(records)` checks exact count, full domain and ordered commitment
matches, zero-based Record.Output ordinals and canonical record framing. It
rejects nil/zero plans. Record.Index is the tree allocation index and is not the
operation output ordinal. This is a structural join, not a proof check. The
separate output-role plans must enforce current immutable disclosure policy and
verify every required recipient/auditor/controller proof against the same ordered
commitments. No private input commitments, selected accounts or paths are exposed
by this public-output join.

## Native trust boundary

The native caller must derive Config after preceding group mutations, obtain each
input root/count from accepted note-root metadata, enforce two fresh distinct
nullifiers, match actual conversion debits/credits, permitted modes/authority,
funding and transaction round. Count bounds here cannot authenticate a root or
prove the private selected index. Note-root history never authorizes historical
eligibility; the current eligibility root is independent and must be rechecked.

Group must be recomputed from every finalized canonical transaction body, output
ciphertext/context, operation position, suite and ordinary atomic-group binding.
CheckOutputs does not perform that recomputation or check body transaction IDs,
nonce correspondence, current round or account consent. Native accepted-root and
full proof verification remain mandatory before one atomic state application.

These plans alone cannot establish input ownership/membership, conservation,
current freeze eligibility or group binding. The complete main relation must prove
all of those together, retaining private input commitments/accounts/paths. Every
required output-role relation must also be cryptographically verified with fresh
independent hiding entropy and a reviewed composed security budget.

Build, vet and source/physical-limit checks are the available evidence. No tests,
cryptographic vectors, traces, constraint evaluations, proofs or node scenarios
have run. The feature remains disabled by default; native transaction execution,
full deterministic verification, runtime/resource evidence and SDK delivery remain.
