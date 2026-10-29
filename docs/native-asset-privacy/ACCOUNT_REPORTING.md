# Native storage in account minimum-balance reporting

Updated 2026-10-29. Full and basic JSON account-information responses now include
native storage obligations in their existing `min-balance`. This is disabled
source integration; no native account response has been exercised at runtime.

## Reading the response

Use the returned `round` together with `min-balance` and the account's
reward-adjusted `amount`. The response adds the ordinary account requirement and:

`Items*NativeAssetPrivacyFlatMinBalance + Bytes*NativeAssetPrivacyByteMinBalance`

The counter includes every complete native key/value footprint attributed to the
creator, including its own permanent 62-byte record. See [storage](NATIVE_STORAGE.md).
The helper uses checked arithmetic and the canonical counter decoder.

Full responses first load the resource-bearing account; basic responses load its
aggregate data. Both obtain consensus parameters for the account's returned round
and read the obligation with `LookupKv` at that same round. They never substitute
a newer counter or different protocol price when the requested round is missing.
A pruned/unavailable round, malformed counter or arithmetic error fails the
request. A missing counter contributes zero; a zero address skips the lookup.

These values describe that ledger snapshot, not a reservation for a future
transaction. Pending transactions, later protocol prices, additional native
writes and ordinary fees can change the required funding. A grouped payment to
the creator must precede the write that consumes its newly funded capacity.

## Format and component boundaries

The behavior requires `algorand_native_privacy` and the snapshot's enabled
consensus flag. Default builds and disabled protocols retain the ordinary value
without native KV lookup. Every current protocol keeps the flag false and prices
zero. No schema field, endpoint or new consensus activation was added here.

Binary account responses still return raw account records. Those records do not
contain the separate KV obligation, and their ordinary `MinBalance` methods
cannot discover it. The pure `AccountDataToAccount` converter also has no ledger
reader and produces the ordinary value; the HTTP handlers add the native value
after conversion. Consumers of raw records or the pure converter must explicitly
join the canonical counter and prices at the same round before using it as a
native funding quote. This does not change indexer or SDK behavior.

## Verification and remaining work

Default and privacy-tagged API builds and vet passed. Authored limits and source
checks passed. No tests, mock requests, account/node execution, funding scenarios
or proof runs occurred. REST output streams, native witness queries, lifecycle
registration, simulation contracts, SDK estimators and wallet support remain
pending in the [delivery status](END_TO_END_STATUS.md).
