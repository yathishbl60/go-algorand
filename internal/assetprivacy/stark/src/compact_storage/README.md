# Private coefficient storage and quotient strips

Optional `native-privacy-compact` groundwork for the bounded universal prover.
This module does not implement a PCS, a native proof kind or an8GiB RAM guarantee.
Default/native backend selection and consensus settings remain unchanged.

`MaskedTrace::from_trace(trace,domain,dft,rng)` consumes a valid rectangular trace
with512..65536 power-of-two rows. `domain` is its original N-row coset. The caller
must supply fresh private cryptographic entropy; predictable masks destroy hiding.
The constructor uses the stock interleaving: every original row is followed by
its fresh random row, and ten random codeword columns are included. It interpolates
the resulting2N evaluations on the same shift using the pinned DFT.

The stored matrix is in natural coefficient order with relation-width+10 columns.
It is private material, including trace information and masks. It has no Clone,
Debug or serialization implementation. `coefficients()` exposes private data for
backend work; it is not a public statement, commitment or export API. This slice
does not establish one-opening enforcement, zeroization or proof disclosure budgets.

`evaluate_columns(range,target,max_values,dft)` evaluates only the selected columns
on the target coset. It rejects empty/out-of-range tiles, undersized domains,
incoherent canonical roots, overflow and allocation/buffer-budget failures.
`max_values` caps the tile's scalar buffer; it does not cap FFT scratch, all live
allocations or total RSS. The DFT must return an owned row-major matrix, so the
result does not incur the generic matrix-to-owned clone. Output remains private.

`QuotientStrips::new(trace_domain,quotient_domain)` fixes a disjoint16N domain.
It supplies eight2N cosets. Strip j starts at `shift*g_16N^j` and advances by
`g_16N^8`. Its natural row r maps to global row `j+8*r`. Root coherence and all
dimension/index arithmetic are checked. The original N trace domain remains
the domain supplied to constraint selectors; its next-row step in a strip is2.

Pinned batch proving doubles the actual quotient chunk count under hiding:
degree8 yields sixteen committed chunks over a16N evaluation domain. Eight
evaluation strips do not imply eight committed chunks. These are different axes.

Integration still needs coefficient-backed commitment/opening reductions, streamed
Merkle leaves with independent salts, FRI input access and stripwise batch-prover
orchestration. Keep stock verifier/proof mathematics and transcript behavior.
The complete relation, wire bounds and native acceptance remain open.
