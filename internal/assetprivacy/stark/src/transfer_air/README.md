# Candidate transfer constraints

For current ordinary input eligibility plus group conservation, use
[SpendGroupAir](../spend_group_air/README.md). This earlier primitive omits
current eligibility and is not the native main-proof selection.

This disabled Rust component composes two actual private input notes and two
private output openings. Each input proves ownership, its serial-derived public
nullifier, and 32-level membership. Each output proves a canonical note opening.
All components use the constructor's network, asset and policy domain.

`TransferAir::new` takes the authoritative public conversion amounts.
`trace` rejects openings that fail their component generators or whose uint64
values violate exact integer conservation. It uses the components' committed
value bits directly in `amounts::eval`; separate unbound value witnesses are
never substituted. Every component must have the candidate's 512-row height.

The fixed public vector contains 232 Goldilocks elements:

| Elements | Contents |
| --- | --- |
| 0–31 | Input 0 nullifier |
| 32–63 | Input 0 membership root |
| 64–95 | Input 1 nullifier |
| 96–127 | Input 1 membership root |
| 128–159 | Output 0 commitment |
| 160–191 | Output 1 commitment |
| 192–195 | Public debit, least-significant uint16 limb first |
| 196–199 | Public credit, least-significant uint16 limb first |
| 200–231 | Finalized group-context SHA3-512 digest |

Hash outputs use `sha3_air::digest_values`: consecutive little-endian uint16
limbs from the original digest bytes. `public_values` constructs this vector.
The conversion limbs are constrained to the constructor's amounts. The final
group-context limbs are transcript inputs: pinned Plonky3 absorbs the complete
public vector before sampling its constraint batching challenge, even when an
AIR constraint does not consume a particular public element. Native verification
must recompute the finalized group digest and supply those exact limbs.

This relation has no optional slots or dummy-note semantics. Both input slots
require actual membership witnesses. Native execution must reject duplicate or
spent nullifiers, accept the selected roots under the ledger's root rules,
validate public balance conversion, and enforce policy, freeze, recovery and
storage funding rules. Ciphertext correctness and recipient/controller key
binding are not constrained here. This component must not be installed as an
accepted native spending verifier before those obligations are implemented.

The horizontal composition has substantial trace memory requirements. No proving
resource bounds or cryptographic security guarantee are established by compiling
this component. No runtime proof generation, proof verification or tests have
been executed for it.
