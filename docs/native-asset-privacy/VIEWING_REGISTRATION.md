# Candidate viewing-key possession registration

Updated 2026-10-30. This describes disabled source integration for ML-KEM-768
viewing roles. All consensus flags remain false, catalogs empty and prices/work
limits zero. No artifacts, proofs, accepted registrations or node runs exist.
The candidate remains unreviewed; compilation establishes no security guarantee.

## Body, role and account authorization

NAPL1 Register2 uses its150-byte header plus role1, account32, keyLengthBE2 and
the exact1184-byte public key:1369 bytes total. RecipientView1, AuditorView3 and
ControllerView4 use this format. RecipientSpend2 retains kind21 owner possession;
admin roles5..7 retain Falcon NAPR1. These witness families are not interchangeable.
Canonical coefficients, bounded matrix admission, exact sizes and unused fields
are checked before state writes. The private seed never enters transaction bytes.

The registered account must equal outer Sender. Actual top-level PQ consent uses
the current sender/AuthAddr; proof possession cannot authorize another account.
The ordered native policy/suite and expected current role epoch must match.
Viewing records grant disclosure access only; they confer no ASA administrative
powers, do not change AuthAddr and do not move public or private value.

## Complete public-key relation

[MlKemKeygenAir](../../internal/assetprivacy/stark/src/mlkem_keygen_air/README.md)
constrains private seed expansion, six noise vectors, six forward transforms,
row-wise matrix products plus error and the exact public key coefficients.
Key-generation matrix orientation follows [FIPS203 Algorithm13](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf).

Registration exposes the public key and its rho seed. The trusted verifier
constructor derives all nine matrix polynomials deterministically from that
public rho using the candidate's bounded sampling rule. The circuit binds the
same rho to its private seed expansion. It constrains every canonical public
coefficient to the proven product-plus-error value; a witness helper's host
equality is not a verification substitute. The relation identity includes the
full public key and native genesis/asset/policy domain, preventing key-specific
constants from sharing a relation identity. Preprocessing remains key independent.

This proves knowledge of a derivation seed for the admitted public key. It does
not expose a secret key or constrain the independent implicit-rejection secret z,
which is not part of the public-key derivation. SDK key generation and storage
must preserve the complete standard decryption key and appropriate randomness.
Interoperability, security and practical proving resources remain unverified.

## Expected statements and native application

The exact bundle has one kind22, MainOutput=0xffffffff, role0 slot, with no output
slots. Its1216 public uint16 cells are1184 individual public-key bytes followed
by32 little-endian limbs of the finalized64-byte group digest. Byte cells must
fit0..255 and equal the signed registration key. Rust reconstructs the concrete
AIR from these exact expected bytes, independently checked by Go native planning.

The native group commits genesis, target position, suite, asset/policy, role,
account, exact key, expected epoch and every finalized transaction field.
The NAPW frame must match that current suite/group. Syntax, key membership,
encryption, an owner proof or a Falcon signature cannot replace kind22 possession.

Execution reserves one coarse proof-check unit before artifact/proof work. Actual
Rust verification precedes key history/current pointer, authorization-root and
eligibility writes in the disposable child state. Later group failure discards
tentative writes and work reservations. The creator funds exact native storage.
Registration pays one proof factor, full signed-byte fees and ordinary/PQ fees;
the full fee/group must be finalized before proving.

## Formats, limits and outstanding evidence

NAPK4 contains13 records/1200 bytes; kind22 has16384 rows,9394 main columns,
723 preprocessing columns and1216 public values. NAPV4 has a1578-byte header
and15746-byte cap. Unapproved older versions reject; no approved historical
reference, preprocessing pin or production migration is created by these changes.

All checks so far are compilation, static analysis and source review. No tests,
vectors, traces, constraints, preprocessing, proofs, signatures or node scenarios
have run. Runtime acceptance, resource calibration, approved artifacts, remaining
lifecycle/modes, APIs, SDK/wallet and all six demonstrations remain required by
[the end-to-end checklist](END_TO_END_STATUS.md).
