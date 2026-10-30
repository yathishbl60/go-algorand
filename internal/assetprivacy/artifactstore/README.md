# Bounded local preprocessing-registry bytes

`artifactstore` is available only with `algorand_native_privacy`. It supplies public
artifact bytes for independently selected digests. It does not approve a suite,
validate registry structure, choose pins/budgets or verify proofs. Every current
consensus version keeps native privacy disabled and its suite catalog empty.

`Provider.Get(registryDigest)` returns a fixed1118-byte value and an error.
`NewDirectory(root)` implements that interface using an operator-supplied trusted
local directory. Construction only remembers the path: it creates no directory,
opens no file and installs no approval. An empty root or zero digest is unavailable.
Relative roots use the process working directory; callers should supply their
stable node-local path. The root path and its parent resolution are trusted local
configuration, not peer-supplied transaction or witness fields.

Get derives only lowercase hex of the complete64-byte digest plus `.napk` below
that root. A peer cannot add separators, choose another filename or supply a path.
On Linux/Darwin it opens with O_NOFOLLOW, O_NONBLOCK and O_CLOEXEC, then requires
a regular file before any read. The atomic leaf no-follow check rejects symlinks;
nonblocking open prevents a FIFO replacement from waiting for a writer before
regular-file rejection. Other platforms fail closed without opening a file.
There is no Lstat/Open time-of-check race and no mutable global cache.

Reads use a fixed1118-byte buffer followed by a one-byte strict EOF probe. A short
file, trailing bytes, failure to establish EOF or digest mismatch returns no
partial artifact data. The returned value owns all bytes; no file/reader/pointer
or caller slice escapes. File handles close before return. Concurrent Get calls
share only the immutable configured root and use independent handles/buffers.

Digest checking uses artifactregistry.Digest's domain-separated SHA3-512 format.
It proves only that the returned public bytes match the independently requested
registry digest. The caller must still select the current consensus suite and
run artifactregistry.Load/proofbridge.NewVerifier to check profile/reference pins,
exact layout and linked backend compatibility. A digest read from the file itself
cannot grant approval. This provider generates no registry or preprocessing data.

Filesystem, configuration, framing and integrity failures satisfy
`errors.Is(err, artifactstore.ErrUnavailable)`. They describe local artifact
availability, never invalid-proof or bad-peer evidence. Consensus-approved local
artifacts must be provisioned before an enabled node can verify their proofs;
this package does not download files, access peers or write ledger state.

## Ledger and node configuration

Tagged ledgers hold an atomic provider snapshot. Their setter/getter change only
local byte availability; they cannot replace verifier checks or approve suites.
Default builds retain empty state and always return nil. Runtime provider types
are excluded from msgp generation and never enter ledger state serialization.

OpenLedger configures the directory from DirsAndPrefix.RootGenesisDir before
tracker replay, because replay can evaluate historical native transactions.
MakeFull reasserts the same RootGenesisDir/native-privacy/registries path after
loading the ledger. The artifact path stays under the root genesis directory even
when hot/cold/tracker databases are relocated. Neither hook creates or opens it.
Legacy string-only ledger prefixes provide no RootGenesisDir, so they install no
automatic provider; native replay callers must use a resolved DirsAndPrefix root.
No global provider is shared between ledgers or nodes.

Only builds, vet and source checks were performed. No Get/read execution,
file fixtures, digest vectors, tests, setup, proofs or node execution ran.
