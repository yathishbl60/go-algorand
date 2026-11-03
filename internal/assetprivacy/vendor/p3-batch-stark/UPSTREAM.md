# Pinned Plonky3 batch quotient scheduler

Upstream: https://github.com/Plonky3/Plonky3
Revision: `1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f`; crate `p3-batch-stark` 0.8.0.
The original source is dual licensed MIT OR Apache-2.0; both licenses are retained.

## Source and patch boundary

The original crate has thirteen Rust source files and 5,142 physical lines,
including its existing inline tests. All original files are retained. Changes
are limited to `src/lib.rs`, `src/prover.rs`, and the added
`src/quotient_scheduler.rs` interface. `quotient-scheduler.patch` is the complete
source diff against the pin; its identical mirror is
`stark/patches/p3-batch-stark-quotient-scheduler.patch`.

`prove_batch` delegates to the new `prove_batch_with_quotient_scheduler` using
`WholeDomainQuotient`. Its default evaluates each complete quotient domain and
preserves the original instance parallelism. An explicitly selected scheduler
may evaluate the same AIR/PCS callback over compatible smaller domains and may
disable parallel instances. The callback retains the existing quotient formula,
original trace-domain selectors, public values, lookups, and periodic columns.
The scheduler must return every quotient value in the original natural row order.

This patch introduces prover scheduling only. It does not edit verifier source,
proof encoding, transcript structure, security parameters, AIR equations, quotient
randomization, or native/default profile selection. A malformed scheduler is a
local prover implementation error; successful verification remains necessary.

## Manifest and provenance

`Cargo.upstream.toml` is the original manifest. The standalone `Cargo.toml`
spells out its workspace metadata, runtime dependencies, and lints. All Plonky3
dependencies retain the exact upstream revision; registry versions match the
application lockfile. The original parallel feature remains available.
Dependency-only vendoring omits external tests, benchmarks, and dev dependencies.
Existing inline test source is retained; standalone upstream tests need their
original dev dependencies. The application excludes this crate from its members.
The lockfile changes only this crate's source from the pinned git revision to
the local patch; package versions and dependency lists remain unchanged.

The source size exception for third-party vendoring applies to the retained
upstream code. Application scheduler implementations and tests follow the
20-line function and 100-line file limits.

`SHA3_512_MANIFEST.json` records original and vendored source digests, copy counts,
metadata hashes, and the exact patch digest; it does not hash itself. Build
identity inputs include every vendored Rust file, metadata, manifest, license,
provenance document, digest manifest, and both patch copies. Any change changes
the implementation reference and requires matching rebuilt binaries/artifacts.

Scheduling is groundwork for bounded proving. It is not a complete native proof
profile, an aggregate RAM guarantee, cryptographic review, or suite approval.
