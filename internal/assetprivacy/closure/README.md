# Candidate post-destruction closure record

This fixed codec is gated by `algorand_native_privacy`. It stores public facts
for durable bounded cleanup after an authorized final destruction. The codec
authenticates no manager, creator, transaction, proof or supply transition.
All consensus versions remain disabled with no approved suite or pricing.

`Key(asset)` is ASCII `aprv:z:` followed by assetBE8. Asset must be nonzero.
The exact313-byte value has these half-open offsets:

| Field | Bytes |
| --- | --- |
| Version1 | 0..1 |
| Manager account | 1..33 |
| Creator account | 33..65 |
| Policy digest | 65..129 |
| Suite digest | 129..193 |
| Finalized group digest | 193..257 |
| Destroy transaction identifier | 257..289 |
| Total, unsigned BE8 | 289..297 |
| Created round, unsigned BE8 | 297..305 |
| Cleanup epoch, unsigned BE8 | 305..313 |

Manager, creator, policy, suite, group and destroy must be nonzero; epoch must
be positive. Total and Created may be zero at codec level. Authoritative native
execution must derive these fields and impose its contextual supply, creation,
transaction/group, authorization and cleanup rules. No supplied record proves
a final destruction occurred. A retained record alone grants no ASA authority.

Encoding returns owned bytes. Decoding copies all arrays and rejects unknown
versions, wrong lengths, trailing bytes and invalid required fields before use.
The codec does not read or mutate a ledger. Resource accounting, cleanup epoch
compare-and-swap and final storage release remain integration responsibilities.

No tests, runtime transitions, proofs or node scenarios have run. Source and
compilation checks do not establish lifecycle or security correctness.
