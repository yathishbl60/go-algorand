# Candidate shield relation

`ShieldAir` proves zero private inputs and exactly two canonical private outputs.
It uses no dummy membership witnesses. The constructor fixes network, asset,
policy and the public debit. The output notes' committed value bits feed the
integer conservation constraints directly; their sum must equal that uint64 debit.
The two unused input bit arrays share a column constrained to zero.

`trace(&[[u8; 512]; 2])` generates the private matrix. `public_values` takes
`PublicStatement { outputs, group }`. Public values are:

| Elements | Contents |
| --- | --- |
| 0–31 | Output 0 commitment |
| 32–63 | Output 1 commitment |
| 64–67 | Debit, least-significant uint16 limb first |
| 68–99 | Finalized group-context digest |

Digests use consecutive little-endian uint16 limbs. The complete vector enters
the pinned backend's transcript. Native verification must reconstruct the domain,
debit, commitments and finalized group digest from authoritative transaction data.

This component does not verify encryption, recipient eligibility, mandatory
disclosures, public asset funding, private supply, fees or storage charges.
It cannot authorize native shielding. No proofs or tests have been executed.
