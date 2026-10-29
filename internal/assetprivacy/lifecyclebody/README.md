# Candidate native lifecycle body codec

This opt-in package implements bounded public `NAPL` version1 lifecycle bodies
for the candidate `aprv` transaction. It performs no state reads, signatures,
proof verification or native execution. It requires `algorand_native_privacy`;
every current consensus version remains disabled. No registration is accepted.

## Canonical layout

| Header byte range | Value |
| --- | --- |
| 0..4 | ASCII NAPL |
| 4..5 | Version1 |
| 5..6 | Operation1 Adopt,3 Assign or4 Freeze |
| 6..14 | Nonzero asset uint64BE |
| 14..78 | Policy digest; zero only for adoption |
| 78..142 | Nonzero suite digest |
| 142..150 | Expected epoch uint64BE; zero for adoption |

Adopt is exactly185 bytes: header followed by modes1, capabilities1,
disclosures1 and auditor32. The policy syntax must be valid, allow Public mode
and declare controller disclosure if clawback is retained. Auditor is nonzero
exactly when auditor disclosure is required. Ledger execution must derive the
capabilities and inclusion-round policy identity from authoritative state.

Assign is exactly182 bytes: header and auditor32. Zero auditor is structurally
allowed because the current policy decides whether an auditor is required.
The controller address is never supplied in this body; native state derives it
from the current ASA authority and retained disclosure assignment.

Freeze is exactly183 bytes: header, nonzero target account32 and one frozen byte
restricted to0 or1. The ledger must compare the expected epoch with current
control state and authenticate the current freeze authority and capability.
Operation2 is reserved for registration and always fails, including with an
otherwise well-framed body. Unknown operations/versions and trailing bytes fail.

## API and authority boundary

`IsLifecycle` recognizes only the four magic bytes. It does not establish a
supported version, operation or complete body. Call `Decode` and check its error.
Decode validates exact length before reading any tail, copies fixed arrays by
value and retains no caller buffer. It makes no input-dependent allocations;
keep the input stable until return. Every failed Decode returns a zero Body.

`Body.Validate` rejects all unused nonzero fields as well as invalid domain,
policy syntax and operation fields. `Encode` validates first and returns owned
canonical bytes. `CheckSender` checks Validate and nonzero sender identity only;
it neither authenticates that account nor compares it to a manager/controller.

The enclosing transaction path must enforce feature/configuration/suite gates,
current top-level PQ account authority, operation principal and policy/suite
identity, expected epoch, empty proof witness for these operations, fees,
creator storage funding and atomic group rollback. A public address or parsing
success grants no manager, freeze, viewing or spending permission. See the
[lifecycle contract](../../../docs/native-asset-privacy/LIFECYCLE_INGRESS.md).

Registration needs new typed possession relations and authoritative verification.
Public key syntax, existing membership proofs or transfer proofs cannot replace
those checks. This codec accepts no key material or private witnesses.

## Current evidence

Tagged compilation, vet and authored physical line limits pass. No tests,
round trips, vectors, ledger operations, proof runs or node execution occurred.
Complete protocol delivery, interoperability and security remain unverified.
