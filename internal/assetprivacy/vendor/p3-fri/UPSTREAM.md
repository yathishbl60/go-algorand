# Pinned Plonky3 FRI input-matrix hook

Upstream: https://github.com/Plonky3/Plonky3
Revision: `1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f`; crate `p3-fri`0.8.0.
The original source is dual licensed MIT OR Apache-2.0; both licenses are retained.

## Exact change

`input-matrix.patch` makes four production edits in `src/prover.rs`: import the
existing Matrix trait and replace the three dense input-data type projections
with `InputMmcs::ProverData<impl Matrix<Val>>`. These occur in `prove_fri`, its
scheduled helper and `open_inputs`. Their six named public generic arguments
remain unchanged. No function body, FRI calculation, query selection, transcript,
proof representation, verifier or protocol parameter changes.

Two existing terminal-height tests in `src/two_adic_pcs.rs` pass empty request
slices. Their input matrix type cannot be inferred after the hook, so the patch
adds explicit dense type annotations to those two empty slices. Every other
source byte matches the pin. No tests were removed or replaced.

All ten Rust files contain10327 physical lines, including unchanged upstream
inline tests. Third-party vendoring uses the user's explicit file/function size
exception. These are upstream lines, not10327 newly designed application lines.
Eight Rust files are byte-identical; only the two files above differ.

The input Matrix must be real: correct full LDE dimensions and actual evaluation
values in the committed row order. This hook does not permit placeholder rows or
representing coefficients as LDE values. Input queries still go through the
MMCS's existing max-height and multi-opening methods. Narrow FRI commit-phase
codeword matrices retain their original dense representation.

## Manifest, validation and provenance

`Cargo.upstream.toml` preserves the original manifest. `Cargo.toml` resolves its
workspace metadata and runtime dependencies explicitly; every Plonky3 dependency
retains the exact pin and registry versions match the application lockfile.
The parallel feature remains. Dev/bench dependencies and automatic external test
discovery are omitted in this dependency-only manifest. Inline test source stays
under its original configuration; standalone upstream tests require their original
dev dependencies. The application excludes this crate from workspace members.

Before vendoring, the patched upstream library and inline tests were compiled in
a temporary standalone consumer with pinned original dev dependencies, including
Circle. `cargo check --tests` and strict Clippy passed without running the suites.
Both ordinary inferred calls and explicit six-argument calls compiled. The check
covered the current non-parallel feature selection; optional Rayon was not enabled.

`SHA3_512_MANIFEST.json` records the pin, every original/vendored Rust digest,
manifest/license/document hashes and copy counts. It does not hash itself.
The proof build fingerprints all vendored source, manifests, licenses, provenance,
the manifest itself and both copies of the patch. Changing any input changes the
implementation identity and invalidates previously pinned artifact references.
The mirrored patch is `stark/patches/p3-fri-input-matrix.patch`.

This hook is groundwork for coefficient-backed proving, not a complete PCS or
an8GiB guarantee. Existing proof/profile selection and disabled consensus defaults
remain unchanged. No security approval follows from compilation or generic types.
