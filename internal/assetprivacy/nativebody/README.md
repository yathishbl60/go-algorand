# Canonical candidate native privacy body

Opt-in `nativebody` encodes/decodes the public `aprv` payload defined by the
[native body contract](../../../docs/native-asset-privacy/NATIVE_TRANSACTION_BODY.md).
All files require `algorand_native_privacy`. The package imports no transaction
or ledger types and performs no authorization, cryptographic proof or state check.

`Decode(encoded)` returns an owned Body. It first validates the bounded header,
operation counts and conversion fields, scans inputs/outputs into fixed two-slot
containers, and validates every record, exact resulting length and envelope frame.
Only then does it allocate returned slices/copy ciphertexts. Nothing in the result
references the input buffer. Keep the input immutable for the complete call.
`Encode(body)` validates first and returns owned canonical bytes; keep every body
slice immutable until it returns. There are no optional padding or trailing bytes.

`Body.Validate()` checks these same structural/semantic rules, including maximum
encoded size. `Body.CheckSender(nonzeroSender)` also binds any positive public
debit to that outer sender; with zero debit the source must be zero and a nonzero
fee sponsor is allowed. Equality is an identity comparison, never a signature or
consent proof. Current rekey/authorizer and PQ signature checks remain mandatory.

## Exact field order

Every integer is unsigned big-endian. HeaderSize=225 and MaxSize=10465 bytes.
The fixed header contains ASCII `NAPT`, version1, operation1, mode1, asset8,
policy64, suite64, public debit8, debit source32, public credit8, receiver32,
input count1 and output count1. Asset/policy/suite must be nonzero. Mode must be
exactly shielded4; public1, confidential2 and combined/unknown values are rejected.

Inputs follow, each136 bytes: nullifier64, nonzero root64, accepted-root count8.
Counts must be1..2^32. Every64-byte nullifier, including zero, is permitted;
two inputs may not repeat the same nullifier. Accepted-root provenance and
nullifier freshness must be checked against current native state separately.

Each output is nonce32, commitment64, disclosure flags1, recipient envelope1629,
then auditor1629 iff flag1 is set, then controller1629 iff flag2 is set. These are
policy.AuditorDisclosure=1 and policy.ControllerDisclosure=2; unknown bits fail.
Output nonce/commitment must be nonzero; two outputs cannot repeat a commitment.
Each present envelope must have exact1629-byte framing and version1. Missing or
extra role fields fail. An output is1726,3355 or4984 bytes. The codec checks the
declared flags; the native evaluator must match them to the immutable policy.

| Operation | Tag | Inputs | Outputs | Conversion |
| --- | ---: | ---: | ---: | --- |
| Transfer | 1 | 2 | 2 | Debit/credit uint64 |
| Unshield | 2 | 2 | 0 | Debit0, credit>0 |
| Shield | 3 | 0 | 2 | Debit>0, credit0 |
| Partial private recovery | 4 | 1 | 2 | Debit/credit0 |
| Full private recovery | 5 | 1 | 1 | Debit/credit0 |
| Partial public recovery | 6 | 1 | 1 | Debit0, credit>0 |
| Full public recovery | 7 | 1 | 0 | Debit0, credit>0 |

Counts are exact operation arities rather than peer-selected capacities.
Shield has no dummy private inputs, a positive public debit from the outer sender,
and no public credit/receiver. Positive debit requires a nonzero source; positive
credit requires a nonzero receiver. With zero conversion the corresponding
address must be zero. Public recovery destinations remain public body fields.

Partial private output0 is recovered value and output1 the retained remainder;
partial public output0 is the retained remainder. The codec cannot authenticate
issuer power or prove remainder ownership/value preservation. The selected main
relation, exact output-envelope variants and current native authority must do so.

The body contains no transaction/group ID, round, tree index, private opening,
viewing epoch/key, selected private input account or membership path. Native
projection supplies output domain/index/round/transaction ID/ordinal from the
actual ledger and finalized target body. The proof stays outside the transaction
body in the exact SignedTxn witness field; proof framing is a separate package.

Current policy identity/suite approval, immutable disclosures, permitted modes,
full body/group binding, accepted roots, fresh markers, conversion holding rules,
funding and atomic state application remain native evaluator obligations.
Canonical body validity alone cannot accept a transaction or enable the feature.
Shield syntax additionally needs actual native ShieldAir/bundle verification,
current recipient/disclosure eligibility and public/private atomic accounting.

No tests, codec round trips, runtime vectors, traces or proofs have run. Builds,
vet, source review, formatting and physical limits establish source/compilation
properties only. Every consensus version remains disabled by default.
