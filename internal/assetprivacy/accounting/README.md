# Native private-supply accounting

This package implements the public aggregate arithmetic used by the native
ledger integration. All Go files require `algorand_native_privacy`. No private
note values, witnesses or keys enter these APIs.

## Rules

`Supply.Total` comes from the existing ASA parameters. `Supply.Private` is the
public amount currently represented by private notes, not one holder's balance.
Both are uint64 amounts in indivisible ASA units.

| Operation | Preconditions | Result |
| --- | --- | --- |
| `Validate` | Private does not exceed total | Error for an inconsistent starting state |
| `Shield(amount)` | Amount does not exceed total minus private | Increase private by amount |
| `Unshield(amount)` | Amount does not exceed private | Decrease private by amount |
| `CheckDestruction(creatorHolding)` | Private is zero and creator holds total | Permit the accounting condition only |

The methods operate on copies. On error they return the original supply and
never change caller state. Bounds are checked before addition/subtraction:
shield cannot overflow uint64, and unshield cannot underflow. Zero movements
are valid arithmetic; transaction-level zero-value rules remain to be specified.
The difference `Total - Private` describes aggregate public value, not the
sender's spendable holding.

A shield still needs an authorized public debit and a valid proof of equal
private output value. An unshield still needs valid note consumption and an equal
public credit. Perform those operations atomically with this accounting change.
Private-to-private transfers leave the aggregate unchanged.

## Development storage layout

The native COW adapter uses the following provisional, inactive format:

- Key: ASCII `aprv:s:` followed by an eight-byte big-endian, nonzero asset ID.
- Value: one version byte equal to `1`, followed by an eight-byte big-endian
  unsigned private-supply amount.
- No trailing data, alternate lengths or unknown versions are accepted.
- A missing key and an explicit zero record are different states.
- The total supply is never duplicated in the record; obtain it from ASA state.

`SupplyKey`, `EncodePrivateSupply` and `DecodePrivateSupply` define this layout.
Decoded values must still be checked against the current asset's total.
The key prefix differs from AVM boxes' `bx:` prefix, so box-name selection cannot
address a private-supply record. Freeze the layout in NAP-02 before activation.
This format is not a transaction encoding or an SDK schema.

## Integration and current limits

See [native integration](../../../docs/native-asset-privacy/NATIVE_ACCOUNTING.md)
for the consensus gate and typed COW accessors. The adapters feed ordinary KV
deltas instead of maintaining an external balance store. Proof verification,
actual balance movement, storage charges, asset destruction, transaction dispatch
and complete recovery validation remain outstanding.

Build and static analysis:

```sh
go build -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/accounting
go vet -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/accounting
```

Tests are deferred. Required later cases include zero/max uint64 boundaries,
inconsistent starting supply, failed conversions leaving state unchanged,
canonical decoding failures, zero asset IDs, missing versus zero records, group
rollback, backend persistence, restart and catchpoint recovery. These are future
acceptance cases, not claims that they have been executed.
