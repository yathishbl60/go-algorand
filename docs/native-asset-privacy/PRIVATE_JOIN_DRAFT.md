# Segmented private joins: mathematical draft

Updated 2026-11-01. DRAFT: algebraically reviewed candidate, not selected,
implemented, security approved, or a complete protocol. No source backend is
adopted here. Native privacy remains disabled; no approved suite or activation
exists. The complete workflow must meet 8 GiB peak RAM and a 5,000,000-byte whole
transaction, including every mandatory disclosure role and PQ authentication.
See [the whole-transaction gate](WHOLE_TRANSACTION_FEASIBILITY.md).

## Candidate shape

Retain univariate AIR relations, private random trace rows and hiding quotient
semantics. Explore a univariate adapter to a hiding multilinear PCS rather than
exposing raw FRI query rows. Segment complete computation into narrow chunks;
height-only segmentation of current wide AIRs does not establish feasibility.
Private joins, authorized keys, note openings and epochs remain private.

Let F be Goldilocks and E its pinned degree-five extension. Each chunk i has an
original domain H_i of size N_i, with Z_i its base-field vanishing polynomial.
Require nested domains with H_min contained in every H_i and Z_min dividing Z_i.
Domain shifts, degrees, chunk ordering and every relation are verifier-bound.

## Terminal pads and the exact global quotient

For each bus choose five base-field pad coordinates r_i,j per chunk, j=0..4,
uniform subject to sum_i r_i,j=0 for each j. Enforce each pad column constant on
its original domain. Its masked polynomial is R_i,j(X)=r_i,j+Z_i(X)S_i,j(X),
where S_i,j is uniform over F-polynomials of degree below N_i.
Add each public fixed dummy tuple exactly once per chunk at an enforced first
row, with multiplicity r_i,j; repeating it per row would change the cancellation.

Shared bus challenges give D_j=alpha+beta*j in E, identical across chunks.
Reject beta=0 or theta=alpha/beta in F, using a specified verifier-replayed rule.
For accepted challenges, theta has degree five because the extension degree is
prime. A dependence among 1/D_j yields a polynomial of degree at most four
vanishing at theta; evaluating that zero polynomial at -j forces every
coefficient to zero. Thus these five inverses form an F-basis of E.
The terminal pad sum_j r_i,j/D_j can hide any one E-valued chunk terminal;
zero-sum coordinates preserve the public aggregate across chunks.

Commit the EXACT Q_j(X)=sum_i R_i,j(X)/Z_min(X) before the shared OOD challenge z.
When pad coordinates sum to zero, Q_j=sum_i (Z_i/Z_min)S_i,j, a polynomial.
Require a verifier-bound degree at most max_i(2*N_i-1)-|H_min|, exact constancy
constraints on R_i,j, and authenticated openings from those same commitments.
Check sum_i R_i,j(z)=Z_min(z)Q_j(z). Q must not be chosen after z, independently
remasked, or replaced by a fresh commitment during this check.

## Limited ideal opening-view argument

Open each R_i,j only at shared z and z*g_i, where g_i preserves H_i, and open
Q_j ONLY at shared z. Reject OOD points in H_i. Then Q_j(z) is redundant with
the shared-z R openings; no extra Q point or coefficient disclosure is allowed.
For any base-field zero-sum change delta r_i,j, let L_i be the least common
multiple of the minimal polynomials of z and z*g_i over F. Its degree is at
most ten; Z_i is invertible modulo L_i. Choose
delta S_i,j=-delta r_i,j * inverse(Z_i) modulo L_i, of degree below ten.
For N_i>=10 this translates the uniform mask space bijectively, preserving both
R openings, Q(z), and the zero-sum relation. Basis coordinates also give a
zero-sum pad compensation between terminal vectors with equal public aggregate.

This establishes only an ideal opening-value lemma. The translation changes
polynomials and commitment hashes. Adaptive PCS composition, Fiat-Shamir,
auxiliary/accumulator openings, quotient masking, randomization polynomials and
the complete AIR transcript still need a joint zero-knowledge proof.

## Univariate evaluation through coefficient multilinear extension

For an EXACT coefficient message (a_0,...,a_(2^k-1)), commit its corresponding
polynomial p(X), with its coefficient order and degree bound authenticated.
Set r_j=z^(2^j)/(1+z^(2^j)) and s(z)=product_j(1+z^(2^j)). Then
p(z)=s(z)*MLE(a)(r_(k-1),...,r_0) for P3's MSB-first table order. Reject zero denominators, including for
translated opening points. Trace evaluations or LDE entries are not coefficients.
Coset conversion, quotient chunks and preprocessing must preserve exact meaning.
Column batching needs another authenticated claim-binding reduction; issuing
many direct column points is not a completed adapter or a resource argument.

A public-vector check using the pinned field types exited0: basis rank5 and both
512/1024-domain correction systems rank10; pad/opening/terminal/Q identities held.
Actual P3 Poly evaluation confirmed reversed/MSB-first point order; the unreversed
case failed. No commitment, private witness or proof ran; this does not test privacy.

## Regeneration and mandatory gates

Two/multipass proving may retain fresh OS-backed private seeds and regenerate
each chunk to its identical commitment within ONE complete proof session.
Bind all main commitments before bus challenges; bind auxiliaries and Q before
their applicable later challenges. Specify phase/chunk-separated randomness,
one opening schedule and an abort policy. No mask/seed reuse across new proofs,
queries or retries; regeneration is not permission to open a commitment again.
All input witnesses, regeneration buffers and setup belong in the peak budget.

Before selection, specify every chunk and private join, degree/constancy/range
constraints, mask disclosure budgets, bounded codecs and challenge exclusions.
Provide composed soundness/ZK/hash/transcript analysis with explicit WHIR rates,
queries and assumptions; existing FRI settings or nominal WHIR targets do not
transfer automatically. Pinned hiding-WHIR diagnostics do not certify its full
hiding reduction. Prove whole-wire and full-pipeline RAM bounds for all required
operations/roles. This draft supplies no prover, acceptance path or security
approval; algebraic plausibility does not satisfy those gates.
The [D10 guard/rank evidence](COMPACT_PRIMITIVE_RESULTS.md) does not transfer this D5 lemma into a complete D10 hiding proof.
