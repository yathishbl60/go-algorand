# Private bounded ML-KEM-768 encapsulation

`MlKemEncapsulationAir` joins canonical key hashing, matrix and noise sampling,
NTT arithmetic and ciphertext packing in one private relation. It compiles only
with `native-privacy`. It is not connected to native transaction acceptance.

Given a private canonical 1184-byte encapsulation key P and private 32-byte m,
the relation constrains H=SHA3-256(P), (K,r)=SHA3-512(m||H), all nine matrix
entries, all seven eta=2 noise polynomials and Ckem=K-PKE.Encrypt(P,m,r).
Only Ckem is public: 544 little-endian uint16 limbs covering all 1088 bytes.
The shared secret, coins, key, rho, message, matrices and noise stay private.

Matrix sampling uses the candidate's fixed sixteen SHAKE128 blocks per entry,
matching [key admission](../../../../../docs/native-asset-privacy/MATRIX_SAMPLING.md).
It keeps the first 256 candidates below 3329 in order; exhaustion fails closed.
This is an additional candidate rule, not proof that every standard key fits.
Coordinates are fixed (column,row); the dot relation consumes the transpose.
Noise nonces are fixed 0..6, all using the same derived r.

## Fixed schedule and layout

| Stage | Row interval, end excluded |
| --- | --- |
| Compact private key/H/G derivation | 0..1024 |
| Nine streamed matrix entries | 1024..37888 |
| Seven noise PRF/CBD samples | 37888..41472 |
| Staged K-PKE and public ciphertext packing | 41472..57856 |
| Padding with unchanged root registers | 57856..65536 |

The relation has 15,906 main columns and 2,039 trusted preprocessed columns.
It shares the PKE layout's first 9,328 columns. Private rho/K/r occupy the next
768 columns; a 5,810-column workspace is reused by derivation/matrix/noise.
PKE root ranges 0..4864 and 6161..9233 plus rho/K/r remain constant across every
row, including phase boundaries and padding. Root rho/K/r are Boolean.
Each active child binds its key/message/seed/coefficients to those same roots.
The PKE stage consumes them directly from the shared physical columns.

Eighteen verifier-owned frame schedules select the exact child row intervals.
Each child receives its own current/next, first/last/transition and active flags.
Preprocessing storage is reused across disjoint stages; children cannot choose
coordinates, nonces, lengths or schedules from the proof. The native verifier
must pin the expected schedule commitment as part of trusted profile artifacts.
No private lookup terminal or intermediate hash is published for composition.
The conservative source degree bound is six after nested frame selectors.
Actual quotient/opening/security budgets remain to be validated.

## Interfaces and limits

`trace(P,m)` constructs an `EncapsulationWitness` containing secret trace columns
and public ciphertext limbs. Caller-supplied m must be fresh, secret and uniform;
the AIR cannot prove randomness quality. `public_values(Ckem)` encodes supplied
wire bytes for verifier input. `wires(row)` exposes private t/rho/m/K/r slices to
the future envelope/authorization relation, only for an exact-width row.

Host encapsulation checks key canonicality and matrix exhaustion before the
large trace allocation. Bounded child writers fill only their own frames in
the destination, avoiding complete child main matrices. Small Keccak scratch
matrices and child preprocessing tables still require temporary memory.

Parent composition can use crate-private `FRAME_ROWS=57856`, which ends exactly
after the complete PKE stage. `write_into(P,m,values,(stride,column,first_row))`
prepares key admission and all host intermediates before mutation, writes only
that bounded rectangle and returns its secret `Encapsulation` host result.
`write_preprocessed_into(values,target)` assembles the trusted child tables in
that same frame directly into the parent. Neither creates a full KEM child
matrix. The parent must set its frame's last/transition selectors at row57855;
all nested child final-row selectors are already present there. Standalone
65,536-row trace and preprocessing APIs retain their unchanged padding.

Raw main storage is 8,339,324,928 bytes and trusted preprocessing is
1,069,023,232 bytes. These already total about 8.76 GiB, before allocator,
child-schedule, hiding, LDE, quotient, commitment or other prover overhead.
This layout establishes explicit source bindings; it is not a practical prover
budget. Further resource reduction and measurement remain necessary.

This component does not authorize P. The enclosing relation must prove the
current recipient/auditor/controller registry and policy bindings, reconstruct
the same canonical P for AAD, connect K/Ckem to HKDF/GCM and the committed note,
enforce every disclosure role and bind finalized transaction/group context.
There is no accepted proof format, native verifier, usable wallet or node demo.

Compilation, strict Clippy, formatting, physical size checks and independent
source reviews are the available evidence. No tests, vectors, traces,
constraint evaluations, proofs, interoperability runs or benchmarks have run.
No security review, constant-time witness generation, zeroization or end-to-end
privacy guarantee follows. The feature remains disabled by default.
