# Serial Owner registration: bounded validation

Updated 2026-11-01. This reduces one genuine primitive's cost. **No native private
transaction has been accepted.** Full recipient/encryption feasibility remains
unresolved; every current consensus version stays disabled.

## What ran

A temporary isolated release worker used the repository's actual revised
OwnerRegistrationAir and pinned P3 revision 1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f.
Rust 1.99.1, aarch64-apple-darwin, unchanged queries/hiding/hash parameters and
matching transitive dependencies were used. Fresh OS entropy supplied the network,
group and private secret. No secret, trace or proof payload was persisted/logged.

The worker generated genuine preprocessing and a fresh private proof, then used
the actual preprocessing key for verification. The compiled symbolic constraints
reported degree5; the actual proof had16 quotient chunks and four FRI input batches.
These were direct P3 calls, not an exercised Prepared API or artifact-bound codec.

| Check | Observed result |
| --- | --- |
| Main trace | 512×1953; 7,999,488 raw bytes |
| Preprocessing | 11 columns |
| Genuine proof | Verified |
| Changed owner, changed group, altered proof opening | All rejected |
| Valid trace checked against every row's constraints | Passed |
| Seven deliberately corrupted traces | All rejected |
| FRI input query-cell payload | 3,285,504 bytes |
| Calculated canonical-layout byte count from actual proof metadata | 4,158,529 bytes |
| Analytical complete-proof upper bound | 4,226,753 bytes |
| Latest worker: full pipeline plus corruption checks | 1.650 seconds wall |
| Latest worker: observed peak RSS | 187.422 MiB |
| Worker exit | 0 |

The byte count traversed actual proof fields with the canonical writer's lengths;
it did not serialize an independently artifact-bound native proof or transaction.
The [bound and assumptions](SERIAL_OWNER_LAYOUT.md) include framing, OOD openings,
salts, authentication paths, FRI and quotient/random data, unlike query payload alone.

Corruptions covered initial state, a round transition, absorption capacity,
post-computation held state, parity, zero nonzero-check inverse and non-Boolean
state. An isolated process-owned panic hook redacted expected constraint failures;
no library-global hook was changed. These spot checks are not a complete soundness
argument or an automated test suite. No repository test files were added.

Three fresh serial-layout proof runs verified. The first ordinary pipeline took
1.719 seconds with168.828 MiB observed peak RSS; the next two included explicit
trace-corruption checks. The old 3540-column run used291.344 MiB and its query
payload was5,712,384 bytes. RSS is sampled, not an exact instrumented peak.

The supervisor was configured for300 seconds wall/CPU, 2 GiB sampled RSS and no
core dumps; neither cutoff triggered. No larger recipient proof was attempted.
Full registry/artifact generation, native signature/group/fee validation, accepted
transactions and six real-node scenarios remain required in [delivery status](END_TO_END_STATUS.md).
