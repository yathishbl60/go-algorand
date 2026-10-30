# Expected recipient owner-registration statement

This tagged Go package derives the64-cell expected statement: owner32 followed
by complete finalized group32, using consecutive little-endian uint16 limbs.
`New` requires a nonzero network/asset/policy domain, group and owner. Fields are
owned fixed values; Values returns a copy. This parser/plan grants no authority.

Native execution derives owner and group from the exact current signed body,
checks current PQ account consent and expected epoch, and selects one kind21
owner-secret proof through the real bridge. No caller-selected proof callback,
public key syntax or member proof establishes private-secret possession.

Native [registration](../../../docs/native-asset-privacy/OWNER_REGISTRATION.md)
includes storage, eligibility and rollback requirements. The feature is disabled.
Build/static checks only; no tests, traces, proofs or native operations have run.
