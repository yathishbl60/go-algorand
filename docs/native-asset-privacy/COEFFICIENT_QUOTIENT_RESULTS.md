# Stripwise quotient proof integration

Recorded2026-11-03. `coefficient_pcs::StripQuotient` now executes the full pinned
batch quotient calculation over eight sequential2N cosets of the16N quotient
domain. It uses the original N-row selector domain and unchanged constraint
evaluator, downstream chunking/masking, transcript and stock verification.
This completes candidate PCS/quotient orchestration for the tested ownership
relations; complete native private-value statements remain unimplemented.

## Explicit scheduling extension

Pinned `p3-batch-stark` at1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f is vendored.
Its thirteen original source files contain5142 lines. Eleven remain byte-for-byte
unchanged; only `lib.rs` and `prover.rs` change, with one58-line scheduling
interface added. Licenses, original manifest, exact replayable patch and SHA3-512
provenance are retained and fingerprinted. Dependency versions do not change;
Cargo.lock switches only this crate's source to the local vendor directory.

The original `prove_batch` selects whole-domain evaluation and parallel instances.
The new explicit scheduling entry supplies the current PCS, original/full domains,
committed matrix references and a fallible evaluator callback. The candidate
checks its geometry/budgets, disables instance parallelism and scatters each strip
to original natural row order. It holds no lock across parallel evaluator work.
No verifier, proof-format or transcript source is modified by this patch.

The evaluator retains main/lookup matrices, public periodic data, selectors and
quotient outputs. Candidate preflight bounds known field payloads, including
three periodic copies and extension coordinates, before any evaluator call.
Constraint task buffers, FFT scratch and other phases remain outside that bound.
No process RAM or complete-prover claim follows from buffer caps alone.

## Proof-pipeline regression

Replace the target in [the regression command](INTEGRATION_TESTS.md) with
`--test coefficient_quotient_strips`. The nine-target serial release run passed;
its strip target took **15.74s**;
strict release Clippy, formatting and authored20/100 limits pass.

For one-note/512-row and two-note/1024-row ownership AIRs, a wrapper compares EVERY
quotient value from the eight strips against a whole16N-domain evaluation using
the same committed private polynomials, lookup challenges, public program and
folding challenge. Each test requires1988 main columns,57 periodic columns,
lookup data and exactly eight2N evaluator calls. Equality covers selector,
next-row, public-program and private-lookup interactions in these real relations.

The wrapper then completes proving, and the original stock compact verifier
accepts both batch proofs. Two more proofs pass with evaluation caps of3million
and6million base scalars respectively: sufficient for strips and quotient LDEs,
insufficient for a full wide16N matrix. Invalid geometry, public preprocessing
and a one-scalar budget reject before any callback. Wrong callback row lengths
reject with `Shape`; callback allocation errors propagate. Each callback failure
stops after the first invocation.

Fresh fixture/FFI/node builds also passed native Owner acceptance, canonical
bad-proof rollback and peer restart in9.04s, with SignedTxn4,157,547 bytes and
group4,160,791 bytes. The normal Go build linked the changed Rust archive without
cache deletion or forced rebuild, exercising the retained-data build marker.
The existing two-node `TestAssetSend` passed in25.61s with privacy disabled.

The actual verifier height gate now accepts extended degree17 (original65536
rows) and rejects18. The earlier16-bit maximum incorrectly bounded the extended
height. Small proofs now verify with the candidate configuration before stock
conversion; all cases assert the actual20 auxiliary base-polynomial claims.

## Full-height ownership worker

A bounded release worker completed a two-note/65536-row ownership proof with
1988 main columns,57 periodic columns,20 auxiliary base columns and16 quotient
chunks. All eight actual131072-row strips ran; both candidate configuration and
unchanged stock verification accepted the complete proof. Worker time was292.877s;
supervisor wall time **293.306s**, with **5653.062MiB sampled peak RSS**
(~5.521GiB). Limits were600s
CPU/wall,7GiB sampled RSS and disabled core dumps, on a64GiB host. Sampling can
miss brief peaks. No seed, private trace or proof artifact was retained.

The first worker stopped on an incorrect auxiliary-count assertion after proof
generation; the assertion was corrected and moved into normal small regressions.
The final completed run checked both candidate and stock configurations.
The optional ignored full-height integration
case compiles with [repeat instructions](../../internal/assetprivacy/stark/tests/support/coefficient_quotient_strips/README.md);
it has not yet been executed as a test target.

## Remaining gates

This covers ownership at65536 rows, not a complete native operation.
Full membership, nullifiers, conservation, recipient encryption
and role composition, worst-case resource measurements, exact native compact
codecs and accepted private-value node transactions remain. All shipped
consensus flags remain off; no suite/artifact approval or cryptographic review
is implied. The six required real-node privacy scenarios remain0/6.
See [delivery status](DELIVERY_PROGRESS.md) and the [PCS checkpoint](COEFFICIENT_PCS_RESULTS.md).
