# Native creator storage obligation

This package is available only under `algorand_native_privacy`. It implements
namespace attribution and checked public storage accounting, not signed authority,
proof approval, funding transfers, ledger mutation or protocol activation.

`Asset(key)` recognizes the current native policy, supply, marker, disclosure,
output, tree, note-creation, key-history and control-history namespaces. The
key-authorization and eligibility namespaces contain wrapped tree keys or their
exact fixed-size stable-index keys. Known key
shapes and positive asset IDs are checked before attribution. Unknown or malformed
`aprv:` keys fail with ErrNamespace; unrelated keys return native=false. Creator
meter keys are excluded so ordinary writes cannot recursively alter obligations.
Domain/hash contents are not cryptographically authenticated by this helper.

`Key(payer)` is ASCII `aprv:storage:` plus the nonzero32-byte creator address.
The record is exactly17 bytes: version1, Itemsu64BE and Bytesu64BE. Unknown
versions, wrong lengths and impossible minimum counters fail. `New(payer)` starts
with one item and62 bytes for its own45-byte key plus17-byte value. The own
counter remains charged even when the last foreign entry is removed.

`Obligation.Change(keyBytes,beforeBytes,existed,afterBytes,present)` replaces the
entire old key/value footprint with the new one. Inserts/deletes alter item count;
overwrites preserve it and charge exact byte growth/shrinkage. Absent legs ignore
those value lengths. A present entry requires a nonempty key. Every addition,
subtraction and multiplication is checked; corrupt or overflowing counters fail.
No caller-owned data buffers are retained.

`Required(flat,perByte)` requires both consensus prices to be nonzero and computes
Items*flat + Bytes*perByte without overflow. Rates, actual creator selection,
old-value provenance, zero/default feature gating and account minimum-balance
checks belong to native ledger integration. A well-formed counter does not prove
that every writer was metered or that an account can afford its obligation.

Charge every native mutation, including incidental holding/key/eligibility changes,
in the same child COW as its actual KV write. The account obligation must prevent
withdrawal and closeout while state persists; deletion releases only the removed
footprint, never a promise to refund a hidden owner or third-party contributor.
Full cleanup, API/AVM reporting and runtime evidence remain separate obligations.

Build, vet and authored source limits are permitted checks. No tests, counter
vectors, runtime package calls, proofs or node execution have run for this package.
No consensus version is enabled or priced by this package.
