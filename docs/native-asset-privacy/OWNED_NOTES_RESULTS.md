# Bounded private ownership composition

Recorded 2026-11-02. This is manually executed feasibility validation, not an
automated project test suite or an approved privacy profile. No native private
transaction was accepted; membership/nullifiers/full spending remain absent.

## Exact component and backend

The optional `compact_owned_notes_air` combines canonical512-byte input notes,
the existing SHA3-512 spending-key commitment and indexed private equality buses
in one AIR. Its typed constructor fixes network, asset, policy, group, count and
height; a canonical digest binds the complete verifier-derived periodic program.
The proof has1988 main columns,57 periodic columns and no preprocessed PCS.
Four35-field interactions pack into one degree8 fraction plus one accumulator:
20 auxiliary base coordinates under D10. The sole public lookup terminal is zero.
There are16 quotient chunks. No per-component private terminal is disclosed.

The final run used repository `compact::prover_config()`: Fp31/D10,192 queries,
blowup8, maximum8-ary FRI, ten salts/codewords, fresh OS entropy and the repository
nine-line vendored matrix fallback. Default/native Goldilocks config is unchanged.
Fixtures used independently computed standard SHA3 owner digests, not a substitute
owner commitment or altered envelope profile. No native artifact was installed.

## Observed results

The temporary release worker was CPU/wall bounded to300s, sampled-RSS bounded
to2GiB and used two Rayon workers with core dumps disabled. It exited0 in
**18.438s**, with **353.844MiB sampled peak RSS** on a64GiB physical host.
Sampling can miss transient peaks; this is not validation on an8GiB machine.

Actual proofs verified for `(note count,height)` `(1,512)`, `(1,1024)` and
`(2,1024)`. All reported the geometry above and a zero terminal. For each proof,
changing the verifier network, policy or group was rejected (nine context cases).
Changing a proof's opened trace value was rejected in all three cases.

Host preflight rejected a zero secret, wrong owner and three invalid arity/height
cases. Mutated inverse and program-relay cells failed local constraints.
Three stronger private-link mismatches passed all ordinary local constraints,
then produced nonzero lookup terminals and were rejected by the verifier:

1. Replace valid hash state/secret inverse while preserving the original note
   and owner bridge.
2. Swap two owner words in an otherwise canonical note, preserving hash/bridge.
3. Swap owners between two canonical notes, preserving indexed hash/bridge data.

This checks actual cryptographic lookup rejection beyond host witness validation.
It does not prove all adversarial cases, anonymity, ZK or complete soundness.

## Implementation finding and limits

An earlier run reached its2GiB sampled stop during lookup expression expansion.
The stock resolver recursively expands shared symbolic additions. Replacing
word-packing `v.double()` by algebraically identical `v * TWO` prevents exponential
DAG-to-tree duplication without changing constraints, degree or the lookup engine.
An initial temporary-config run then passed in18.930s/412.859MiB; the published
factory run above is the authoritative configuration result.

Only small512/1024-row ownership programs were executed. Larger permitted heights,
full transaction relations, exact native serialization, complete wire size and
whole-prover8GiB remain open. No proof bytes, private openings, seeds or raw traces
were retained in the repository. No new automated project tests were added/run.
See [universal plan](UNIVERSAL_PROOF_PLAN.md) and [delivery gates](DELIVERY_PROGRESS.md).
