# Candidate proving resource constraints

Updated 2026-10-24. These are static column/element counts, not benchmark results.
No proofs, timings or peak-memory measurements have run. The candidate must not
be activated before realistic prover and deterministic verifier budgets exist.

## Existing components

One 32-level membership base matrix contains 640,548,864 bytes of Goldilocks
elements. Generating/copying/dropping one hash stage at a time reduces matrix
overlap during assembly from 1,276,641,280 to 671,211,520 bytes. Allocator overhead,
hash state, commitments, hiding expansion and prover temporaries are additional.

Transfer and unshield builders now allocate their destination once, then generate,
copy and drop each component separately. Transfer no longer holds all four source
matrices alongside the destination. Its final horizontal matrix is unchanged;
this optimization alone does not establish practical proving requirements.

The [AES-256 component](../../internal/assetprivacy/stark/src/aes256_air/README.md)
uses 78,388 field columns per block, including 14,452 for its expanded key.
At eight bytes per element that is 627,104 bytes per witness row. Repeating the
block over the candidate minimum 512 rows would occupy 321,077,248 bytes before
any proving overhead. These typed components are not yet a multi-block AIR.

## Complete encryption implications

A 512-byte GCM payload requires 32 counter-mode block encryptions. Computing the
GHASH subkey and tag mask requires two more AES block encryptions. Allocating a
complete independent 512-row AES matrix for each of those 34 blocks would use
10,916,626,432 bytes of field elements for one envelope's AES alone. This is an
illustrative rejected layout, not an implemented prover or proposed node limit.
Required auditor/controller disclosures would add further encryption work.

Share a constrained expanded key across blocks. Evaluate scheduling, vertical
trace reuse and appropriate hash/cipher layouts before committing to the full
encryption AIR. Any redesign must preserve exact cross-component bindings,
bounded polynomial degree and the backend's actual hiding/opening budget.
Changing the encryption construction would require a documented profile change
and a new security analysis; optimizing witness code cannot replace that work.

## Required measurements and limits

The fixed GCM component has 4,025,556 logical fields including GHASH/external
bits; a naive 512-row allocation would require 16,488,677,376 base-element bytes.
See [private trace scheduling](TRACE_ARCHITECTURE.md) for the implementation path
and the observed privacy obstacle to cross-AIR lookup buses.
The [vertical product](../../internal/assetprivacy/stark/src/ghash_vertical/README.md)
now uses 769 main columns and 512 rows: 3,149,824 main bytes plus 524,288 preprocessed bytes.
The [37-block chain](../../internal/assetprivacy/stark/src/ghash_chain/README.md)
now has 1,537 columns/8,192 rows: 100,728,832 main bytes plus 10,813,440 preprocessed bytes.
Both exclude prover overhead. Its public ciphertext is 256 uint16 limbs; H/AAD/result
stay private. Noise PRF/CBD uses 5,522 columns/512 rows, or 22,618,112 base-element bytes.
The [vertical AES stage](../../internal/assetprivacy/stark/src/aes_gcm_vertical/README.md)
uses 11,216 columns/512 rows: 45,940,736 main plus 249,856 preprocessed bytes.
It covers all 34 blocks; composing it with GHASH/KDF/KEM remains pending.
Private NTT uses 7,016 columns: forward 1,024 rows/57,475,072 main bytes;
inverse 2,048 rows/114,950,144. Preprocessing adds 4,210,688/8,421,376 bytes.
The complete NTT-domain product has 1,195 columns/512 rows: 4,894,720 main
bytes plus 528,384 preprocessed bytes. All counts exclude proving overhead.
Private polynomial packing uses 512 rows and 575/1,343/2,879/3,391 columns for
D=1/4/10/12 respectively; every variant adds 1,048,576 preprocessed bytes.
Candidate matrix sampling combines SHAKE128 and rejection in shared columns:
24,932 columns/2048 rows, or 408,485,888 main plus 29,622,272 preprocessed bytes.
The direct writer's Keccak temporary adds 43,139,072 element bytes during
generation; allocator/hiding/prover overhead remains additional and unmeasured.
Private canonical-key/KEM derivation uses 15,844 columns/1024 rows: 129,794,048
main plus 6,291,456 preprocessed bytes. Its direct SHA3 writer retains one
21,569,536-byte Keccak temporary. Staged complete GCM uses 11,728 columns/8192
rows: 768,606,208 main plus 15,335,424 preprocessed bytes. Neither is benchmarked.

Before selecting protocol limits, measure completed relations across shield,
transfer, unshield and controller recovery, including each mandatory disclosure.
Record trace height/width, polynomial degrees, quotient chunks, disclosure budget,
proof size, prover peak memory/time and verifier allocations/time. Re-evaluate
security for those dimensions and the full Fiat-Shamir composition.

Only trusted profile/consensus values may bound proof bytes, decoded dimensions,
verification work, group resources and state growth. The current outer witness
length bound is not a bound on nested proof allocation or verification cost.
Builds and Clippy establish compilation/static properties only. Test, vector,
adversarial and benchmark work remains deferred until the user requests it.
