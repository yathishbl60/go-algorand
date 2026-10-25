# Staged ordinary transfer and unshield spending relation

`SpendGroupAir::new(network,asset,policy,Operation)` composes two real holder
inputs with current eligibility and exact uint64 conservation. Operation is
`Transfer(PublicValue{debit,credit})` or `Unshield{credit}`. The native verifier
must choose the exact variant and conversion amounts from finalized bodies.
The Rust `native-privacy` feature remains disabled by default.

Each [SpendableNoteAir](../spendable_note_air/README.md) proves canonical private
opening, committed owner secret, domain-bound nullifier, accepted note membership
and current effective-unfrozen eligibility for the same hidden account. Key
rotation does not replace old notes' committed owners. Private input commitments,
accounts, paths, serials and secrets never become public joining values.
The common current eligibility root must come from current group-local state;
accepted note-root history cannot authenticate stale eligibility.

Transfer has exactly two real canonical outputs. Unshield has no private outputs:
its two output-amount registers are constrained zero and no output-note relation
or placeholder opening is instantiated. There are no optional/dummy input slots.
Zero-valued canonical notes are still actual notes, with normal ownership,
serial/blinding, membership and nullifier obligations.

## Public statements and private inputs

PublicStatement contains inputs:[InputPublic{nullifier,root};2], outputs:Vec<64-byte
commitment>, group:64-byte finalized-body/order digest, and
current_eligibility_root:64 bytes. Outputs must have length2 for Transfer or0 for
Unshield. Structural validation rejects duplicate input nullifiers, zero expected
roots/group/eligibility root/output commitments, and duplicate output commitments.
Every64-byte nullifier is representable, including all zero. Distinctness and
freshness still require native ledger checks; this AIR does not query state.

`public_values(&statement)` returns Option<Vec<Value>> with the exact schema:

| Scalars, end excluded | Transfer264 | Unshield196 |
| --- | --- | --- |
| 0..64 | Input0 nullifier32/root32 limbs | Same |
| 64..128 | Input1 nullifier32/root32 limbs | Same |
| 128..192 | Two output commitments,32 limbs each | None |
| Conversion | 192..200 debit4/credit4 limbs | 128..132 credit4 limbs |
| Group digest | 200..232,32 limbs | 132..164,32 limbs |
| Current eligibility root | 232..264,32 limbs | 164..196,32 limbs |

Hash digests use little-endian uint16 limbs. Conversion uses four least-significant
uint16 limbs per uint64. The complete vector, including group digest, enters the
pinned proof transcript. The constructor fixes conversion constants and the AIR
asserts their equality to the exact public conversion positions.

GroupInput holds two SpendInput values and a borrowed slice of zero or two
canonical512-byte output openings. `trace(&input,&statement)` checks exact shape,
canonical notes/ownership, both input roots and computed expected nullifiers,
output commitments and checked-u128 conservation before parent allocation.
It then writes bounded child frames into one matrix and returns
GroupWitness{trace,public_values}. Secret witnesses and traces must remain local.
These construction checks do not replace cryptographic verification.

## Fixed layout and binding

| Stage | Rows, end excluded |
| --- | --- |
| Holder input0 and current eligibility | 0..7356 |
| Holder input1 and current eligibility | 7356..14712 |
| Transfer output0 only | 14712..15224 |
| Transfer output1 only | 15224..15736 |
| Remaining rows | Padding to16384 |

Main width8506 is259 fixed root registers plus8247 reusable child scratch. Only
four64-bit amount fields and three carry bits are copied into parent roots.
All stay constant through every stage/padding; `amounts::eval` constrains Boolean
ranges and exact integer conservation, including signed carry, without field wrap.
Each active input/output child binds its own committed value to the corresponding
same root bits. No separate full child main matrix is allocated.

Preprocessing width80 contains the input's64 schedule columns and16 trusted outer
frame flags. Each input sees exactly its own public nullifier/root pair64 plus
the shared eligibility root32. MappedBuilder copies those96 entries only from
fixed parent-public indices, then delegates boundary/transition/active masks to
FrameBuilder. It cannot route a private column or proof-chosen index publicly.
Trusted variant, schedule and verifier artifact must bind these exact semantics.

Raw main storage is1,114,898,432 bytes; preprocessing adds10,485,760 bytes. These exclude
hiding/LDE, quotient, commitments, allocator and other prover overhead. The source
degree upper bound is10 through nested eligibility membership masks. Configuration
is unchanged; actual quotient/hiding/security and resource budgets remain
unvalidated. Builds and source counts establish no practical proof performance.

Transfer output role proofs must independently bind the same already-public new
commitments to current recipient eligibility/keys and every mandated disclosure.
The native verifier must verify all required roles plus this main relation, exact
finalized bodies/group context, policy/mode, accepted input roots, fresh distinct
nullifiers, real public debits/credits, fees, storage and atomic state updates.
No header comparison or expected-vector planner can verify these proofs.

Recovery/clawback, frozen original-owner remainders, other arities, full native
proof decoding/verification/ingress, wallet/SDK and real-node demonstrations remain
unfinished. This ordinary holder path cannot grant viewing keys spending power
or restore relinquished issuer authority. No tests, vectors, traces, constraint
evaluations, proofs or benchmarks have run. No security review is established.
