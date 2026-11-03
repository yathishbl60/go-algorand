# Exact stripwise quotient evaluation

Select `StripQuotient` with the pinned batch prover's explicit scheduling entry:

```rust
let proof = p3_batch_stark::prove_batch_with_quotient_scheduler(
    &config, &instances, &prover_data, &coefficient_pcs::StripQuotient,
)?;
```

`config` is `coefficient_pcs::Config`. Ordinary `prove_batch` keeps its original
whole-domain behavior and instance parallelism. No verifier choice changes.

This scheduler supports the candidate's original N-row trace and disjoint16N
quotient domain. Eight shifted2N cosets cover it exactly. Strip `j`, local row
`r`, writes full quotient row `j+8*r`. The callback always receives the original
N-row selector domain and each actual shifted strip domain. The stock evaluator
computes the same constraints, next-row accesses, periodic polynomials, lookup
relations and vanishing division. Quotient chunks and correlated masks use the
existing downstream implementation unchanged.

Strips and candidate instances execute sequentially. There is no lock held
across Rayon work; nested parallel constraint evaluation remains available.
The preflight checks actual committed matrix widths/heights, supported geometry,
preprocessing exclusion and the current evaluator PCS limits. Before any callback
or quotient allocation, it bounds known retained field payloads: main and lookup
evaluations, three periodic copies, four selectors, strip output and full quotient
output. Extension values count ten base scalars. This excludes FFT scratch,
constraint metadata/task buffers, vector overhead and other live prover phases;
the cap is not a process RAM limit or an8GiB proof.

Public preprocessing and other quotient geometries reject explicitly. Bad callback
lengths reject instead of silently omitting or reordering rows. A scheduler is
trusted local prover code; every final proof still needs the stock verifier.

`coefficient_quotient_strips` compares all full-versus-strip quotient values using
the same private data/challenges, including full-period public columns and lookup
constraints. It also generates four stock-verified ownership proofs; constrained
evaluation budgets admit strips while excluding a whole wide domain. See
[the result ledger](../../../../../../docs/native-asset-privacy/COEFFICIENT_QUOTIENT_RESULTS.md).
