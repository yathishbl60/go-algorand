# Explicit artifact capability scopes

Updated 2026-11-02. NAPK7 and trusted NAPV7 replace unapproved versions1..6.
No shipped consensus enables native privacy or approves any suite/artifact.
This change permits an honestly scoped owner-registration validation; it does
not make a partial suite usable for private assets or complete a privacy scenario.

## Exact scope contract

The registry remains1774 bytes:134-byte header and twenty82-byte physical records
in compiled order kinds1..14,17..22. Header count20 denotes physical slots, not
present keys. Every slot retains exact canonical kind/row/width/public dimensions.
An absent capability has flags0 and all64 root bytes zero. A present key has
flags3 (both next-row openings) and a nonzero64-byte raw SHA3 root. Other flags,
inconsistent roots, wrong dimensions/order, trailing bytes and empty scopes reject.
An absent slot is not a zero-key placeholder and cannot bind a proof.

Rust `encode_registry(profile,entries)` accepts a nonempty canonical-order subset
of actual typed public preprocessing entries. Duplicate/unknown/unordered entries
reject. `Registry::load` validates the entire fixed file before publishing only
present entries. Go retains the fixed physical value array; `Entry(kind)` returns
false for absent kinds. The supported historical implementation-reference rules
remain strict; the file cannot select its own independent pins.

Digest bytes are SHA3-512 of ASCII
`Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV7/` followed by the entire file.
The marker is `NAPK\x07`; the trusted request marker is `NAPV\x07`. Fixed request
header2152 bytes and C ABI2 remain unchanged. NAPV is never peer-supplied authority.

## Consensus and policy guard

`config.NativeAssetPrivacySuite.ProofKinds` independently pins a uint32 mask with
bit k for kind k. Only bits1..14 and17..22 are permitted; zero/reserved masks reject.
`Registry.ProofKinds()` reports present keys. `Suite.Verifier` requires exact
equality with the consensus mask before constructing the Rust verifier. Missing
kinds deterministically reject; missing/corrupt local files remain availability
failures and cannot be substituted with witness metadata or fallback keys.

`Suite.CheckPolicy` accepts an incomplete suite only when Modes is exactlyPublic.
Any Confidential or Shielded mode currently requires all twenty native kinds.
Policy writes, evaluator reads and ledger snapshot queries enforce this guard.
This conservative rule prevents creation of private inventory under a declared
partial suite that cannot prove spending/withdrawal. It does not certify runtime
feasibility of a complete suite. Trusted approvals/bootstrap must validate installed
manifests against their masks before enabling a private protocol; mask metadata
alone does not prove file availability, artifact provenance or cryptographic safety.

## Current evidence and remaining work

The `scoped_owner_registry` release integration test uses actual Goldilocks Owner
setup, a real one-key NAPK7, canonical artifact-bound proving/self-check and the
typed verifier. A run passed in1.39s with4,152,065 proof bytes. Proof length varies
with random query/path choices; this is one measured proof, not a worst-case bound.
Changed pins, absent/malformed slots, unsupported kinds, changed statements,
wrong roots/artifacts and corrupted/truncated proofs reject. Go scope integration
tests check parser/catalog equality and policy restrictions with explicitly
synthetic roots; they are metadata tests, not substitutes for real proof evidence.

Native Owner registration, bad-proof rollback and key-epoch publication now
[pass on two nodes with peer restart](NATIVE_OWNER_E2E_RESULTS.md). This establishes
key registration only. Private value operations and the six real-node privacy
scenarios remain separate requirements in [delivery status](DELIVERY_PROGRESS.md).
