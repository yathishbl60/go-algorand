# Hybrid compact input MMCS

`InputMmcs` combines the pinned stock hiding Merkle implementation with an
explicit coefficient-backed fast path. Its generic `Mmcs::commit` delegates to
stock for every Matrix type and mixed-height batch. It never substitutes false
dimensions, placeholder rows or partial data to trigger optimized behavior.

`commit_coefficients(coefficients,domain,limits)` takes owned Arc references to
one to sixteen real masked coefficient matrices of equal coefficient height.
It stores real `SafeMatrix<CoefficientMatrix<Arc<_>>,Value32>` values and a streamed
tree over the fixed target coset. The stored matrices include all ten random
codeword columns. Coefficient buffers are shared, not cloned or self-referenced.

`StreamedInputLimits` fixes column tile width, scalar FFT tile capacity and scalar
query-output capacity. Output capacity means query count times the sum of every
committed matrix width. That aggregate bound and all query indices are checked
before any FFT gathering/output allocation. Query order and duplicates remain
unchanged; each column tile is evaluated once per matrix for a multi-opening.

`try_open_multi` and `try_open_batch` report shape, budget, allocation or private
random-state failures. Dense queries also validate indices and checked output
arithmetic before stock access; dense allocation behavior remains stock's.
The upstream Mmcs methods have no fallible return type, so they require valid
prebounded calls and panic on a rejected request. They never fabricate success.

Streamed proofs use the exact stock hiding-MMCS proof types: independent salt
rows and either full binary paths or minimal pruned boundary digests. The binary
frontier sorts/deduplicates indices only for authentication-path pruning; returned
rows/salts keep all original queries. Both verification methods delegate entirely
to the pinned stock verifier, including trusted dimensions and width enforcement.

`InputMmcs::new()` obtains independent OS-seeded dense and streamed salt sources.
Clones share monotonic Arc-backed RNG states. A streamed commitment draws one
fresh private local seed while locked, then releases the lock before FFT/hash
work. Poisoned state is an error; no weak/default entropy fallback exists.

No private prover data implements Clone, Debug or serialization. No HidingMmcs
marker is asserted yet. This stage does not select a PCS or establish one-use
masking/opening budgets, a full proof, total RSS, or native transaction validity.
Those remain explicit requirements of the next backend integration boundary.
