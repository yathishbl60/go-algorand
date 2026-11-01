# Conditional compact wire budget

This is a design calculation, not a selected profile, complete construction,
serialized proof measurement, security approval or native acceptance result.
Native privacy remains disabled in every consensus version. No 8GiB peak is established.

The candidate uses a 4-byte Fp31 base field, degree 10 extension (40 bytes),
192 queries, 10 base-field hiding codewords per private matrix and 10 base-field
salt elements (40 bytes, approximately 310 bits). These parameters and the
required replacement codec are not implemented as a complete proof profile.

| Table | Rows N | Main including 10 pads | Auxiliary before 10 random |
|---|---:|---:|---:|
| Hash |32768|1930|20|
| Arithmetic |65536|410|50|
| Sponge |65536|246|50|
| Sampler |65536|100|50|
| Notes |2048|21|20|
| Key codec |4096|100|30|
| Merkle |4096|100|30|
| Amounts |512|35|30|
| AES |65536|251|90|
| GHASH |32768|291|50|
| Total widths | |3484|420|

Main 3484 means 3384 relation columns plus 100 private pad columns; the additional
100 PCS random columns are separate. Auxiliary 420 receives another 100 random
columns. None of these bus widths is a completed bound for a full construction.
Preprocessing 30 columns per table (P=300) is a placeholder, not an established
upper bound. Actual public schedules must be derived and authenticated.

The baseline schema has main/auxiliary/preprocessing/Q/randomization batches:
5 batches and 32 matrices. The best variant removes the 10 preprocessing
matrices, leaving 4 batches and 22 matrices. Its verifier must independently
evaluate the exact trusted public polynomials at both required points; this
introduces O(NP) public computation and cannot accept prover-chosen schedules.

The target aggregate quotient degree is 524288: hash/GHASH 32768×16 and the
largest 65536-row relations×8. Every actual constraint and hiding adjustment
must satisfy that bound. Relative to Nmax 65536, Q has 8 extension chunks,
packed as 80 base columns plus 10 random columns in ONE matrix. Existing P3
per-chunk hiding instead uses 160 columns; that adds 62048 bytes to this variant
and exceeds 5MB. The packed quotient needs a checked replacement construction.

Private matrices have doubled hiding height and blowup 8, so oracle height is
16N. Required height cuts 20,19,16,15,13 give the maximum 8-ary FRI schedule
20→19→16→15→13→10→7→4→3. Shared salts are per injected height, independently
for each commitment: 5 main groups, 5 auxiliary groups, Q and randomization.
This needs canonical typed row concatenation and hiding/binding analysis.

The codec must deduplicate late FRI rows by transcript-derived indices in
canonical first-occurrence order. Group domains 128,16,8 force repetition.
Same-group queries can exclude different slots: reconstruct the full row from
the representative's running claim and siblings, then check every duplicate's
known slot against that row before folding. Simply discarding repeated sibling
vectors is incorrect. This adapter is not implemented. No worst-case input-row
dedup saving is assumed. Pruned Merkle frontiers retain 64-byte digests.

| Best-variant component | Bytes |
|---|---:|
| Input values |3236352|
| Input salts |92160|
| Input framing |32304|
| Input Merkle paths |606208|
| Public out-of-domain values/framing |316196|
| Hidden out-of-domain values/framing |17076|
| FRI phases and their commitments |671008|
| Remaining outer framing/commitments |401|
| Conditional proof upper bound |4971705|
| Bounded transaction wrapping |15463|
| Conditional complete SignedTxn upper bound |4987168|
| Slack below 5000000 |12832|

Wrapping assumes one aggregate proof slot, body≤10593, NAPW 137 and bundle 17;
it includes all common header fields, note≤1024, genesis ID≤128, optional
signer, and Falcon1024 authorization. Existing Falcon constants give 1793-byte
public keys and signatures≤1423; the PQSig map is≤3244 bytes. The proof already
contains its 73-byte artifact header. Any new statement, descriptor or codec
fields consume the remaining 12832 bytes. Extra transactions need a separate
group/block budget and may be rejected when that budget is exhausted.

For comparison, P300/current-per-matrix salts/binary FRI gives 6058989 bytes;
P300/per-height salts/max8/dedup gives 5429437. These are conditional upper
bounds, not lower bounds proving other designs impossible.

`note_bytes_air` is separately implemented with 11 raw columns, 9 preprocessing
columns and degree 3. Goldilocks and Fp31/D10 manual proofs for note counts
1,3,4 verified. That evidence includes no private bus, pads, native deployment or
complete compact proof. Automated tests remain deferred.

Sources: `internal/assetprivacy/stark/src/verification/{write_fri,write_openings}.rs`,
the pinned P3 hiding PCS/MMCS, `note_bytes_air`, `proofbundle/types.go`,
`witness/format.go`, `nativebody/types.go`, transaction generated MessagePack
encoding, and `crypto/falconWrapper.go`. See [assessment](COMPACT_PROFILE_ASSESSMENT.md).
