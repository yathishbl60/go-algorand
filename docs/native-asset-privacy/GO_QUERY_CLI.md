# Candidate goal privacy queries

Updated 2026-11-01. The feature-tagged `goal asset privacy` command group exposes
six read-only queries through [the typed Go client](GO_QUERY_CLIENT.md). All
consensus versions remain disabled and suite catalogs empty. These commands do
not activate the protocol or supply wallet transfers, local proofs or node demos.
Source integration and build/static checks are the available evidence.

## Build and configure

From the repository root:

```sh
go build -mod=readonly -tags algorand_native_privacy -o /tmp/goal-privacy ./cmd/goal
```

The command group is absent from a default build. Select one existing algod data
directory with `-d` or `ALGORAND_DATA`; queries use its configured algod URL and
API token. No KMD request, wallet selection or signing operation is required.
Keep credentials in the node configuration rather than command arguments.

## Commands and arguments

Every query requires positive `--assetid`. Optional `--round` selects one exact
round, including an explicitly supplied `--round 0`; omission asks for latest.
`--timeout` accepts a positive Go duration and defaults to `30s`.

| Command after `goal asset privacy` | Additional arguments | Result |
| --- | --- | --- |
| `snapshot` | None | Lifecycle, policy and supply |
| `authorization` | None | Current authorities, assigned views and roots |
| `outputs` | `--start`, `--limit`, `--next` | Bounded encrypted-output page |
| `witness` | Required `--index` as uint32 | Public note membership path |
| `key` | Required `--account`, `--role` | Public registration, assignment and path |
| `eligibility` | Required `--account` | Effective freeze and eligibility/path |

Accounts must be nonzero checksummed Algorand addresses. Key roles are
`recipient-view`, `recipient-spend`, `auditor-view`, `controller-view`,
`freeze-authority`, `clawback-authority` and `manager-authority`.

Examples below illustrate invocation only; they have not been executed. Replace
`123`, `456`, `789` and `$ACCOUNT` with your asset, round, note index and address:

```sh
/tmp/goal-privacy -d "$ALGORAND_DATA" asset privacy snapshot --assetid 123
/tmp/goal-privacy -d "$ALGORAND_DATA" asset privacy authorization --assetid 123 --round 456
/tmp/goal-privacy -d "$ALGORAND_DATA" asset privacy witness --assetid 123 --index 789 --round 456
/tmp/goal-privacy -d "$ALGORAND_DATA" asset privacy key --assetid 123 --account "$ACCOUNT" --role recipient-view --round 456
/tmp/goal-privacy -d "$ALGORAND_DATA" asset privacy eligibility --assetid 123 --account "$ACCOUNT" --round 456
```

A successful invocation writes one JSON object followed by a newline to stdout.
An unsuccessful query exits nonzero. Preserve the returned round when composing
subsequent requests; independently requesting latest can mix ledger snapshots.
No automatic retries, transaction submissions or latest-round fallbacks occur.

## Bounded output pagination

`outputs --start` defaults to 0 and accepts values up to 2^32. `--limit` defaults
to 32, accepts 0 as 32, and is capped at 64. One invocation requests one page.
A nonterminal page contains `next-token`; a terminal page omits that field.

```sh
/tmp/goal-privacy -d "$ALGORAND_DATA" asset privacy outputs --assetid 123 --round 456 --start 0 --limit 32
/tmp/goal-privacy -d "$ALGORAND_DATA" asset privacy outputs --assetid 123 --next "$NEXT_TOKEN" --limit 64
```

A nonempty `--next` cannot combine with any explicitly supplied `--round` or
`--start`, including `--start 0`. The cursor pins asset, round, policy, root,
count and position; the client checks the returned page against those fields.
An expired/pruned cursor fails instead of silently switching snapshots. Restart
at an available round and deduplicate deliberately when building a scanner.

## Interpretation and remaining work

The client bounds response bodies/JSON structures and checks request scope,
page continuity, framing and selected record consistency. It does not
cryptographically verify Merkle paths or prove consensus finality. Responses
come from the configured RPC source; see [REST errors](REST_QUERIES.md).

A registered key can remain present after its issuer assignment changes.
Membership alone does not confer current issuer authority. Ordinary eligibility
also requires `frozen=false`; native execution must recheck current roots after
preceding group changes. Query output contains public records, ciphertext and
paths, not private openings or wallet secrets.

Local proving, scanning/receipts, backups, transfer commands, standalone SDK
integration and all six real-node scenarios remain in
[the delivery checklist](END_TO_END_STATUS.md). Tests, HTTP/runtime queries,
signatures, proof runs and demos remain deferred.
