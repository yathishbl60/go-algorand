# Typed relations and independently pinned public artifacts

Updated 2026-10-27. Reference Rust selection and the Go registry reader now compile.
No preprocessing roots/files have been generated, no suite/pins are approved, and
no codec conformance, public setup, trace, constraint or proof run has occurred.

## Public schedule keys and exact relations

`Prepared::transfer`, `unshield`, `recovery`, `recipient`, `disclosure`, `retained`
and `recovered` construct concrete native AIR types. Caller-supplied AIRs, relation
labels, dimensions and key overrides are not public construction paths. The six
AIR types use ten distinct layout kinds:1,2,4,5,6,7,17,18,19,20. Exact operation,
amount/authority and envelope context enter the relation identity separately.

The relation ID is SHA3-512 of ASCII
`Algorand/AssetPrivacy/Candidate1/RelationV1/NullifierV2/`, compiled source ID64,
layout-kind byte, argument lengthu32LE and canonical public argument bytes:

| Factory | Ordered argument bytes |
| --- | --- |
| transfer | domain104, debitBE8, creditBE8 |
| unshield | domain104, creditBE8 |
| recovery | domain104, creditBE8 (zero for private variants), current authority32 |
| recipient/retained/recovered | complete public envelope context248 |
| disclosure | public envelope context248, assigned authority32 |

Domain104 is network32, assetBE8, policy64. Envelope context includes public role,
nonce, output ordinal and commitment; it excludes the private key epoch. Constructors
feed exactly these same values to the AIR. Note plaintext, ownership secrets, serials,
blindings, private epochs and witnesses never enter these public IDs.

Source audit finds preprocessing independent of dynamic constructor values. Group
schedules vary by output count; key/view, membership, eligibility and envelope
schedules are fixed. Full-private/partial-public recovery share raw schedules;
disclosure/retained recipients also share raw schedules. Equal roots therefore do
not merge kinds, public schemas, constraints or authorization. Typed selection and
full relation IDs remain mandatory. Runtime evidence for this invariant is missing.

## Public artifact production and loading

`prepared.setup_public()` invokes actual pinned preprocessing setup offline and
returns public `PreprocessedProverData` plus an immutable `Entry`. It is expensive,
not called during verification/builds, and has not been executed. Each call starts
fresh deterministic generators for public preprocessing only. Underlying hiding
Merkle trees still have five salts per leaf; zero cap height does not remove salts.
Private proof salts/masks always use separate fresh OS-seeded `prover_config()`.
The deterministic configuration is private and never returned by the setup API.

Gather one Entry for each kind in canonical order and call `encode_registry(profile,
entries)`. This produces a small file, not approval. Obtain its digest with
`registry_digest`; approve that digest/profile/reference source independently of
any file, transaction, RPC request or witness. `Registry::load(bytes, Pin)` checks
all pins, exact shape/order and the compiled source before publishing keys.
`Registry::bind(prepared)` selects the matching prekey and exact artifact identity.
`Registry::check_setup` compares public setup width/degree/root with the pinned key
before private proving; it does not validate the entire reusable prover data.

The fixed954-byte NAPK format and all dimensions are specified by the
[Go reader contract](../../internal/assetprivacy/artifactregistry/README.md).
Digest is SHA3-512 of ASCII
`Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV1/` followed by the complete file.
Both implementations check the same format; cross-language conformance is unverified.
All64-byte root values, including zero, are allowed; provenance comes from pins.

## Reference source identity

The opt-in build script hashes the checked-out crate sources under ASCII
`Algorand/AssetPrivacy/Candidate1/ImplementationSourceV1/`. It hashes file countu32LE,
then sorted relative UTF-8 paths with `/` separators: path lengthu32LE/path bytes,
content lengthu64LE/raw content. Workspace-relative inputs include core/FFI `.rs`
sources, build support/script, C header, manifests, shared lock/toolchain. File/directory rebuild
triggers include new source files. Absolute paths, times and generated OUT_DIR
output are excluded. Default-feature builds run an inert build script.

`verification::implementation_id()` exposes this compiled digest. Source edits,
including formatting, change candidate artifacts/IDs. Preserve exact source bytes
for reproducible reference artifacts. This identity is not compiler/cache provenance,
a security audit, a formal constraint digest or approval of an independent client.
Native release compatibility and normative profile/version rules still need definition.
The complete artifact ID uses domain VerifierArtifactV2, backend label, source64,
profile64, registry digest64, full relation64, dimensions/parameters and trusted root; see
[the per-AIR codec](../../internal/assetprivacy/stark/src/verification/README.md).

## Native boundary still required

Native spend/recovery snapshots now preserve Domain/Suite/Group by value and return
copied plans. Unknown suites must be rejected by independent consensus-approved
lookup; a nonzero policy Suite or a local evidence Manifest grants no approval.
Do not infer context from output slots: unshield/full-public recovery have none.
The [Go/Rust bridge](NATIVE_VERIFIER_BRIDGE.md) links; approved pins/artifacts, native signatures/body matching,
work pricing and atomic execution remain unfinished. Source/build checks do not
establish correctness, privacy, interoperability or practical resource use.
