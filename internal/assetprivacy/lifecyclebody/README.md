# Candidate native lifecycle body codec

This opt-in package implements bounded public `NAPL` version1 lifecycle bodies
for candidate `aprv` operations and ordinary `acfg` creation declarations. It performs no state reads, signatures,
proof verification or native execution. It requires `algorand_native_privacy`;
every current consensus version remains disabled. Registration syntax currently
accepts viewing keys, RecipientSpend commitments and admin Falcon keys; parsing proves no possession.

## Canonical layout

| Header byte range | Value |
| --- | --- |
| 0..4 | ASCII NAPL |
| 4..5 | Version1 |
| 5..6 | Operation1 Adopt,2 Register,3 Assign,4 Freeze,5 Create,6 Retire or7 Cleanup |
| 6..14 | Asset uint64BE; zero exactly for Create |
| 14..78 | Policy digest; zero exactly for Adopt/Create |
| 78..142 | Nonzero suite digest |
| 142..150 | Expected epoch uint64BE; zero for Adopt/Create |

Adopt is exactly185 bytes: header followed by modes1, capabilities1,
disclosures1 and auditor32. The policy syntax must be valid, allow Public mode
and declare controller disclosure if clawback is retained. Auditor is nonzero
exactly when auditor disclosure is required. Ledger execution must derive the
capabilities and inclusion-round policy identity from authoritative state.

Create has the same185-byte declaration shape, with Public optional. Only an
ordinary `acfg` with ConfigAsset0 may carry it; no witness is allowed. ASA fields
stay in their ordinary encoding. Current PQ creator consent, exact actual powers,
suite and atomic ID/round-derived policy initialization are enforced by the
[creation integration](../../../docs/native-asset-privacy/NATIVE_CREATION.md).

Assign is exactly182 bytes: header and auditor32. Zero auditor is structurally
allowed because the current policy decides whether an auditor is required.
The controller address is never supplied in this body; native state derives it
from the current ASA authority and retained disclosure assignment.

Freeze is exactly183 bytes: header, nonzero target account32 and one frozen byte
restricted to0 or1. The ledger must compare the expected epoch with current
control state and authenticate the current freeze authority and capability.
Unknown operations/versions and trailing bytes fail.

Retire6 is150 bytes with expected epoch0 and no tail. Cleanup7 is152 bytes with
the current nonzero cleanup epoch and limitBE2 in1..32. All unrelated fields
and witnesses must be empty. Current PQ manager authority, retirement/supply
checks and bounded inventory deletion belong to the [native cleanup path](../../../docs/native-asset-privacy/RETIREMENT_CLEANUP.md).

Register carries role1, nonzero account32, keyLengthBE2 and exact role material.
Viewing roles RecipientView1, AuditorView3 and ControllerView4 are1369 bytes with
an exact1184-byte ML-KEM-768 public key. RecipientSpend2 is249 bytes total with a
nonzero owner commitment64. Admin roles
FreezeAuthority5, ClawbackAuthority6 and ManagerAuthority7 are1978 bytes with an
exact1793-byte Falcon1024 public key. Policy/suite/asset
must be nonzero; expected epoch leaves room for increment and unrelated fields
are zero. Exact role/declared size is checked before tail reads and owned copying.
`keys.ValidatePublicKey` validates syntax, not possession. The ledger authenticates
the registered account and additionally matches admin roles to current ASA powers.

## API and authority boundary

`IsLifecycle` recognizes only the four magic bytes. It does not establish a
supported version, operation or complete body. Call `Decode` and check its error.
Decode validates exact length before reading any tail, copies fixed arrays by
value and validates registration key material before making its bounded owned
copy. It retains no caller buffer; keep input stable until return. Every failed
Decode returns a zero Body. Encode also returns owned public key bytes.

`Body.Validate` rejects all unused nonzero fields as well as invalid domain,
policy syntax and operation fields. `Encode` validates first and returns owned
canonical bytes. `CheckSender` validates syntax/nonzero sender and requires Register
account equal Sender. It authenticates neither account nor current ASA role.

The enclosing transaction path must enforce feature/configuration/suite gates,
current top-level PQ account authority, operation principal and policy/suite
identity, expected epoch, operation-specific witness rules, fees, creator storage
funding and atomic group rollback. Non-registration operations require an empty witness;
administrative registration requires actual new-key possession verification. A public address or parsing
success grants no manager, freeze, viewing or spending permission. See the
[lifecycle contract](../../../docs/native-asset-privacy/LIFECYCLE_INGRESS.md).

Administrative registration now verifies a separately typed [NAPR signature](../registrationproof/README.md)
of the finalized registration group under the exact new public key, in addition
to current top-level PQ account authorization. It never rekeys AuthAddr or grants
powers beyond the current ASA role. RecipientSpend instead needs the exact
NAPW kind21 owner-preimage proof. Viewing registration requires the exact kind22
key-generation possession proof and1216-cell expected public statement. Existing
membership proofs or transfer proofs cannot replace those checks. This codec carries public
material only; no private key or note secret is accepted.

## Current evidence

Tagged compilation, vet and authored physical line limits pass. No tests,
round trips, vectors, ledger operations, proof runs or node execution occurred.
Complete protocol delivery, interoperability and security remain unverified.
