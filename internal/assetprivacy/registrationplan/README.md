# Expected owner and viewing registration statements

This tagged Go package derives the64-cell owner statement: owner32 followed
by complete finalized group32, using consecutive little-endian uint16 limbs.
`New` requires a nonzero network/asset/policy domain, group and owner. Fields are
owned fixed values; Values returns a copy. This parser/plan grants no authority.

Native execution derives owner and group from the exact current signed body,
checks current PQ account consent and expected epoch, and selects one kind21
owner-secret proof through the real bridge. No caller-selected proof callback,
public key syntax or member proof establishes private-secret possession.

`NewViewing` derives the1216-cell statement from an exact canonical1184-byte
ML-KEM-768 public key followed by the same32 finalized-group limbs. Each key byte
occupies one uint16 cell. It requires a nonzero domain/group and validates the
candidate's canonical coefficients and bounded matrix admission before copying.
`ViewingPlan.Values` returns owned cells; no caller buffer or independent key
can replace the key derived from the native registration body.

Native viewing registration selects one kind22 proof for roles RecipientView1,
AuditorView3 and ControllerView4, with current PQ self-authorization and expected
epoch checks. The selected AIR binds private seed/noise/transforms and public
matrix products to the exact key. These plans do not verify proofs themselves.

Native [registration](../../../docs/native-asset-privacy/OWNER_REGISTRATION.md)
includes storage, eligibility and rollback requirements. The feature is disabled.
[Viewing registration](../../../docs/native-asset-privacy/VIEWING_REGISTRATION.md)
uses the same atomic native obligations with its separate possession relation.
Build/static checks only; no tests, traces, proofs or native operations have run.
