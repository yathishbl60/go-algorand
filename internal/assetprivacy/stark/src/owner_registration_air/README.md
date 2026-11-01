# Recipient owner-secret registration relation

OwnerRegistrationAir checks the existing network-bound SHA3-512 SpendingKey
commitment for a private nonzero32-byte secret. Public values are64 uint16 cells:
owner32 then finalized group32, both consecutive little-endian limbs.

The fixed message is ASCII `Algorand/AssetPrivacy/Candidate1/SpendingKey/`, network32
and secret32:109 bytes. Prefix bits are fixed by the verifier, secret bits use the
SHA3 AIR's Boolean/message constraints, and a bounded secret-bit sum times a private
inverse equals1. Global group limbs are constant and equal the public group.

One512-row trusted frame has4 active/first/last/transition preprocessing flags.
There are3540 main columns:33 root columns and3507 SHA3 workspace columns. Native
selection pins these dimensions under kind21 and constructor-domain identity.

`public_values` and `trace` reject zero owner/group; trace construction also rejects
zero secret or an owner mismatch before allocation. These host checks prepare the
witness; independent AIR constraints enforce the actual relation.

A valid proof grants no account authority by itself. Native callers authenticate
current PQ Sender/AuthAddr, require registeredAccount==Sender, compare current
policy/suite/epoch, recompute complete group binding and verify this exact typed
one-slot NAPW bundle before writing key/eligibility state. Viewing-key possession
uses the separate typed ViewingRegistration relation (kind22). See the [native contract](../../../../../docs/native-asset-privacy/OWNER_REGISTRATION.md).

[Bounded direct-P3 Owner proofs](../../../../../docs/native-asset-privacy/FEASIBILITY_RESULTS.md) verified with changed owner/group/proof rejection.
No artifact-bound native transaction or automated test suite has run; the
primitive result does not establish soundness, privacy or native delivery.
