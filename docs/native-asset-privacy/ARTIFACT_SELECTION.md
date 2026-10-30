# Typed relations and independently pinned public artifacts

Updated 2026-10-30. Native shield selection is now wired in source; compilation
of this increment is tracked separately. No preprocessing files, approved suite entries, setup, codec conformance,
traces, constraints or proofs have been executed/generated for this candidate.

## Public schedule keys and exact relations

Prepared::owner_registration, shield, transfer, unshield, recovery and output factories
construct eight AIR types across twelve kinds:1,2,3,4,5,6,7,17,18,19,20,21. There
are no public arbitrary AIR/relation/dimension/key override constructors. Exact
operation, amounts/authority and envelope context enter the relation identity.

prepared.relation_id(&registry) hashes ASCII
Algorand/AssetPrivacy/Candidate1/RelationV1/NullifierV2/, selected registry reference64,
layout-kind byte, argument lengthu32LE and canonical public argument bytes:

| Factory | Ordered argument bytes |
| --- | --- |
| transfer | domain104, debitBE8, creditBE8 |
| unshield | domain104, creditBE8 |
| shield | domain104, debitBE8; staged ShieldGroupAir |
| owner_registration | domain104; owner/group are64 public cells |
| recovery | domain104, creditBE8 (zero for private variants), current authority32 |
| recipient/retained/recovered | complete public envelope context248 |
| disclosure | public envelope context248, assigned authority32 |

Domain104 is network32, assetBE8, policy64. Context includes role/nonce/output
ordinal/commitment, without private epoch. Constructors feed those same values
to AIR constraints. Plaintext, secrets, serials, blindings and paths stay private.
relation_id requires the registry: the former no-argument signature is obsolete.

Source audit finds schedules independent of dynamic constructor values. Group
schedules vary by output count. Full-private/partial-public recovery share raw
schedules; disclosure/retained recipients also share them. Equal preprocessing
roots never merge relation kinds, public schemas or authorization. Exact typed
selection remains necessary; runtime evidence for this invariant is missing.

## Public artifact production and loading

prepared.setup_public() performs real expensive pinned preprocessing offline,
returning public prover data and an immutable Entry. It is not called during
verification/builds and has not run. Each call uses fresh deterministic generators
for public preprocessing only. Hiding Merkle leaves still have five salts; cap
height zero does not remove them. Private proofs use fresh OS-seeded prover_config;
the deterministic configuration is private and never returned by setup.

Gather all twelve Entries in canonical order, then encode_registry(profile,entries).
The encoder writes the current build fingerprint as the new registry reference.
The resulting file/digest is not approval. Independently approve profile,
reference and full registry digest through [consensus metadata](NATIVE_SUITE_CATALOG.md).
Registry::load checks exact pins, shape/order and supported reference before
publishing keys. Registry::bind(prepared) selects the expected layout and derives
its artifact identity using that same reference. Registry::check_setup checks
width/degree/root against a pinned key, not complete reusable prover data.

The1118-byte NAPK version3 format/dimensions are in the [Go registry contract](../../internal/assetprivacy/artifactregistry/README.md).
Its digest hashes ASCII Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV3/ followed
by the entire file. Root words allow every64-byte value; approval/provenance comes
from independent pins. Cross-language conformance remains unverified.

## Compiled fingerprint versus frozen reference

verification::implementation_id() exposes the current compiled source fingerprint.
The opt-in build script hashes sorted workspace-relative core/FFI Rust sources,
build support/script, C header, manifests, shared lock/toolchain under ASCII
Algorand/AssetPrivacy/Candidate1/ImplementationSourceV1/. Encoding is file countu32LE,
then path lengthu32LE/path UTF-8 bytes and content lengthu64LE/raw bytes. Absolute
paths/times/generated OUT_DIR output are excluded; default builds are inert.
This is source identity, not compiler/cache provenance, audit or constraint proof.

Registry::reference_id() returns its selected frozen reference. reference_supported
accepts the current fingerprint or a compiled COMPATIBLE_REFERENCES entry. That
historical array is currently explicitly EMPTY, so there is no historical source
compatibility today. Peers, manifests and registry files cannot extend it.

Future reviewed identical constraints, schedules, encodings, artifact-ID
construction, decoder charges and parameters may support a frozen reference
while build bytes change. Prepared relation IDs and
VerifierArtifactV2 IDs bind that selected reference, keeping their preimages
stable under approved compatibility. Incompatible changes need versioned dispatch;
adding an old fingerprint without establishing equivalence is insufficient.

The artifact ID hashes domain VerifierArtifactV2, backend/configuration label,
reference64, profile64, registry digest64, full relation64, dimensions/parameters,
row-access shape and trusted root. See the [codec](../../internal/assetprivacy/stark/src/verification/README.md).
Live asset suites and historical proofs require retention across upgrades; the
[catalog contract](NATIVE_SUITE_CATALOG.md) describes the outstanding normative rule.

## Native boundary

Native plans retain value-copy Domain/Suite/Group and copied statements, including
zero-output operations. Authoritative VerifyAssetPrivacySpend/Recovery rebuild
those plans and select current consensus pins; generic caller-supplied Verifier
helpers do not establish this provenance. All current catalogs remain empty.
NAPK3/NAPV3 supersede unapproved versions1/2; no historical approved references exist.
Approved artifacts, practical work pricing and runtime execution remain missing. No build/source result establishes proof correctness,
privacy, interoperability or practical resource use. External specs/SDK integration
is still required; sibling checkouts are absent.
