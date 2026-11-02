# Real native Owner registration on two nodes

Recorded 2026-11-02. `TestNativeOwnerRegistration` passes with actual Rust proving,
Falcon account authorization, native node validation and block acceptance. This
is owner-key registration for a public-only ASA policy; it is **not a private
asset-value transaction**, a complete privacy scenario or protocol activation.

## Final expanded run

The targeted regression passed in **8.43s** with freshly rebuilt native/FFI-tagged
`algod`, `goal` and `kmd`. It used an isolated two-node network and operator-local
Future consensus override. Shipped/current/Future defaults remain disabled; the
test asserts the parent Future default is false. No public network was changed.

The test produced a real scoped Owner preprocessing registry, independently
selected its pins/capability mask and checked `Suite.Verifier` before startup.
It funded a real Falcon1024 account, created an ordinary100-unit ASA and rejected
an otherwise valid signed Public|Shielded adoption under that incomplete suite.
The rejected adoption published no policy and left holdings/supply unchanged.
It then adopted the permitted public-only policy.

The finalized two-transaction group contains a123-microAlgo payment followed by
Owner registration. Its real proof binds network, asset, policy and final group.
The fixture canonically changes one opened field coordinate and independently
requires exact cryptographic `InvalidProof`, preserving valid framing. The node
must report Rust proof rejection for that group; preceding payment, key record,
root and count remain unchanged. The original valid group then confirms, publishes
the exact Owner commitment at epoch1 and executes the payment once. Private supply
remains0 and total supply100.

| Measured wire item | Bytes |
|---|---:|
| NAPW witness |4152795|
| Complete registration SignedTxn |4156264|
| Complete atomic group |4159513|

Both transaction and complete group are asserted below5,000,000 bytes. Sizes vary
with randomized proofs/signatures; these values are observations, not worst-case
proof bounds. No transport/block limit was raised.

The second node reaches the confirmed round and agrees on exact key record,
key root/count, policy identity and public supply. It is stopped and restarted
against the same database, then all those checks pass again.

## Regression found and fixed

The initial acceptance/rollback test passed in8.88s. Adding restart exposed
`trackerEvalVerified ... native privacy Rust verifier is unavailable` during
tracker replay. The real ledger already configured artifacts before replay;
`accountUpdatesLedgerEvaluator` did not forward the optional provider interface.
The feature-gated forwarding method now delegates to the real ledger's atomic
provider snapshot. It adds no tracker-lock reentry, alternate verifier, fallback
key or proof-verification bypass. The expanded run above verifies the fix.

## Reproduce and limits

See [build/run commands](../../test/e2e-go/features/assetprivacy/README.md).
The Rust example uses a deliberately public test secret and must never hold real
assets. No proof/seed/private trace files were committed. The test has a5-minute
deadline and its prover child a2-minute deadline; aggregate RAM was not measured.

This closes native Owner registration and its tested rejection/restart path only.
Full privacy operations, complete bounded prover/transport gates, wallet recovery,
catchup/upgrades, approved artifacts and security review remain pending. The six
required privacy scenarios remain0/6 in [delivery status](DELIVERY_PROGRESS.md).
