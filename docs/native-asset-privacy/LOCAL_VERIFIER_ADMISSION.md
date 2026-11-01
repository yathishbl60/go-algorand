# Candidate local verifier admission

Updated 2026-11-01. The `algorand_native_privacy` build bounds concurrent native
bundle verification and distinguishes local failures from invalid transactions.
Every consensus version remains disabled. This is source integration with build/
static evidence; no tests, proofs, node queries or runtime contention cases ran.

## One process-wide bundle lease

[proofbridge admission](../../internal/assetprivacy/proofbridge/admission.go) uses
one process-wide, nonblocking lease. [Verifier.Verify](../../internal/assetprivacy/proofbridge/verify.go)
checks readiness first, then acquires the lease before copying the owned bundle
payload, constructing the backend request or invoking bundle verification.
A busy lease immediately returns `ErrBusy`, which wraps `ErrResource`; it does
not enqueue another request. A deferred release covers every subsequent return,
including ordinary Go panic unwinding. A call that never acquired releases nothing.

The lease covers one bundle verification at a time across verifier instances.
It does not bound all allocations or CPU use in the node, gate every PQ/admin
verification, cancel an already-running FFI call, or provide per-peer fairness.
It supplies no resource calibration, aggregate memory quota or wall-clock bound.
It also provides no guarantee that a blocked caller will eventually get the lease.

## Local failures remain retryable

[verify.IsLocalNativePrivacyFailure](../../data/transactions/verify/assetprivacy_local_enabled.go)
uses `errors.Is` through wrappers for `ErrUnavailable` and `ErrResource`, including
`ErrBusy`. Rejected proofs, malformed framing and configured-limit errors are
excluded. Local resource/availability failure is not invalid-proof or bad-peer
evidence, and no success or acceptance is manufactured from such a failure.

[data/txHandler.go](../../data/txHandler.go) evicts raw/canonical message dedupe
entries when pool `Test` or `Remember` returns a classified local failure, so a
later delivery can retry. The `Remember` path skips application-limiter peer
penalties for those failures; the preliminary `Test` path also adds no such
penalty. Transactions still require ordinary signature/native validation before
pool inclusion; dedupe eviction grants no verification-cache acceptance.

## Preserve the bounded pending queue

[Pool deferral](../../data/pools/assetprivacy_deferred_enabled.go) changes only
recomputation of already-admitted groups. On the first classified local failure,
it retains the affected group and subsequent dependent suffix in the existing
pending queue instead of evaluating that suffix against a missing prefix.
There is no additional retry queue and retention cannot grow the pending set.

Deferred groups bypass invalid-status caching and invalid-removal counters; they
also receive no successful-reevaluation count. Existing committed-group removal
and expiration checks continue. A truly expired group follows the normal expiry
status/counting path. The evaluator contains only the validated prefix, which
alone can supply an assembled block; retained groups gain no inclusion permission.

Fresh `Test` and `Remember` calls return a local resource error while that suffix
is unresolved. The next `OnNewBlock` or explicit recompute retries groups in order
through full ordinary evaluation. This intentionally stalls fresh admission until
recompute; it avoids accepting new groups against an incomplete pending prefix.
Pending RPC/txsync snapshots can include previously admitted deferred groups that
have not yet been revalidated for the current round. Pending presence is not a
current validation result or promise of block inclusion.

## Agreement and catchup behavior

[Agreement retry](../../agreement/assetprivacy_retry_enabled.go) revalidates the
same existing proposal request/context after a classified local failure, with a
10ms retry delay. Waiting observes context cancellation and verifier shutdown.
Cancellation is reported as cancellation; local failure is not converted into an
invalid/malformed proposal result. A nonlocal validation failure still rejects.

The retry path adds no unbounded request queue, but may repeatedly validate an
already-checked proposal prefix. Timed resource budgets, scheduling fairness and
starvation remain unresolved. Cancellation during the wait does not interrupt a
synchronous backend call already in progress.

[Catchup validation](../../catchup/service.go) returns a wrapped local validation
error to its caller instead of treating the condition as demonstrated peer
misbehavior. Runtime catchup/retry behavior remains unverified.

`POST /v2/transactions` maps classified local failures to HTTP 503 with a fixed
public message; nonlocal admission errors retain HTTP 400. The existing Swagger
503 response already covers this status. Asynchronous submission keeps its
existing service-unavailable error behavior.

## Defaults and remaining scope

Without the build tag, the classifier is false, pool helpers preserve ordinary
behavior and agreement performs its ordinary validation call. No native lease or
retry policy is activated; default native verification remains unavailable.
All native consensus flags, suite catalogs and execution prices remain inactive.

This lease and retry handling do not resolve per-peer admission, failed-attempt
CPU budgets, aggregate memory accounting, denial-of-service resistance or proof
feasibility. See [simulation diagnostics](SIMULATION_WORK.md) and the complete
[delivery status](END_TO_END_STATUS.md) for the distinct implemented/pending scope.
Tests, vectors, signatures, proofs and all real-node scenarios remain deferred.
