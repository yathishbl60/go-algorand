# Native asset privacy implementation progress

Updated 2026-10-23. [Current end-to-end status](END_TO_END_STATUS.md) is the
canonical completion checklist. The full feature remains in progress.

## Current user instructions

- Necessary edits to existing consensus, transaction, ledger and codec files are
  authorized; the earlier new-files-only restriction has been superseded.
- New functions: at most 20 physical lines, including declaration and braces.
- New handwritten files: at most 100 lines. Generated dependency lockfiles may
  exceed 100 lines, explicitly authorized on 2026-10-22.
- Keep the implementation disabled by default.
- Do not write or run tests yet; builds and static analysis are permitted.
- Proceed without a supplied cryptographer-reviewed profile. Keep the selected
  candidate and its review status explicit; do not claim defect-free security.

## Delivered components

1. Profile evidence manifest preparation and integrity checking. These commands
   hash supplied artifacts; they do not authenticate reviewers or approve crypto.
2. Policy lifecycle checks for allowed modes, immutable disclosures, existing-ASA
   adoption and nonexpansion of relinquished issuer capabilities.
3. Aggregate supply arithmetic, inactive consensus flag and native COW KV access.
4. Native asset-destruction hook requiring zero outstanding private value; failed
   lookups, invalid counters and missing feature implementations fail closed.
5. Pinned Plonky3 hiding-STARK configuration, full dependency lock and uint64
   conservation constraints. SHA3/note-opening/private-ownership AIRs now compose
   without exposing consumed-note commitments. Nullifier and 32-level membership
   constraints now compose; no complete transfer verifier yet.
6. Fixed-format note openings, SHA3-512 commitment/nullifier/owner derivation,
   ML-KEM-768/AES-256-GCM envelopes, and local context-checked encrypt/decrypt APIs.

The new Go packages use `algorand_native_privacy`. The Rust crate's optional
`native-privacy` feature is off by default. No consensus version enables native
asset privacy. See [candidate choices](CANDIDATE_PROFILE.md) and
[native accounting](NATIVE_ACCOUNTING.md).

7. Persistent content-addressed depth-32 commitment tree, atomic COW appends and
   membership reads, bounded accepted-root history; spent-marker and policy-record
   access/mutations; full-body group digest and public-scalar encoding. Fixed2-in/
   2-out transfer AIR now composes membership, conservation and group context.
   Native transaction callers remain absent.
8. Role-separated key epochs with canonical ML-KEM/Falcon key checks and owner
   commitments; bounded opaque proof framing; zero-input shield and zero-output
   unshield relations. AES-256 block rounds/key expansion and GHASH are constrained.
   Embedded proof decoding, complete encryption and native acceptance are absent.
9. Account freeze/control epochs, close-out preservation and current eligibility;
   anonymous eligibility proofs and authenticated administrative ingress remain.
10. SHA3-256/512 messages through 4096 bytes and private HMAC/HKDF constraints;
    full KEM/GCM binding and practical enclosing schedules remain unfinished.
11. Shared-key AES blocks and fixed GCM counter/CTR/GHASH/tag constraints with
    witness helpers; KEM/note binding, scheduling and native verification remain.
12. Private vertical GHASH product with trusted preprocessing, 769 columns/512 rows.
    Full-chain composition and final hiding/resource budgets remain unfinished.

## Verification scope

Feature-enabled Go compilation/static analysis and default native-package builds
pass. The pinned Rust backend compiles and passes Clippy with warnings denied.
Formatting and new-source size checks pass. These checks establish build/static
properties only; no tests, cryptographic vectors, proofs, transfers, replay or
multi-node demo have run. No security review has been supplied or completed.

## Next work

Complete proof-constrained encryption and issuer controls, remaining operation
arities and native transaction verification/execution. Membership assembly now
retains one stage at a time; proving resource feasibility remains unmeasured. Bind the finalized ordered group
and current authorization state; finish embedded proof decoding and verification.
Then complete native transaction/types/state, API/SDK/wallet/CLI and the real-node
scenarios. Maintain the full [work-package scope](IMPLEMENTATION.md).
