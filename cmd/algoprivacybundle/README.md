# Prepare a privacy evidence bundle

This companion to [algoprivacy](../algoprivacy/README.md) creates a manifest from
existing evidence files. It removes manual checksum assembly for developers and
agents. Neither command generates proofs or establishes cryptographic approval.

## Prepare, create, verify

Obtain actual evidence from the cryptographic profile owners. Place these eight
nonempty regular files in one stable local directory:

| Evidence role | Conventional filename |
| --- | --- |
| Cryptographic specification | `cryptographic-specification.md` |
| Security analysis | `security-analysis.md` |
| Reference implementation | `reference-implementation.tar` |
| Positive vectors | `positive-vectors.json` |
| Negative vectors | `negative-vectors.json` |
| Benchmarks | `benchmarks.json` |
| Independent cryptographic review | `cryptographic-review.md` |
| Dependencies and licenses | `license-manifest.json` |

The command hashes opaque bytes; extensions do not imply content validation.
It does not execute or unpack the archive, run vectors, authenticate a reviewer,
or interpret benchmark results. Do not substitute empty claims or dummy artifacts
for missing review evidence.

From the repository root:

```sh
go build -mod=readonly -tags algorand_native_privacy -o /tmp/algoprivacybundle ./cmd/algoprivacybundle
go build -mod=readonly -tags algorand_native_privacy -o /tmp/algoprivacy ./cmd/algoprivacy

/tmp/algoprivacybundle -directory /absolute/path/to/evidence -id candidate-profile-v1
/tmp/algoprivacy -profile /absolute/path/to/evidence/profile.json
```

The first command prints the new manifest's path. The checker prints a JSON
integrity report. Successful commands exit zero; errors exit nonzero.
Use `-h` for help. Missing flags and positional arguments are rejected.
A profile ID must be nonblank, at most 128 bytes, without surrounding whitespace.

## File handling

- Creates only `profile.json` in the supplied, already-existing directory.
- Opens the new manifest exclusively with owner read/write permissions
  (`0600`, subject to platform permission semantics); existing paths fail.
- Reads each artifact through a directory-confined filesystem root and rejects
  paths that escape it, including through symlinks.
- Reuses the checker's 256 MiB per-artifact and 64 KiB manifest limits.
- Writes manifest version 1, fixed role order and lowercase SHA-384 digests.
  Identical IDs and artifact bytes produce identical manifest bytes.
- Flushes and closes the new manifest, reporting write, flush and close errors.
- A failed write or interrupted process can leave a partial new manifest.
  Inspect that file before explicitly removing it and retrying.
- Reads local evidence only; it does not fetch profiles or evidence at runtime.

Keep the directory unchanged for the entire create-and-verify workflow. This is
not a filesystem snapshot or an atomic publication mechanism. An output error
can also occur after the manifest was successfully created; inspect the reported
directory before retrying. No artifact or existing manifest is overwritten.

For another revision, use a separate evidence directory. The existing checker
also accepts manually assembled manifests with other relative filenames.

## Integration boundaries

Every Go file requires the `algorand_native_privacy` build tag; default builds
exclude these tools. The tag is a development switch, not a consensus activation
mechanism. No dependency, node, wire format, ledger or SDK changes are included.

The manifest itself needs a trusted review/distribution process. A successful
checksum report establishes consistency with that manifest, not authenticity,
quantum resistance, proof soundness or readiness to activate native privacy.

Tests remain deferred at the user's request. Build and static checks do not
establish end-to-end privacy behavior. See the
[current delivery status](../../docs/native-asset-privacy/END_TO_END_STATUS.md).
