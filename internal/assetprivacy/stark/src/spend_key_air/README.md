# Private recipient spending-key record

`SpendKeyAir` constrains the private authorization leaf
`SHA3-512("Algorand/AssetPrivacy/Candidate1/KeyAuthorization/" || keys.Encode(record))`
for key role RecipientSpend=2. This is the registered 64-byte note-owner
commitment, rather than a viewing key or administrative signature key.
The component has no public values and remains behind the native privacy feature.

`new(network,asset,policy)` fixes nonzero network/asset/policy and role 2.
The complete hash message is 268 bytes: the 50-byte ASCII domain followed by the
218-byte canonical spending-key record.

| Hash-message byte range | Contents |
| --- | --- |
| 0..50 | KeyAuthorization ASCII domain |
| 50..51 | Record version 1 |
| 51..83 | Network/genesis hash |
| 83..91 | Asset ID, big endian |
| 91..155 | Policy digest |
| 155..187 | Private nonzero account |
| 187..188 | Fixed role 2 |
| 188..196 | Private nonzero epoch, big endian |
| 196..204 | Private Created round, big endian |
| 204..268 | Private nonzero 64-byte owner commitment |

All message bits are Boolean and constant across 512 trace rows. The full
four-block SHA3-512 computation finishes after 96 round rows. Its 32 private
uint16 digest limbs stay constant and equal the final hash output. Three bit-sum
inverses reject zero account, epoch and owner commitment. Those bounded bit sums
cannot wrap in the base field. Created may be zero, matching the Go contract.

`wires(row)` returns private account, epoch, Created, owner and commitment slices
only for an exact-width row. Bits are LSB-first inside each byte. Epoch and Created
retain big-endian wire byte order; the digest limbs are little-endian uint16.

`trace(owner,account,epoch,created)` checks canonical record syntax before
allocating private columns. `commitment(...)` performs the same checks without
trace allocation. The crate-private
`write_into(owner,account,epoch,created,values,(stride,column,first_row))` validates
bounds and syntax before mutation, writes exactly one 512-row frame and returns
the leaf. It preserves surrounding parent cells and avoids a complete child main
matrix by using the bounded SHA3 writer.

The enclosing relation must prove membership of this leaf against the
authoritative current key root for the same network/asset/policy/account and
RecipientSpend role. It must bind the owner to the output note and the account
to the note and recipient viewing record, and enforce current eligibility.
Viewing and spending records have independent role-specific epochs and Created
rounds; do not equate them. Created is committed but no validation round is a
parameter here. Authoritative root construction/read rules must reject future
creation rounds and stale/non-current pointers.

Creating an output note does not require the sender to know the recipient's
spending secret. This component therefore constrains the registered owner hash,
without a secret-preimage witness. Spending that note requires the separate
ownership/preimage relation. Registry admission/possession checks, current-state
membership, policy authority and complete native acceptance remain separate.

There are 4816 main columns and no preprocessed columns over 512 rows:
19,726,336 raw main bytes. SHA3 construction uses a 10,784,768-byte Keccak
temporary. Counts exclude allocator, hiding, LDE/quotient, commitments and other
prover overhead. With the current SHA3 schedule, the conservative absorption
degree is six standalone and seven after one active-frame selector; recompute
full relation degree and quotient budgets after further composition.

Source review, compilation, formatting and physical size checks are the current
checks. No tests, vectors, traces, constraint evaluations, proofs, interoperability
runs or benchmarks have been executed. Security review, resource feasibility
and the full native protocol remain unfinished; no consensus version is enabled.
