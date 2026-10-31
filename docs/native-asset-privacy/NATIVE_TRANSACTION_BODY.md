# Candidate native shield/spend/recovery transaction body

Candidate contract, 2026-10-30. NAPT codec/admission and evaluator routing exist;
activation and runtime evidence remain absent. [NAPL lifecycle](LIFECYCLE_INGRESS.md)
uses a separate family, including typed Falcon admin registration. See [suites](NATIVE_SUITE_CATALOG.md) and [bridge](NATIVE_VERIFIER_BRIDGE.md).

TxType `aprv` exposes inert raw byte fields `Transaction.NativeAssetPrivacy`
(`codec:"napb"`) and `SignedTxn.NativeAssetPrivacyProof` (`codec:"napw"`). Both
have allocation bounds and ordinary canonical msgp encoding. Raw fields avoid
build-dependent Transaction/SignedTxn layouts and new generated struct methods.
Absent fields retain historical bytes. The disabled-build stub and disabled
consensus reject the new type and every nonempty privacy field. Reject body fields
on unrelated types and orphan witnesses. The sole other body placement is [Create5 on ordinary ASA creation](NATIVE_CREATION.md). Behavior uses `algorand_native_privacy`.

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
two inputs/two outputs give10465 bytes in Shielded4; Confidential2 appends128 bytes, giving10593. Envelopes must pass
canonical framing before expensive verification. No keys, epochs, private input
commitments or membership paths appear; only confidential mode discloses selected note accounts.

| Operation | Native tag/main proof kind | Inputs | Outputs | Conversion fields |
| --- | ---: | ---: | ---: | --- |
| Transfer | tag1; main1/8 | 2 | 2 | Debit/credit are uint64 |
| Unshield | tag2; main2/9 | 2 | 0 | Debit0, positive credit |
| Shield | tag3; main3/10 | 0 | 2 | Positive debit from Sender, credit/receiver0 |
| Partial private recovery | 4 | 1 | 2 | Debit/credit0 |
| Full private recovery | 5 | 1 | 1 | Debit/credit0 |
| Partial public recovery | 6 | 1 | 1 | Debit0, positive credit |
| Full public recovery | 7 | 1 | 0 | Debit0, positive credit |

Recoveryplan.Kind is tag-3. For partial private recovery output0 is recovered and
output1 retained; partial public recovery output0 is retained. [Confidential2](CONFIDENTIAL_MODE.md)
ordinary operations append input0/input1/output0/output1 accounts,32 bytes each.
Actual note slots are nonzero; absent slots zero. Mode4 has no trailer. Main kinds8/9/10 bind these exact committed accounts.
Public1 uses ordinary ASA transfers; confidential recovery and unknown modes reject.

## Projection, authorization and application

Parse the target body under current protocol limits. Derive shield/spend/recovery
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
calibration, viewing/spending registration and other modes remain pending. [Native shield](NATIVE_SHIELD.md) is source-integrated.
