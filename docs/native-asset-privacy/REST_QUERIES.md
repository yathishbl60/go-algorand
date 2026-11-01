# Candidate native privacy REST queries

Updated 2026-11-01. Six read-only routes expose the
[pinned ledger queries](LEDGER_QUERIES.md) through ordinary API-token middleware.
All consensus feature flags remain false and suite catalogs empty. No request,
proof, test or node demonstration has run. Source/build checks are not end-to-end
validation, and these endpoints do not activate a protocol feature.

## Requests

| Method and path | Query parameters |
| --- | --- |
| GET /v2/assets/{asset-id}/privacy | round optional |
| GET /v2/assets/{asset-id}/privacy/outputs | round, start, limit, next optional |
| GET /v2/assets/{asset-id}/privacy/notes/{note-index}/witness | round optional |
| GET /v2/assets/{asset-id}/privacy/authorization | round optional |
| GET /v2/assets/{asset-id}/privacy/accounts/{address}/keys/{role} | round optional |
| GET /v2/assets/{asset-id}/privacy/accounts/{address}/eligibility | round optional |

Use the existing X-Algo-API-Token authentication. The routes are registered only
in algorand_native_privacy builds. In those builds, the ledger additionally checks
the selected round's consensus feature gate and suite catalog. Default builds
register no routes. A node's build or an HTTP request cannot approve a suite.

Asset IDs must be positive uint64 decimal values. Explicit round must be uint64
and no greater than the node's selected latest round; absence selects latest at
request selection. Note index is0..2^32-1. Output start defaults to0 and is at most
2^32; the selected stream must also contain that index or its terminal count.
Limit defaults to32 and must be1..64. Unknown or duplicate query fields, empty
numeric fields, negative/overflowing numbers and invalid limits reject.

For continuation, send next from the preceding next-token with the same path
asset. It cannot be combined with round or start; limit may change. The exact
215-character public cursor fixes round/asset/policy/root/count/next. Every field
is rechecked against the ledger. It grants no authority or permanent retention.
The terminal response omits next-token. Output arrays are present, including an
empty terminal array. No call silently advances a pruned round to latest.

## JSON responses

The snapshot reports selected round/asset, existence, creator/current manager,
fixed total, aggregate private supply and creator holding when present. Optional
policy includes immutable identity/suite/creation, modes, current/initial capability
bits and disclosure bits. Optional retirement/closure reports cleanup authority,
epoch and bound context. Inventory reports logical data items/bytes, excluding
index overhead; use account min-balance for actual total storage obligations.
An ordinary ASA returns a snapshot without a native policy. A retained destroyed
ASA returns exists=false plus its closure; fully cleaned assets return404.

The output page reports round, asset, policy, root, count, next, optional next-token
and at most64 encrypted outputs. Each output includes its global index, inclusion
round/transaction, ordinal, nonce, commitment, disclosure flags and required
envelopes. No private note opening, private amount, owner secret or viewing epoch
is returned. Hashes, transaction ID bytes and envelopes use standard padded base64;
account addresses use checksummed Algorand address strings. The cursor uses
canonical unpadded base64url, a separate encoding.

The witness reports selected domain/round, root/count/index/leaf and32 bottom-up
siblings. This is a public membership path, not ownership, current eligibility or
an assurance that its root remains accepted when a future transaction executes.
Retired/closed streams reject: cleanup can remove historical ciphertexts and paths.
Wallets still need encrypted backups and retained history for restoration.

## Error contract

All handled failures use the existing ErrorResponse message shape; internal
storage details are logged and are not copied into the public message.

| Status | Meaning |
| --- | --- |
| 400 | Invalid syntax, range, page parameters, cursor or conflicting query fields |
| 401 | Existing authentication middleware rejected the API token |
| 404 | Disabled feature, missing asset/native policy or absent note index |
| 410 | Pinned state/header pruned or output stream retired/closed |
| 500 | Malformed, incompatible or inconsistent native stored state |
| 503 | Query-capable ledger or local data temporarily unavailable |

Any mid-query retention/database failure discards the whole result. Clients may
restart at an available round and deduplicate outputs; they must not combine
partial data from failed responses with another round's witness or policy state.

## Schema and delivery

The [Swagger2 source](../../daemon/algod/api/algod.oas2.json) and
[OpenAPI3 source](../../daemon/algod/api/algod.oas3.yml) contain the route/JSON schemas.
Pinned oapi-codegen v2.4.1 generates models and embedded specs. The native-privacy
tag is excluded from generated route packages; small handwritten tagged handlers
and a default no-op registration bridge keep runtime availability feature-gated.
The main router passes exactly the existing public middleware to this bridge.

[Authorization/key/eligibility HTTP contracts](AUTHORIZATION_QUERIES.md) also exist. Transaction decoding/simulation, SDK/wallet callers,
rate/resource calibration, validation and all six demos remain in
[the delivery checklist](END_TO_END_STATUS.md). Tests, HTTP execution, cursor
vectors, race/retention cases and proof execution remain deferred.
