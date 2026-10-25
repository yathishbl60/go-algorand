# Private viewing-key record commitment

`ViewKeyAir` constrains the private key-authorization leaf
`SHA3-512("Algorand/AssetPrivacy/Candidate1/KeyAuthorization/" || keys.Encode(record))`.
The ASCII domain is 50 bytes; the encoded viewing record is 1338 bytes, for a
1388-byte hash message. The leaf, account, key, epoch and Created round remain
private. This component has no public values and compiles only with native privacy.

`new(network,asset,policy,role)` fixes nonzero network/asset/policy and one viewing
key role: RecipientView=1, AuditorView=3 or ControllerView=4. These key-role values
differ from the envelope role values 1/2/3; the enclosing proof must map them
explicitly. Spending and administrative signing roles are rejected here.

| Hash-message byte range | Contents |
| --- | --- |
| 0..50 | KeyAuthorization ASCII domain |
| 50..51 | Record version 1 |
| 51..83 | Network/genesis hash |
| 83..91 | Asset ID, big endian |
| 91..155 | Policy digest |
| 155..187 | Private nonzero account |
| 187..188 | Fixed viewing role |
| 188..196 | Private nonzero epoch, big endian |
| 196..204 | Private Created round, big endian |
| 204..1388 | Canonical ML-KEM-768 encapsulation key |

The SHA3 component is fixed to 1024 rows. All message bits are Boolean and
constant across rows; the complete 20-block hash finishes after 480 round rows.
Every prefix/version/domain/network/asset/policy/role byte is fixed by the AIR.
The private digest is constant across every row and bound to the final hash.
Account and epoch bit-sum inverses reject zero without modular-wrap ambiguity.
Created may be zero, as in the Go record contract.

Trusted preprocessing selects one of the 768 public-key coefficients on each
of rows 0..767. Its twelve little-endian message bits reconstruct the selected
canonical witness and must be below 3329. Later rows select zero. The same key
message bits feed SHA3; no independent polynomial or key-byte witness can be
substituted. The final 32 key bytes are the private rho bits.

`wires(row)` returns private account, epoch, Created, rho, coefficient-bit and
commitment slices only for an exact-width row. `coefficient_bits(row,index)`
returns exactly twelve bits for coefficient indices 0..767. Bits are LSB-first
inside each byte; epoch and Created keep big-endian wire byte order.

`trace(P,account,epoch,created)` checks key length/canonicality and nonzero account
and epoch before allocating private columns. `commitment(...)` performs those
same checks and returns the leaf without trace allocation. The crate-private
`write_into(P,account,epoch,created,values,(stride,column,first_row))` checks bounds
before mutation, writes exactly 1024 rows in the parent and returns the leaf.
It preserves surrounding cells and uses the bounded SHA3 writer.

This checks canonical key encoding, not the candidate's nine bounded matrix
expansions. Go `keys.Encode` also performs that admission check. The authoritative
current-root provenance and the enclosing KEM relation must establish admission;
a standalone leaf here cannot replace either. No host-only admission shortcut
is used as algebraic enforcement.

The enclosing proof must authenticate this leaf against the current key registry
root for the correct network/asset/policy/role, bind its account/epoch/key to the
note and envelope, and enforce current issuer/policy eligibility. Created is
committed but this component has no validation-round parameter or freshness
comparison. Native root construction/read rules must reject future Created
rounds; root membership must refer to the authoritative current snapshot. Old
records, root hashes or knowledge of public key bytes do not authorize spending.
No key-possession, current-state membership or complete native operation is proved.

The layout has 13816 main columns and 768 preprocessing columns over 1024 rows:
113,180,672 raw main bytes plus 6,291,456 preprocessing bytes. SHA3 witness
construction adds a 21,569,536-byte Keccak temporary. These static counts omit
allocator, masking, commitments, quotient/LDE and other prover overhead.
The conservative source degree is six standalone and seven inside one frame.
Re-evaluate the SHA3 block/round selector degree and quotient budget
in each enclosing relation; builds do not establish a practical proof budget.

Only source review, compilation, formatting and physical size checks are
available. No tests, vectors, traces, constraint evaluations, proofs, runtime
interoperability or benchmarks have run. Security review and complete native
verification remain unfinished; the feature remains disabled by default.
