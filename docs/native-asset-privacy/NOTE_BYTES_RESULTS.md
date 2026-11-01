# Canonical note-byte component results

Updated 2026-11-01. The generic [note_bytes_air](../../internal/assetprivacy/stark/src/note_bytes_air/README.md)
implements canonical opening constraints with11 main and9 trusted preprocessing
columns. It is compiled only with `native-privacy`. No native suite, proof-profile
selection, private lookup, hash, ownership or authorization was added.

## Why this component matters

The compact design had tentatively carried the whole decoded note across rows.
A byte stream instead checks one byte per row and exposes fixed byte/overlap-word
expressions. Its constructor fixes asset, policy and one to four notes; verifier
preprocessing must bind those exact arguments. Bytes still require authenticated
private links to every hash, envelope, amount and authority consumer.
The actual component has11 columns. Adding10 private pads would give21; the
estimated20 lookup-auxiliary columns are future design costs, not implemented.

## Bounded manual execution

A disposable worker outside the repository used the pinned P3 implementation,
the current Goldilocks/quintic hiding configuration and fresh OS-seeded proving
configuration. It ran actual preprocessing, proving and verification. This is not
the proposed Fp31/D10 profile, an artifact-bound local prover or native execution.

| Note count | Rows | Main / preprocessing | Symbolic degree | Quotient chunks |
| ---: | ---: | ---: | ---: | ---: |
|1 |512 |11 /9 |3 |8 |
|3 |2048 |11 /9 |3 |8 |
|4 |2048 |11 /9 |3 |8 |

Two bounded runs verified all three cases. The expanded run took1.093 seconds;
observed peak RSS was82.031MiB, sampled by the supervisor. The first run took
1.353 seconds and observed77.484MiB. These are sampled process observations,
not hard peak-allocation bounds or complete-prover RAM measurements.

The expanded run's canonical-layout byte counters were815793,1078809 and1066457
for1/3/4notes. These counters traverse actual proof metadata using the existing
writer layout; no artifact-bound serialization or transaction acceptance occurred.
Fresh masks/queries mean sizes differ between runs. No raw opening, private trace
or proof payload was persisted by the worker.

## Rejection and representation evidence

-13 malformed host cases rejected: counts0/5, zero asset/policy, count mismatch,
  wrong version/asset/policy, nonzero reserved padding and each zero secret field.
- All canonical trace constraints passed for counts1,2,3,4. Nine corrupted traces
  per count rejected, including non-Boolean bytes, wrong public/padding bytes,
  broken previous-byte/sum state, inverse changes and nonzero dummy cells.
- Every byte and applicable overlapping word matched the source opening across
  5120 real rows. Zero and maximum uint64 amounts remained valid shapes.
- Each real proof rejected a verifier key for a different policy and an altered
  proof opening. This authenticates the component's preprocessing dependency,
  not the missing complete transaction statement.

The supervisor imposed300 seconds CPU/wall and a2GiB sampled RSS stop threshold,
disabled core dumps and redacted expected constraint-panic details. No automated
test files/suites were added. Scratch worker location was
`/tmp/algorand-note-bytes-feasibility`; it is not a maintained test deliverable.
Current profile/defaults remain unchanged. See [conditional whole-proof costs](COMPACT_WIRE_BUDGET.md)
and [delivery status](DELIVERY_PROGRESS.md) for what remains.
