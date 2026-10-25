# Private current-key membership

`RegisteredKeyAir` joins a canonical private key record commitment with a
32-level private membership path under the authoritative current key registry
root. Only that root is public, encoded as 32 little-endian uint16 limbs.
Account, epoch, Created, key/owner, record leaf, path index and siblings remain
private. The component is opt-in and does not execute a native transaction.

`view(network,asset,policy,role)` accepts viewing-key roles 1/3/4 through
`ViewKeyAir`. `spend(network,asset,policy)` fixes RecipientSpend role 2 through
`SpendKeyAir`. Network, asset, policy and record kind are verifier-selected;
a witness cannot switch kind or role. Viewing and spending epochs are separate.

`RegisteredKeyInput` carries common account/epoch/Created fields, a private
`MembershipPath`, and either `KeyMaterial::View(P)` or `KeyMaterial::Spend(owner)`.
The wrong material kind is rejected. Canonical syntax and the computed path/root
are checked before trace allocation. These host checks construct witnesses;
record and membership AIR bindings independently enforce the same leaf relation.

| Root main range | Private content |
| --- | --- |
| 0..768 | Viewing-key canonical coefficients, or zero for spending |
| 768..1024 | Account bits |
| 1024..1088 | Epoch bits |
| 1088..1152 | Created-round bits |
| 1152..1408 | Viewing-key rho bits, or zero for spending |
| 1408..1920 | Spending owner bits, or zero for viewing |
| 1920..1952 | Private record-leaf limbs |
| 1952..15768 | Reused record/membership workspace |

All 1952 root registers remain constant across stage boundaries and padding.
Account through owner registers are Boolean. Unused key-kind registers are
constrained to zero. Viewing coefficients equal the same twelve packed bits
checked by `ViewKeyAir`; rho and record metadata bind to those same key bytes.
Spending owner and record metadata bind directly to `SpendKeyAir`. The private
record digest equals the same leaf consumed by every membership stage.

Trusted preprocessing selects the record frame followed by compact membership.
Viewing uses 1024 record rows and 3105 membership rows, ending at row 4129.
Spending uses 512 plus 3105, ending at row 3617. The membership frame retains all
32 hash levels and its final root-carry row. Both standalone traces pad to 8192
rows while preserving root globals; no child remains active in that padding.
The verifier must pin the exact record-kind schedule commitment and dimensions.

`wires(row)` returns private key/account/epoch/Created/rho/owner/leaf slices only
for an exact-width row. `public_values(root)` encodes the expected authoritative
root. `trace(input,root)` constructs private columns. Crate-private `check_inputs`
performs host validation without a trace; `write_into` and
`write_preprocessed_into` fill bounded compact frames in a parent stride.
Bounds are checked before mutation, writes preserve surrounding parent cells,
and standalone trace construction restores root globals through all padding.

The caller must supply the current key registry root from native state, never a
proof-selected, note-tree or historical root. Go registry rotation replaces the
current leaf at its stable index; old records cannot authenticate under the new
root. This component has no validation-round input: authoritative root provenance
and native key-record rules must exclude future Created rounds and stale pointers.
Current policy/issuer eligibility, note and envelope account/key/owner bindings,
key possession/registration authority and transaction/group bindings remain
required enclosing obligations. Viewing-key matrix admission is supplied by
current-root provenance and the enclosing KEM, rather than this leaf component.
Creating an output does not require knowledge of its recipient's spending secret.

There are 15768 main columns and 776 preprocessing columns over 8192 rows:
1,033,371,648 raw main bytes plus 50,855,936 preprocessing bytes. Compact child
writers avoid full child main matrices, but child schedules, Keccak scratch,
allocator, hiding/LDE/quotient matrices and proving overhead remain additional.
The nested membership hash has conservative source degree eight; further frame
composition requires a fresh degree/quotient and backend resource assessment.

Source review, compilation, formatting and physical size checks are the current
evidence. No tests, traces, constraint evaluations, proofs, runtime vectors,
interoperability runs or benchmarks have been executed. Complete native
verification, security review and practical proving limits remain unfinished.
