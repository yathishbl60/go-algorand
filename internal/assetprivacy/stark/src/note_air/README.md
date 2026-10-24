# Canonical output-note commitment constraints

`NoteCommitmentAir::new(network,asset,policy)` fixes the note domain fields from
the verifier's bound public statement. Its SHA3-512 commitment uses the canonical
512-byte opening, zero reserved tail and nonzero owner/account/serial/blinding
fields. Spending authority, membership and current key/control state are separate.

The crate-private `commitment(opening)` validates canonical encoding and computes
the 64-byte digest without allocating a trace. `write_into(opening,values,target)`
writes exactly 512 rows at `(stride,column,first)` and returns that digest.
Checked row/column bounds and canonical validation precede mutation. The SHA3
writer populates parent columns directly; only Keccak scratch is retained.
The four nonzero inverse columns are overwritten on every frame row.

Standalone `trace` delegates to the same bounded writer after early validation.
Private opening bits can be bound through `opening_bit_column`; they must match
the same opening encrypted by the enclosing envelope relation. The commitment
uses 32 public little-endian uint16 limbs, or the enclosing relation's bound
private digest wires through a private-output adapter.

This code compiles only with `native-privacy` and is not a native acceptance
path. Builds, formatting and source inspection are the available evidence. No
tests, traces, constraint evaluations, proofs or runtime equivalence checks ran.
