# Compact FRI candidate assessment

Updated 2026-11-02. The full replacement profile is **not selected or composed**.
Preferred direction: [one universal AIR](UNIVERSAL_PROOF_PLAN.md) preserves the
existing SHA3/envelope contracts. [Joint ownership proofs](OWNED_NOTES_RESULTS.md)
verified using the published compact factory in18.438s/353.844MiB sampled RSS.
The historical cSHAKE/multi-AIR budgets below are unselected and must not be
combined with the universal plan's stock wire estimate or treated as requirements.
Optional field arithmetic and generic byte/hash components now exist, with
[bounded primitive proofs](COMPACT_PRIMITIVE_RESULTS.md). No native transaction,
artifact approval or end-to-end result follows. The complete local prover must
fit **8 GiB peak RAM**, including all operations, roles and proving phases.
Transaction transport remains 5,000,000 bytes. Both complete gates remain open.

## Implemented arithmetic and narrow components

The optional `native-privacy-compact` feature exports a scalar four-byte field
with p=2013265921 and degree-ten extension `Fp[X]/(X^10-11)`, about309.069 bits
of cardinality. Default/native Value, Challenge and Config remain unchanged.
The [public certificates](COMPACT_FIELD_METADATA.json) establish field metadata;
implementation checks do not establish a post-quantum security estimate.
The arithmetic crate delegates prime operations, supplies generic D10 products
and rejects noncanonical serialization. A full-degree helper excludes both
maximal proper subfields; merely checking non-base membership is insufficient.

The proof crate retains `forbid(unsafe_code)`. The separate arithmetic crate
permits P3's ten required unsafe trait-method declarations, each with a safe
reducing body; no unsafe block or pointer cast was added. Its sources and
manifest are included in implementation fingerprinting.

`note_bytes_air` has11 raw columns,9 trusted preprocessing columns and degree3.
It checks canonical opening bytes with immutable asset/policy/note-count scope.
`compact_keccak_air` has1920 raw columns,11 trusted preprocessing columns and
degree5. Boolean post-Theta/parity state reconstructs the original input words;
Rho/Pi/Chi/Iota define output expressions and fixed round continuations.
Neither primitive alone implements a whole statement. `compact_owned_notes_air`
joins both with indexed private buses,57 periodic columns and20 auxiliary cells.

Unmodified pinned P3 panics during D10 opening compression because a matrix
kernel assumes at most eight coordinates. The validated nine-line fallback is
now vendored with pinned provenance and included in implementation fingerprinting.
See [runtime evidence and limitations](COMPACT_PRIMITIVE_RESULTS.md).

## Historical alternative hash profile (unselected)

Standard cSHAKE256 with64-byte output could precompute public customization S;
N must be empty, private X follows the byte-padded prefix, suffix0x04/final0x80.
This changes SHA3-512's capacity/profile and needs a fresh complete security
argument. Output length alone does not preserve the previous security claim.
See [NIST SP800-185, sections3.3-3.4 and7.1](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf).

To retain full-key binding, private Cpk=cSHAKE(PK[1184],64 bytes,S_key) costs
nine permutations. A viewing leaf account[32]/epoch[8]/created[8]/Cpk[64] costs
one. AAD private X=epoch[8]/Cpk[64] costs one; public S binds canonical public
context[248], KEM ciphertext[1088] and typed framing. Later-generated GCM
nonce[12], ciphertext and tag stay outside this pre-KDF hash. Existing public
Context.Nonce32 remains bound. Registration, KEM and AAD must prove equality
of their canonical key/Cpk. ML-KEM's own key hash remains separate.
No new canonical profile bytes or cSHAKE relation have been frozen.

| Operation |0/2/4 additional role-envelope permutations |
| --- | ---: |
| Shield |564 /793 /1022 |
| Two-input/two-output transfer |706 /935 /1164 |
| Partial-private recovery |502 /731 /960 |

Counts share each output-note hash and prove equal authority key/epoch/leaf/path
across each role pair. Public prefix work is additional. Metadata/chunk counts
must not leak private reuse. AES can tentatively fit64272 rows for six envelopes,
including word/byte bridges; its fixed linear ports/program remain unimplemented.

## Historical multi-table resource sketch (unselected)

The earlier [conditional wire ledger](COMPACT_WIRE_BUDGET.md) totals4971705 proof
bytes plus15463 transaction wrapping, leaving12832 below5MB. It depends on exact
public-polynomial reevaluation, per-height shared salts,8-ary FRI, correct derived
query deduplication, a packed global quotient and all assumed table widths.
Those changes are not a complete implemented construction or security proof.
Earlier2020-column/330-column hash/note sketches are superseded by the narrower
representations; their larger budget is not the current candidate calculation.

For32768 hash rows,1920 core +10 private pads +10 hiding columns gives a main
LDE of4068474880 bytes,3880MiB. That is one allocation, not total peak RAM.
The [multipass allocation draft](STREAMING_PROVER_DRAFT.md) proposes5997MiB of
controlled buffers, with unverified caps. A [full-capacity hash run](FULL_CAPACITY_HASH_RESULTS.md)
observed5.504GiB RSS; it does not validate the complete allocation policy.
Stock batch proving retains/clones data incompatible with the intended schedule.

Fp31 also requires four16-bit amount limbs and separately bounded carries;
existing32-bit limb equations cannot be reused. Private joins, all constraints,
quotient/Fiat-Shamir/hiding arguments, exact native codecs and complete proving
must close together. Current consensus defaults remain disabled and no compact
artifact/native verifier is selected. [Authoritative delivery status](DELIVERY_PROGRESS.md).
