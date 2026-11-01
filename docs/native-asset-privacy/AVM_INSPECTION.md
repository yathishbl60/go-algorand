# Candidate AVM privacy inspection

Updated 2026-11-01. Public transaction inspection is source-integrated in
[data/transactions/logic](../../data/transactions/logic/assetprivacy_fields_enabled.go).
It requires the `algorand_native_privacy` build tag, AVM version 14 or later, and
`NativeAssetPrivacyExecutionConfigured()` for the executing consensus version.
Every current consensus version remains disabled; this is no activation or
runtime demonstration. Tests, vectors, TEAL execution and proof runs are deferred.

## Appended selectors and values

Selectors 69–82 append to the existing transaction field enum. Names, enum slots
and the `aprv` TypeEnum value 9 are reserved in both builds. The assembler accepts
`NativeAssetPrivacy` as the corresponding type alias. Default builds assign these
field specifications the maximum uint64 version and reject the runtime helper.

| Index | Selector | Public value |
| --- | --- | --- |
| 69 | `NativePrivacyFamily` | 0 ordinary, 1 NAPT value, 2 NAPL lifecycle |
| 70 | `NativePrivacyAsset` | Body asset ID |
| 71 | `NativePrivacyMode` | NAPT mode, or NAPL adoption/creation allowed-mode mask |
| 72 | `NativePrivacyOperation` | Operation number within the body family |
| 73 | `NativePrivacyDebit` | Public units debited into private supply |
| 74 | `NativePrivacyCredit` | Public units credited from private supply |
| 75 | `NativePrivacyDebitSource` | 32-byte public debit source |
| 76 | `NativePrivacyCreditReceiver` | 32-byte public credit receiver |
| 77 | `NativePrivacyPolicy` | 64-byte policy identity |
| 78 | `NativePrivacySuite` | 64-byte suite identity |
| 79 | `NativePrivacyNumInputs` | NAPT private input count |
| 80 | `NativePrivacyNumOutputs` | NAPT private output count |
| 81 | `NativePrivacyInputNullifiers` | Array of 64-byte spent markers |
| 82 | `NativePrivacyOutputCommitments` | Array of 64-byte note commitments |

An ordinary transaction has family 0, zero numeric values, 32 zero address bytes
and 64 zero policy/suite bytes. Its input/output counts are 0, so an array read
fails. Each array index must be below the corresponding count; invalid indices
fail instead of returning an empty or zero element. Body arities remain bounded.

NAPL has no NAPT inputs, outputs or conversion amounts. Its mode value is the
allowed-mode mask on adoption/creation and 0 for other lifecycle operations.
Operation numbers are family-specific; inspect family before interpreting them.
Native ASA creation uses an ordinary `acfg` carrying Create5 NAPL metadata:
family 2. `NativePrivacyAsset` reads body asset ID 0, not the allocated ID. Read
`CreatedAssetID` when ordinary AVM effect-access rules permit for that allocated ID.

## Canonical inspection boundary

[nativebody.Inspect](../../internal/assetprivacy/nativebody/inspect.go) validates
the complete canonical NAPT body through preflight before exposing a fixed-size
public summary. It does not copy ciphertext envelopes. Lifecycle inspection uses
its canonical decoder and checks body/type placement. Malformed native bodies
fail; a body on an incompatible transaction type does not become ordinary data.

The selectors expose no private opening, owner/viewing secret, encrypted envelope
or proof bytes. Transaction inspection does not perform ledger authorization,
verify STARK proofs, establish spendability or prove consensus finality. A field
value alone gives no assurance that native execution will accept the transaction.
Atomic-group native validation must still check all ordinary and native rules.

All 14 selectors have `itxVersion=0`; none can be set through `itxn_field`.
The inner-transaction type allowlist is unchanged. These additions do not permit
creating native privacy operations as inner transactions.

## Example metadata guard

This unexecuted TEAL fragment checks only public metadata on group transaction 0.
It is not a complete contract or swap proof. `123` is an illustrative asset ID;
array index 0 requires at least one output and otherwise fails.

```teal
#pragma version 14
gtxn 0 NativePrivacyFamily
int 1
==
assert
gtxn 0 NativePrivacyAsset
int 123
==
assert
gtxna 0 NativePrivacyOutputCommitments 0
len
int 64
==
return
```

Use exact commitments and the protocol's group-binding rules when designing a
real contract. Build/static checks alone establish neither secure composition nor
runtime acceptance. Full SDK/wallet delivery, calibrated resources and six
real-node scenarios remain in [the delivery checklist](END_TO_END_STATUS.md).
