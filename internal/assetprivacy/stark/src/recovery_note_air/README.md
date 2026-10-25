# Canonical issuer-recovery input and shared nullifier

RecoveryNoteAir joins one canonical private note opening, its serial-derived
nullifier and accepted depth32 note membership. It deliberately requires neither
the holder's spending secret nor effective Frozen=0. Current native issuer
signatures and policy checks must authorize recovery outside this component.
The Rust native-privacy feature stays disabled by default.

RecoveryInput contains opening512 and MembershipPath. The commitment, owner,
account, amount, serial, path/index remain private. The exact179-byte nullifier
message matches ordinary spending: candidate Nullifier domain43, network32,
assetBE8, policy64 and serial32. Every64-byte hash output, including zero, is
valid. Both holder spending and recovery must consume this same native marker.
[Output freshness remains a separate unresolved candidate gate](../../../../../docs/native-asset-privacy/NULLIFIER_FRESHNESS.md).

new(network,asset,policy) fixes the same validated note/hash/tree domain.
check_inputs(input,accepted_root) returns the computed nullifier only after
canonical opening and exact path/root checks, before allocation. trace returns
RecoveryNoteWitness{trace,public_values}. The64 public scalars are nullifier32
LEu16 limbs followed by accepted note root32 LEu16 limbs. No extra input digest,
account or opening becomes a public join. All private witnesses stay local.

Bounded write_into(input,root,values,(stride,col,first)) and the corresponding
preprocessed writer validate shapes before writes and use only FRAME_ROWS3690.
Standalone ROWS4096 pads constant private roots through the final row. wires(row)
exposes private account/value/serial bits and owner/commitment limbs to the parent.

| Stage | Rows, end excluded |
| --- | --- |
| Canonical note and private commitment | 0..512 |
| Public nullifier from the same private serial | 512..585 |
| Private commitment and accepted note membership | 585..3690 |
| Standalone padding | 3690..4096 |

Main width7990 comprises640 private root registers and7350 shared scratch.
Account256, value64LSB and serial256 are Boolean; owner32 and commitment32 are
LEu16 digest limbs constrained through the canonical note/hash relations. Every
root is constant across stages/padding. The exact same commitment feeds private
hash output and membership leaf. The nullifier hash frame includes every round
and a final row with cleared absorption flags. Preprocessing width48 comprises
36 member schedule columns and12 trusted outer flags.

Raw standalone main storage is261,816,320 bytes; the active frame is235,864,800
bytes. Preprocessing adds1,572,864 bytes. These omit Keccak/schedule scratch,
hiding/LDE, quotient, commitments and allocator overhead. Conservative source
degree is8 through staged membership; an enclosing recovery group raises it to9.
Actual hiding/security/resource budgets remain unvalidated.

Anyone knowing an opening, including an authorized auditor, may construct these
mathematical witnesses. That does not confer issuer recovery power. Native
verification must require current retained CanClawback, current nonzero ASA
clawback account and valid ordinary/PQ authorization over the finalized bodies.
Accepted note-root history never restores a removed authority. The complete
recovery group must enforce conservation and original-owner remainder retention;
all required output envelopes, native state/funding checks and signature checks
remain mandatory. This relation is never an ordinary holder-spend substitute.

No tests, vectors, traces, constraint evaluations, proofs, benchmarks or node demos
have run. Source review/build/Clippy/limits establish compilation properties only.
Native proof decoding, verifier/ingress and full security/resource validation
remain unfinished. Historical viewing/decryption material must be retained;
new roots or current keys cannot recover an unavailable old note opening.
