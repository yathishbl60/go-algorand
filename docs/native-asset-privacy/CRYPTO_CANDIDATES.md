# NAP-01 candidate assessment

Checked 2026-10-22 against primary project and standards sources. This is a
selection assessment, not an independent security audit or an approved profile.
The pinned Plonky3/encryption candidate is selected for implementation in
[CANDIDATE_PROFILE.md](CANDIDATE_PROFILE.md), not approved for production. The release still requires
128-bit post-quantum security, witness privacy, recipient anonymity, native
controls, bounded verification and local proving as a complete construction.

## Candidates and gaps

| Candidate | Evidence | Decision for this implementation |
| --- | --- | --- |
| Winterfell | Project README describes an unaudited research implementation and cautions that its current proofs may leak secret inputs | Do not use its current defaults for private transfers |
| RISC Zero | Repository describes 98-bit conjectured security with defaults; security documentation lists lower-than-128-bit component estimates and cautions about critical privacy requirements | A default deployment does not establish our target; no Groth16 wrapper |
| Plonky3 hiding FRI | Current architecture documents hiding FRI support and explicit ZK paths, while requiring analysis of the complete composition | Selected and pinned for implementation; complete relation and security argument remain pending |
| ML-KEM-768 | FIPS 203 standardizes key encapsulation; NIST separately publishes KEM-use guidance | Candidate encryption component only; recipient anonymity and complete note encryption still require analysis |

These conclusions are scoped to the reviewed sources. They do not establish
that no suitable implementation can exist.

## Source details

[Winterfell's repository](https://github.com/facebook/winterfell) explicitly
describes production-readiness and secret-input limitations. Choosing a STARK
library does not by itself establish that a transfer proof hides its witness.

[RISC Zero's repository](https://github.com/risc0/risc0) and
[security model](https://dev.risczero.com/api/security-model) describe different
security estimates and stages. The latter distinguishes the STARK provers from
the pairing-based STARK-to-SNARK stage, which it identifies as quantum-vulnerable.
It also requests caution for critical privacy applications. Our inference:
these statements cannot substantiate the planned 128-bit PQ privacy profile.
A pinned release and its precise receipt format need their own assessment.

[Plonky3's architecture](https://github.com/Plonky3/Plonky3/blob/main/docs/architecture.md)
distinguishes hiding FRI from non-hiding backends. It explicitly requires full
protocol analysis beyond hiding commitments. Its
[2024 audit description](https://leastauthority.com/blog/audit-of-plonky3/)
covers a non-hiding STARK implementation, so it cannot alone establish current
hiding-path privacy. Review the exact audited commits and later changes, including
the project's [security advisories](https://github.com/Plonky3/Plonky3/security).

[NIST FIPS 203](https://csrc.nist.gov/pubs/fips/203/final) describes ML-KEM and links
potential errata. [SP 800-227](https://csrc.nist.gov/pubs/sp/800/227/final) provides
KEM-use guidance. A KEM establishes a shared key; neither selecting ML-KEM nor
adding symmetric encryption alone discharges our recipient-anonymity, note
binding, nonce, key-validation and proof-of-correct-encryption obligations.

## Concrete selection handoff

The user authorized candidate implementation without a supplied review. Track
these remaining evidence items while implementing the complete adapter:

1. Exact source revisions, dependencies, licenses, deterministic build commands
   and an explicit list of accepted proof/receipt variants.
2. A complete privacy argument for trace masking, openings, lookup arguments,
   transcript messages, variable execution lengths and public metadata.
3. A quantitative quantum-security analysis across field challenges, hashes,
   commitments, Fiat-Shamir, encryption, spending and control authorization.
   A security-level flag or a classical estimate does not establish this.
4. Statement/witness definitions for every
   [operation](PROOF_RELATIONS.md), including mandatory auditor/controller
   ciphertexts and a common holder/controller consumption marker.
5. Concrete serialization, rejection rules, allocation/work limits and measured
   prover/verifier cost on the full transfer relation, including worst cases.
6. Independent review covering this construction and pinned versions, plus
   actual interoperability vectors and resolution of applicable advisories.

The existing evidence tools can package those artifacts once they exist; they
cannot supply or approve them. Current sources are discovery references and
must be pinned before being used as release evidence.

## Work that can continue now

The policy lifecycle component implements requirements independent of the proof
primitive; see [its API contract](../../internal/assetprivacy/policy/README.md).
The candidate backend, note encryption and integer constraints now compile.
Native wire encodings, the complete proof format and resource constants remain
unfrozen. Existing-file integration is authorized; missing independent-review
evidence does not block candidate implementation.
