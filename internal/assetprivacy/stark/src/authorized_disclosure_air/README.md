# Assigned auditor/controller envelope relation

`AuthorizedDisclosureAir::new(context,authority)` accepts envelope role2
(auditor, registered key role3) or role3 (controller, registered key role4).
The nonzero public authority must come from the native current assignment;
constructor inputs supplied by a prover cannot designate an authority.
The Rust `native-privacy` feature remains disabled by default.

The complete EnvelopeAir and RegisteredKeyAir share private canonical t/rho and
viewing epoch. The registered account equals the expected public authority.
The note's private account remains the recipient account: there is deliberately
no equality between that account and the issuer's viewing account. Disclosure
membership proves no ownership secret, spending power or issuer recovery right.

`DisclosureInput` contains EnvelopeInputs, viewing Created and membership path.
`trace(input,current_root)` checks canonical note/key admission and the exact
record/path/root before allocating the main matrix. It writes bounded child
frames directly into one parent, then returns DisclosureWitness with trace,
public vector and 1629-byte envelope. All secret witnesses must remain local.

`public_values(envelope,current_root)` returns 1132 scalars: the EnvelopeAir
vector1068, current key root32 little-endian uint16 limbs, and authority32 bytes.
The public context248 excludes the private key epoch. No recipient account,
viewing key, epoch, index/path or private leaf digest is added publicly. The
same current key root must be obtained after preceding group state changes;
accepted note-root history is never a source for authorization roots.

Each recipient/auditor/controller envelope relation commits its canonical opening
to the same already-public output note commitment under the same network, asset
and policy. Independent proofs can join on that commitment under SHA3-512 binding,
without publishing private-input commitments or private lookup terminals. Native
verification must require every policy role, exact finalized context/ciphertext,
current key root and assigned account; the main conservation proof must bind the
same ordered outputs and finalized group bodies. This is a composition design,
not an implemented native verifier or a completed composed security argument.
Every proof needs independent fresh hiding entropy. Missing, extra, duplicate or
wrong-role proofs must fail; framing/header comparisons cannot verify proofs.

The active schedule is Envelope0..47882 then viewing-record membership47882..52011,
followed by padding to65536. There are23973 main columns:1088 fixed private root
registers plus22885 shared scratch. The opening stays inside EnvelopeAir; no
redundant parent copy or amount accessor is required for this public-output join. Every root is constant across all stages;
rho/epoch bits are Boolean and t is canonical through the child relations.
Preprocessing has2067 columns. The conservative source degree bound is9 through
the deepest staged membership hash; actual quotient/security analysis is pending.

The raw main matrix is12,568,756,224 bytes and preprocessing is1,083,703,296 bytes, before
hiding/LDE, quotient, commitments, allocator or other prover overhead. Bounded
writers avoid full child main matrices. Practical resource feasibility remains
unmeasured and unresolved. These relations must not be exposed as a usable wallet
or enabled in consensus until the required runtime/resource evidence exists.

Current eligibility/freeze controls, full spending/group composition, embedded
proof decoding, deterministic native verification, work limits and execution
remain pending. No tests, vectors, traces, constraints, proofs, interoperability
runs, benchmarks or node demos have run. Builds, Clippy and source/size checks
establish compilation and source properties only; no cryptographic review or
recipient anonymity guarantee is established.
