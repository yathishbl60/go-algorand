# Multipass prover allocation draft

Updated 2026-11-01. Proposed allocation policy, **not implemented or measured**.
The complete local prover must stay within8GiB peak RSS for all supported
operations and roles. These buffer caps are obligations to establish, not proof
that this target is met. Stock batch-STARK cannot provide this schedule directly.

## Pinned ownership facts

At P3 revision1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f, the owned radix-two DFT
consumes its matrix and operates in place. Owned dense conversion moves its Vec;
borrowed conversion copies. Buffer growth may temporarily retain the old masked
matrix while allocating the full LDE. Preallocation or a transient cap is needed.

The evaluation getter borrows only when the requested shift is Val::GENERATOR
and its size fits the stored LDE. Its other path reconstructs coefficients and
allocates a new matrix while retaining the original. The largest hash phase
must forbid that fallback. Hiding commit copies the base into its masked matrix.
Stock batch proving clones input traces and retains all commitment matrices;
stock hiding MMCS also retains salts and tree layers. New streaming adapters are
therefore required. DFT consumer hooks alone still allocate the full output.

## Conditional largest hash allocation

For32768 rows, raw core1920 plus10 private pads and10 hiding columns, with4-byte
base values, the base is252968960 bytes; masked matrix508559360; main LDE
4068474880. These are241.25/485/3880MiB. Auxiliary20 plus10 hiding columns adds
62914560 bytes,60MiB. A [full-capacity raw-core proof](FULL_CAPACITY_HASH_RESULTS.md)
now observed5.504GiB RSS; it excludes pads/joins and does not establish this full allocation policy.

| Controlled category | Proposed MiB cap |
| --- | ---: |
| Largest main LDE |3880 |
| Concurrent same-instance auxiliary LDE |120 |
| Two active height-group row-hashing buffers |512 |
| Main/auxiliary/quotient tree layers |384 |
| Public-program quotient evaluation buffers |128 |
| Global/current quotient vectors |80 |
| Regenerated salts |80 |
| FFT twiddle caches |64 |
| Worker/symbolic scratch |128 |
| Seeds, claims, query rows and encoded proof |16 |
| Masked-buffer reallocation transient |485 |
| **Total** |**5997** |

A6144MiB controlled cap would leave2048MiB within8GiB for allocator/runtime
overhead. Every category still needs enforcement and adversarial measurement.
Public preprocessing omitted from PCS is not free: its prover evaluation and
verifier reevaluation need their own bounded implementation. Largest program
widths and worker allocations are unresolved;128MiB is not an established bound.

## Required phase order

1. Commit main matrices one height group at a time, in fixed within-height order.
   Incrementally hash rows, discard each LDE and finalize tree injections. Retain
   only commitments and fresh secret regeneration seeds for that complete proof.
2. After all main caps and join challenges, regenerate witnesses and auxiliary
   traces. Use independent commitment/salt namespaces and the same bounded order.
3. After all required caps and constraint-folding challenges, regenerate one
   main/auxiliary pair. Borrow exact-domain views, accumulate global quotient
   values and discard the pair. Commit quotient data in a separate phase.
4. Regenerate matrices individually for ordered OOD claims, dropping each before
   sampling the PCS batching challenge. Preserve exact opening and alpha order.
5. Regenerate again to accumulate reduced opening vectors by height. At maximum
   LDE height2^20, one extension vector is40MiB; distinct heights total<80MiB.
6. Commit FRI phases and derive queries, retaining their smaller codewords/trees.
7. Regenerate each input LDE/tree for selected rows, salts and authentication
   paths. Check its cap equals the original before emitting any proof.

Opening inverses and compressed current-matrix rows require a separate phase
envelope: current caching can cost120MiB plus40MiB. They need not coexist with
quotient/program buffers. Process height groups sequentially; retaining sponge
states at all heights invalidates the proposed512MiB cap. Budget256 bytes per
active row state, rather than only200 bytes of raw Keccak lanes.

All passes belong to **one** opening transcript. Independent stock `open` calls
per regenerated matrix change the protocol. Masks, private pads, salts and
witnesses must regenerate byte-for-byte, with fresh seeds for each complete
proof/retry. No secret seed or trace may enter public preprocessing or logs.
Cache geometry, salt sharing, full private-join hiding and global quotient
soundness still require validation. [Wire obligations](COMPACT_WIRE_BUDGET.md)
must hold simultaneously; no accepted native transaction is demonstrated.
