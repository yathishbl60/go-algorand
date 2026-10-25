# Ordinary input spending, conservation and native statement planning

Updated 2026-10-25. Disabled components now join ordinary holder ownership,
accepted note membership and current hidden-account eligibility to conservation
and finalized group context. A complete native verifier/transaction still does
not exist. These relations do not implement issuer recovery or all arities.

## Exact private input and group relations

[SpendableNoteAir](../../internal/assetprivacy/stark/src/spendable_note_air/README.md)
constrains the canonical opening, its committed owner's nonzero secret, the exact
serial-derived public nullifier, accepted note membership and current eligibility
for the same hidden account. The owner is the one already committed in the note;
there is no equality to the latest registered output owner after key rotation.
Input commitment, account, serial, secret and membership paths remain private.

The staged input uses8247 main columns over8192 rows, with7356 active rows and64
preprocessed columns. Note512, owner hash49, nullifier hash73, note membership3105
and current eligibility3617 share scratch. The compact hash frames retain every
permutation and a final flag-clear padding row. Private digest outputs and direct
account/value/serial/owner bindings replace public private-input joining values.

[SpendGroupAir](../../internal/assetprivacy/stark/src/spend_group_air/README.md)
uses two actual inputs and either two actual canonical outputs (Transfer) or no
private outputs (Unshield). No dummy/optional inputs or placeholder output notes
are instantiated. Unshield explicitly constrains both absent output amounts zero.
Exact Boolean uint64 values and signed carry enforce integer conservation without
field wrap. Public debit/credit constants must match the native operation bodies.

| Public field | Transfer264 | Unshield196 |
| --- | --- | --- |
| Input0 nullifier32/root32 LEu16 | 0..64 | 0..64 |
| Input1 nullifier32/root32 LEu16 | 64..128 | 64..128 |
| New output commitments,32 LEu16 each | 128..192 | Absent |
| Conversion,4 LEu16 per uint64 | 192..200 debit/credit | 128..132 credit |
| Finalized group digest,32 LEu16 | 200..232 | 132..164 |
| Current eligibility root,32 LEu16 | 232..264 | 164..196 |

Each input receives only its own expected nullifier/root pair plus the common
current eligibility root, through fixed public-variable indices. This mapping
never reads private columns or proof-provided indices. Current eligibility roots
cannot come from note-root history. Every64-byte nullifier, including zero, is
valid as a hash output; duplicate nullifiers are rejected before native verification.

The group has16384 rows,8506 main columns and80 preprocessed columns. Active input
stages end at14712; Transfer's two512-row outputs end at15736. Raw main storage is
1,114,898,432 bytes plus10,485,760 preprocessing bytes, excluding all prover overhead.
The source degree upper bound is10. Existing configuration is unchanged; actual
quotient sizing, masking/opening budget, soundness and resource feasibility remain
unvalidated. Source counts and compilation do not establish usable proving costs.

## Native expected statements and freshness

[spendplan.Build](../../internal/assetprivacy/spendplan/README.md) snapshots exact
main vectors from a trusted domain/current eligibility root. It enforces2 real
inputs, Transfer2/Unshield0 outputs, distinct nullifiers/output commitments and
bounded nonzero accepted-root counts. Inputs/outputs/vectors are returned as
copies. Count is native root-acceptance metadata, not an added AIR public value.
CheckOutputs matches exact domain, ordered commitments and operation ordinals.

Native AssetPrivacySpendProofPlan derives policy identity, suite and current
eligibility root after checking the disabled feature. It recomputes statement.Bind
from finalized full group bodies and replaces any caller-supplied draft.Group.
Group target indexing follows bounded preflight. MaxGroupSize must match current
consensus; body/group estimate budgets remain trusted pinned-profile arguments,
not arbitrary request values or copied ordinary wire-size ceilings.

The native planner checks accepted input root/count pairs and fresh nullifiers,
then output ordering/commitments, target transaction ID/current-round metadata,
and every output's exact native recipient/disclosure proof plan. Unshield has no
output plans and need not read current output key/disclosure assignments. Planning
is read-only: it consumes no nullifier, applies no balance and verifies no proof.

Native private transaction fields and their parser are still absent. The helper
therefore does not compare draft kind/asset/debit/credit or record nonce/ciphertext
to actual native body fields. Bind does not check ordinary validity at the current
round, signatures, policy/mode, holder consent, actual debits/credits or funding.
Those checks, exact trusted artifact selection, bounded embedded-proof decoding,
cryptographic verification and atomic execution remain mandatory implementation work.

For Transfer, every output-role proof must commit the same canonical opening to
the same already-public new output commitment as the main proof. All mandated
roles must verify against native-derived context, keys/eligibility/assignments.
Independent fresh hiding entropy and composed Fiat-Shamir/security analysis remain
required. A planned vector or matching witness header cannot verify any proof.

Plans are serialized COW snapshots. Rebuild/recheck after relevant preceding
spends, freezes, keys, assignments or body/group mutations; inputs must not be
concurrently changed during construction. Old note roots may be accepted while
current eligibility remains mandatory. Recovery/frozen-owner remainders need a
[separate recovery relation](RECOVERY.md), with current issuer signatures mandatory.
[Output marker freshness](NULLIFIER_FRESHNESS.md) remains an unresolved candidate gate.

No tests, vectors, traces, constraint evaluations, proofs, benchmarks or real-node
demos have run. Builds, vet, Clippy, source review and physical limits are the
available evidence. Feature activation, full verifier/ingress, other arities,
recovery, SDK/wallet/API and all six north-star demonstrations remain unfinished.
