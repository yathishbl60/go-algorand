# Trusted native verifier request

`verify_request(request, bundle)` reconstructs concrete AIRs and checks the exact
proof set. This is a trusted in-process ABI input, **not** a transaction, witness or
RPC format. It cannot authenticate its own registry selection or native statements.
The native caller supplies current policy/Suite/Domain/Group and exact plan vectors.
No keys, witnesses, signatures or private epochs are obtained from proof metadata.

Request length is1578..15746 bytes. Header fields have these exact byte intervals:

| Interval, end excluded | Encoding |
| --- | --- |
| 0..5 | ASCII NAPV then version4 |
| 5..69 | Independently approved profile/Suite64 |
| 69..133 | Independently selected verifier reference64 |
| 133..197 | Independently approved registry digest64 |
| 197..229 | Native genesis/network32 |
| 229..237 | Native assetu64BE |
| 237..301 | Native policy identity64 |
| 301..365 | Recomputed finalized-body group digest64 |
| 365..369 | Trusted maximum bundle bytesu32BE |
| 369..373 | Trusted maximum individual proof bytesu32BE |
| 373..377 | Trusted per-decoder canonical allocation chargeu32BE |
| 377..1577 | Exact1200-byte NAPK version4 registry file |
| 1577 | Slot count byte,1..7 |

Each slot follows as kindu8, outputu32BE, roleu8, scalarCountu16BE, then exactly
scalarCountu16LE cells. Unknown kinds and counts differing from compiled layout
schemas fail before cell allocation. Entire input must be consumed; no trailing
extensions/aliases/optional keys are allowed. Main precedes exact ordered output
roles as defined by [bundle framing](../../../../../../docs/native-asset-privacy/PROOF_BUNDLES.md).

The parser checks nonzero domain/group, registry pins plus supported reference, bounded
limits and slots. The main public group must equal header Group. For main transfer,
public debit/credit come from cells192/196; unshield credit from128. Shield debit
comes from64 with no private inputs; group starts68. Owner registration has owner0/group32.
Viewing registration kind22 has exact public-key byte cells0..1184 and group1184..1216.
Each key cell must fit u8; canonical polynomial decoding and bounded public matrix
derivation precede constructing its verifier. Domain and complete public key bind
its relation identity; group must equal the trusted header. Both registration
proofs require one main slot and no outputs. NAPV/NAPK version3 are rejected. Recovery public
credit starts96 for partial-public and64 for full-public; private variants use0.
Current recovery authority is the last32 byte cells. These values select the exact
Prepared factory; they never grant issuer signatures or administrative power.

Output context is the first248 byte cells. Its domain, role and ordinal must match
the native header/slot; disclosure authority is cells1100..1132. Role-specific
concrete verifier variants enforce ordinary, disclosure, retained and recovered
relations. Bundle validation joins every public output commitment to main outputs,
checks the exact required order/profile/registry and preflights artifact IDs/degrees.
Actual Rust P3 verification must succeed for every component; no callback can replace it.

The C boundary owns no buffers. Go keeps copied request/payload arrays alive and
stable during the call; Rust borrows and drops all references before returning.
The bounded small request adds metadata allocations outside the proof decoder quota;
backend work/copies and caller/Go payload buffers are separate. No aggregate resource
meter exists. Guard ceilings remain512 MiB bundle,128 MiB proof,512 MiB per-decoder.
No safe node budget or fee schedule is established by those ceilings.

`Failure::Request` identifies trusted-context/registry construction failures;
`Failure::Proof` identifies bundle verification failures. ABI2 distinguishes local
allocation failure and backend panics from invalid proof evidence in either phase.
Canonical element charges use the same fixed64-bit model on every architecture;
an actual allocation failure is local unavailability, never a quota violation.

The [full bridge contract](../../../../../../docs/native-asset-privacy/NATIVE_VERIFIER_BRIDGE.md)
describes build flags, ABI safety, source identity and remaining integration. The
node links against the real verifier, but no runtime request/proof/ABI conformance
or setup has been executed. Native consent/body checks, approved suite/artifacts,
complete native execution and end-to-end scenarios remain unfinished. Features stay off.
