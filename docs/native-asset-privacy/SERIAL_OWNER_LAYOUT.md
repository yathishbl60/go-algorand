# Candidate serial Owner registration layout

Updated 2026-11-01. This is the implemented Owner-only design and resource gate.
It does not resolve the recipient/encryption layout, produce approved artifacts,
or demonstrate a native transaction. [Bounded primitive results](SERIAL_OWNER_RESULTS.md) now exist.
Every consensus version stays disabled and no suite approval is implied.

## Preserve the relation and cryptographic parameters

Owner remains SHA3-512 of the spending-key domain, network and nonzero secret,
with the exact finalized group bound separately. The 109-byte hash message needs
two sponge blocks at SHA3-512's 72-byte rate. Use all 24 Keccak rounds per block,
standard padding and output order from [FIPS 202](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf), sections 3, 5 and 6.1.
FIPS specifies the hash; it does not validate this AIR or its implementation.

The proposal serializes each round into Theta, Rho/Pi, then Chi/Iota stages:
2 blocks × 24 rounds × 3 transitions = 144 transitions, plus absorb/boundary rows.
The frame remains 512 rows. Main width1953 is state1600 + C-parity320 + inverse1
+ group32. Preprocessing width11 is schedule4 + seven round-constant bits.
All state bits must be Boolean. Each C parity is an ungated XOR of five state
bits, so its nominal degree is five. Stage selectors must not multiply this
parity constraint. Secret nonzero, capacity/prefix/padding, block absorption,
round permutation, final digest and group binding must all remain constrained.

The target effective symbolic degree is at most five, including selector products.
The bounded run measured degree five; an informal expression
count or an incorrect degree hint is insufficient. Current Goldilocks/extension,
SHA3 commitments/transcript, 192 queries, five hiding codewords/salts, binary FRI,
blowup2^3 and final polynomial length1 remain unchanged. No security parameter
reduction or alternate hash is part of this proposal.

## Conditional complete canonical proof bound

Assume N=2^n=512, W=1953, P=11, Q=192 and effective degree d<=5.
Pinned P3 uses q=2×next_power_of_two(d), so q<=16 quotient chunks.
All four input batches have LDE height H=16N=8192 (h=13): random width10,
main W+5, q quotient matrices width10 each, preprocessing widthP.
There are R=h-3=10 binary FRI rounds. Assume both main/preprocessing next-row
openings, one root per commitment, five base-field salts per matrix/query,
field/challenge/digest encodings of8/40/64 bytes and four-byte list lengths.

For a binary tree of depth k, a conservative maximum pruned sibling count is:

```text
m = min(Q, 2^(k-1)); S(k,Q) = 2-m + sum(j=1..k-1, min(m,2^j)); S(0,Q)=0
I = 48 + Q*(296+128*q+8*(W+P)) + 256*S(h,Q)
F = R*(12+92*Q) + 64*sum(k=3..h-1, S(k,Q))
U = 1225 + 80*(W+P) + 412*q + 72*R + I + F
```

I includes every input row, matrix/list prefix, salt and all four multiproofs.
F includes every FRI sibling-value list, salt and multiproof. Maximizing each
proof separately is conservative even when those maxima cannot occur together.
Here S(13,192)=1024 and sum(S(3..12,192))=2428.

| Complete proof component | Upper bound, bytes |
| --- | ---: |
| Magic, artifact identity and degree byte | 73 |
| Three commitment roots | 192 |
| OOD main/preprocessing/quotient/random openings | 160608 |
| Wrapper random-codeword openings | 3980 |
| Full FRI encoding, including final polynomial and POW slots | 4061892 |
| OOD POW slot | 8 |
| **Canonical NAPSTK proof U** | **4226753** |

The FRI subtotal includes input openings3728948 and phase openings332156, with
outer list prefixes; its remaining fields total788 bytes. These are analytical
bounds on this exact layout/configuration, not measured proof sizes.
Reserving 256KiB for one Owner-registration transaction gives4488897 bytes,
conditionally below the 5000000-byte gossip cap by511103. That reserve covers
NAPB/NAPW framing, body, PQ envelope and ordinary header metadata for one chosen
transaction; it is not an allowance for an arbitrary full transaction group.
Exact serialized native acceptance still requires separate verification.

## Nominal matrix dimensions

| Matrix | Bytes, excluding allocator and other temporaries |
| --- | ---: |
| Main 512×1953×8 | 7999488 |
| Preprocessing 512×11×8 | 45056 |
| Hiding-expanded main 1024×1958×8 | 16039936 |
| Main LDE 8192×1958×8 | 128319488 |
| Preprocessing LDE 8192×11×8 | 720896 |
| Sixteen quotient LDEs 8192×160×8 | 10485760 |
| Randomization LDE 8192×10×8 | 655360 |

These dimensions are not total or peak-memory budgets. Extra quotient evaluation,
Keccak construction, FRI, Merkle data and allocator overlap remain additional.
The current [typed prover](LOCAL_PROVING.md) limits only the owned main trace.

## Source and evidence boundary

The bound follows pinned [P3 symbolic quotient sizing](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/uni-stark/src/symbolic.rs),
[FRI prover/openings](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/fri/src/prover.rs),
[binary pruned paths](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/merkle-tree/src/mmcs/mod.rs),
and the local [canonical writer](../../internal/assetprivacy/stark/src/verification/write_fri.rs).
The [earlier primitive evidence](FEASIBILITY_RESULTS.md) covers the older Owner;
[serial-layout evidence](SERIAL_OWNER_RESULTS.md) excludes the artifact-bound API. Full registry pins,
native authorization, runtime rejection cases and node acceptance remain separate.
