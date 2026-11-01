# Pinned Plonky3 matrix compatibility fix

Upstream: https://github.com/Plonky3/Plonky3
Revision: `1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f`; crate `p3-matrix`0.8.0.
The original source is dual licensed MIT OR Apache-2.0; both licenses are retained.

## Exact production change

`d10.patch` adds nine lines to `Matrix::rowwise_packed_dot_product` in `src/lib.rs`.
The pinned optimized coefficient kernel returns eight accumulators and panics for
extension degree10. For degrees above eight, the new branch computes the same
packed mixed dot product with generic extension-by-base multiplication. Degrees
at most eight retain their original optimized path. No parameters, commitments,
transcript, serialization, AIR constraints or native activation are changed.

The patch contains18 lines including unified-diff context, and changes one file.
All ten source files are otherwise exact upstream copies. Their8694 physical
lines include unchanged upstream inline tests. These third-party files and
functions use the user's explicit exception to the handwritten size limits.
No automated tests were authored or run as part of this vendoring step.

## Dependency-only manifest

`Cargo.upstream.toml` preserves the original manifest. `Cargo.toml` resolves its
workspace package metadata and runtime dependencies explicitly. Plonky3 runtime
dependencies retain the exact upstream revision; registry versions match the
application lockfile. Upstream benches/dev-dependencies are omitted, automatic
bench/integration-test discovery is disabled, and the parallel feature remains.
The application excludes this dependency from its workspace members.
Existing inline tests remain in source behind their original test configuration;
this dependency-only manifest does not promise standalone upstream test support.

`SHA3_512_MANIFEST.json` records the source revision, original/modified SHA3-512
digests, manifest hashes, license/document hashes and copy counts. It does not
hash itself. The proof build fingerprints all vendored Rust sources, manifests,
licenses, provenance and patch, so an approved artifact cannot silently outlive
a changed dependency source. No compact artifact is currently approved.

Offline library compilation and package-selected Clippy passed using an isolated
temporary consumer. Direct standalone offline resolution requires cached optional
Rayon metadata; the application currently does not enable that feature.
The fallback is a compatibility repair, not field or protocol security approval.
