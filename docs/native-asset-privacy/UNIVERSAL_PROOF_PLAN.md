# Universal parallel proof direction

This is an architectural direction, not an approved or release proof profile.
Native privacy remains disabled in every consensus version. Complete relation
coverage, security review, peak memory and production performance remain open.

Use ONE AIR with 65536 rows and parallel column regions for the ten logical
tables in [the compact ledger](COMPACT_WIRE_BUDGET.md). Smaller programs force
inactive rows to zero. The provisional main width is 3384 relation columns plus
extra bridges E and R lookup relay columns; no cross-AIR terminal pads are needed.
One global LogUp accumulator must end at zero. Auxiliary 180 means 17 extension
fractions plus one accumulator at extension degree 10. Compatible bus grouping,
all emissions, multiplicities, degree bounds and cancellation must be proved;
180 is a target, not an implemented complete auxiliary-width bound.

Keep the existing SHA3-512 commitment, owner, nullifier and Merkle contracts,
and the exact Go envelope bytes/AAD, ML-KEM, HKDF and AES-GCM construction.
ML-KEM's existing SHA3-256 operations also remain unchanged. No cSHAKE, new
commitment hash, public-key commitment or replacement AAD format is required.
Reuse only complete verifier-derived PUBLIC prefix blocks with exact domain,
padding, capacity and subsequent private absorption bindings. No private key,
authority, epoch or path equality is assumed merely because roles coincide.

At 24 rows per permutation, 2730 complete Keccak permutations occupy 65520 rows.
`compact_keccak_air` now permits 2730 and trusted padding to 65536 rows. This
round component still needs sponge, message, digest and private bus binding.
The following conditional counts come from [PROOF_WORKLOAD.md](PROOF_WORKLOAD.md)
and its cited sources, after public-prefix reuse without private-authority sharing.
r counts additional auditor/controller envelopes, at most two per private output.

| Operation | Private outputs | Private Keccak permutations |
|---|---:|---:|
| Transfer, two inputs |2|1128+285r≤2268|
| Shield |2|848+285r≤1988|
| Unshield, two inputs |0|280|
| Partial-private recovery |2|726+285r≤1866|
| Full-private recovery |1|433+285r≤1003|
| Partial-public recovery |1|367+285r≤937|
| Full-public recovery |0|74|
| Viewing registration |N/A|7 private;144 public matrix derivation|
| Owner registration |N/A|1|

Recovery retains its actual weaker original-owner remainder relation and native
issuer authorization requirements. Disclosure arity follows actual outputs;
full-public recovery has no role envelopes. These counts do not omit recipient,
auditor/controller, ownership, eligibility or membership obligations. They are
hash schedule counts, not evidence that the full arithmetic program fits.

Represent the trusted public program as full-period periodic columns. The pinned
batch prover/verifier evaluates periodic polynomials, but does NOT bind the AIR's
`periodic_statement` automatically. Define a canonical program digest from the
exact schedule and statement, require verifier-owned public-value equality, and
bind it before the FIRST RELEVANT CHALLENGE. Stock `batch-stark` observes the
main commitment then all public values before lookup challenges; this ordering
is sufficient. The verifier's typed constructor fixes the expected program.
The lookup builder cannot consume concrete periodic expressions directly:
use main relay cells constrained equal to the corresponding periodic values.
Never accept witness-selected schedules, row identities or emission counts.

The wire target retains stock 16 quotient chunks, independent 10-element salts,
4-byte base values, degree 10 extension values and maximum 8-ary FRI. No quotient
packing, shared salts, query dedup, 31-bit wire codec or ad hoc terminal pads are
required. All constraints, including lookup grouping and selector products, must
fit maximum degree 8 and the resulting hidden q16 geometry. Its four commitment
batches contain main, auxiliary, 16 quotient matrices and randomization:
19 matrices total. Main and auxiliary open current+next; quotient/random open z.

| Conditional bound, auxiliary180/R0 | Bytes |
|---|---:|
| Proof |4697385|
| Native transaction wrapping |15463|
| Complete SignedTxn |4712848|
| Slack below 5000000 |287152|

Each additional current+next main or auxiliary base column costs 848 bytes:
complete wire≤4712848+848×(E+R+A−180), with auxiliary width A. At A=180/E=0,
R=338 gives 4999472 bytes; R=339 exceeds 5MB. R=256 leaves 70064 bytes. Any new
descriptor or statement bytes consume this margin. Public program width P has
no direct proof opening bytes only when its exact contents are verifier-derived;
periodic evaluation, metadata, memory and CPU costs remain unresolved.

Dense main LDE storage alone is 13.258GiB before relays, requiring a column
streaming/storage adapter. A planning ledger retains roughly 2GiB masked
coefficients, 0.8GB salts and 640MiB trees, plus FFT/hash scratch, trace builders,
auxiliary generation, periodic evaluations and final assembly. These allocations
and their combined lifetimes are not a proof of an8GiB peak. Coefficient storage
and tiled FFT evaluation now exist; a [full-dimension storage run](COMPACT_STORAGE_RESULTS.md)
observed2.740GiB sampled RSS. Commitment/opening/FRI orchestration remains pending.

`compact_owned_notes_air` now proves counts 1/2: canonical notes, existing SHA3
owner commitments, one AIR/batch, private buses and program binding. Its 34 owner
bridge/inverse cells must be counted in E unless absorbed by existing regions.
[Bounded runtime evidence](OWNED_NOTES_RESULTS.md) covers 512/1024 rows only.
This milestone does not establish a complete native transaction or membership.
Source references: `compact_keccak_air`, `note_bytes_air`, `recoveryplan/kind.go`,
`proofbundle/outputs.go`, pinned P3 `batch-stark/{prover,transcript}.rs` and
`verifier/mod.rs`. Proof-pipeline regressions pass; native scenario tests/activation remain pending.
