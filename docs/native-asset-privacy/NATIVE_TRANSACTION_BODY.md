# Candidate native spend/recovery transaction body

Candidate contract, 2026-10-29. NAPT codec/admission and evaluator routing exist;
activation and runtime evidence remain absent. [NAPL lifecycle](LIFECYCLE_INGRESS.md)
uses a separate family. See [suites](NATIVE_SUITE_CATALOG.md) and [bridge](NATIVE_VERIFIER_BRIDGE.md).

TxType `aprv` exposes inert raw byte fields `Transaction.NativeAssetPrivacy`
(`codec:"napb"`) and `SignedTxn.NativeAssetPrivacyProof` (`codec:"napw"`). Both
have allocation bounds and ordinary canonical msgp encoding. Raw fields avoid
build-dependent Transaction/SignedTxn layouts and new generated struct methods.
Absent fields retain historical bytes. The disabled-build stub and disabled
consensus reject the new type and every nonempty privacy field. Reject body fields
on other types and orphan witnesses. Behavior uses `algorand_native_privacy`.

## Canonical body version1

Inner bytes are fixed fields in this order; integers are unsigned big-endian.
Every field is mandatory, including zero fields. No optional padding or trailing
bytes exist. Validate counts and exact resulting length before allocating copies.

| Field | Byte interval | Rule |
| --- | ---: | --- |
| Magic/version | 0..5 | ASCII `NAPT`, byte1 |
| Operation, mode | 5..6,6..7 | Table below; one policy.Mode value |
| ASA | 7..15 | Nonzero, equal current authoritative asset |
| Policy, suite | 15..79,79..143 | Exact current policy identity and approved suite |
| Public debit, debit source | 143..151,151..183 | Source equals outer Sender iff debit>0; otherwise zero |
| Public credit, credit receiver | 183..191,191..223 | Nonzero receiver iff credit>0; otherwise zero |
| Input count, output count | 223..224,224..225 | Exact operation arity, never peer-selected capacity |
| Inputs in order | From225,136 each | Nullifier64, root64, accepted-root count8 |
| Outputs in order | After inputs | Nonce32, commitment64, disclosure flags1, role envelopes |

Intervals exclude their end. Exact length is225+136*inputCount plus each output's
97+1629*(1+number of disclosure bits); enforce this before allocating any copies.
The fixed header is225 bytes. Each accepted-root count is1..2^32; roots are nonzero, and all
64-byte nullifier patterns are allowed. Duplicate nullifiers/commitments fail.
Output nonces/commitments are nonzero. Each output includes one1629-byte recipient
envelope, then auditor and controller envelopes iff the immutable policy flags
require them. Unknown flags and missing/extra roles fail. Output maximum4984 bytes;
two inputs/two outputs give a hard body ceiling of10465 bytes. Envelopes must pass
canonical framing before expensive verification. No keys, epochs, private input
commitments, selected private accounts or membership paths appear in this body.

| Operation | Native tag/main proof kind | Inputs | Outputs | Conversion fields |
| --- | ---: | ---: | ---: | --- |
| Transfer | 1 | 2 | 2 | Debit/credit are uint64 |
| Unshield | 2 | 2 | 0 | Debit0, positive credit |
| Shield | 3 reserved | — | — | Reject until complete relation exists |
| Partial private recovery | 4 | 1 | 2 | Debit/credit0 |
| Full private recovery | 5 | 1 | 1 | Debit/credit0 |
| Partial public recovery | 6 | 1 | 1 | Debit0, positive credit |
| Full public recovery | 7 | 1 | 0 | Debit0, positive credit |

Recoveryplan.Kind is tag-3. For partial private recovery output0 is recovered and
output1 retained; partial public recovery output0 is retained. The current AIRs
support shielded mode4 only. Public1/confidential2 must fail until their complete
native semantics and participant-identity proof joins exist; a body mode alone
cannot prove disclosed identities. Future modes/arities need explicit dispatch.

## Projection, authorization and application

Parse the target body once under current protocol limits. Derive spend/recovery
drafts from those fields, never accept an independent caller draft at ingress.
Recompute the final ordinary group ID and group statement from every canonical
body, suite and target position. Derive output Record.Domain from network/ASA/
policy, Index from the current native tree count, Round from current evaluation,
Transaction from target.ID(), and Output from its zero-based body position.
Copy nonce, commitment, disclosures and envelopes from that output. Index/round/
transaction metadata stay outside the body, avoiding a transaction-ID cycle.

Require approved top-level PQsig over the complete body and ordinary current
rekey/authorizer validation. For recovery, Sender must also equal current ASA
clawback authority with retained policy power. Sender equality or proof success
alone grants no consent. Private-input ownership remains in the main proof;
outer Sender may sponsor fees when public debit is zero. Nonzero public debit
uses Sender's actual holding and authorization, never an arbitrary third party.
Check public conversion holding/opt-in/freeze rules, current mode/exit permissions,
funding, roots and fresh nullifiers after preceding group mutations. Private-only
final reconciliation requires its authorized atomic destruction workflow.

Put the exact NAPW frame in SignedTxn only. It remains outside TXID/group IDs but
inside block commitment/size charging. Approved suite caps bound bundle bytes;
the137-byte NAPW header is additional. Default decoder bounds remain zero until
an enabled protocol supplies a catalog. Network framing, admission queues and
protocol byte limits must reject excessive sizes before allocating witness data;
do not expose a blanket512MiB historical/default SignedTxn decoder allowance.

Verification-cache identity must include exact witness length and SHA3-512 bytes,
suite and statement under the same consensus version; retain owned immutable
identity material. Existing signature-only cache hits cannot skip privacy proof
verification. Cache cryptography only; current ledger checks run every execution.
Apply supply, public holdings, markers, commitments and outputs in one child COW
that commits only after every required check succeeds. Local verifier availability
or allocation failures must not become invalid-proof cache entries.

The [integration guide](NATIVE_TRANSACTION_INTEGRATION.md) distinguishes codec,
admission and execution. No tests/round trips/proofs ran. Practical byte/work
calibration, registration, shield and other modes remain pending. [Execution/storage](NATIVE_EXECUTION.md) integrate.
