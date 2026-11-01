# Complete proof workload accounting

These are fixed source-schedule counts, not runtime benchmarks, native acceptance, approved security parameters or proof that the complete prover fits the user's
8GiB peak-RAM and 5MB transaction-wire targets. Both targets cover viewing-key
registration, spending, recovery and every mandatory role, including preprocessing,
FFTs, auxiliaries and final assembly. Shield is the first feasibility milestone.

## Current fixed Keccak workload

| One envelope component | Keccak permutations |
| --- | ---: |
| ML-KEM key hash and shared-key/coin expansion | 10 |
| Nine matrix polynomials, sixteen blocks each | 144 |
| Seven noise PRFs | 7 |
| AAD hash | 36 |
| HKDF extract and expand, two HMACs | 8 |
| Note commitment | 9 |
| **Complete envelope** | **214** |

Every matrix executes all sixteen blocks even after collecting256 coefficients.
One depth32 membership path uses32 four-permutation node hashes:128 permutations.
Recipient authorization adds viewing leaf20, spending leaf4, eligibility leaf3,
and three paths384:411. Recipient total is625. Disclosure adds viewing leaf20
and one path128:148; disclosure total is362. Shield hashes two outputs itself18.

| Additional role envelopes | Current Shield:18+2×625+r×362 |
| ---: | ---: |
| 0 | 1,268 |
| 2 | 1,992 |
| 4 | 2,716 |

## Conditional reuse in a different joint relation

Verifier-derived public prefix states save two permutations per Merkle node,
two per key/eligibility leaf, two per AAD and one per note. This preserves exact
domain/padding and private absorption/capacity bindings. Computing each output
note once for the main relation and all its envelopes then gives848/1418/1988
private permutations for0/2/4roles, assuming other keys/paths are distinct.
These are conditional design counts, not universal lower bounds or existing code.
Two envelopes proving the same private key, epoch, viewing leaf and path can
share another235: key hash9 + matrix144 + remaining view leaf18/path64.
Two-output auditor/controller pairs could therefore reach1183/1518 for2/4roles.
Exact equality must be proved privately; role labels alone do not establish it.
Fresh encapsulation coins, noise, NTT outputs, AAD, HKDF and AES keys cannot
generally be reused. Public metadata/chunk counts must not reveal private reuse.

## Per-envelope nonhash work

| Logical work | Count |
| --- | ---: |
| Matrix candidate checks / CBD coefficients | 16,128 / 1,792 |
| NTT butterflies / inverse scalings | 6,272 / 1,024 |
| Quadratic polynomial-pair products | 1,536 |
| Modular multiplications / additions-subtractions | 14,976 / 20,736 |
| Ciphertext coefficient compressions | 1,024 |
| AES-256 encryptions / key expansions | 34 / 1 |
| AES S-box applications, including key expansion | 7,668 |
| GHASH field multiplications / serial bit steps | 37 / 4,736 |

Multiply envelope work by2/4/6 for Shield with0/2/4roles. Counts are logical
operations, not rows of a proposed narrow AIR; parallelism trades width for rows.

## Segmentation is not yet a closed resource budget

At24rounds/permutation,2716 permutations occupy65184 hash rows. An8192-row
table holds341 complete permutations, so eight tables cover this Keccak core
only. The2633-column core excludes wrapper, join and context overhead. A possible
max-role ledger also needs three sampling tables at four candidates/row, eight
arithmetic tables at one logical job/row, six AES S-box tables and four GHASH
bit-step tables, each8192rows. Their complete widths are not established.
Sponge absorption, CBD/canonical checks, AES linear/key wiring and authenticated
private joins add work. No complete table list, peak allocation or whole wire
bound exists. Original24869-column recipient globals cannot remain persistent.

## Other mandatory operations

One holder input uses274 current permutations, or140 after public-prefix reuse.
Transfer with two inputs/two recipients has1816+362r current permutations;
conditional joint reuse gives1128+285r:1128/1698/2268 for0/2/4roles.
Unshield has two holder inputs and no output envelopes:548 current,280 reused.
Recovery input is141 current,74 reused; it requires neither holder secret nor
unfrozen eligibility. Partial-private recovery adds recovered recipient494,
retained recipient362 and two main output hashes18:1015+362r current,
726+285r reused. Full-private/full-public/partial-public reused bases are433/74/367;
their role-envelope arity follows their actual output count, not always0/2/4.
Viewing registration proves seven private permutations: seed expansion1/noise6,
six forward NTTs and the existing four-dot frame. Its public rho permits public
matrix derivation144; these are outside the private AIR and may be cached only
against the exact public key. Owner registration uses two permutations, one after
public-prefix reuse. Native authorization, tree updates and scanning remain work.

Sources: [KEM](../../internal/assetprivacy/stark/src/mlkem_encapsulation_air/params.rs),
[derivation](../../internal/assetprivacy/stark/src/mlkem_stream_derivation_air/README.md),
[matrix](../../internal/assetprivacy/stark/src/mlkem_stream_matrix_air/README.md),
[AAD](../../internal/assetprivacy/stark/src/envelope_aad_air/README.md), [HKDF](../../internal/assetprivacy/stark/src/hkdf_air/mod.rs),
[note](../../internal/assetprivacy/stark/src/note_air/mod.rs), [membership](../../internal/assetprivacy/stark/src/membership_vertical/README.md),
[PKE](../../internal/assetprivacy/stark/src/mlkem_pke_air/README.md), [NTT](../../internal/assetprivacy/stark/src/mlkem_ntt_air/program.rs),
[product](../../internal/assetprivacy/stark/src/mlkem_product_air/relation.rs), [GCM](../../internal/assetprivacy/stark/src/gcm_vertical/README.md),
[holder](../../internal/assetprivacy/stark/src/spendable_note_air/README.md), [recovery](../../internal/assetprivacy/stark/src/recovery_group_air/README.md),
[registration](../../internal/assetprivacy/stark/src/mlkem_keygen_air/README.md). See [whole-transaction limits](WHOLE_TRANSACTION_FEASIBILITY.md).
