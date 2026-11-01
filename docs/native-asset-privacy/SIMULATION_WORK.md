# Candidate native privacy simulation work

Updated 2026-11-01. Simulation results can expose optional `native-privacy-work`
on both individual transaction results and the transaction-group result. The
[OpenAPI schema](../../daemon/algod/api/algod.oas2.json) defines five required
uint64 fields inside each present object. Emission requires the
`algorand_native_privacy` build; every current consensus version remains disabled.
No simulation, proof, node query, test or benchmark has been executed for this work.

## Counter meanings

| Field | What it counts |
| --- | --- |
| `reserved-units` | Successful native COW work-admission reservations |
| `bundle-verification-calls` | Actual single-bundle backend calls after Go preflight |
| `bundle-verification-successes` | Those backend calls returning nil |
| `admin-possession-calls` | Calls to Go `registrationproof.Verify` |
| `admin-possession-successes` | Those Go calls returning nil |

Reserved units are not actual proof checks. Existing cleanup admission reserves
a unit even though cleanup does not invoke a proof backend. A rejected reservation
emits no reservation event. A reservation can survive in these diagnostics even
when subsequent framing, local-artifact access, verification or application fails.

Bundle calls occur only after the Go verifier has completed its preflight and
is about to invoke the single-bundle backend. They do not count each constituent
proof inside a bundle. A missing backend fails before that call and can leave
reservations without any bundle-call event. A backend's nil return establishes
only that verification result; later native checks or a later group leg can fail.

Admin-possession calls count entry to `registrationproof.Verify`, including calls
that fail that API's own preflight. They exclude current account PQ-signature
consent checks. Admin success likewise describes the possession verifier's return,
not transaction acceptance, authority assignment or consensus finality.

## Attribution and rollback

[NativePrivacyWork](../../ledger/ledgercore/assetprivacy_work.go) contains numeric
counters only. The [evaluator observer](../../ledger/eval/assetprivacy_work_observer.go)
sends synchronous deltas outside COW state. The
[simulation tracer](../../ledger/simulation/assetprivacy_work.go) independently
adds each delta to its active top-level transaction and group accumulators.
All additions saturate at uint64's maximum instead of wrapping. Group totals are
accumulated directly, not reconstructed from transaction results after rollback.

These diagnostics remain after transaction or group state rolls back, including
failed local verification/application attempts that already emitted events.
Invalid signatures rejected before evaluation produce no native-work events.
Illegal native inner execution fails before observer attachment, so it receives
no work attribution. A diagnostic event without an active top-level simulation
transaction is an internal invariant violation, not silently assigned elsewhere.

The protocol's accepted-block work reservation counter remains separate. It still
obeys COW commit/rollback and enforces configured consensus admission limits.
Diagnostic accumulation neither commits failed work nor changes those limits.
The observer cannot authorize execution, alter verification results or grant
additional work capacity. No proof/key buffers are retained by these counters.

## Response boundary and limits

The [tagged REST converter](../../daemon/algod/api/server/v2/assetprivacy_simulation_enabled.go)
returns an owned five-counter model. The object is omitted when no events were
observed; the default-build converter always omits it. Omission is not a proof
that no other node work occurred. Ordinary simulation errors and failure paths
continue to indicate why a transaction or group failed. Failures that return no
simulation result expose no diagnostic object.

The [proofbridge observer](../../internal/assetprivacy/proofbridge/observer.go)
counts synchronous calls only. It provides no CPU, memory, elapsed-time, per-peer,
queue, fee estimate or resource-calibration measurement. Native fee-factor
reporting already follows the ordinary simulation group-usage path and is a
separate quantity. Backend successes do not prove accepted-block inclusion.

[Local single-bundle admission](LOCAL_VERIFIER_ADMISSION.md) now limits overlap; busy calls emit
no backend-call event. Per-peer scheduling, aggregate memory and denial-of-service
resistance remain unresolved. Numeric diagnostics alone establish none
of those properties and do not make the candidate proof configuration feasible.
The complete pending scope remains in [delivery status](END_TO_END_STATUS.md).
Tests, vectors, signatures, proofs and runtime evidence remain deferred.
