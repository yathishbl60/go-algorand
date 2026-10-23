# Native asset privacy: north-star implementation plan

Status: implementation in progress; inactive candidate, not end-to-end or security-approved.
Updated: 2026-10-22. See [current delivery status](END_TO_END_STATUS.md).

Start here, then use [IMPLEMENTATION.md](IMPLEMENTATION.md) to pick a work package.
This document fixes product behavior and architectural boundaries. The first two
work packages define the cryptographic and consensus specifications. The user
authorized implementation of a pinned, explicitly unreviewed candidate without
a supplied cryptographer-reviewed profile. See [candidate details](CANDIDATE_PROFILE.md).

## 1. Outcome and fixed requirements

Algorand nodes natively validate confidential ASA transfers. A user selects a
privacy mode permitted by the asset, retains control of their funds, and can
provide verifiable payment disclosures. Private transfers participate in atomic
groups with other private transfers and ordinary Algorand transactions.

The deliverable is an upstream-ready production release: protocol specification,
node implementation, reviewed cryptography, persistent ledger support, Go SDK,
CLI and local wallet tooling, operational documentation, and a working usage demo.
Mainnet approval and activation are separate network decisions.

| Requirement | Decision |
| --- | --- |
| Implementation layer | Native transaction validation and ledger state in `go-algorand` |
| Assets | New ASAs and eligible existing ASAs, retaining the same asset ID |
| Configuration | Issuer declares allowed modes and authority capabilities; users choose within them |
| Quantum requirement | New privacy proofs, commitments, encryption, and private spending authorization must meet a reviewed post-quantum security profile at launch |
| Disclosure | Holder-selected payment receipts; optional issuer-declared auditor access |
| Issuer controls | Preserve declared freeze/clawback rights across public and private holdings |
| Completion | Production release candidate suitable for upstream review, including independent security reviews and a multi-node demonstration |

Private ALGO, changes to consensus selection, confidential general-purpose AVM
execution, and a confidential AMM are outside this release. Atomic swaps with
agreed terms are included. The new feature's cryptographic claims remain
conditional on the host chain's consensus security; adding privacy does not
make every existing Algorand component quantum-resistant.

### Privacy modes

| Mode | Publicly visible | Hidden by the protocol |
| --- | --- | --- |
| Public | Asset, participants, transferred amounts and ordinary public holdings | Nothing beyond existing behavior |
| Confidential | Asset and authenticated participant identities, transaction timing and size | Private transfer amounts and private note values |
| Shielded | Asset, commitments, spent markers, transaction timing/size, fee payer and group membership | Private owners, note values and input-to-output ownership links |

Both private modes use the same note ledger and proof system. Confidential mode
adds participant disclosures checked by the proof. It does not introduce a
second encrypted-balance accounting implementation.

Public balances, aggregate private supply, and public/private conversion amounts
remain public. An observer may infer information from small anonymity sets,
known payments, timing, or public conversions. Shielded mode means ledger
privacy, not IP anonymity. The SDK must display these boundaries when converting
or mixing modes. A transfer cannot make its earlier public history secret.

## 2. Protocol behavior and architecture

### Asset policy and existing assets

- Extend asset configuration with a versioned privacy-policy reference backed by
  native state. The policy fixes allowed modes, disclosure requirements, and
  whether freeze/clawback capabilities exist. Allowed modes and disclosure rules
  are immutable after adoption. Authority capabilities cannot be expanded;
  irrevocable relinquishment remains irrevocable. Key rotation uses explicit
  epochs within the existing rights.
- New ASAs declare the policy at creation. A private-only policy may prohibit
  ordinary public transfers and withdrawals. Initial allocation to the creator
  remains the ordinary creation operation; the creator can convert that supply
  into private notes after the asset ID exists.
- An existing ASA's current manager may attach a policy once. Public transfers
  remain permitted, public balances do not move, and each holder independently
  converts an amount after accepting the policy. Preserve existing freeze and
  clawback capabilities; do not introduce new seizure powers through this path.
- A legacy ASA whose manager has been permanently removed cannot acquire a new
  policy through a fabricated issuer authority. It remains public in this release.
  Supporting such immutable assets requires a separately reviewed protocol rule.
- Conversion keeps the asset ID and supply. There is no wrapper token, application
  escrow, bridge, or operator holding the private assets.
- Assets cannot be destroyed while private value remains outstanding. Destruction
  must reconcile all supply back to the existing creator holding, as well as
  satisfy the existing destruction authorization rules.
  For private-only assets, permit final reconciliation only atomically with
  authorized destruction; this does not enable ordinary public withdrawals.

### Native state and value conservation

Use private notes: committed records representing an amount of one ASA owned by
a private spending key. A spend consumes notes and creates new notes. A
nullifier is a deterministic public marker preventing a note from being spent
again without publicly identifying the consumed note.

- Maintain an append-only commitment tree, spent-nullifier set, bounded accepted
  root history, policy/key records, and aggregate outstanding private supply.
- Support native operations for privacy registration, public-to-private
  conversion, private transfer, private-to-public conversion, and authorized
  recovery/clawback. Use existing asset configuration/freeze operations where
  their semantics remain appropriate.
- Prove note membership, spending authority, amount bounds, exact conservation,
  correct nullifiers, allowed modes, and required ciphertext/disclosure binding.
  Check nullifier freshness and current authorization state in ledger evaluation.
- All values use ASA integer units; reject overflow and modular-wrap inflation.
  Public debits/credits and private supply deltas commit or roll back together.
- Accept recent commitment roots for spend construction. Authorization and freeze
  state must be current at evaluation; a previously valid proof cannot bypass a
  subsequent freeze. Return a distinct stale-state error so wallets can rebuild.
- Persist native privacy records in a reserved key namespace through the existing
  tracker-backed KV and copy-on-write machinery. AVM box operations cannot access
  or mutate that namespace. Add typed accessors and consensus-defined encoding.
- Include every consensus-relevant record in replay, snapshots, catchpoints,
  catchup and crash recovery. Retaining a commitment-tree frontier alone is not
  enough for a fresh node to serve historical membership witnesses.

The starting code locations are `data/transactions`, `ledger/apply`,
`ledger/eval`, `ledger/ledgercore`, and `ledger/store/trackerdb`. Existing ASA
holdings are public `uint64` amounts; extend accounting explicitly instead of
using zero to stand in for a hidden amount.

### Transaction and proof binding

- Add a dedicated native privacy transaction with a tagged, bounded payload;
  the proposed transaction type is `aprv`. Freeze final wire tags in the
  consensus specification before generating codecs or SDK types.
- Store the privacy proof in a separately bounded `SignedTxn` witness field.
  It is required in addition to the ordinary outer authorization for operations
  that need it. The outer sender may be a fee sponsor; it is not implicitly the
  owner of private inputs.
- The transaction body commits to the operation, asset/policy/suite versions,
  roots, nullifiers, output commitments, ciphertexts and public disclosures.
  The proof binds that body, genesis/network domain, validity window and final
  atomic group. Proof bytes stay outside the transaction ID so proof generation
  does not create a circular dependency with group construction.
- Construct output data, finalize all group transaction bodies, then generate
  private proofs and outer signatures. A group move, changed payment term or
  changed ciphertext invalidates the affected proof. Block encoding still commits
  to the complete proof witness.
- Verify stateless cryptography in the bounded verification pipeline, then
  recheck state-dependent conditions during ordered block/group execution.
  Cache keys include the exact proof, suite and statement; cached proof validity
  never substitutes for current-state checks.
- Require approved PQ authorization for privacy-specific registrations, value
  operations and administrative changes, including existing configuration/freeze
  transaction types when they govern privacy-enabled assets. Enabling a controlled
  asset therefore requires its relevant authorities to adopt approved PQ keys.
  Ordinary public holder transfers retain their existing authorization semantics.
  The privacy spending proof must not depend on an Ed25519/ECDSA ownership
  relation hidden inside the circuit.

### Issuer authority and disclosure

- Holders can disclose one payment with a receipt that proves its relationship
  to the confirmed transfer. Viewing permission never implies spending authority.
- If an asset requires auditor access, each applicable transfer proves that its
  audit ciphertext encrypts the same information used by the spending proof.
  Merely attaching an arbitrary encrypted blob is insufficient.
- Controlled assets must also support their declared freeze/clawback rights in
  shielded mode. Use authenticated holder eligibility/revocation state and a
  separate recovery authority path, reviewed as part of the proof relation.
  A holder spend and a clawback must consume the same underlying spend marker.
- Mandatory controller access to note recovery information is disclosed in the
  asset policy. Auditor-only keys must not receive recovery authority. Partial
  clawback must conserve the remainder to the original owner.
- Key rotation is prospective and epoch-bound. Retain the key material required
  to decrypt/recover older outputs; rotation cannot revoke data already disclosed.
  An authority cannot activate a new epoch without the required key material and
  proof-of-control checks. Freeze/clawback actions follow deterministic ledger
  ordering, including conflicts with an in-flight ordinary spend.

### Storage, fees and denial-of-service limits

- Charge verification and encoded-byte fees, and account for persistent state
  through minimum-balance requirements. Track the additional storage obligation
  on the asset creator account; it cannot be withdrawn while that state exists.
  Any user can fund a shortfall atomically with their operation. Contributions
  do not promise a refund to the hidden note owner.
- Price permanent nullifier/tree growth, not only currently unspent notes. Do
  not remove spent markers while their asset can still accept spends. Only
  release storage minimum balance after the corresponding state is safely removed.
- Define consensus limits for proof bytes, input/output counts, ciphertexts,
  verifier work, per-block privacy work and state growth. Bound decoding and
  verification before expensive allocation; fees alone do not stop invalid-proof
  spam. Rate-limit and bound peer verification queues separately.
- Use fixed consensus work units, never measured execution time, to decide
  block validity. Benchmark results determine reviewed constants before release.
  Do not assume ordinary-transfer throughput applies to private transfers.

### AVM and atomic settlement

Support private transactions as top-level members of ordinary atomic groups.
Expose only their public metadata, policy identifiers and receipt/settlement
commitments to AVM group inspection. Add type names and field version gates.

The release includes two-party atomic exchange with agreed terms: each party
checks the group and authorizes its own leg. Public applications may validate
public settlement commitments. They cannot read hidden amounts or treat a
commitment as a number. Generation of privacy proofs from inner transactions
and arbitrary user-supplied proof circuits are deferred.

## 3. Cryptographic specification: the mandatory first dependency

The chosen direction is a transparent, genuinely zero-knowledge STARK-family
proof system, post-quantum recipient/auditor encryption, and approved PQ
authorization. There is no classical proof fallback or pairing-based compression
stage in the production path.

This is an architectural choice, not a claim that a reviewed implementation is
already available in this checkout. Existing pairing/MiMC AVM operations do not
provide this feature's quantum-safe proof system.

The cryptography work package must deliver a pinned `PrivacyCryptoProfileV1`:

1. Exact prover/verifier implementation versions and licenses; proof relation,
   field, hash/transcript, masking, soundness parameters and canonical encoding.
2. A security argument covering quantum soundness, witness privacy, commitment
   hiding/binding, nullifier unlinkability, spending authority and recovery.
   Target at least 128-bit post-quantum security, supported by an explicit
   analysis of the complete construction rather than a library setting.
3. Recipient and auditor encryption with bound associated data and key
   validation. Evaluate ML-KEM-768 plus AES-256-GCM as the initial candidate,
   including current NIST errata. ML-KEM is a KEM, not a complete encryption or
   anonymity protocol. Shielded mode additionally needs a reviewed recipient
   anonymity/key-privacy argument; never publish the recipient key as routing
   metadata and assume the ciphertext hides it.
4. Formal public-input and witness definitions for every operation, including
   issuer controls, policy changes, encryption correctness and group binding.
5. Independent positive and negative vectors, including nonzero real transfers,
   corrupted proofs, malformed keys and authority/holder spend conflicts.
6. Reproducible benchmarks covering proving time/RAM, verification time/RAM,
   proof/transaction size and storage growth, including worst-case valid inputs.
7. Reviewed fixed limits and an exact interoperability manifest. Neither the
   sender nor an asset issuer can select arbitrary verifier parameters.

The small node-facing verifier has deterministic inputs, bounded work, no
network access and no trusted proving service. Wallets own witnesses and prove
locally. A faster prover is optional only if it produces the same approved
format without changing the trust model.

**Current implementation authorization:** proceed with a pinned candidate without
a supplied review report. Keep implementation choices, unresolved security
arguments and review status explicit. Do not weaken the target, add a permissive
verifier or describe compilation as proof of security. The release-quality review
criteria below are not a prerequisite for writing the authorized implementation.

Primary references:

- [NIST FIPS 203](https://csrc.nist.gov/pubs/fips/203/final) standardizes ML-KEM;
  its publication page also links errata and KEM-use guidance.
- [STARK overview and papers](https://starkware.co/stark/) explain the transparent
  proof-system direction. A specific library still needs its own evaluation.
- [Winterfell's README](https://github.com/facebook/winterfell) currently warns
  that it is unaudited and lacks perfect zero knowledge. Do not choose a library
  solely because it calls its proofs STARKs.
- [Algorand's PQ roadmap](https://algorand.co/blog/algorand-post-quantum-cryptography-roadmap)
  supplies host-chain context, not an assurance for this proposed feature.

## 4. Developer interfaces and usage demo

### Interfaces to ship

| Surface | Required capability |
| --- | --- |
| Protocol/specification | Privacy policy, native operations, proof witness, state transition rules, fees/limits, upgrade behavior and vectors |
| `algod` | Discover policy/suite/limits; retrieve roots and membership data; page encrypted outputs; query nullifier status; simulate and submit native transactions |
| Go SDK / `libgoal` | Build each operation; inspect disclosures; assemble an atomic group before proving; verify/decrypt receipts; distinguish stale-state from invalid-proof errors |
| Local privacy wallet | Separate spending/view/recovery keys; encrypted backup; local scanning, witness maintenance and proving; explicit key rotation and recovery workflows |
| `goal` | Create/enable policy, register, shield, transfer with a mode, unshield, disclose/verify receipts, and exercise declared administrative controls |
| Indexer/explorer integration | Decode the new transaction/witness metadata, show visibility accurately, and index public encrypted-output events without pretending to know private balances |

Read endpoints return round-qualified data with bounded pagination. Never send
private amounts, note openings, viewing keys or proof witnesses to `algod`
simulation or telemetry. A node verifies proofs; it does not need the plaintext.
An unavailable private balance is absent/explicitly unknown, never numeric zero.

Wallet rescan must work using an archive node plus authenticated state/witness
data, and from an encrypted wallet checkpoint. A pruned node need not retain all
historical ciphertexts; APIs must explicitly report retention limits. A remote
membership lookup may leak which commitment is of interest, so the default
shielded wallet builds witnesses from public streams/checkpoints and avoids
owner-specific requests. Current balances cannot be reconstructed from a viewing
key unless the selected profile supplies the necessary spend-tracking material.

Use a thin local demo application backed by the same SDK, local prover and
production node binaries. Keep its source separate from consensus packages. It
must run these scripted scenarios on a network of at least four nodes:

1. Create an ASA with all three modes; show the public view and each holder view
   for public, confidential and shielded payments.
2. Enable privacy on an existing ASA; demonstrate holder consent, unchanged
   public holdings and supply conservation across conversion in both directions.
3. Run private payroll to multiple recipients; reveal one verifiable payment
   receipt while leaving the other payments undisclosed.
4. Execute a two-asset atomic swap; corrupt one proof and show that neither leg
   changes balances, commitments or nullifiers.
5. Exercise a controlled asset's freeze, partial clawback and audit access; show
   that an auditor alone cannot spend.
6. Restore a wallet and a node, then spend restored funds. Repeat through a
   protocol upgrade and catchpoint catchup.

The demo cannot bypass validation, use fake proofs or expose a server-side
plaintext balance store as the source of truth. Successful transfers produce
normal block confirmations and independently verifiable artifacts.

## 5. Delivery gates and definition of done

| Gate | Required evidence |
| --- | --- |
| G1: cryptography and semantics frozen | Reviewed crypto profile, consensus specification, compatibility matrix, reference vectors and resource measurements |
| G2: native lifecycle complete | Real create/enable/register/shield/transfer/unshield/control operations, atomic rollback, storage funding and malformed-input rejection |
| G3: operational recovery complete | Replay, all tracker backends, restart, catchpoints, catchup and encrypted wallet recovery agree on state |
| G4: ecosystem ready | Go SDK, CLI, public API, AVM inspection, explorer/indexer decoding and all demo scenarios use the same protocol |
| G5: release candidate accepted | Independent cryptography and implementation audits resolved; fuzz/race/E2E/codegen checks pass; adversarial load and upgrade evidence published |

Initially gate the feature under `ConsensusFuture` and a dedicated development
network. Never change historical consensus-version behavior. Exact activation
versions and network resource limits follow the frozen specification and
maintainer review. No production node-local switch can change validation.

Release evidence must include a fixed hardware/workload description, p50/p95/p99
verification and proving results, maximum-block validation budgets, node memory
and state-growth measurements, mixed public/private workloads, and long-running
recovery tests. Numeric performance limits are a G1 deliverable derived from
measurement, not invented promises in this plan. Tests cannot establish a
cryptographic security theorem; independent analysis is required as well.

Protocol rollout is forward-only after activation: old binaries cannot safely
resume validating new private state. Document future protocol upgrade and suite
migration procedures, including retaining verification of historical blocks.
Do not introduce an operator kill switch that seizes assets or silently changes
consensus. Publish aggregate metrics without private values or wallet identifiers.

The Algorand specification and Go SDK repositories are required deliverables but
are not present at the expected sibling paths in this workspace. Obtain their
canonical checkouts before their work packages; do not claim those changes are
complete based only on node code. Indexer changes are another explicit cross-repo
dependency. Repository contribution policy requires subject-matter review;
upstream acceptance and mainnet activation are not guaranteed by this plan.
