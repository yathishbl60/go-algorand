# Private seed-to-matrix polynomial relation

`MlKemMatrixAir::candidate(first, second)` composes SHAKE128 and the first-256
rejection sampler in a single hiding AIR. It fixes the candidate admission
bound at 16 rate blocks. `new(first, second, blocks)` also supports 1–16 blocks
for verifier-fixed component layouts; the native candidate must use 16.
Coordinates are exactly the appended bytes, without an implicit transpose:
Go's A[i,j] uses first=j, second=i. K-PKE's A-transpose binding remains a parent
obligation. Every seed, squeezed byte and coefficient remains private.

The SHAKE output and sampler prefix use the same physical columns. Both
component constraints execute against those columns inside one AIR; no public
digest, compressed private lookup terminal or host-only equality connects them.
The result is the first 256 consecutive 12-bit candidates below 3329 from
SHAKE128(rho||first||second), in order. After 256 accepts the cursor absorbs;
the rest of the prefix cannot replace or reorder outputs.

`trace(seed32)` returns one composed private matrix, or None if the fixed prefix
contains fewer than 256 accepted candidates. No fallback, reseeding or polynomial
padding occurs. `seed_bits(row)` and `coefficients(row)` expose private bindings.
The enclosing KEM must bind rho to its actual canonical encapsulation key and
share rho across all nine coordinate entries before using them in vector products.
This relation does not establish key registration, note ownership or a full KEM.

Candidate key admission additionally requires all nine entries to finish within
16 blocks. Otherwise standard-valid keys are rejected if a prefix exhausts;
coverage of all standard keys has not been established. This is a protocol
eligibility rule; ML-KEM encryption and its sampling rule are unchanged.
Generation/restoration helpers do not silently retry. Callers must explicitly
check admissibility before registration/use. A transaction may not choose the
bound or preprocessing. See [profile](../../../../../docs/native-asset-privacy/CANDIDATE_PROFILE.md).

With b blocks, main width is 3428+1344*b. Height is the sampler's
max(512,next_power_of_two(112*b+1)). SHAKE is promoted to that height, preserving
all rate/capacity continuation constraints. Preprocessing concatenates b squeeze
end flags and 112*b candidate flags with the same height. The internal builder
forwards the same row boundaries and slices both main/preprocessed columns;
its compiled children require no public or periodic values.

At the candidate bound, main width 24,932 and height 2048 use 408,485,888 raw
element bytes. Preprocessing width 1808 adds 29,622,272 bytes. Trace generation
allocates the destination once, writes SHAKE using a 43,139,072-byte Keccak
temporary, then writes sampler rows directly. It avoids retaining a second full
SHAKE/sampler matrix. Allocator, hiding and proving overhead are additional;
these are static counts, not peak-memory measurements or usable protocol limits.

Use trusted exact preprocessing at base log-height 11 for the candidate, extended
to 12 by the hiding configuration. Other fixed component bounds use their actual
height. Full profile artifact generation, security/resource analysis and native
verification remain unfinished. Builds/Clippy/source/format/size checks only;
no traces, vectors, constraint tests or proofs ran.
