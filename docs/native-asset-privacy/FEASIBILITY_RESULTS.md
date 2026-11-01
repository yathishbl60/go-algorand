# Native privacy feasibility gate — 2026-11-01

**Owner primitive validation passed. The complete candidate layout fails the
current transport/resource gate. No native private transaction was accepted.**
The later [serial Owner layout and bounded run](SERIAL_OWNER_RESULTS.md) reduce
the Owner cost; the recipient and full-operation blockers below still apply.
The user authorized bounded proof/setup and isolated-node validation; writing new
automated test files remains deferred. Native consensus stays disabled.

## Earlier bounded validation: 3540-column Owner

An isolated temporary worker used the real OwnerRegistrationAir with the pinned
Plonky3 revision 1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f and Rust 1.99.1,
aarch64-apple-darwin, release build. Transitive dependency versions matched the
repository lock. Ephemeral network/group/secret data came from OS entropy;
no live wallet, secrets, raw traces or proof payloads were logged or persisted.

The worker performed genuine public preprocessing, a fresh-entropy private proof,
and verification using the actual preprocessing key. It did not fabricate a
NAPK6 registry or weaken verification. These were direct pinned-P3 calls, not
an exercised Prepared API, native artifact-bound codec or accepted transaction.

| Latest run | Result |
| --- | --- |
| Full setup/trace/prove/verification wall time | 2.014 seconds |
| Observed peak RSS | 291.344 MiB |
| Valid owner-relation proof | Verified |
| Changed owner public input | Rejected |
| Changed group public input | Rejected |
| Altered proof opening | Rejected |
| FRI input query-cell payload alone | 5,712,384 bytes |
| Worker exit | 0 |

Two fresh positive proof runs completed; the table describes the second, expanded
check. The supervisor was configured to stop at 300 seconds wall/CPU or 2 GiB
sampled RSS and prevented core dumps. Neither cutoff was reached. RSS sampling
is not an exact instrumented peak measurement.
No larger recipient proof was attempted. No automated test suite was added/run.

## Hard transport incompatibility

The candidate fixes 192 FRI queries and encodes each Goldilocks field cell in 8
bytes. Pinned P3 preserves one opened matrix row per query, including duplicate
indices; Merkle pruning removes sibling paths, not those row values.

| Relation/set | Main-trace query-cell lower bound, bytes |
| --- | ---: |
| Earlier Owner registration: 192×3540×8 | 5,437,440 |
| Shield main: 192×7641×8 | 11,736,576 |
| Recipient output: 192×24869×8 | 38,198,784 |
| Shield main plus two recipient outputs | 88,134,144 |

These bounds omit mask columns, preprocessing/quotient/random openings, salts,
paths, framing, signatures and transaction bodies. Query collisions do not reduce
this encoding count. Current [transaction gossip](../../protocol/tags.go) permits
5,000,000 bytes; current ordinary block capacity is 5,242,880 bytes.
The complete Shield operation cannot fit those existing limits.
Reducing main-trace width alone cannot resolve this: Shield preprocessing has
8 columns and each recipient has 2075. Their preprocessing query cells alone
cost 192×(8+2×2075)×8 = 6,386,688 bytes, before any main-trace opening.
The redesign must address both main and preprocessing representations.

## Static recipient memory dimensions

Recipient main trace: 65536×24869×8 = 13,038,518,272 bytes (~12.143 GiB).
Pinned hiding PCS expands it to 2N rows and adds 5 random columns. Blowup 2^3 then
materializes a dense 1048576×24874 LDE: 208,658,235,392 bytes (~194.328 GiB).
Preprocessing LDE alone adds approximately 16.211 GiB. These are matrix dimensions,
not measured peaks, and exclude Merkle, quotient, allocator and other overhead.

Source chain: local `authorized_recipient_air/params.rs`, `parameters.rs`,
`verification/write_fri.rs`, `verification/write_scalars.rs`; pinned P3
`fri/src/transcript.rs::query_phase`, `fri/src/prover.rs::open_inputs`,
`merkle-tree/src/mmcs/mod.rs::open_batch_pruned`, `fri/src/hiding_pcs.rs::commit`
and `fri/src/two_adic_pcs.rs::coset_ldes`. A small Owner proof does not validate
these recipient-resource requirements.

## Delivery decision

Preserve the current implementation as a checkpoint. Prioritize proof-layout/
representation redesign that preserves the required relation and privacy roles.
Simply reducing security queries or enlarging network limits is not an approved
resolution. Artifact generation and full private-node acceptance remain blocked
by the present layout; no dummy artifact or state-write bypass demonstrates them.

[Delivery gates](DELIVERY_PROGRESS.md), [detailed status](END_TO_END_STATUS.md) and
[local proving API](LOCAL_PROVING.md) distinguish source, primitive evidence and
the still-missing native end-to-end result.
