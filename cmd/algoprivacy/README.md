# Privacy profile evidence checker

First implementation increment for NAP-01. This local tool verifies a profile
bundle's required files and pinned SHA-384 checksums. It does not evaluate the
cryptographic claims, authenticate reviewers, or enable protocol transactions.

All Go files require the `algorand_native_privacy` build tag. Default builds
exclude the command and its supporting package. Existing repository files and
dependencies are unchanged. A future consensus activation flag is separate from
this development build flag.

## Build and use

From the repository root:

```sh
go build -mod=readonly -tags algorand_native_privacy -o /tmp/algoprivacy ./cmd/algoprivacy
/tmp/algoprivacy -profile /absolute/path/to/bundle/profile.json
```

Success prints a JSON integrity report and exits zero. Invalid or incomplete
bundles exit nonzero with an error. Use `-h` for command help. The command reads
local files only and does not fetch dependencies, evidence or profiles at runtime.

## Manifest contract

The manifest is a single JSON object with exactly these lowercase keys:

| Key | Value |
| --- | --- |
| `version` | Integer `1`; this versions the offline manifest, not consensus |
| `profile_id` | Nonblank string, at most 128 bytes, no surrounding whitespace |
| `artifacts` | Array containing exactly one entry for each role below |

Each artifact has `role`, `path` and `sha384` string fields. Paths are relative
to the manifest directory, use forward slashes, and cannot escape that directory,
including through a symlink. Each path and role must occur once. The `sha384`
field is the file's 96-character hex SHA-384 digest; hex case is ignored.

Required roles:

- `cryptographic-specification`
- `security-analysis`
- `reference-implementation`
- `positive-vectors`
- `negative-vectors`
- `benchmarks`
- `cryptographic-review`
- `license-manifest`

Use an archive when one role needs multiple source files. The tool hashes the
archive without unpacking or executing it. No sample crypto evidence is shipped.

The manifest is limited to 64 KiB. Each evidence file must be a nonempty regular
file no larger than 256 MiB. Unknown fields, duplicate JSON keys, trailing JSON
values and excessive JSON nesting are rejected. These are tooling limits, not
proposed on-chain resource limits.

Use a stable local bundle while checking. Obtain the manifest itself through
the review process: a checksum verifies consistency with that manifest, not who
authored or approved it. Separate roles also require review of their actual
contents; distinct file names do not establish independent authorship.

## Current boundaries

No cryptographic profile or proof verifier is approved or implemented by this
increment. A successful report cannot pass G1 by itself. Complete the reviewed
profile and consensus specification before implementing dependent transfer code.

Tests are deferred at the user's request. Compilation, formatting and static
analysis are the current checks; adversarial behavior is not yet test-validated.

See [implementation status](../../docs/native-asset-privacy/PROGRESS.md) and
[work packages](../../docs/native-asset-privacy/IMPLEMENTATION.md).
