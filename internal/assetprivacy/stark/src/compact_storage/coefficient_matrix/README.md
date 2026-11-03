# Honest coefficient-backed evaluation matrix

`CoefficientMatrix::new(source,domain)` creates a logical bit-reversed evaluation
matrix backed by real masked coefficients. Source may be `&MaskedTrace<Value32>`,
an owned MaskedTrace, or `Arc<MaskedTrace<Value32>>`; no self-reference or private
coefficient copy is needed. Its height is the checked target coset size and its
width includes all ten random codeword columns. Shift, height, canonical roots
and logical dimension arithmetic are checked before construction.

`get(row,column)` evaluates the actual coefficient polynomial at the target point
whose natural index is the bit reversal of the committed row index. `row(row)`
does the same for all columns. These honest Horner fallbacks are slow: O(coefficient
height) per cell. They supply no zero/placeholder values or fabricated dimensions.

`into_matrix()` moves the source into the separate checked-row bridge, producing
an actual P3 Matrix implementation with the same values and dimensions. The proof
crate retains its unsafe-code prohibition. `matrix.inner()` gives typed access to
the coefficient source and its bounded fast path.

`gather(indices,tile_columns,max_tile_values,max_output_values,dft)` evaluates each
column FFT tile once, then extracts every requested bit-reversed row. Query order
and duplicates are preserved. All indices and checked scalar output/tile budgets
are validated before output allocation or FFT work. Empty queries with a nonzero
tile width return an empty output without FFTs. No arbitrary query-count cap is
invented; the caller bounds total output cells. Invalid ranges/budgets return None.

`mask()` exposes the private coefficient source for typed backend fast paths.
Neither type supplies Clone, Debug or serialization. Budgets do not include FFT
scratch, vector metadata, all live allocations or total RSS. No PCS selection,
opening-disclosure enforcement or complete prover resource claim follows.
