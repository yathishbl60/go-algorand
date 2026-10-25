# Ordinary spendable note with current hidden-account eligibility

`SpendableNoteAir::new(network,asset,policy)` joins a canonical private input note,
its committed owner's secret, serial-derived public nullifier, accepted note
membership and current effective-unfrozen account membership. The opt-in
`native-privacy` feature remains disabled by default. This component does not
implement a complete transaction, conservation, issuer recovery or native verifier.

The input commitment, account, owner, serial, secret, amount, paths, indices and
eligibility revision remain private trace data. No private lookup terminal or
extra private-input commitment is published for composition. The account in the
canonical opening must equal the account authenticated by CurrentEligibilityAir.
That current snapshot constrains effective Frozen=0, not merely marker.Frozen=0.

The spending secret hashes to the owner already committed by this input note.
There is deliberately no equality to the latest registered RecipientSpend owner:
prospective key rotation must not invalidate older notes or bypass current freeze
state. The nonzero secret is 32 private bytes, constrained Boolean with a bit-sum
inverse. Note canonicality independently requires its account, owner, serial and
blinding fields to be nonzero and fixes all reserved padding to zero.

The exact owner hash message is the45-byte ASCII candidate SpendingKey domain,
network32 and secret32 (109 bytes). The nullifier message is the43-byte candidate
Nullifier domain, network32, assetBE8, immutable policy64 and committed serial32
(179 bytes). Both use SHA3-512. The public nullifier may be any64-byte hash output,
including all-zero; ledger freshness and group distinctness are separate checks.

`SpendInput` is Clone+Copy and contains opening512, secret32, note_path,
eligibility_revision, eligibility_updated and eligibility_path. All witnesses
must remain local to the prover. `check_inputs(input,note_root,eligibility_root)`
returns the computed nullifier only after canonical opening/owner-secret checks
and exact membership under both expected roots, without allocating the main
matrix. `trace(...)` performs those checks before allocation and returns
SpendNoteWitness with trace and96 public field scalars.

`public_values(nullifier,note_root,eligibility_root)` serializes each digest as32
little-endian uint16 limbs, in that order. Accepted note roots may come from the
native bounded note-root history. The eligibility root must be the exact current
group-local root after preceding changes; note-root history and current key roots
can never supply eligibility authorization. The proof cannot choose its expected
root, network, asset, policy, schedule or trusted artifact.

Crate-wide bounded `write_into(input,note_root,eligibility_root,values,shape)`
validates inputs/shape before mutation and returns the computed nullifier.
`shape` is (stride,column,first_row). `write_preprocessed_into` emits the trusted
schedule for the same active frame. Child main frames write directly into shared
scratch; complete child main matrices are not materialized. `wires(row)` exposes
private account, least-significant-first amount64, serial/secret bits and private
owner/commitment limbs for an enclosing conservation relation.

| Stage | Row interval, end excluded |
| --- | --- |
| Canonical note and private commitment | 0..512 |
| Private ownership-secret hash | 512..561 |
| Public serial-derived nullifier | 561..634 |
| Private commitment, accepted note membership | 634..3739 |
| Same private account, current eligibility | 3739..7356 |
| Constant private root padding | 7356..8192 |

There are8247 main columns:897 private root registers plus7350 shared scratch.
Roots contain account256 bits, amount64 bits, serial256 bits, secret256 bits,
owner32 limbs, commitment32 limbs and inverse1. All roots remain constant through
all stages/padding; all832 bit registers are Boolean. Note account/value/serial/
owner and both hash prefixes/preimages are constrained against those same roots.
Ownership and commitment digests use private output columns. The73/49-row hash
frames include a final pad row with every absorption flag cleared.

Preprocessing has64 columns:44 shared child schedule columns and20 stage flags.
FRAME_ROWS=7356; standalone ROWS=8192. Raw standalone main storage is540,475,392
bytes, active-frame main storage485,319,456 bytes and standalone preprocessing
4,194,304 bytes. Counts exclude child Keccak/schedule scratch, hiding/LDE,
quotient, commitment, allocator and all other prover overhead. Resource feasibility
remains unmeasured. Conservative source degree is9 through nested eligibility
membership; a further enclosing stage raises it to10. Actual quotient, hiding,
composed soundness and quantum budgets remain to be evaluated.

The native transaction must still require valid mode/consent, distinct fresh
nullifiers, accepted note roots, current eligibility, complete conservation and
finalized body/group binding, exact output disclosures and funding. Frozen-holder
issuer recovery needs its own authenticated relation; this ordinary holder-secret
component grants no clawback/freezer/manager power and cannot substitute for it.

No tests, vectors, traces, constraint evaluations, proofs, interoperability runs,
benchmarks or node demos have run. Builds, Clippy, formatting and source limits
establish only source/compilation properties; no security approval, zeroization,
recipient anonymity or usable native private transfer is established.
