# Compact FRI candidate assessment

Updated 2026-11-01. Design investigation only: **not selected or implemented**.
It changes the field, hash profile, private joins, quotient packing and prover
schedule. No native transaction, artifact approval or end-to-end result follows.
The user's target is **8 GiB peak RAM for the entire local prover**, all operations
and roles. Existing transaction transport remains5000000 bytes. Both gates must
close before expanding this candidate into another full implementation.

## Field arithmetic evidence

Candidate base p=2013265921 with extension `Fp[X]/(X^10-11)` has about 309.069
bits of field cardinality. Four-byte base encoding, ten random codewords and ten
salt coordinates would replace the current eight-byte/quintic representation.
This is not a type alias or an established quantum security estimate.
[Public metadata and certificates](COMPACT_FIELD_METADATA.json) record the full
factorization of p^10-1, primitive extension generator 2+X, Frobenius constants
and a compatible2^28 root 17094607X^5, squaring to BabyBear's2^27 root 440564289.
The initially considered 121X^5 has the right order but wrong square.
11 is not primitive in Fp; its binomial nevertheless is irreducible: for every
root a, a^p=z*a where z=1403701133 has exact order 10, so its orbit has length10.
Two separately written public-arithmetic checkers verified the complete factor
product, 51 recursive full-factor Lucas primality certificates and all generator
order exclusions. These are mathematical checks, not field-wrapper validation.
Pinned P3 Monty31 scalar extension multiplication supports only degrees 4/5/8.
A local wrapper with generic D10 multiplication is structurally possible, but
requires many field traits and validated serialization. Its required unsafe
trait-method declarations conflict with the current proof crate's forbid lint,
even with safe method bodies. No lint policy or dependency was changed.

## Hash profile and private binding

Standard cSHAKE256 with 64-byte output could precompute public customization S;
N must be empty, private X follows the byte-padded prefix, suffix 0x04/final 0x80.
This changes SHA3-512's capacity/profile and needs a fresh complete security
argument. Output length alone does not preserve the previous security claim.
See [NIST SP800-185, sections 3.3-3.4 and 7.1](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf).

To retain full-key binding, compute private Cpk=cSHAKE(PK[1184],64 bytes,S_key), costing
nine permutations. A viewing leaf then includes account[32]/epoch[8]/created[8]/Cpk[64]
and costs one. AAD private X=epoch[8]/Cpk[64] costs one; public S binds canonical
public context[248] and KEM ciphertext[1088] with typed framing. The later-generated
GCM nonce[12] and GCM ciphertext/tag must stay outside this pre-KDF hash. Existing
public Context.Nonce32 remains bound. Registration, KEM and AAD must share the
same canonical key/Cpk through proved private equality. ML-KEM's own key hash
remains separate. No canonical new profile bytes have been frozen.

| Operation |0/2/4 additional role-envelope permutations |
| --- | ---: |
| Shield |564 /793 /1022 |
| Two-input/two-output transfer |706 /935 /1164 |
| Partial-private recovery |502 /731 /960 |

These conditional counts share each output-note hash and prove equal authority
key/epoch/leaf/path across each role pair. Public prefix processing is additional.
No private reuse may be inferred from role labels or leaked through chunk counts.

## Narrow table evidence and remaining costs

A proposed one-round Keccak core carries 100 packed input words, 320 parity bits
and 1600 post-Theta bits: 2020 columns. XOR parity equations reconstruct the input;
Rho/Pi/Chi/Iota produce the next state. A disposable public worker matched 20
SHA3-512 and 20 SHAKE256 messages against Python's standard library, including
padding boundaries and multi-block squeezing, and checked parity-map rank 320.
This validates public algebra only: no new AIR, hiding proof or native execution.
With 10 private pad columns and 10 hiding columns, 32768 rows have main LDE
`16*32768*2040*4=4278190080` bytes, about 3.984 GiB. That is one allocation only.

An AES schedule can budget 64272 rows for six envelopes, including S-boxes,
round/key XOR, MixColumns, ciphertext/tag XOR and 138 word/byte bridge rows per
envelope. It leaves 1264 rows in 65536 before final scheduling/padding obligations.
241 scratch columns plus 10 pads are conditional. Fixed linear bus ports are
essential: opcode-multiplexed payloads can raise lookup degree and break the
assumed quotient bound. Program-address/preprocessing widths are not closed.

## Whole-proof wire check

Ten tentative tables total 3893 main columns including private pads, 460 auxiliary
columns before hiding, and placeholder preprocessing 30 per table. At 192 queries,
4-byte base values, 40-byte extension values and 64-byte digests, conditional codec
bounds are 6439741 bytes with binary FRI, 6113293 with maximum 4-ary folds,
and 5961245 with maximum 8-ary folds. These already exclude transaction reserve.
They are **conditional upper bounds, not impossibility lower bounds**; the actual
full AIR widths and public-program representation remain unspecified.

Even the optimistic 8-ary variant omitting preprocessing PCS via trusted verifier
reevaluation and salting once per mixed-height group budgets 5359897 bytes.
Neither modification exists. Public reevaluation adds verifier work; salts cannot
simply be reused across independent commitments or differently injected rows.
A packed global quotient assumes one 80-column matrix plus 10 hiding columns;
current P3 instead randomizes eight separate chunks to 160 committed columns.
The optimistic packing therefore also needs a checked new protocol path.

The proposed multipass prover must regenerate identical fresh masked commitments,
stream quotient/FRI accumulators and retain bounded hash/tree state. No allocation
timeline or measured whole-pipeline8 GiB peak exists. Fp31 also requires four 16-bit
amount limbs with separately bounded carries; current 32-bit limb equations cannot
be reused. D10 private-join masks need rank 10/CRT-degree<20 checks and a complete
hiding/Fiat-Shamir argument, beyond the existing limited D5 draft.
Neither complete resource gate is closed; profile/defaults are unchanged. [Delivery status](DELIVERY_PROGRESS.md).
