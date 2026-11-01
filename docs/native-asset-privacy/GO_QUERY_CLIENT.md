# Candidate Go privacy query client

Updated 2026-11-01. The node repository now exposes NativePrivacyClient through
[the REST client package](../../daemon/algod/api/client/assetprivacy_types.go) and
libgoal. These are typed, read-only calls for the six
[REST endpoints](REST_QUERIES.md), not a complete wallet or standalone language SDK.
All consensus flags remain false and suite catalogs empty. No HTTP query or node
demo has run; this is source integration with build/static evidence only.

## Construct and pin requests

Build callers with algorand_native_privacy. Starting with an already configured
client.RestClient, use rest.NativePrivacyQueries(nil). Starting with libgoal.Client,
use goalClient.NativePrivacyQueries(nil); it obtains that client's algod URL/token.
Both return (*client.NativePrivacyClient,error). A supplied *http.Client is cloned;
its Transport/Jar and positive Timeout are retained. Default timeout is30 seconds.
Redirects are always blocked. Each method accepts context.Context for cancellation.
Endpoint URLs must use HTTP(S), contain a host and omit userinfo/query/fragment.
Any configured base path is preserved by URL path joining.

Within your caller, with configured rest, asset and ctx:

```go
queries, err := rest.NativePrivacyQueries(nil)
if err != nil { return err }
state, err := queries.Snapshot(ctx, asset, nil)
if err != nil { return err }
round := basics.Round(state.Round)
authorities, err := queries.Authorization(ctx, asset, &round)
if err != nil { return err }
```

A nil round selects the server's latest round. A nonnil pointer preserves the
exact requested round, including0. Keep arguments immutable for the call. Results
use generated NativeAssetPrivacy* models with owned decoded buffers.

| Method | Inputs after ctx | Result |
| --- | --- | --- |
| Snapshot | asset, round | Lifecycle/policy/supply state |
| Authorization | asset, round | Authorities, assigned views and separate current roots |
| Outputs | asset, NativePrivacyOutputOptions | Bounded ciphertext page |
| NoteWitness | asset, indexuint32, round | Public note membership path |
| Key | asset, account, NativePrivacyRole, round | Current public registration, assignment flags and path |
| Eligibility | asset, account, round | Effective freeze and indexed eligibility/path |

Roles are NativePrivacyRecipientView, RecipientSpend, AuditorView, ControllerView,
FreezeAuthority, ClawbackAuthority and ManagerAuthority, each with the same
NativePrivacy prefix. Zero accounts, invalid roles/assets and invalid page options
reject locally before HTTP. Credentials use X-Algo-API-Token; calls send no wallet
key, owner secret, private note opening or proof.

## Scan one snapshot

```go
page, err := queries.Outputs(ctx, asset, client.NativePrivacyOutputOptions{
    Round: &round, Limit: 32,
})
if err != nil { return err }
if page.NextToken != nil {
    page, err = queries.Outputs(ctx, asset, client.NativePrivacyOutputOptions{
        Next: *page.NextToken, Limit: 64,
    })
}
```

Limit0 defaults to32; the maximum is64. Start is at most2^32. Next cannot combine
with Round or nonzero Start. Cursor asset, round, policy, root, count and next
must match both the preceding snapshot and the returned page. Records must be
contiguous and fit the returned count; a terminal page has no next-token.
No retry loop or latest-round fallback is added. On410, callers can restart at an
available round and deduplicate; failed calls return a zero response and no advance.

## Response and failure limits

Success bodies are limited to1 MiB after HTTP decompression. JSON preflight
requires one object, at most8192 tokens, depth16, and64 entries per array/object.
Duplicate case-equivalent field names, multiple JSON values, malformed JSON and
oversized structures reject before generated-model materialization. Unknown
fields within these bounds are tolerated for compatible server extensions.
The client checks requested asset/account/role/round, response shapes, path/root
counts, lifecycle consistency, and complete page/cursor/envelope framing.

Non200 responses return the existing client.HTTPError with StatusCode; server-
controlled error bodies and request URLs are not included. Transport failures
return NativePrivacyTransportError; errors.Is can still identify context
cancellation/deadline causes. ErrNativePrivacyRequest identifies local invalid
arguments/configuration; ErrNativePrivacyResponse identifies invalid response data.
Client string formatting omits credentials. No signing/submission fallback exists.

These checks do not cryptographically verify Merkle paths or establish consensus
finality. Data comes from the configured RPC source. Registration membership is
not live issuer authority; member=true still requires frozen=false for ordinary
eligibility. Returned roots must be rechecked for native execution after group
changes. Full local proving, wallet backups/scanning, CLI transfer support,
standalone SDK/indexer updates and all six demos remain in
[the delivery checklist](END_TO_END_STATUS.md). Tests, HTTP execution, vectors,
redirect/cancellation cases, signatures and proof runs remain deferred.

[Local historical-key scanning](LOCAL_SCANNING.md) composes this client; it never uploads viewing keys or openings.
