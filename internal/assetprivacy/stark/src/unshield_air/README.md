# Candidate unshield relation

`UnshieldAir` proves ownership, nullifier derivation and 32-level membership of
exactly two actual private inputs, with no private outputs. It uses no dummy
output notes. The constructor fixes network, asset, policy and the public credit.
The inputs' committed value bits feed integer conservation directly; their sum
must equal that uint64 credit. Unused output bits share a constrained-zero column.

`trace(&[InputWitness; 2])` builds the matrix by generating and dropping one input
component at a time. `InputWitness` and `InputPublic` share the transfer component's
types. `public_values` takes `PublicStatement { inputs, group }`:

| Elements | Contents |
| --- | --- |
| 0–31 | Input 0 nullifier |
| 32–63 | Input 0 membership root |
| 64–95 | Input 1 nullifier |
| 96–127 | Input 1 membership root |
| 128–131 | Credit, least-significant uint16 limb first |
| 132–163 | Finalized group-context digest |

Digests use consecutive little-endian uint16 limbs. The complete vector enters
the pinned transcript. Native verification must reconstruct all these values,
reject duplicate/spent nullifiers, require accepted roots, enforce current
eligibility, credit the correct public recipient and decrease private supply
atomically. Fees, storage charges and issuer recovery are separate obligations.

There is no optional input slot or one-input witness. Partial withdrawal with
change belongs to a relation with private outputs. This component cannot authorize
native unshielding. No proofs, tests or measured proving budgets exist yet.
