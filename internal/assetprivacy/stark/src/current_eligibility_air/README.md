# Current effective-unfrozen membership

`CurrentEligibilityAir::new(network,asset,policy)` joins a private canonical
[eligible record](../eligible_record_air/README.md) to depth-32 current eligibility
membership. The library remains disabled by default via `native-privacy`.
This component does not prove consent, note ownership, issuer power or a transfer.

The native evaluator supplies the current typed eligibility root from group-local
state after preceding mutations. Historical note roots, proof-advertised roots
and current-key roots cannot substitute. Native publication derives effective
Frozen from the current holding/control/default state; Frozen=0 in the private
record is independently constrained by the record AIR. Missing registry entries
are never implicitly eligible. Public holding closeout or key rotation must not
remove the account snapshot or reset its frozen state.

`EligibilityInput` contains private account32, revision64, Updated64 and a
MembershipPath. Revision is its own stream, independent of key/control epochs.
`trace(input,current_root)` checks nonzero canonical fields and exact leaf/path/
root before allocating the main matrix. The returned EligibilityWitness contains
only trace and the32-scalar public vector. `public_values(root64)` uses32
little-endian uint16 limbs; no selected account, index, revision, Updated,
private leaf, path or intermediate digest is added publicly.

Crate-wide `check_inputs`, `write_into(input,root,values,(stride,col,first))` and
`write_preprocessed_into(values,shape)` support enclosing relations. Both bounded
writers validate target dimensions before writes. Child record/member main
frames write directly into shared parent scratch, without a complete child main
matrix. Standalone assembly fills fixed root registers through all4096 rows;
bounded assembly fills only FRAME_ROWS=3617. Private shape-checked `wires(row)`
exposes account/revision/Updated bits and leaf limbs for direct parent bindings.

The active schedule is record0..512, current membership512..3617, followed by
padding3617..4096. There are5366 main columns:416 fixed private registers
(account256 bits, revision64, Updated64, leaf32 limbs) and4950 shared scratch.
All root registers remain constant across stages and padding. All384 private
account/revision/Updated bits are Boolean, with exact equality to the record
hash fields. The same private digest equals the membership leaf. Trusted
preprocessing has44 columns:36 member schedule columns and8 child frame flags.

The conservative source degree bound is8 on the deepest membership path:
SHA3 absorption6, membership hash frame+1, eligibility stage frame+1. An
additional enclosing stage raises that bound and requires a revised actual
quotient/hiding/security budget. Trusted schedule artifacts, complete proof
composition, native decoding/verifying and work limits remain unfinished.

Raw standalone main storage is175,833,088 bytes; bounded active main storage is
155,270,576 bytes. Standalone preprocessing adds1,441,792 bytes. These counts
exclude child schedule/Keccak scratch, allocator, hiding/LDE, quotient,
commitment and all other prover overhead. Practical resources remain unmeasured.

A recipient parent must equate this private account to the canonical opening's
account; a spending parent must bind it to the account already committed by its
private input note, without publishing that note commitment. Output registered
owner rotation must not disable old-note spending. State root membership alone
cannot replace those account bindings or the separate note ownership secret.

No tests, vectors, traces, constraint evaluations, proofs, interoperability runs,
benchmarks or node demonstrations have run. Builds, Clippy, formatting and source
limits establish only compilation/source properties; no completed security,
recipient anonymity, consent or runtime correctness claim is made.
